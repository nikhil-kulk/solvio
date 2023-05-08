use std::hash::Hash;

use segment::types::{PointIdType, ScoredPoint};
use serde_json::Value;
use AggregatorError::BadKeyType;

#[derive(PartialEq, Debug)]
pub enum AggregatorError {
    AllGroupsFull,
    BadKeyType,
    KeyNotFound,
    GroupFull,
    EnoughGroups,
}

/// Abstraction over serde_json::Value to be used as a key in a HashMap/HashSet
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct GroupKey(serde_json::Value);

impl TryFrom<serde_json::Value> for GroupKey {
    type Error = AggregatorError;

    /// Only allows Strings and Numbers to be converted into GroupKey
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            serde_json::Value::String(_) | serde_json::Value::Number(_) => Ok(Self(value)),
            _ => Err(BadKeyType),
        }
    }
}

#[cfg(test)] // TODO: Not sure how "idiomatic" this is... 🫤
impl From<&str> for GroupKey {
    fn from(str: &str) -> Self {
        Self(serde_json::Value::String(str.into()))
    }
}

#[cfg(test)] // TODO: Not sure how "idiomatic" this is... 🫤
impl From<i64> for GroupKey {
    fn from(num: i64) -> Self {
        Self(serde_json::Value::Number(num.into()))
    }
}

impl From<GroupKey> for serde_json::Value {
    fn from(key: GroupKey) -> Self {
        key.0
    }
}

impl Hash for GroupKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match &self.0 {
            Value::Number(n) => n.hash(state),
            Value::String(s) => s.hash(state),
            _ => unreachable!("GroupKey should only be a number or a string"),
        }
    }
}

/// Abstraction over ScoredPoint to be used in a HashSet
#[derive(Eq, Debug, PartialEq, Clone)]
pub struct HashablePoint(ScoredPoint);

impl HashablePoint {
    pub fn minimal_from(point: &ScoredPoint) -> Self {
        Self(ScoredPoint {
            id: point.id,
            version: point.version,
            score: point.score,
            payload: None,
            vector: None,
        })
    }

    pub fn id(&self) -> PointIdType {
        self.0.id
    }

    #[cfg(test)] // TODO: Not sure how "idiomatic" this is... 🫤
    pub fn payload(&self) -> Option<&segment::types::Payload> {
        self.0.payload.as_ref()
    }
}

impl Hash for HashablePoint {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
        self.0.version.hash(state);
    }
}

impl From<ScoredPoint> for HashablePoint {
    fn from(point: ScoredPoint) -> Self {
        Self(point)
    }
}

impl From<HashablePoint> for ScoredPoint {
    fn from(point: HashablePoint) -> Self {
        point.0
    }
}
