use tonic::{Request, Response, Status};

use crate::common::points::do_update_points;
use crate::tonic::api::common::error_to_status;
use crate::tonic::solvio::points_server::Points;
use crate::tonic::solvio::{
    FloatPayload, GeoPayload, GeoPoint, IntegerPayload, KeywordPayload, PointStruct,
    PointsOperationResponse, UpdateResult, UpsertPoints,
};
use collection::operations::point_ops::{PointInsertOperations, PointOperations, PointsList};
use collection::operations::types::UpdateResult as CollectionUpdateResult;
use collection::operations::CollectionUpdateOperations;
use segment::types::{PayloadInterface, PayloadInterfaceStrict, PayloadVariant, PointIdType};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use std::time::Instant;
use storage::content_manager::toc::TableOfContent;

pub struct PointsService {
    toc: Arc<TableOfContent>,
}

impl PointsService {
    pub fn new(toc: Arc<TableOfContent>) -> Self {
        Self { toc }
    }
}

#[tonic::async_trait]
impl Points for PointsService {
    async fn upsert(
        &self,
        request: Request<UpsertPoints>,
    ) -> Result<Response<PointsOperationResponse>, Status> {
        let UpsertPoints {
            collection,
            wait,
            points,
        } = request.into_inner();

        let points = points
            .into_iter()
            .map(|point| point.try_into())
            .collect::<Result<_, _>>()?;
        let operation = CollectionUpdateOperations::PointOperation(PointOperations::UpsertPoints(
            PointInsertOperations::PointsList(PointsList { points }),
        ));

        let timing = Instant::now();
        let result = do_update_points(
            self.toc.as_ref(),
            &collection,
            operation,
            wait.unwrap_or(false),
        )
        .await
        .map_err(error_to_status)?;

        let response = PointsOperationResponse::from((timing, result));
        Ok(Response::new(response))
    }
}

impl TryFrom<PointStruct> for collection::operations::point_ops::PointStruct {
    type Error = Status;

    fn try_from(value: PointStruct) -> Result<Self, Self::Error> {
        let PointStruct {
            id,
            vector,
            payload,
        } = value;

        let mut converted_payload = HashMap::new();
        for (key, payload_value) in payload.into_iter() {
            let value = if let Some(keyword) = payload_value.keyword {
                keyword.into()
            } else if let Some(integer) = payload_value.integer {
                integer.into()
            } else if let Some(float) = payload_value.float {
                float.into()
            } else if let Some(geo) = payload_value.geo {
                geo.into()
            } else {
                return Err(Status::failed_precondition("Unknown payload type"));
            };
            converted_payload.insert(key, value);
        }

        Ok(Self {
            id: PointIdType::NumId(id),
            vector,
            payload: Some(converted_payload),
        })
    }
}

impl From<KeywordPayload> for PayloadInterface {
    fn from(value: KeywordPayload) -> Self {
        PayloadInterface::Payload(PayloadInterfaceStrict::Keyword(PayloadVariant::List(
            value.value,
        )))
    }
}

impl From<IntegerPayload> for PayloadInterface {
    fn from(value: IntegerPayload) -> Self {
        PayloadInterface::Payload(PayloadInterfaceStrict::Integer(PayloadVariant::List(
            value.value,
        )))
    }
}

impl From<FloatPayload> for PayloadInterface {
    fn from(value: FloatPayload) -> Self {
        PayloadInterface::Payload(PayloadInterfaceStrict::Float(PayloadVariant::List(
            value.value,
        )))
    }
}

impl From<GeoPayload> for PayloadInterface {
    fn from(value: GeoPayload) -> Self {
        let variant =
            PayloadVariant::List(value.value.into_iter().map(|point| point.into()).collect());
        PayloadInterface::Payload(PayloadInterfaceStrict::Geo(variant))
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

impl From<(Instant, CollectionUpdateResult)> for PointsOperationResponse {
    fn from(value: (Instant, CollectionUpdateResult)) -> Self {
        let (timing, response) = value;
        Self {
            result: Some(response.into()),
            time: timing.elapsed().as_secs_f64(),
        }
    }
}

impl From<CollectionUpdateResult> for UpdateResult {
    fn from(value: CollectionUpdateResult) -> Self {
        Self {
            operation_id: value.operation_id,
            status: value.status as i32,
        }
    }
}
