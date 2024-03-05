use num_cmp::NumCmp;
use ordered_float::OrderedFloat;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::types::{FloatPayloadType, Payload, Range};

const INTERNAL_KEY_OF_ORDER_BY_VALUE: &str = "____ordered_with____";

#[derive(Debug, Deserialize, Serialize, JsonSchema, Copy, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    #[default]
    Asc,
    Desc,
}

impl Direction {
    pub fn as_range(&self, from: FloatPayloadType) -> Range<FloatPayloadType> {
        match self {
            Direction::Asc => Range {
                gte: Some(from),
                ..Default::default()
            },
            Direction::Desc => Range {
                lte: Some(from),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Validate, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct OrderBy {
    /// Payload key to order by
    pub key: String,

    /// Direction of ordering: `asc` or `desc`. Default is ascending.
    pub direction: Option<Direction>,

    /// Which payload value to start scrolling from. Default is the lowest value for `asc` and the highest for `desc`
    pub start_from: Option<FloatPayloadType>,
}

impl OrderBy {
    pub fn as_range(&self) -> Range<FloatPayloadType> {
        match self.start_from {
            Some(start_from) => self.direction.unwrap().as_range(start_from),
            None => Range {
                ..Default::default()
            },
        }
    }

    pub fn direction(&self) -> Direction {
        self.direction.unwrap_or_default()
    }

    pub fn start_from(&self) -> OrderingValue {
        self.start_from
            .unwrap_or_else(|| match self.direction() {
                Direction::Asc => f64::NEG_INFINITY,
                Direction::Desc => f64::INFINITY,
            })
            .into()
    }

    pub fn insert_order_value_in_payload(
        payload: Option<Payload>,
        value: impl Into<serde_json::Value>,
    ) -> Payload {
        let mut new_payload = payload.unwrap_or_default();
        new_payload
            .0
            .insert(INTERNAL_KEY_OF_ORDER_BY_VALUE.to_string(), value.into());
        new_payload
    }

    pub fn remove_order_value_from_payload(&self, payload: Option<&mut Payload>) -> f64 {
        payload
            .and_then(|payload| payload.0.remove(INTERNAL_KEY_OF_ORDER_BY_VALUE))
            .and_then(|v| v.as_f64())
            .unwrap_or_else(|| match self.direction() {
                Direction::Asc => std::f64::MAX,
                Direction::Desc => std::f64::MIN,
            })
    }
}

pub enum OrderingValue {
    Float(FloatPayloadType),
    Int(i64),
}

impl From<OrderingValue> for serde_json::Value {
    fn from(value: OrderingValue) -> Self {
        match value {
            OrderingValue::Float(value) => serde_json::Number::from_f64(value)
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            OrderingValue::Int(value) => serde_json::Value::Number(serde_json::Number::from(value)),
        }
    }
}

impl From<FloatPayloadType> for OrderingValue {
    fn from(value: FloatPayloadType) -> Self {
        OrderingValue::Float(value)
    }
}

impl From<i64> for OrderingValue {
    fn from(value: i64) -> Self {
        OrderingValue::Int(value)
    }
}

impl Eq for OrderingValue {}

impl PartialEq for OrderingValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OrderingValue::Float(a), OrderingValue::Float(b)) => {
                OrderedFloat(*a) == OrderedFloat(*b)
            }
            (OrderingValue::Int(a), OrderingValue::Int(b)) => a == b,
            (OrderingValue::Float(a), OrderingValue::Int(b)) => a.num_eq(*b),
            (OrderingValue::Int(a), OrderingValue::Float(b)) => a.num_eq(*b),
        }
    }
}

impl PartialOrd for OrderingValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderingValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (OrderingValue::Float(a), OrderingValue::Float(b)) => {
                OrderedFloat(*a).cmp(&OrderedFloat(*b))
            }
            (OrderingValue::Int(a), OrderingValue::Int(b)) => a.cmp(b),
            (OrderingValue::Float(a), OrderingValue::Int(b)) => {
                // num_cmp() might return None only if the float value is NaN. We follow the
                // OrderedFloat logic here: the NaN is always greater than any other value.
                a.num_cmp(*b).unwrap_or(std::cmp::Ordering::Greater)
            }
            (OrderingValue::Int(a), OrderingValue::Float(b)) => {
                // Ditto, but the NaN is on the right side of the comparison.
                a.num_cmp(*b).unwrap_or(std::cmp::Ordering::Less)
            }
        }
    }
}
