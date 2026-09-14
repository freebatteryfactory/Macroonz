//! Transient display data, with no record admission or identity derivation.

use serde_json::Value;
use std::fmt::Write;

pub(super) fn object<const N: usize>(fields: [(&str, Value); N]) -> Value {
    Value::Object(
        fields
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
    )
}

pub(super) fn array(values: impl Iterator<Item = Value>) -> Value {
    Value::Array(values.collect())
}

pub(super) fn optional<T>(value: Option<T>, project: impl FnOnce(T) -> Value) -> Value {
    value.map_or(Value::Null, project)
}

pub(super) fn tagged(kind: &str, value: Value) -> Value {
    object([("kind", kind.into()), ("value", value)])
}

pub(super) fn hex(bytes: &[u8]) -> Value {
    let mut shown = String::new();
    for byte in bytes {
        let _written = write!(shown, "{byte:02x}");
    }
    Value::String(shown)
}
