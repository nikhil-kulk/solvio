use std::fmt::Debug;

use schemars::JsonSchema;
use segment::common::anonymize::Anonymize;
use serde;
use serde::Serialize;

pub fn get_git_commit_id() -> Option<String> {
    option_env!("GIT_COMMIT_ID")
        .map(ToString::to_string)
        .filter(|s| !s.trim().is_empty())
}

#[derive(Serialize, JsonSchema)]
pub struct VersionInfo {
    pub title: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
}

impl Default for VersionInfo {
    fn default() -> Self {
        VersionInfo {
            title: "solvio - vector search engine".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            commit: get_git_commit_id(),
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ApiStatus {
    Ok,
    Error(String),
    Accepted,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ApiResponse<D> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<D>,
    pub status: ApiStatus,
    pub time: f64,
    #[serde(skip_serializing_if = "is_usage_none_or_empty")]
    pub usage: Option<Usage>,
}

/// Usage of the hardware resources, spent to process the request
#[derive(Debug, Serialize, JsonSchema, Anonymize, Clone)]
#[serde(rename_all = "snake_case")]
#[anonymize(false)]
pub struct Usage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hardware: Option<HardwareUsage>,
}

impl Usage {
    pub fn is_empty(&self) -> bool {
        let Usage { hardware } = self;
        hardware.is_none()
    }
}

fn is_usage_none_or_empty(u: &Option<Usage>) -> bool {
    u.as_ref().is_none_or(|usage| usage.is_empty())
}

/// Usage of the hardware resources, spent to process the request
#[derive(Debug, Serialize, JsonSchema, Anonymize, Clone)]
#[serde(rename_all = "snake_case")]
#[anonymize(false)]
pub struct HardwareUsage {
    pub cpu: usize,
    pub payload_io_read: usize,
    pub payload_io_write: usize,
    pub payload_index_io_read: usize,
    pub payload_index_io_write: usize,
    pub vector_io_read: usize,
    pub vector_io_write: usize,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CollectionDescription {
    pub name: String,
}

fn example_collections_response() -> CollectionsResponse {
    CollectionsResponse {
        collections: vec![
            CollectionDescription {
                name: "arxiv-title".to_string(),
            },
            CollectionDescription {
                name: "arxiv-abstract".to_string(),
            },
            CollectionDescription {
                name: "medium-title".to_string(),
            },
            CollectionDescription {
                name: "medium-text".to_string(),
            },
        ],
    }
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[schemars(example = "example_collections_response")]
pub struct CollectionsResponse {
    pub collections: Vec<CollectionDescription>,
}
