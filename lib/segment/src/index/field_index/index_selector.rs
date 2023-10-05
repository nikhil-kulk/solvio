use std::sync::Arc;

use parking_lot::RwLock;
use rocksdb::DB;

use super::binary_index::BinaryIndex;
use crate::index::field_index::full_text_index::text_index::FullTextIndex;
use crate::index::field_index::geo_index::GeoMapIndex;
use crate::index::field_index::map_index::MapIndex;
use crate::index::field_index::numeric_index::NumericIndex;
use crate::index::field_index::FieldIndex;
use crate::types::{
    FloatPayloadType, IntPayloadType, PayloadFieldSchema, PayloadSchemaParams, PayloadSchemaType,
};

/// Selects index types based on field type
pub fn index_selector(
    field: &str,
    payload_schema: &PayloadFieldSchema,
    db: Arc<RwLock<DB>>,
    is_appendable: bool,
) -> Vec<FieldIndex> {
    match payload_schema {
        PayloadFieldSchema::FieldType(payload_type) => match payload_type {
            PayloadSchemaType::Keyword => {
                vec![FieldIndex::KeywordIndex(MapIndex::new(
                    db,
                    field,
                    is_appendable,
                ))]
            }
            PayloadSchemaType::Integer => vec![
                FieldIndex::IntMapIndex(MapIndex::new(db.clone(), field, is_appendable)),
                FieldIndex::IntIndex(NumericIndex::<IntPayloadType>::new(
                    db,
                    field,
                    is_appendable,
                )),
            ],
            PayloadSchemaType::Float => {
                vec![FieldIndex::FloatIndex(
                    NumericIndex::<FloatPayloadType>::new(db, field, is_appendable),
                )]
            }
            PayloadSchemaType::Geo => vec![FieldIndex::GeoIndex(GeoMapIndex::new(db, field))],
            PayloadSchemaType::Text => vec![FieldIndex::FullTextIndex(FullTextIndex::new(
                db,
                Default::default(),
                field,
            ))],
            PayloadSchemaType::Bool => vec![FieldIndex::BinaryIndex(BinaryIndex::new(db, field))],
        },
        PayloadFieldSchema::FieldParams(payload_params) => match payload_params {
            PayloadSchemaParams::Text(text_index_params) => vec![FieldIndex::FullTextIndex(
                FullTextIndex::new(db, text_index_params.clone(), field),
            )],
        },
    }
}
