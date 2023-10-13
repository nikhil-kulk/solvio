use std::borrow::Cow;
use std::collections::HashMap;

use common::validation::{validate_move_shard_different_peers, validate_range_generic};
use validator::{Validate, ValidationError, ValidationErrors};

use super::solvio::{GeoLineString, NamedVectors};

pub trait ValidateExt {
    fn validate(&self) -> Result<(), ValidationErrors>;
}

impl Validate for dyn ValidateExt {
    #[inline]
    fn validate(&self) -> Result<(), ValidationErrors> {
        ValidateExt::validate(self)
    }
}

impl<V> ValidateExt for ::core::option::Option<V>
where
    V: Validate,
{
    #[inline]
    fn validate(&self) -> Result<(), ValidationErrors> {
        (&self).validate()
    }
}

impl<V> ValidateExt for &::core::option::Option<V>
where
    V: Validate,
{
    #[inline]
    fn validate(&self) -> Result<(), ValidationErrors> {
        self.as_ref().map(Validate::validate).unwrap_or(Ok(()))
    }
}

impl<V> ValidateExt for Vec<V>
where
    V: Validate,
{
    #[inline]
    #[allow(clippy::manual_try_fold)] // `try_fold` can't be used because it shortcuts on Err
    fn validate(&self) -> Result<(), ValidationErrors> {
        let errors = self
            .iter()
            .filter_map(|v| v.validate().err())
            .fold(Err(ValidationErrors::new()), |bag, err| {
                ValidationErrors::merge(bag, "?", Err(err))
            })
            .unwrap_err();
        errors.errors().is_empty().then_some(()).ok_or(errors)
    }
}

impl<K, V> ValidateExt for HashMap<K, V>
where
    V: Validate,
{
    #[inline]
    #[allow(clippy::manual_try_fold)] // `try_fold` can't be used because it shortcuts on Err
    fn validate(&self) -> Result<(), ValidationErrors> {
        let errors = self
            .values()
            .filter_map(|v| v.validate().err())
            .fold(Err(ValidationErrors::new()), |bag, err| {
                ValidationErrors::merge(bag, "?", Err(err))
            })
            .unwrap_err();
        errors.errors().is_empty().then_some(()).ok_or(errors)
    }
}

impl Validate for crate::grpc::solvio::vectors_config::Config {
    fn validate(&self) -> Result<(), ValidationErrors> {
        use crate::grpc::solvio::vectors_config::Config;
        match self {
            Config::Params(params) => params.validate(),
            Config::ParamsMap(params_map) => params_map.validate(),
        }
    }
}

impl Validate for crate::grpc::solvio::vectors_config_diff::Config {
    fn validate(&self) -> Result<(), ValidationErrors> {
        use crate::grpc::solvio::vectors_config_diff::Config;
        match self {
            Config::Params(params) => params.validate(),
            Config::ParamsMap(params_map) => params_map.validate(),
        }
    }
}

impl Validate for crate::grpc::solvio::quantization_config::Quantization {
    fn validate(&self) -> Result<(), ValidationErrors> {
        use crate::grpc::solvio::quantization_config::Quantization;
        match self {
            Quantization::Scalar(scalar) => scalar.validate(),
            Quantization::Product(product) => product.validate(),
            Quantization::Binary(binary) => binary.validate(),
        }
    }
}

impl Validate for crate::grpc::solvio::quantization_config_diff::Quantization {
    fn validate(&self) -> Result<(), ValidationErrors> {
        use crate::grpc::solvio::quantization_config_diff::Quantization;
        match self {
            Quantization::Scalar(scalar) => scalar.validate(),
            Quantization::Product(product) => product.validate(),
            Quantization::Binary(binary) => binary.validate(),
            Quantization::Disabled(_) => Ok(()),
        }
    }
}

impl Validate for crate::grpc::solvio::update_collection_cluster_setup_request::Operation {
    fn validate(&self) -> Result<(), ValidationErrors> {
        use crate::grpc::solvio::update_collection_cluster_setup_request::Operation;
        match self {
            Operation::MoveShard(op) => op.validate(),
            Operation::ReplicateShard(op) => op.validate(),
            Operation::AbortTransfer(op) => op.validate(),
            Operation::DropReplica(op) => op.validate(),
        }
    }
}

impl Validate for crate::grpc::solvio::MoveShard {
    fn validate(&self) -> Result<(), ValidationErrors> {
        validate_move_shard_different_peers(self.from_peer_id, self.to_peer_id)
    }
}

impl Validate for crate::grpc::solvio::condition::ConditionOneOf {
    fn validate(&self) -> Result<(), ValidationErrors> {
        use crate::grpc::solvio::condition::ConditionOneOf;
        match self {
            ConditionOneOf::Field(field_condition) => field_condition.validate(),
            ConditionOneOf::Nested(nested) => nested.validate(),
            ConditionOneOf::Filter(filter) => filter.validate(),
            ConditionOneOf::IsEmpty(_) => Ok(()),
            ConditionOneOf::HasId(_) => Ok(()),
            ConditionOneOf::IsNull(_) => Ok(()),
        }
    }
}

impl Validate for crate::grpc::solvio::FieldCondition {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let all_fields_none = self.r#match.is_none()
            && self.range.is_none()
            && self.geo_bounding_box.is_none()
            && self.geo_radius.is_none()
            && self.geo_polygon.is_none()
            && self.values_count.is_none();

        if all_fields_none {
            let mut errors = ValidationErrors::new();
            errors.add(
                "match",
                ValidationError::new("At least one field condition must be specified"),
            );
            Err(errors)
        } else {
            Ok(())
        }
    }
}

/// Validate the value is in `[1, ]` or `None`.
pub fn validate_u64_range_min_1(value: &Option<u64>) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |v| validate_range_generic(v, Some(1), None))
}

/// Validate the value is in `[1, ]` or `None`.
pub fn validate_u32_range_min_1(value: &Option<u32>) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |v| validate_range_generic(v, Some(1), None))
}

/// Validate the value is in `[100, ]` or `None`.
pub fn validate_u64_range_min_100(value: &Option<u64>) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |v| validate_range_generic(v, Some(100), None))
}

/// Validate the value is in `[1000, ]` or `None`.
pub fn validate_u64_range_min_1000(value: &Option<u64>) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |v| validate_range_generic(v, Some(1000), None))
}

/// Validate the value is in `[4, ]` or `None`.
pub fn validate_u64_range_min_4(value: &Option<u64>) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |v| validate_range_generic(v, Some(4), None))
}

/// Validate the value is in `[4, 10000]` or `None`.
pub fn validate_u64_range_min_4_max_10000(value: &Option<u64>) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |v| validate_range_generic(v, Some(4), Some(10_000)))
}

/// Validate the value is in `[0.5, 1.0]` or `None`.
pub fn validate_f32_range_min_0_5_max_1(value: &Option<f32>) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |v| validate_range_generic(v, Some(0.5), Some(1.0)))
}

/// Validate the value is in `[0.0, 1.0]` or `None`.
pub fn validate_f64_range_1(value: &Option<f64>) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |v| validate_range_generic(v, Some(0.0), Some(1.0)))
}

/// Validate the value is in `[1.0, ]` or `None`.
pub fn validate_f64_range_min_1(value: &Option<f64>) -> Result<(), ValidationError> {
    value.map_or(Ok(()), |v| validate_range_generic(v, Some(1.0), None))
}

/// Validate the list of named vectors is not empty.
pub fn validate_named_vectors_not_empty(
    value: &Option<NamedVectors>,
) -> Result<(), ValidationError> {
    // If length is non-zero, we're good
    match value {
        Some(vectors) if !vectors.vectors.is_empty() => return Ok(()),
        Some(_) | None => {}
    }

    let mut err = ValidationError::new("length");
    err.add_param(Cow::from("min"), &1);
    Err(err)
}

/// Validate that GeoLineString has at least 4 points and is closed.
pub fn validate_geo_polygon_line_helper(line: &GeoLineString) -> Result<(), ValidationError> {
    let points = &line.points;
    let min_length = 4;
    if points.len() < min_length {
        let mut err: ValidationError = ValidationError::new("min_line_length");
        err.add_param(Cow::from("length"), &points.len());
        err.add_param(Cow::from("min_length"), &min_length);
        return Err(err);
    }

    let first_point = &points[0];
    let last_point = &points[points.len() - 1];
    if first_point != last_point {
        return Err(ValidationError::new("closed_line"));
    }

    Ok(())
}

pub fn validate_geo_polygon_exterior(line: &Option<GeoLineString>) -> Result<(), ValidationError> {
    match line {
        Some(l) => {
            if l.points.is_empty() {
                return Err(ValidationError::new("not_empty"));
            }
            validate_geo_polygon_line_helper(l)?;
            Ok(())
        }
        _ => Err(ValidationError::new("not_empty")),
    }
}

pub fn validate_geo_polygon_interiors(lines: &Vec<GeoLineString>) -> Result<(), ValidationError> {
    for line in lines {
        validate_geo_polygon_line_helper(line)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use validator::Validate;

    use crate::grpc::solvio::{
        CreateCollection, CreateFieldIndexCollection, GeoLineString, GeoPoint, GeoPolygon,
        SearchPoints, UpdateCollection,
    };

    #[test]
    fn test_good_request() {
        let bad_request = CreateCollection {
            collection_name: "test_collection".into(),
            timeout: Some(10),
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_ok(),
            "good collection request should not error on validation"
        );

        // Collection name validation must not be strict on non-creation
        let bad_request = UpdateCollection {
            collection_name: "no/path".into(),
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_ok(),
            "good collection request should not error on validation"
        );

        // Collection name validation must not be strict on non-creation
        let bad_request = UpdateCollection {
            collection_name: "no*path".into(),
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_ok(),
            "good collection request should not error on validation"
        );
    }

    #[test]
    fn test_bad_collection_request() {
        let bad_request = CreateCollection {
            collection_name: "".into(),
            timeout: Some(0),
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_err(),
            "bad collection request should error on validation"
        );

        // Collection name validation must be strict on creation
        let bad_request = CreateCollection {
            collection_name: "no/path".into(),
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_err(),
            "bad collection request should error on validation"
        );

        // Collection name validation must be strict on creation
        let bad_request = CreateCollection {
            collection_name: "no*path".into(),
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_err(),
            "bad collection request should error on validation"
        );
    }

    #[test]
    fn test_bad_index_request() {
        let bad_request = CreateFieldIndexCollection {
            collection_name: "".into(),
            field_name: "".into(),
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_err(),
            "bad index request should error on validation"
        );
    }

    #[test]
    fn test_bad_search_request() {
        let bad_request = SearchPoints {
            collection_name: "".into(),
            limit: 0,
            vector_name: Some("".into()),
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_err(),
            "bad search request should error on validation"
        );

        let bad_request = SearchPoints {
            limit: 0,
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_err(),
            "bad search request should error on validation"
        );

        let bad_request = SearchPoints {
            vector_name: Some("".into()),
            ..Default::default()
        };
        assert!(
            bad_request.validate().is_err(),
            "bad search request should error on validation"
        );
    }

    #[test]
    fn test_geo_polygon() {
        let bad_polygon = GeoPolygon {
            exterior: Some(GeoLineString { points: vec![] }),
            interiors: vec![],
        };
        assert!(
            bad_polygon.validate().is_err(),
            "bad polygon should error on validation"
        );

        let bad_polygon = GeoPolygon {
            exterior: Some(GeoLineString {
                points: vec![
                    GeoPoint { lat: 1., lon: 1. },
                    GeoPoint { lat: 2., lon: 2. },
                    GeoPoint { lat: 3., lon: 3. },
                ],
            }),
            interiors: vec![],
        };
        assert!(
            bad_polygon.validate().is_err(),
            "bad polygon should error on validation"
        );

        let bad_polygon = GeoPolygon {
            exterior: Some(GeoLineString {
                points: vec![
                    GeoPoint { lat: 1., lon: 1. },
                    GeoPoint { lat: 2., lon: 2. },
                    GeoPoint { lat: 3., lon: 3. },
                    GeoPoint { lat: 4., lon: 4. },
                ],
            }),
            interiors: vec![],
        };

        assert!(
            bad_polygon.validate().is_err(),
            "bad polygon should error on validation"
        );

        let bad_polygon = GeoPolygon {
            exterior: Some(GeoLineString {
                points: vec![
                    GeoPoint { lat: 1., lon: 1. },
                    GeoPoint { lat: 2., lon: 2. },
                    GeoPoint { lat: 3., lon: 3. },
                    GeoPoint { lat: 1., lon: 1. },
                ],
            }),
            interiors: vec![GeoLineString {
                points: vec![
                    GeoPoint { lat: 1., lon: 1. },
                    GeoPoint { lat: 2., lon: 2. },
                    GeoPoint { lat: 3., lon: 3. },
                    GeoPoint { lat: 2., lon: 2. },
                ],
            }],
        };

        assert!(
            bad_polygon.validate().is_err(),
            "bad polygon should error on validation"
        );

        let good_polygon = GeoPolygon {
            exterior: Some(GeoLineString {
                points: vec![
                    GeoPoint { lat: 1., lon: 1. },
                    GeoPoint { lat: 2., lon: 2. },
                    GeoPoint { lat: 3., lon: 3. },
                    GeoPoint { lat: 1., lon: 1. },
                ],
            }),
            interiors: vec![],
        };
        assert!(
            good_polygon.validate().is_ok(),
            "good polygon should not error on validation"
        );
    }
}
