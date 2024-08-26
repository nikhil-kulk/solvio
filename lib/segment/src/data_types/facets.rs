use std::cmp::Reverse;
use std::collections::HashMap;
use std::hash::Hash;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::json_path::JsonPath;
use crate::types::{Filter, ValueVariants};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, Validate)]
pub struct FacetParams {
    pub key: JsonPath,

    #[validate(range(min = 1))]
    pub limit: usize,
    pub filter: Option<Filter>,
    #[serde(default)]
    pub exact: bool,
}

impl FacetParams {
    pub const DEFAULT_LIMIT: usize = 10;
    pub const DEFAULT_EXACT: bool = false;
}

#[derive(Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum FacetValueRef<'a> {
    Keyword(&'a str),
}

impl<'a> FacetValueRef<'a> {
    pub fn to_owned(&self) -> FacetValue {
        match self {
            FacetValueRef::Keyword(s) => FacetValue::Keyword((*s).to_string()),
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
pub enum FacetValue {
    Keyword(String),
    // other types to add?
    // Bool(bool),
    // Integer(IntPayloadType),
    // FloatRange(FloatRange),
}

pub trait FacetValueTrait: Clone + PartialEq + Eq + Hash + Ord {}

impl FacetValueTrait for FacetValue {}
impl FacetValueTrait for FacetValueRef<'_> {}

pub type FacetValueHit = FacetHit<FacetValue>;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FacetHit<T: FacetValueTrait> {
    pub value: T,
    pub count: usize,
}

pub struct FacetResponse {
    pub hits: Vec<FacetValueHit>,
}

impl<T: FacetValueTrait> Ord for FacetHit<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count
            .cmp(&other.count)
            // Reverse so that descending order has ascending values when having the same count
            .then_with(|| Reverse(&self.value).cmp(&Reverse(&other.value)))
    }
}

impl<T: FacetValueTrait> PartialOrd for FacetHit<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn aggregate_facet_hits<T: FacetValueTrait>(
    hits: impl IntoIterator<Item = FacetHit<T>>,
) -> HashMap<T, usize> {
    hits.into_iter()
        .fold(HashMap::new(), |mut map, FacetHit { value, count }| {
            *map.entry(value).or_insert(0) += count;
            map
        })
}

impl From<FacetValue> for ValueVariants {
    fn from(value: FacetValue) -> Self {
        match value {
            FacetValue::Keyword(s) => ValueVariants::String(s),
        }
    }
}
