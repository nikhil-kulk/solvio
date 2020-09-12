use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    pub storage_path: String,
    pub wal_path: String,
    pub wal_capacity_mb: usize,
    pub wal_segments_ahead: usize,
}


#[derive(Debug, Deserialize, Clone)]
pub struct PerformanceConfig {
    pub max_search_threads: usize,
    pub max_optimize_threads: usize,
}


#[derive(Debug, Deserialize, Clone)]
pub struct OptimizersConfig {
    pub deleted_threshold: f64,
    pub vacuum_min_vector_number: usize,
    pub max_segment_number: usize,
}


#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub host: String,
    pub port: usize,
}


#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub debug: bool,
    pub log_level: String,
    pub storage: StorageConfig,
    pub service: ServiceConfig,
    pub performance: PerformanceConfig,
    pub optimizers: OptimizersConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/config"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or("development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `solvio_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("solvio"))?;

        // Now that we're done, let's access our configuration
        println!("debug: {:?}", s.get_bool("debug"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
