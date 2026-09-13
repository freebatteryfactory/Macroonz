//! Supplied filesystem spellings without filesystem access or portable identity claims.

use super::value::{hex, object};
use serde_json::Value;
use std::path::Path;

pub(super) fn path(record: &Path) -> Value {
    let path = record.as_os_str();
    object([
        ("encoding", "rust-os-str-platform-dependent".into()),
        ("bytes", hex(path.as_encoded_bytes())),
        ("shown", path.to_string_lossy().as_ref().into()),
        (
            "fidelity",
            if path.to_str().is_some() {
                "unicode"
            } else {
                "lossy"
            }
            .into(),
        ),
    ])
}
