use crate::grpc::models::{CollectionsResponse, VersionInfo};
use crate::grpc::solvio::condition::ConditionOneOf;
use crate::grpc::solvio::point_id::PointIdOptions;
use crate::grpc::solvio::r#match::MatchValue;
use crate::grpc::solvio::with_payload_selector::SelectorOptions;
use crate::grpc::solvio::{
    CollectionDescription, CollectionOperationResponse, Condition, FieldCondition, Filter,
    GeoBoundingBox, GeoPoint, GeoRadius, HasIdCondition, HealthCheckReply, HnswConfigDiff,
    IsEmptyCondition, ListCollectionsResponse, ListValue, Match, PayloadExcludeSelector,
    PayloadIncludeSelector, PayloadSchemaInfo, PayloadSchemaType, PointId, Range, ScoredPoint,
    SearchParams, Struct, Value, ValuesCount, WithPayloadSelector,
};

use crate::grpc::solvio::value::Kind;
use chrono::{NaiveDateTime, Timelike};
use segment::types::{PayloadSelector, WithPayloadInterface};
use std::collections::{HashMap, HashSet};
use std::time::Instant;
use tonic::Status;
use uuid::Uuid;

pub fn payload_to_proto(payload: segment::types::Payload) -> HashMap<String, Value> {
    payload
        .into_iter()
        .map(|(k, v)| (k, json_to_proto(v)))
        .collect()
}

fn json_to_proto(json_value: serde_json::Value) -> Value {
    match json_value {
        serde_json::Value::Null => Value {
            kind: Some(Kind::NullValue(0)),
        },
        serde_json::Value::Bool(v) => Value {
            kind: Some(Kind::BoolValue(v)),
        },
        serde_json::Value::Number(n) => Value {
            kind: if let Some(int) = n.as_i64() {
                Some(Kind::IntegerValue(int))
            } else {
                Some(Kind::DoubleValue(n.as_f64().unwrap()))
            },
        },
        serde_json::Value::String(s) => Value {
            kind: Some(Kind::StringValue(s)),
        },
        serde_json::Value::Array(v) => {
            let list = v.into_iter().map(json_to_proto).collect();
            Value {
                kind: Some(Kind::ListValue(ListValue { values: list })),
            }
        }
        serde_json::Value::Object(m) => {
            let map = m.into_iter().map(|(k, v)| (k, json_to_proto(v))).collect();
            Value {
                kind: Some(Kind::StructValue(Struct { fields: map })),
            }
        }
    }
}

pub fn proto_to_payloads(proto: HashMap<String, Value>) -> Result<segment::types::Payload, Status> {
    let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    for (k, v) in proto.into_iter() {
        map.insert(k, proto_to_json(v)?);
    }
    Ok(map.into())
}

fn proto_to_json(proto: Value) -> Result<serde_json::Value, Status> {
    match proto.kind {
        None => Ok(serde_json::Value::default()),
        Some(kind) => match kind {
            Kind::NullValue(_) => Ok(serde_json::Value::Null),
            Kind::DoubleValue(n) => {
                let v = match serde_json::Number::from_f64(n) {
                    Some(f) => f,
                    None => return Err(Status::invalid_argument("cannot convert to json number")),
                };
                Ok(serde_json::Value::Number(v))
            }
            Kind::IntegerValue(i) => Ok(serde_json::Value::Number(i.into())),
            Kind::StringValue(s) => Ok(serde_json::Value::String(s)),
            Kind::BoolValue(b) => Ok(serde_json::Value::Bool(b)),
            Kind::StructValue(s) => {
                let mut map = serde_json::Map::new();
                for (k, v) in s.fields.into_iter() {
                    map.insert(k, proto_to_json(v)?);
                }
                Ok(serde_json::Value::Object(map))
            }
            Kind::ListValue(l) => {
                let mut list = Vec::new();
                for v in l.values.into_iter() {
                    list.push(proto_to_json(v)?);
                }
                Ok(serde_json::Value::Array(list))
            }
        },
    }
}

impl From<VersionInfo> for HealthCheckReply {
    fn from(info: VersionInfo) -> Self {
        HealthCheckReply {
            title: info.title,
            version: info.version,
        }
    }
}

impl From<(Instant, CollectionsResponse)> for ListCollectionsResponse {
    fn from(value: (Instant, CollectionsResponse)) -> Self {
        let (timing, response) = value;
        let collections = response
            .collections
            .into_iter()
            .map(|desc| CollectionDescription { name: desc.name })
            .collect::<Vec<_>>();
        Self {
            collections,
            time: timing.elapsed().as_secs_f64(),
        }
    }
}

impl From<segment::types::PayloadIndexInfo> for PayloadSchemaInfo {
    fn from(schema: segment::types::PayloadIndexInfo) -> Self {
        PayloadSchemaInfo {
            data_type: match schema.data_type {
                segment::types::PayloadSchemaType::Keyword => PayloadSchemaType::Keyword,
                segment::types::PayloadSchemaType::Integer => PayloadSchemaType::Integer,
                segment::types::PayloadSchemaType::Float => PayloadSchemaType::Float,
                segment::types::PayloadSchemaType::Geo => PayloadSchemaType::Geo,
            }
            .into(),
        }
    }
}

impl TryFrom<PayloadSchemaInfo> for segment::types::PayloadIndexInfo {
    type Error = Status;

    fn try_from(schema: PayloadSchemaInfo) -> Result<Self, Self::Error> {
        match segment::types::PayloadSchemaType::from_index(schema.data_type) {
            None => Err(Status::invalid_argument("No PayloadSelector".to_string())),
            Some(payload_schema_type) => Ok(segment::types::PayloadIndexInfo {
                data_type: payload_schema_type,
            }),
        }
    }
}

impl From<(Instant, bool)> for CollectionOperationResponse {
    fn from(value: (Instant, bool)) -> Self {
        let (timing, result) = value;
        CollectionOperationResponse {
            result,
            time: timing.elapsed().as_secs_f64(),
        }
    }
}

impl From<segment::types::GeoPoint> for GeoPoint {
    fn from(geo: segment::types::GeoPoint) -> Self {
        Self {
            lon: geo.lon,
            lat: geo.lat,
        }
    }
}

impl TryFrom<WithPayloadSelector> for segment::types::WithPayloadInterface {
    type Error = Status;

    fn try_from(value: WithPayloadSelector) -> Result<Self, Self::Error> {
        match value.selector_options {
            Some(options) => Ok(match options {
                SelectorOptions::Enable(flag) => segment::types::WithPayloadInterface::Bool(flag),
                SelectorOptions::Exclude(s) => {
                    segment::types::PayloadSelectorExclude::new(s.fields).into()
                }
                SelectorOptions::Include(s) => {
                    segment::types::PayloadSelectorInclude::new(s.fields).into()
                }
            }),
            _ => Err(Status::invalid_argument("No PayloadSelector".to_string())),
        }
    }
}

impl From<segment::types::WithPayloadInterface> for WithPayloadSelector {
    fn from(value: segment::types::WithPayloadInterface) -> Self {
        let selector_options = match value {
            WithPayloadInterface::Bool(flag) => SelectorOptions::Enable(flag),
            WithPayloadInterface::Fields(fields) => {
                SelectorOptions::Include(PayloadIncludeSelector { fields })
            }
            WithPayloadInterface::Selector(selector) => match selector {
                PayloadSelector::Include(s) => {
                    SelectorOptions::Include(PayloadIncludeSelector { fields: s.include })
                }
                PayloadSelector::Exclude(s) => {
                    SelectorOptions::Exclude(PayloadExcludeSelector { fields: s.exclude })
                }
            },
        };
        WithPayloadSelector {
            selector_options: Some(selector_options),
        }
    }
}

impl From<SearchParams> for segment::types::SearchParams {
    fn from(params: SearchParams) -> Self {
        Self {
            hnsw_ef: params.hnsw_ef.map(|x| x as usize),
        }
    }
}

impl From<segment::types::SearchParams> for SearchParams {
    fn from(params: segment::types::SearchParams) -> Self {
        Self {
            hnsw_ef: params.hnsw_ef.map(|x| x as u64),
        }
    }
}

impl From<segment::types::PointIdType> for PointId {
    fn from(point_id: segment::types::PointIdType) -> Self {
        PointId {
            point_id_options: Some(match point_id {
                segment::types::PointIdType::NumId(num) => PointIdOptions::Num(num),
                segment::types::PointIdType::Uuid(uuid) => PointIdOptions::Uuid(uuid.to_string()),
            }),
        }
    }
}

impl From<segment::types::ScoredPoint> for ScoredPoint {
    fn from(point: segment::types::ScoredPoint) -> Self {
        Self {
            id: Some(point.id.into()),
            payload: point.payload.map(payload_to_proto).unwrap_or_default(),
            score: point.score,
            vector: point.vector.unwrap_or_default(),
            version: point.version,
        }
    }
}

impl TryFrom<ScoredPoint> for segment::types::ScoredPoint {
    type Error = Status;

    fn try_from(point: ScoredPoint) -> Result<Self, Self::Error> {
        Ok(Self {
            id: match point.id {
                None => return Err(Status::invalid_argument("Point does not have an ID")),
                Some(id) => id.try_into()?,
            },
            payload: Some(proto_to_payloads(point.payload)?),
            score: point.score,
            vector: Some(point.vector),
            version: point.version,
        })
    }
}

impl TryFrom<PointId> for segment::types::PointIdType {
    type Error = Status;

    fn try_from(value: PointId) -> Result<Self, Self::Error> {
        match value.point_id_options {
            Some(PointIdOptions::Num(num_id)) => Ok(segment::types::PointIdType::NumId(num_id)),
            Some(PointIdOptions::Uuid(uui_str)) => Uuid::parse_str(&uui_str)
                .map(segment::types::PointIdType::Uuid)
                .map_err(|_err| {
                    Status::invalid_argument(format!("Unable to parse UUID: {}", uui_str))
                }),
            _ => Err(Status::invalid_argument(
                "No ID options provided".to_string(),
            )),
        }
    }
}

fn conditions_helper_from_grpc(
    conditions: Vec<Condition>,
) -> Result<Option<Vec<segment::types::Condition>>, tonic::Status> {
    if conditions.is_empty() {
        Ok(None)
    } else {
        let vec = conditions
            .into_iter()
            .map(|c| c.try_into())
            .collect::<Result<_, _>>()?;
        Ok(Some(vec))
    }
}

fn conditions_helper_to_grpc(conditions: Option<Vec<segment::types::Condition>>) -> Vec<Condition> {
    match conditions {
        None => vec![],
        Some(conditions) => {
            if conditions.is_empty() {
                vec![]
            } else {
                conditions.into_iter().map(|c| c.into()).collect()
            }
        }
    }
}

impl TryFrom<Filter> for segment::types::Filter {
    type Error = Status;

    fn try_from(value: Filter) -> Result<Self, Self::Error> {
        Ok(Self {
            should: conditions_helper_from_grpc(value.should)?,
            must: conditions_helper_from_grpc(value.must)?,
            must_not: conditions_helper_from_grpc(value.must_not)?,
        })
    }
}

impl From<segment::types::Filter> for Filter {
    fn from(value: segment::types::Filter) -> Self {
        Self {
            should: conditions_helper_to_grpc(value.should),
            must: conditions_helper_to_grpc(value.must),
            must_not: conditions_helper_to_grpc(value.must_not),
        }
    }
}

impl TryFrom<Condition> for segment::types::Condition {
    type Error = Status;

    fn try_from(value: Condition) -> Result<Self, Self::Error> {
        if let Some(condition) = value.condition_one_of {
            return match condition {
                ConditionOneOf::Field(field) => {
                    Ok(segment::types::Condition::Field(field.try_into()?))
                }
                ConditionOneOf::HasId(has_id) => {
                    Ok(segment::types::Condition::HasId(has_id.try_into()?))
                }
                ConditionOneOf::Filter(filter) => {
                    Ok(segment::types::Condition::Filter(filter.try_into()?))
                }
                ConditionOneOf::IsEmpty(is_empty) => {
                    Ok(segment::types::Condition::IsEmpty(is_empty.into()))
                }
            };
        }
        Err(Status::invalid_argument("Malformed Condition type"))
    }
}

impl From<segment::types::Condition> for Condition {
    fn from(value: segment::types::Condition) -> Self {
        let condition_one_of = match value {
            segment::types::Condition::Field(field) => ConditionOneOf::Field(field.into()),
            segment::types::Condition::IsEmpty(is_empty) => {
                ConditionOneOf::IsEmpty(is_empty.into())
            }
            segment::types::Condition::HasId(has_id) => ConditionOneOf::HasId(has_id.into()),
            segment::types::Condition::Filter(filter) => ConditionOneOf::Filter(filter.into()),
        };

        Self {
            condition_one_of: Some(condition_one_of),
        }
    }
}

impl From<IsEmptyCondition> for segment::types::IsEmptyCondition {
    fn from(value: IsEmptyCondition) -> Self {
        segment::types::IsEmptyCondition {
            is_empty: segment::types::PayloadField { key: value.key },
        }
    }
}

impl From<segment::types::IsEmptyCondition> for IsEmptyCondition {
    fn from(value: segment::types::IsEmptyCondition) -> Self {
        Self {
            key: value.is_empty.key,
        }
    }
}

impl TryFrom<HasIdCondition> for segment::types::HasIdCondition {
    type Error = Status;

    fn try_from(value: HasIdCondition) -> Result<Self, Self::Error> {
        let set: HashSet<segment::types::PointIdType> = value
            .has_id
            .into_iter()
            .map(|p| p.try_into())
            .collect::<Result<_, _>>()?;
        Ok(Self { has_id: set })
    }
}

impl From<segment::types::HasIdCondition> for HasIdCondition {
    fn from(value: segment::types::HasIdCondition) -> Self {
        let set: Vec<PointId> = value.has_id.into_iter().map(|p| p.into()).collect();
        Self { has_id: set }
    }
}

impl TryFrom<FieldCondition> for segment::types::FieldCondition {
    type Error = Status;

    fn try_from(value: FieldCondition) -> Result<Self, Self::Error> {
        let FieldCondition {
            key,
            r#match,
            range,
            geo_bounding_box,
            geo_radius,
            values_count,
        } = value;

        let geo_bounding_box =
            geo_bounding_box.map_or_else(|| Ok(None), |g| g.try_into().map(Some))?;
        let geo_radius = geo_radius.map_or_else(|| Ok(None), |g| g.try_into().map(Some))?;
        Ok(Self {
            key,
            r#match: r#match.map_or_else(|| Ok(None), |m| m.try_into().map(Some))?,
            range: range.map(|r| r.into()),
            geo_bounding_box,
            geo_radius,
            values_count: values_count.map(|r| r.into()),
        })
    }
}

impl From<segment::types::FieldCondition> for FieldCondition {
    fn from(value: segment::types::FieldCondition) -> Self {
        let segment::types::FieldCondition {
            key,
            r#match,
            range,
            geo_bounding_box,
            geo_radius,
            values_count,
        } = value;

        let geo_bounding_box = geo_bounding_box.map(|g| g.into());
        let geo_radius = geo_radius.map(|g| g.into());
        Self {
            key,
            r#match: r#match.map(|m| m.into()),
            range: range.map(|r| r.into()),
            geo_bounding_box,
            geo_radius,
            values_count: values_count.map(|r| r.into()),
        }
    }
}

impl TryFrom<GeoBoundingBox> for segment::types::GeoBoundingBox {
    type Error = Status;

    fn try_from(value: GeoBoundingBox) -> Result<Self, Self::Error> {
        match value {
            GeoBoundingBox {
                top_left: Some(t),
                bottom_right: Some(b),
            } => Ok(Self {
                top_left: t.into(),
                bottom_right: b.into(),
            }),
            _ => Err(Status::invalid_argument("Malformed GeoBoundingBox type")),
        }
    }
}

impl From<segment::types::GeoBoundingBox> for GeoBoundingBox {
    fn from(value: segment::types::GeoBoundingBox) -> Self {
        Self {
            top_left: Some(value.top_left.into()),
            bottom_right: Some(value.bottom_right.into()),
        }
    }
}

impl TryFrom<GeoRadius> for segment::types::GeoRadius {
    type Error = Status;

    fn try_from(value: GeoRadius) -> Result<Self, Self::Error> {
        match value {
            GeoRadius {
                center: Some(c),
                radius,
            } => Ok(Self {
                center: c.into(),
                radius: radius.into(),
            }),
            _ => Err(Status::invalid_argument("Malformed GeoRadius type")),
        }
    }
}

impl From<segment::types::GeoRadius> for GeoRadius {
    fn from(value: segment::types::GeoRadius) -> Self {
        Self {
            center: Some(value.center.into()),
            radius: value.radius as f32, // TODO lossy ok?
        }
    }
}

impl From<GeoPoint> for segment::types::GeoPoint {
    fn from(value: GeoPoint) -> Self {
        Self {
            lon: value.lon,
            lat: value.lat,
        }
    }
}

impl From<Range> for segment::types::Range {
    fn from(value: Range) -> Self {
        Self {
            lt: value.lt,
            gt: value.gt,
            gte: value.gte,
            lte: value.lte,
        }
    }
}

impl From<segment::types::Range> for Range {
    fn from(value: segment::types::Range) -> Self {
        Self {
            lt: value.lt,
            gt: value.gt,
            gte: value.gte,
            lte: value.lte,
        }
    }
}

impl From<ValuesCount> for segment::types::ValuesCount {
    fn from(value: ValuesCount) -> Self {
        Self {
            lt: value.lt.map(|x| x as usize),
            gt: value.gt.map(|x| x as usize),
            gte: value.gte.map(|x| x as usize),
            lte: value.lte.map(|x| x as usize),
        }
    }
}

impl From<segment::types::ValuesCount> for ValuesCount {
    fn from(value: segment::types::ValuesCount) -> Self {
        Self {
            lt: value.lt.map(|x| x as u64),
            gt: value.gt.map(|x| x as u64),
            gte: value.gte.map(|x| x as u64),
            lte: value.lte.map(|x| x as u64),
        }
    }
}

impl TryFrom<Match> for segment::types::Match {
    type Error = Status;

    fn try_from(value: Match) -> Result<Self, Self::Error> {
        match value.match_value {
            Some(mv) => Ok(match mv {
                MatchValue::Keyword(kw) => kw.into(),
                MatchValue::Integer(int) => int.into(),
                MatchValue::Boolean(flag) => flag.into(),
            }),
            _ => Err(Status::invalid_argument("Malformed Match condition")),
        }
    }
}

#[allow(deprecated)]
impl From<segment::types::Match> for Match {
    fn from(value: segment::types::Match) -> Self {
        let match_value = match value {
            segment::types::Match::Value(value) => match value.value {
                segment::types::ValueVariants::Keyword(kw) => MatchValue::Keyword(kw),
                segment::types::ValueVariants::Integer(int) => MatchValue::Integer(int),
                segment::types::ValueVariants::Bool(flag) => MatchValue::Boolean(flag),
            },
            segment::types::Match::Keyword(kw) => MatchValue::Keyword(kw.keyword),
            segment::types::Match::Integer(int) => MatchValue::Integer(int.integer),
        };
        Self {
            match_value: Some(match_value),
        }
    }
}

impl From<HnswConfigDiff> for segment::types::HnswConfig {
    fn from(hnsw_config: HnswConfigDiff) -> Self {
        Self {
            m: hnsw_config.m.unwrap_or_default() as usize,
            ef_construct: hnsw_config.ef_construct.unwrap_or_default() as usize,
            full_scan_threshold: hnsw_config.full_scan_threshold.unwrap_or_default() as usize,
            max_indexing_threads: hnsw_config.max_indexing_threads.unwrap_or_default() as usize,
        }
    }
}

pub fn date_time_to_proto(date_time: NaiveDateTime) -> prost_types::Timestamp {
    prost_types::Timestamp {
        seconds: date_time.timestamp(), // number of non-leap seconds since the midnight on January 1, 1970.
        nanos: date_time.nanosecond() as i32,
    }
}
