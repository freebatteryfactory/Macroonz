//! Complete publication file rosters and their distinct byte commitments.

use super::formatting;
use crate::compiler::{Kind, Role};
use crate::native_publication::{InventoryError, PreparedPublication, Publication};
use crate::presentation::value::{array, hex, object, optional, tagged};
use serde_json::Value;

pub(super) fn declared<K: Kind>(record: &Publication<K>) -> Value {
    object([
        ("kind", K::NAME.into()),
        (
            "limits",
            object([
                ("files", record.limits().files.into()),
                ("bytes", record.limits().bytes.into()),
            ]),
        ),
        (
            "files",
            array(record.files().map(|file| {
                let unit = file.unit();
                object([
                    ("path", file.path().spelling().into()),
                    ("site", optional(file.site(), Value::from)),
                    ("source", file.source().into()),
                    ("canonical_bytes", hex(&file.tokens().canonical_bytes())),
                    ("canonical_digest", hex(file.canonical_digest().as_bytes())),
                    (
                        "unit",
                        object([
                            ("role", unit.role().name().into()),
                            ("identity", hex(unit.identity().as_bytes())),
                            ("semantic_key", hex(unit.semantic_key().as_bytes())),
                            ("digest", hex(unit.digest().as_bytes())),
                            (
                                "address",
                                optional(unit.address(), |address| {
                                    object([
                                        ("subject", address.subject.into()),
                                        ("bytes", hex(&address.bytes)),
                                    ])
                                }),
                            ),
                        ]),
                    ),
                ])
            })),
        ),
    ])
}

pub(super) fn prepared<K: Kind>(record: &PreparedPublication<K>) -> Value {
    object([
        ("publication", declared(record.publication())),
        (
            "files",
            array(record.files().map(|file| {
                object([
                    ("path", file.path().spelling().into()),
                    ("bytes", hex(file.bytes())),
                    ("canonical_digest", hex(file.canonical_digest().as_bytes())),
                    ("published_digest", hex(file.published_digest().as_bytes())),
                ])
            })),
        ),
        (
            "formatting",
            array(record.formatting().map(formatting::output)),
        ),
    ])
}

pub(super) fn error(record: &InventoryError) -> Value {
    let kind = match record {
        InventoryError::Path => "path",
        InventoryError::PathCollision => "path-collision",
        InventoryError::Binding => "binding",
        InventoryError::Stamp => "stamp",
        InventoryError::Landing => "landing",
        InventoryError::FileBound => "file-bound",
        InventoryError::ByteBound => "byte-bound",
    };
    tagged(kind, Value::Null)
}
