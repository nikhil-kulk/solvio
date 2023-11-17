use std::path::{Path, PathBuf};

use io::file_operations::{atomic_save_json, read_json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::anonymize::Anonymize;
use crate::common::operation_error::OperationResult;

pub const SPARSE_INDEX_CONFIG_FILE: &str = "sparse_index_config.json";

#[derive(Debug, Deserialize, Serialize, JsonSchema, Copy, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct SparseIndexConfig {
    /// We prefer a full scan search upto (excluding) this number of vectors.
    ///
    /// Note: this is number of vectors, not KiloBytes.
    pub full_scan_threshold: usize,
    /// Store index on disk. If set to false, the index will be stored in RAM. Default: false
    pub on_disk: Option<bool>,
}

impl Anonymize for SparseIndexConfig {
    fn anonymize(&self) -> Self {
        SparseIndexConfig {
            full_scan_threshold: self.full_scan_threshold,
            on_disk: self.on_disk,
        }
    }
}

impl SparseIndexConfig {
    pub fn new(full_scan_threshold: usize, on_disk: Option<bool>) -> Self {
        SparseIndexConfig {
            full_scan_threshold,
            on_disk,
        }
    }

    pub fn get_config_path(path: &Path) -> PathBuf {
        path.join(SPARSE_INDEX_CONFIG_FILE)
    }

    pub fn load(path: &Path) -> OperationResult<Self> {
        Ok(read_json(path)?)
    }

    pub fn save(&self, path: &Path) -> OperationResult<()> {
        Ok(atomic_save_json(path, self)?)
    }
}
