use std::iter;

use common::math::scaled_fast_sigmoid;
use common::types::ScoreType;

use super::context_query::ContextPair;
use super::{Query, TransformInto};
use crate::data_types::vectors::{QueryVector, Vector};

type RankType = i32;

impl<T> ContextPair<T> {
    /// Calculates on which side of the space the point is, with respect to this pair
    fn rank_by(&self, similarity: impl Fn(&T) -> ScoreType) -> RankType {
        let positive_similarity = similarity(&self.positive);
        let negative_similarity = similarity(&self.negative);

        // if closer to positive, return 1, else -1
        positive_similarity.total_cmp(&negative_similarity) as RankType
    }
}

#[derive(Debug, Clone)]
pub struct DiscoveryQuery<T> {
    pub target: T,
    pub pairs: Vec<ContextPair<T>>,
}

impl<T> DiscoveryQuery<T> {
    pub fn new(target: T, pairs: Vec<ContextPair<T>>) -> Self {
        Self { target, pairs }
    }

    pub fn flat_iter(&self) -> impl Iterator<Item = &T> {
        let pairs_iter = self.pairs.iter().flat_map(|pair| pair.iter());

        iter::once(&self.target).chain(pairs_iter)
    }

    fn rank_by(&self, similarity: impl Fn(&T) -> ScoreType) -> RankType {
        self.pairs
            .iter()
            .map(|pair| pair.rank_by(&similarity))
            // get overall rank
            .sum()
    }
}

impl<T, U> TransformInto<DiscoveryQuery<U>, T, U> for DiscoveryQuery<T> {
    fn transform<F>(self, mut f: F) -> DiscoveryQuery<U>
    where
        F: FnMut(T) -> U,
    {
        DiscoveryQuery::new(
            f(self.target),
            self.pairs
                .into_iter()
                .map(|pair| pair.transform(&mut f))
                .collect(),
        )
    }
}

impl<T> Query<T> for DiscoveryQuery<T> {
    fn score_by(&self, similarity: impl Fn(&T) -> ScoreType) -> ScoreType {
        let rank = self.rank_by(&similarity);

        let target_similarity = similarity(&self.target);
        let sigmoid_similarity = scaled_fast_sigmoid(target_similarity);

        rank as ScoreType + sigmoid_similarity
    }
}

impl From<DiscoveryQuery<Vector>> for QueryVector {
    fn from(query: DiscoveryQuery<Vector>) -> Self {
        QueryVector::Discovery(query)
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;

    use common::types::ScoreType;
    use rstest::rstest;

    use super::*;

    fn dummy_similarity(x: &isize) -> ScoreType {
        *x as ScoreType
    }

    /// Considers each "vector" as the actual score from the similarity function by
    /// using a dummy identity function.
    #[rstest]
    #[case::no_pairs(vec![], 0)]
    #[case::closer_to_positive(vec![(10, 4)], 1)]
    #[case::closer_to_negative(vec![(4, 10)], -1)]
    #[case::equal_scores(vec![(11, 11)], 0)]
    #[case::neutral_zone(vec![(10, 4), (4, 10)], 0)]
    #[case::best_zone(vec![(10, 4), (4, 2)], 2)]
    #[case::worst_zone(vec![(4, 10), (2, 4)], -2)]
    #[case::many_pairs(vec![(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (0, 4)], 4)]
    fn context_ranking(#[case] pairs: Vec<(isize, isize)>, #[case] expected: RankType) {
        let pairs = pairs.into_iter().map(ContextPair::from).collect();

        let _target = 42;

        let query = DiscoveryQuery::new(_target, pairs);

        let rank = query.rank_by(dummy_similarity);

        assert_eq!(
            rank, expected,
            "Ranking is incorrect, expected {}, but got {rank}",
            expected
        );
    }

    /// Compares the score of a query against a fixed score
    #[rstest]
    #[case::no_pairs(1, vec![], Ordering::Less)]
    #[case::just_above(1, vec![(1,0),(1,0)], Ordering::Greater)]
    #[case::just_below(-1, vec![(1,0),(1,0)], Ordering::Less)]
    #[case::bad_target_good_context(-1000, vec![(1,0),(1,0),(1, 0)], Ordering::Greater)]
    #[case::good_target_bad_context(1000, vec![(1,0),(0,1)], Ordering::Less)]
    fn score_better(
        #[case] target: isize,
        #[case] pairs: Vec<(isize, isize)>,
        #[case] expected: Ordering,
    ) {
        let fixed_score: f32 = 2.5;

        let pairs = pairs.into_iter().map(ContextPair::from).collect();

        let query = DiscoveryQuery::new(target, pairs);

        let score = query.score_by(dummy_similarity);

        assert_eq!(
            score.total_cmp(&fixed_score),
            expected,
            "Comparison is incorrect, expected {expected:?} for {score} against {fixed_score}"
        );
    }
}
