use common::types::{PointOffsetType, ScoreType};

use crate::data_types::vectors::{VectorElementType, VectorType};
use crate::types::Distance;
use crate::vector_storage::query::reco_query::RecoQuery;
use crate::vector_storage::query_scorer::QueryScorer;

pub struct QuantizedRecoQueryScorer<'a, TEncodedQuery, TEncodedVectors>
where
    TEncodedVectors: quantization::EncodedVectors<TEncodedQuery>,
{
    original_query: RecoQuery<VectorType>,
    query: RecoQuery<TEncodedQuery>,
    quantized_storage: &'a TEncodedVectors,
    distance: Distance,
}

impl<'a, TEncodedQuery, TEncodedVectors>
    QuantizedRecoQueryScorer<'a, TEncodedQuery, TEncodedVectors>
where
    TEncodedVectors: quantization::EncodedVectors<TEncodedQuery>,
{
    pub fn new(
        raw_query: RecoQuery<VectorType>,
        quantized_storage: &'a TEncodedVectors,
        distance: Distance,
    ) -> Self {
        let original_query = raw_query.transform(|v| distance.preprocess_vector(v));
        let query = original_query
            .clone()
            .transform(|v| quantized_storage.encode_query(&v));

        Self {
            original_query,
            query,
            quantized_storage,
            distance,
        }
    }
}

impl<TEncodedQuery, TEncodedVectors> QueryScorer
    for QuantizedRecoQueryScorer<'_, TEncodedQuery, TEncodedVectors>
where
    TEncodedVectors: quantization::EncodedVectors<TEncodedQuery>,
{
    fn score_stored(&self, idx: PointOffsetType) -> ScoreType {
        self.query
            .score_by(|this| self.quantized_storage.score_point(this, idx))
    }

    fn score(&self, v2: &[VectorElementType]) -> ScoreType {
        debug_assert!(
            false,
            "This method is not expected to be called for quantized scorer"
        );
        self.original_query
            .score_by(|this| self.distance.similarity(this, v2))
    }

    fn score_internal(&self, _point_a: PointOffsetType, _point_b: PointOffsetType) -> ScoreType {
        unimplemented!("Recommendation scorer compares against multiple vectors, not just one")
    }
}
