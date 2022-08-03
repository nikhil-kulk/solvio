#[cfg(feature = "web")]
mod actix;
pub mod common;
mod consensus;
mod greeting;
mod settings;
mod snapshots;
mod startup;
mod tonic;

use std::io::Error;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use ::tonic::transport::Uri;
use api::grpc::transport_channel_pool::TransportChannelPool;
use clap::Parser;
use collection::shard::ChannelService;
use consensus::Consensus;
use slog::Drain;
use storage::content_manager::consensus::operation_sender::OperationSender;
use storage::content_manager::consensus::persistent::Persistent;
use storage::content_manager::consensus_state::{ConsensusState, ConsensusStateRef};
use storage::content_manager::toc::TableOfContent;
use storage::dispatcher::Dispatcher;

use crate::common::helpers::create_search_runtime;
use crate::common::telemetry::TelemetryCollector;
use crate::greeting::welcome;
use crate::settings::Settings;
use crate::snapshots::{recover_full_snapshot, recover_snapshots};
use crate::startup::setup_logger;

/// Solvio (read: quadrant ) is a vector similarity search engine.
/// It provides a production-ready service with a convenient API to store, search, and manage points - vectors with an additional payload.
///
/// This CLI starts a Solvio peer/server.
#[derive(Parser, Debug)]
#[clap(version, about)]
struct Args {
    /// Uri of the peer to bootstrap from in case of multi-peer deployment.
    /// If not specified - this peer will be considered as a first in a new deployment.
    #[clap(long, value_parser, value_name = "URI")]
    bootstrap: Option<Uri>,
    /// Uri of this peer.
    /// Other peers should be able to reach it by this uri.
    ///
    /// This value has to be supplied if this is the first peer in a new deployment.
    ///
    /// In case this is not the first peer and it bootstraps the value is optional.
    /// If not supplied then solvio will take internal grpc port from config and derive the IP address of this peer on bootstrap peer (receiving side)
    #[clap(long, value_parser, value_name = "URI")]
    uri: Option<Uri>,

    /// Force snapshot re-creation
    /// If provided - existing collections will be replaced with snapshots.
    /// Default is to not recreate from snapshots.
    #[clap(short, long, action, default_value_t = false)]
    force_snapshot: bool,

    /// List of paths to snapshot files.
    /// Format: <snapshot_file_path>:<target_collection_name>
    #[clap(long, value_name = "PATH:NAME", alias = "collection-snapshot")]
    snapshot: Option<Vec<String>>,

    /// Path to snapshot of multiple collections.
    /// Format: <snapshot_file_path>
    #[clap(long, value_name = "PATH")]
    storage_snapshot: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let settings = Settings::new().expect("Can't read config.");

    setup_logger(&settings.log_level);
    let args = Args::parse();

    if let Some(full_snapshot) = args.storage_snapshot {
        recover_full_snapshot(
            &full_snapshot,
            &settings.storage.storage_path,
            args.force_snapshot,
        );
    } else if let Some(snapshots) = args.snapshot {
        // recover from snapshots
        recover_snapshots(
            &snapshots,
            args.force_snapshot,
            &settings.storage.storage_path,
        );
    }

    welcome();

    // Create and own search runtime out of the scope of async context to ensure correct
    // destruction of it
    let runtime = create_search_runtime(settings.storage.performance.max_search_threads)
        .expect("Can't create runtime.");
    let runtime_handle = runtime.handle().clone();

    // Create a signal sender and receiver. It is used to communicate with the consensus thread.
    let (propose_sender, propose_receiver) = std::sync::mpsc::channel();

    // High-level channel which could be used to send User-space consensus operations
    let propose_operation_sender = OperationSender::new(propose_sender);

    // Saved state of the consensus.
    let persistent_consensus_state =
        Persistent::load_or_init(&settings.storage.storage_path, args.bootstrap.is_none())?;

    // Channel service is used to manage connections between peers.
    // It allocates required number of channels and manages proper reconnection handling
    let mut channel_service = ChannelService::default();

    if settings.cluster.enabled {
        // We only need channel_service in case if cluster is enabled.
        // So we initialize it with real values here
        let p2p_grpc_timeout = Duration::from_millis(settings.cluster.grpc_timeout_ms);
        let connection_timeout = Duration::from_millis(settings.cluster.connection_timeout_ms);
        channel_service.channel_pool = Arc::new(TransportChannelPool::new(
            p2p_grpc_timeout,
            connection_timeout,
            settings.cluster.p2p.connection_pool_size,
        ));
        channel_service.id_to_address = persistent_consensus_state.peer_address_by_id.clone();
    }

    // Table of content manages the list of collections.
    // It is a main entry point for the storage.
    let toc = TableOfContent::new(
        &settings.storage,
        runtime,
        channel_service.clone(),
        persistent_consensus_state.this_peer_id(),
        propose_operation_sender.clone(),
    );

    // Here we load all stored collections.
    runtime_handle.block_on(async {
        for collection in toc.all_collections().await {
            log::debug!("Loaded collection: {}", collection);
        }
    });

    let toc_arc = Arc::new(toc);
    let storage_path = toc_arc.storage_path();

    // Holder for all actively running threads of the service: web, gPRC, consensus, etc.
    let mut handles: Vec<JoinHandle<Result<(), Error>>> = vec![];

    // Router for external queries.
    // It decides if query should go directly to the ToC or through the consensus.
    let mut dispatcher = Dispatcher::new(toc_arc.clone());
    let consensus_state: ConsensusStateRef = ConsensusState::new(
        persistent_consensus_state,
        toc_arc.clone(),
        propose_operation_sender,
        storage_path,
    )
    .into();

    if settings.cluster.enabled {
        dispatcher = dispatcher.with_consensus(consensus_state.clone());
    }
    let dispatcher_arc = Arc::new(dispatcher);

    // Monitoring and telemetry.
    let telemetry_collector = TelemetryCollector::new(settings.clone(), dispatcher_arc.clone());
    let tonic_telemetry_collector = telemetry_collector.tonic_telemetry_collector.clone();

    if settings.cluster.enabled {
        // `raft` crate uses `slog` crate so it is needed to use `slog_stdlog::StdLog` to forward
        // logs from it to `log` crate
        let slog_logger = slog::Logger::root(slog_stdlog::StdLog.fuse(), slog::o!());

        // Runs raft consensus in a separate thread.
        // Create a pipe `message_sender` to communicate with the consensus
        let p2p_port = settings.cluster.p2p.port.expect("P2P port is not set");

        let handle = Consensus::run(
            &slog_logger,
            consensus_state,
            args.bootstrap,
            args.uri.map(|uri| uri.to_string()),
            settings.service.host.clone(),
            p2p_port,
            settings.cluster.consensus.clone(),
            channel_service,
            propose_receiver,
            tonic_telemetry_collector.clone(),
            toc_arc.clone(),
        )
        .expect("Can't initialize consensus");

        handles.push(handle);

        runtime_handle
            .block_on(async {
                toc_arc
                    .cancel_outgoing_all_transfers("Source peer restarted")
                    .await
            })
            .unwrap();
    } else {
        log::info!("Distributed mode disabled");
    }

    #[cfg(feature = "web")]
    {
        let dispatcher_arc = dispatcher_arc.clone();
        let telemetry_collector = Arc::new(tokio::sync::Mutex::new(telemetry_collector));
        let settings = settings.clone();
        let handle = thread::Builder::new()
            .name("web".to_string())
            .spawn(move || actix::init(dispatcher_arc.clone(), telemetry_collector, settings))
            .unwrap();
        handles.push(handle);
    }

    if let Some(grpc_port) = settings.service.grpc_port {
        let settings = settings.clone();
        let handle = thread::Builder::new()
            .name("grpc".to_string())
            .spawn(move || {
                tonic::init(
                    dispatcher_arc,
                    tonic_telemetry_collector,
                    settings.service.host,
                    grpc_port,
                )
            })
            .unwrap();
        handles.push(handle);
    } else {
        log::info!("gRPC endpoint disabled");
    }

    #[cfg(feature = "service_debug")]
    {
        use std::fmt::Write;

        use parking_lot::deadlock;

        const DEADLOCK_CHECK_PERIOD: Duration = Duration::from_secs(10);

        thread::Builder::new()
            .name("deadlock_checker".to_string())
            .spawn(move || loop {
                thread::sleep(DEADLOCK_CHECK_PERIOD);
                let deadlocks = deadlock::check_deadlock();
                if deadlocks.is_empty() {
                    continue;
                }

                let mut error = format!("{} deadlocks detected\n", deadlocks.len());
                for (i, threads) in deadlocks.iter().enumerate() {
                    writeln!(error, "Deadlock #{}", i).expect("fail to writeln!");
                    for t in threads {
                        writeln!(
                            error,
                            "Thread Id {:#?}\n{:#?}",
                            t.thread_id(),
                            t.backtrace()
                        )
                        .expect("fail to writeln!");
                    }
                }
                log::error!("{}", error);
            })
            .unwrap();
    }

    for handle in handles.into_iter() {
        handle.join().expect("Couldn't join on the thread")?;
    }
    drop(toc_arc);
    drop(settings);
    Ok(())
}
