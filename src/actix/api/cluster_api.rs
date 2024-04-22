use std::future::Future;

use actix_web::{delete, get, post, web, HttpResponse};
use actix_web_validator::Query;
use rbac::jwt::Claims;
use serde::Deserialize;
use storage::content_manager::claims::check_manage_rights;
use storage::content_manager::consensus_ops::ConsensusOperations;
use storage::content_manager::errors::StorageError;
use storage::content_manager::toc::TableOfContent;
use storage::dispatcher::Dispatcher;
use validator::Validate;

use crate::actix::auth::Extension;
use crate::actix::helpers;

#[derive(Debug, Deserialize, Validate)]
struct QueryParams {
    #[serde(default)]
    force: bool,
    #[serde(default)]
    #[validate(range(min = 1))]
    timeout: Option<u64>,
}

#[get("/cluster")]
fn cluster_status(
    dispatcher: web::Data<Dispatcher>,
    claims: Extension<Claims>,
) -> impl Future<Output = HttpResponse> {
    helpers::time(async move {
        check_manage_rights(claims.into_inner().as_ref())?;
        Ok(dispatcher.cluster_status())
    })
}

#[post("/cluster/recover")]
fn recover_current_peer(
    toc: web::Data<TableOfContent>,
    claims: Extension<Claims>,
) -> impl Future<Output = HttpResponse> {
    helpers::time(async move {
        check_manage_rights(claims.into_inner().as_ref())?;
        toc.request_snapshot()?;
        Ok(true)
    })
}

#[delete("/cluster/peer/{peer_id}")]
fn remove_peer(
    dispatcher: web::Data<Dispatcher>,
    peer_id: web::Path<u64>,
    Query(params): Query<QueryParams>,
    claims: Extension<Claims>,
) -> impl Future<Output = HttpResponse> {
    helpers::time(async move {
        check_manage_rights(claims.into_inner().as_ref())?;

        let dispatcher = dispatcher.into_inner();
        let peer_id = peer_id.into_inner();

        let has_shards = dispatcher.peer_has_shards(peer_id).await;
        if !params.force && has_shards {
            return Err(StorageError::BadRequest {
                description: format!("Cannot remove peer {peer_id} as there are shards on it"),
            });
        }

        match dispatcher.consensus_state() {
            Some(consensus_state) => {
                consensus_state
                    .propose_consensus_op_with_await(
                        ConsensusOperations::RemovePeer(peer_id),
                        params.timeout.map(std::time::Duration::from_secs),
                    )
                    .await
            }
            None => Err(StorageError::BadRequest {
                description: "Distributed mode disabled.".to_string(),
            }),
        }
    })
}

// Configure services
pub fn config_cluster_api(cfg: &mut web::ServiceConfig) {
    cfg.service(cluster_status)
        .service(remove_peer)
        .service(recover_current_peer);
}
