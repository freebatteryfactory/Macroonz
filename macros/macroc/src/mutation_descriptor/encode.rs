//! Binding-independent operation meaning for generated mutation discoveries.

use crate::plane::{encode_bytes, encode_length};

/// Encode one owner-declared cause order without a Rust crate binding, path, or source coordinate.
pub(crate) fn declared_order_operation<'rows>(
    family: &str,
    rows: impl IntoIterator<Item = (&'rows str, &'rows str)>,
) -> Vec<u8> {
    let rows: Vec<(&str, &str)> = rows.into_iter().collect();
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&1_u32.to_be_bytes());
    encode_bytes(b"declared-cause-order", &mut bytes);
    encode_bytes(family.as_bytes(), &mut bytes);
    encode_length(rows.len(), &mut bytes);
    for (local_key, spelling) in rows {
        encode_bytes(local_key.as_bytes(), &mut bytes);
        encode_bytes(spelling.as_bytes(), &mut bytes);
    }
    bytes
}
