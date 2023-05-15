use std::collections::HashMap;
use std::future::Future;

use segment::data_types::vectors::DEFAULT_VECTOR_NAME;
use segment::types::{
    AnyVariants, Condition, FieldCondition, Filter, IsNullCondition, Match, PointGroup,
    ScoredPoint, WithPayloadInterface, WithVector,
};
use serde_json::Value;
use tokio::sync::RwLockReadGuard;

use super::aggregator::GroupsAggregator;
use crate::collection::Collection;
use crate::operations::consistency_params::ReadConsistency;
use crate::operations::types::{
    BaseGroupRequest, CollectionResult, RecommendGroupsRequest, RecommendRequest,
    SearchGroupsRequest, SearchRequest, UsingVector,
};
use crate::recommendations::recommend_by;
use crate::shards::shard::ShardId;

const MAX_GET_GROUPS_REQUESTS: usize = 5;
const MAX_GROUP_FILLING_REQUESTS: usize = 5;

#[derive(Clone, Debug)]
pub enum SourceRequest {
    Search(SearchRequest),
    Recommend(RecommendRequest),
}

impl SourceRequest {
    fn vector_field_name(&self) -> &str {
        match self {
            SourceRequest::Search(request) => request.vector.get_name(),
            SourceRequest::Recommend(request) => {
                if let Some(UsingVector::Name(name)) = &request.using {
                    name
                } else {
                    DEFAULT_VECTOR_NAME
                }
            }
        }
    }

    fn merge_filter(&mut self, filter: &Filter) {
        match self {
            SourceRequest::Search(request) => {
                request.filter = Some(request.filter.clone().unwrap_or_default().merge(filter))
            }
            SourceRequest::Recommend(request) => {
                request.filter = Some(request.filter.clone().unwrap_or_default().merge(filter))
            }
        }
    }

    fn with_payload(&self) -> Option<WithPayloadInterface> {
        match self {
            SourceRequest::Search(request) => request.with_payload.clone(),
            SourceRequest::Recommend(request) => request.with_payload.clone(),
        }
    }

    fn with_vector(&self) -> Option<WithVector> {
        match self {
            SourceRequest::Search(request) => request.with_vector.clone(),
            SourceRequest::Recommend(request) => request.with_vector.clone(),
        }
    }
}

#[derive(Clone)]
pub struct GroupRequest {
    /// Request to use (search or recommend)
    pub source: SourceRequest,

    /// Path to the field to group by
    pub group_by: String,

    /// Limit of points to return per group
    pub group_size: usize,

    /// Limit of groups to return
    pub limit: usize,
}

impl GroupRequest {
    pub fn with_limit_from_request(
        source: SourceRequest,
        group_by: String,
        group_size: usize,
    ) -> Self {
        let limit = match &source {
            SourceRequest::Search(request) => request.limit,
            SourceRequest::Recommend(request) => request.limit,
        };
        Self {
            source,
            group_by,
            group_size,
            limit,
        }
    }

    async fn r#do<'a, F, Fut>(
        &self,
        collection: &Collection,
        // only used for recommend
        collection_by_name: F,
        read_consistency: Option<ReadConsistency>,
        shard_selection: Option<ShardId>,
    ) -> CollectionResult<Vec<ScoredPoint>>
    where
        F: Fn(String) -> Fut,
        Fut: Future<Output = Option<RwLockReadGuard<'a, Collection>>>,
    {
        // Hack: "with_payload" returns empty payload when the requested field ends with `[]`.
        // Remove the ending `[]`.
        let include_group_by = match self.group_by.as_str() {
            s if s.ends_with("[]") => s[..s.len() - 2].to_owned(),
            s => s.to_owned(),
        };

        let only_group_by_key = Some(WithPayloadInterface::Fields(vec![include_group_by]));

        let key_not_null = Filter::new_must_not(Condition::IsNull(IsNullCondition::from(
            self.group_by.clone(),
        )));

        match self.source.clone() {
            SourceRequest::Search(mut request) => {
                request.limit = self.limit * self.group_size;

                request.filter = Some(request.filter.unwrap_or_default().merge(&key_not_null));

                // We're enriching the final results at the end, so we'll keep this minimal
                request.with_payload = only_group_by_key;
                request.with_vector = None;

                collection
                    .search(request, read_consistency, shard_selection)
                    .await
            }
            SourceRequest::Recommend(mut request) => {
                request.limit = self.limit * self.group_size;

                request.filter = Some(request.filter.unwrap_or_default().merge(&key_not_null));

                // We're enriching the final results at the end, so we'll keep this minimal
                request.with_payload = only_group_by_key;
                request.with_vector = None;

                recommend_by(request, collection, collection_by_name, read_consistency).await
            }
        }
    }
}

impl From<SearchGroupsRequest> for GroupRequest {
    fn from(request: SearchGroupsRequest) -> Self {
        let SearchGroupsRequest {
            vector,
            filter,
            params,
            with_payload,
            with_vector,
            score_threshold,
            group_request:
                BaseGroupRequest {
                    group_by,
                    group_size,
                    limit,
                },
        } = request;

        let search = SearchRequest {
            vector,
            filter,
            params,
            limit: 0,
            offset: 0,
            with_payload,
            with_vector,
            score_threshold,
        };

        GroupRequest {
            source: SourceRequest::Search(search),
            group_by,
            group_size: group_size as usize,
            limit: limit as usize,
        }
    }
}

impl From<RecommendGroupsRequest> for GroupRequest {
    fn from(request: RecommendGroupsRequest) -> Self {
        let RecommendGroupsRequest {
            positive,
            negative,
            filter,
            params,
            with_payload,
            with_vector,
            score_threshold,
            using,
            lookup_from,
            group_request:
                BaseGroupRequest {
                    group_by,
                    group_size,
                    limit,
                },
        } = request;

        let recommend = RecommendRequest {
            positive,
            negative,
            filter,
            params,
            limit: 0,
            offset: 0,
            with_payload,
            with_vector,
            score_threshold,
            using,
            lookup_from,
        };

        GroupRequest {
            source: SourceRequest::Recommend(recommend),
            group_by,
            group_size: group_size as usize,
            limit: limit as usize,
        }
    }
}

/// Uses the request to fill up groups of points.
pub async fn group_by<'a, F, Fut>(
    request: GroupRequest,
    collection: &Collection,
    // Obligatory for recommend
    collection_by_name: F,
    read_consistency: Option<ReadConsistency>,
    shard_selection: Option<ShardId>,
) -> CollectionResult<Vec<PointGroup>>
where
    F: Fn(String) -> Fut + Clone,
    Fut: Future<Output = Option<RwLockReadGuard<'a, Collection>>>,
{
    let score_ordering = {
        let vector_name = request.source.vector_field_name();
        let collection_params = collection.collection_config.read().await;
        let vector_params = collection_params.params.get_vector_params(vector_name)?;
        vector_params.distance.distance_order()
    };

    let mut aggregator = GroupsAggregator::new(
        request.limit,
        request.group_size,
        request.group_by.clone(),
        score_ordering,
    );

    // Try to complete amount of groups
    for _ in 0..MAX_GET_GROUPS_REQUESTS {
        // TODO: should we break early if we have some amount of "enough" groups?
        if aggregator.len_of_filled_best_groups() >= request.limit {
            break;
        }

        let mut request = request.clone();

        let source = &mut request.source;

        // construct filter to exclude already found groups
        let full_groups = aggregator.keys_of_filled_groups();
        if !full_groups.is_empty() {
            if let Some(match_any) = match_on(request.group_by.clone(), full_groups) {
                let exclude_groups = Filter::new_must_not(match_any);
                source.merge_filter(&exclude_groups);
            }
        }

        // exclude already aggregated points
        let ids = aggregator.ids();
        if !ids.is_empty() {
            let exclude_ids = Filter::new_must_not(Condition::HasId(ids.into()));
            source.merge_filter(&exclude_ids);
        }

        let points = request
            .r#do(
                collection,
                collection_by_name.clone(),
                read_consistency,
                shard_selection,
            )
            .await?;

        if points.is_empty() {
            break;
        }

        aggregator.add_points(&points)
    }

    // Try to fill up groups
    for _ in 0..MAX_GROUP_FILLING_REQUESTS {
        if aggregator.len_of_filled_best_groups() >= request.limit {
            break;
        }

        let mut request = request.clone();

        let source = &mut request.source;

        // construct filter to only include unsatisfied groups
        let unsatisfied_groups = aggregator.keys_of_unfilled_best_groups();
        if let Some(match_any) = match_on(request.group_by.clone(), unsatisfied_groups) {
            let include_groups = Filter::new_must(match_any);
            source.merge_filter(&include_groups);
        }

        // exclude already aggregated points
        let ids = aggregator.ids();
        if !ids.is_empty() {
            let exclude_ids = Filter::new_must_not(Condition::HasId(ids.into()));
            source.merge_filter(&exclude_ids);
        }

        let points = request
            .r#do(
                collection,
                collection_by_name.clone(),
                read_consistency,
                shard_selection,
            )
            .await?;

        if points.is_empty() {
            break;
        }

        aggregator.add_points(&points);
    }

    // extract best results
    let mut groups = aggregator.distill();

    // flatten results
    let bare_points = groups
        .iter()
        .cloned()
        .flat_map(|group| group.hits)
        .collect();

    // enrich with payload and vector
    let enriched_points: HashMap<_, _> = collection
        .fill_search_result_with_payload(
            bare_points,
            request.source.with_payload(),
            request.source.with_vector().unwrap_or_default(),
            read_consistency,
            None,
        )
        .await?
        .into_iter()
        .map(|point| (point.id, point))
        .collect();

    // hydrate groups with enriched points
    groups
        .iter_mut()
        .for_each(|group| group.hydrate_from(&enriched_points));

    // turn into output form
    let groups = groups.into_iter().map(PointGroup::from).collect();

    Ok(groups)
}

/// Uses the set of values to create a Match::Any, if possible
fn match_on(path: String, values: Vec<Value>) -> Option<Condition> {
    match values.first() {
        Some(Value::Number(_)) => Some(Match::new_any(AnyVariants::Integers(
            values.into_iter().filter_map(|v| v.as_i64()).collect(),
        ))),
        Some(Value::String(_)) => Some(Match::new_any(AnyVariants::Keywords(
            values
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_owned()))
                .collect(),
        ))),
        _ => None, // also considers the case of empty values
    }
    .map(|m| Condition::Field(FieldCondition::new_match(path, m)))
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use segment::data_types::groups::GroupId;
    use segment::types::{Payload, ScoredPoint};

    use crate::grouping::types::Group;

    #[test]
    fn test_hydrated_from() {
        // arrange
        let mut groups: Vec<Group> = Vec::new();
        [
            (
                "a",
                [
                    ScoredPoint {
                        id: 1.into(),
                        version: 0,
                        score: 1.0,
                        payload: None,
                        vector: None,
                    },
                    ScoredPoint {
                        id: 2.into(),
                        version: 0,
                        score: 1.0,
                        payload: None,
                        vector: None,
                    },
                ],
            ),
            (
                "b",
                [
                    ScoredPoint {
                        id: 3.into(),
                        version: 0,
                        score: 1.0,
                        payload: None,
                        vector: None,
                    },
                    ScoredPoint {
                        id: 4.into(),
                        version: 0,
                        score: 1.0,
                        payload: None,
                        vector: None,
                    },
                ],
            ),
        ]
        .into_iter()
        .for_each(|(key, points)| {
            let group = Group {
                key: GroupId::from(key),
                hits: points.into_iter().collect(),
            };
            groups.push(group);
        });

        let payload_a = Payload::from(serde_json::json!({"some_key": "some value a"}));
        let payload_b = Payload::from(serde_json::json!({"some_key": "some value b"}));

        let hydrated = vec![
            ScoredPoint {
                id: 1.into(),
                version: 0,
                score: 1.0,
                payload: Some(payload_a.clone()),
                vector: None,
            },
            ScoredPoint {
                id: 2.into(),
                version: 0,
                score: 1.0,
                payload: Some(payload_a.clone()),
                vector: None,
            },
            ScoredPoint {
                id: 3.into(),
                version: 0,
                score: 1.0,
                payload: Some(payload_b.clone()),
                vector: None,
            },
            ScoredPoint {
                id: 4.into(),
                version: 0,
                score: 1.0,
                payload: Some(payload_b.clone()),
                vector: None,
            },
        ];

        let set: HashMap<_, _> = hydrated.into_iter().map(|p| (p.id, p)).collect();

        // act
        groups.iter_mut().for_each(|group| group.hydrate_from(&set));

        // assert
        assert_eq!(groups.len(), 2);
        assert_eq!(groups.get(0).unwrap().hits.len(), 2);
        assert_eq!(groups.get(1).unwrap().hits.len(), 2);

        let a = groups.get(0).unwrap();
        let b = groups.get(1).unwrap();

        assert!(a
            .hits
            .iter()
            .all(|x| x.payload.as_ref() == Some(&payload_a)));
        assert!(b
            .hits
            .iter()
            .all(|x| x.payload.as_ref() == Some(&payload_b)));
    }
}
