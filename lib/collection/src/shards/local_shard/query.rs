use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
use futures::FutureExt;
use itertools::Itertools as _;
use segment::types::ScoredPoint;
use tokio::runtime::Handle;

use super::LocalShard;
use crate::operations::types::CollectionResult;
use crate::operations::universal_query::planned_query::{
    PlannedQuery, PrefetchMerge, PrefetchPlan, PrefetchSource,
};

impl LocalShard {
    pub async fn do_planned_query(
        &self,
        request: PlannedQuery,
        search_runtime_handle: &Handle,
        timeout: Option<Duration>,
    ) -> CollectionResult<Vec<ScoredPoint>> {
        let core_results = self
            .do_search(Arc::clone(&request.batch), search_runtime_handle, timeout)
            .await?;

        self.recurse_prefetch(&request.merge_plan, &core_results)
            .await

        // TODO(universal-query): Implement with_vector and with_payload
    }

    fn recurse_prefetch<'shard, 'query>(
        &'shard self,
        prefetch: &'query PrefetchPlan,
        core_results: &'query Vec<Vec<ScoredPoint>>,
    ) -> BoxFuture<'query, CollectionResult<Vec<ScoredPoint>>>
    where
        'shard: 'query,
    {
        async move {
            let mut sources = Vec::with_capacity(prefetch.sources.len());

            for source in prefetch.sources.iter() {
                let vec: Vec<ScoredPoint> = match source {
                    PrefetchSource::BatchIdx(idx) => {
                        core_results.get(*idx).cloned().unwrap_or_default() // TODO(universal-query): don't clone, by using something like a hashmap instead of a vec
                    }
                    PrefetchSource::Prefetch(prefetch) => {
                        self.recurse_prefetch(prefetch, core_results).await?
                    }
                };
                sources.push(vec);
            }

            self.merge_prefetches(sources, &prefetch.merge).await
        }
        .boxed()
    }

    async fn merge_prefetches(
        &self,
        sources: Vec<Vec<ScoredPoint>>,
        merge: &PrefetchMerge,
    ) -> CollectionResult<Vec<ScoredPoint>> {
        if let Some(_rescore) = merge.rescore.as_ref() {
            // TODO(universal-query): Implement rescore
        }

        let top = sources
            .into_iter()
            .flatten()
            .sorted()
            .take(merge.limit)
            .collect();

        Ok(top)
    }
}
