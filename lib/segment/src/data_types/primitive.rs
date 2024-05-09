use std::borrow::Cow;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::vectors::TypedMultiDenseVector;
use crate::data_types::vectors::{VectorElementType, VectorElementTypeByte};
use crate::spaces::metric::Metric;
use crate::spaces::simple::{CosineMetric, DotProductMetric, EuclidMetric, ManhattanMetric};
use crate::types::{Distance, QuantizationConfig, VectorStorageDatatype};

pub trait PrimitiveVectorElement:
    Copy + Clone + Default + Serialize + for<'a> Deserialize<'a>
{
    fn slice_from_float_cow(vector: Cow<[VectorElementType]>) -> Cow<[Self]>;

    fn slice_to_float_cow(vector: Cow<[Self]>) -> Cow<[VectorElementType]>;

    fn quantization_preprocess<'a>(
        quantization_config: &QuantizationConfig,
        distance: Distance,
        vector: &'a [Self],
    ) -> Cow<'a, [f32]>;

    fn datatype() -> VectorStorageDatatype;

    fn from_float_multivector(
        multivector: Cow<TypedMultiDenseVector<VectorElementType>>,
    ) -> Cow<TypedMultiDenseVector<Self>>;

    fn into_float_multivector(
        multivector: Cow<TypedMultiDenseVector<Self>>,
    ) -> Cow<TypedMultiDenseVector<VectorElementType>>;
}

impl PrimitiveVectorElement for VectorElementType {
    fn slice_from_float_cow(vector: Cow<[VectorElementType]>) -> Cow<[Self]> {
        vector
    }

    fn slice_to_float_cow(vector: Cow<[Self]>) -> Cow<[VectorElementType]> {
        vector
    }

    fn quantization_preprocess<'a>(
        _quantization_config: &QuantizationConfig,
        _distance: Distance,
        vector: &'a [Self],
    ) -> Cow<'a, [f32]> {
        Cow::Borrowed(vector)
    }

    fn datatype() -> VectorStorageDatatype {
        VectorStorageDatatype::Float32
    }

    fn from_float_multivector(
        multivector: Cow<TypedMultiDenseVector<VectorElementType>>,
    ) -> Cow<TypedMultiDenseVector<Self>> {
        multivector
    }

    fn into_float_multivector(
        multivector: Cow<TypedMultiDenseVector<Self>>,
    ) -> Cow<TypedMultiDenseVector<VectorElementType>> {
        multivector
    }
}

impl PrimitiveVectorElement for VectorElementTypeByte {
    fn slice_from_float_cow(vector: Cow<[VectorElementType]>) -> Cow<[Self]> {
        Cow::Owned(vector.iter().map(|&x| x as u8).collect())
    }

    fn slice_to_float_cow(vector: Cow<[Self]>) -> Cow<[VectorElementType]> {
        Cow::Owned(vector.iter().map(|&x| x as VectorElementType).collect_vec())
    }

    fn quantization_preprocess<'a>(
        quantization_config: &QuantizationConfig,
        distance: Distance,
        vector: &'a [Self],
    ) -> Cow<'a, [f32]> {
        if let QuantizationConfig::Binary(_) = quantization_config {
            Cow::from(
                vector
                    .iter()
                    .map(|&x| (x as VectorElementType) - 127.0)
                    .collect_vec(),
            )
        } else {
            let vector = vector.iter().map(|&x| x as VectorElementType).collect_vec();
            let preprocessed_vector = match distance {
                Distance::Cosine => <CosineMetric as Metric<VectorElementType>>::preprocess(vector),
                Distance::Euclid => <EuclidMetric as Metric<VectorElementType>>::preprocess(vector),
                Distance::Dot => {
                    <DotProductMetric as Metric<VectorElementType>>::preprocess(vector)
                }
                Distance::Manhattan => {
                    <ManhattanMetric as Metric<VectorElementType>>::preprocess(vector)
                }
            };
            Cow::from(preprocessed_vector)
        }
    }

    fn datatype() -> VectorStorageDatatype {
        VectorStorageDatatype::Uint8
    }

    fn from_float_multivector(
        multivector: Cow<TypedMultiDenseVector<VectorElementType>>,
    ) -> Cow<TypedMultiDenseVector<Self>> {
        Cow::Owned(TypedMultiDenseVector::new(
            multivector
                .flattened_vectors
                .iter()
                .map(|&x| x as Self)
                .collect_vec(),
            multivector.dim,
        ))
    }

    fn into_float_multivector(
        multivector: Cow<TypedMultiDenseVector<Self>>,
    ) -> Cow<TypedMultiDenseVector<VectorElementType>> {
        Cow::Owned(TypedMultiDenseVector::new(
            multivector
                .flattened_vectors
                .iter()
                .map(|&x| x as VectorElementType)
                .collect_vec(),
            multivector.dim,
        ))
    }
}
