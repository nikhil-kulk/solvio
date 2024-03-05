use std::fmt::Debug;

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::solution::Solution;

/// Type of the issue code
pub type CodeType = String;

pub trait Issue {
    fn code(&self) -> CodeType;
    fn description(&self) -> String;
    fn solution(&self) -> Solution;
}

/// An issue that can be identified by its code
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct IssueRecord {
    pub code: CodeType,
    pub description: String,
    pub solution: Solution,
    pub timestamp: DateTime<Utc>,
}

impl<I: Issue> From<I> for IssueRecord {
    fn from(val: I) -> Self {
        Self {
            code: val.code(),
            description: val.description(),
            solution: val.solution(),
            timestamp: Utc::now(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct DummyIssue {
    pub code: String,
}

impl DummyIssue {
    #[cfg(test)]
    pub fn new(code: impl Into<String>) -> Self {
        Self { code: code.into() }
    }
}

impl Issue for DummyIssue {
    fn code(&self) -> CodeType {
        self.code.clone()
    }

    fn description(&self) -> String {
        "".to_string()
    }

    fn solution(&self) -> Solution {
        Solution::None
    }
}
