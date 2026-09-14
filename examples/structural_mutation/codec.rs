//! Codec member-order alternatives and a separately authored full-wire observation.

use macroonz::compiler::Bounded;
use macroonz::compiler::codec::{
    AssemblyPosture, Cardinality, CodecAssembly, CodecContent, CodecDirection, CodecMember,
    CodecMemberShape, CodecPlacement, CodecShape, CodecTypePath, PathRooting,
};
use macroonz::compiler::descriptor::mutation::{
    DECLARED_ORDER_FAMILY, Surface, completed_from_codec_order,
};

pub(super) const OBSERVER: &str = r#"
use std::io::Write;

#[derive(Debug, PartialEq, Eq)]
struct Record { first: u64, second: u64 }

impl Record {
    fn assembled(first: u64, second: u64) -> Self { Self { first, second } }
}

fn main() -> std::io::Result<()> {
    let value = Record::assembled(1u64, 2u64);
    let expected = [
        0u8, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 2,
    ];
    let mut bytes = Vec::new();
    value.encode_canonical(&mut bytes);
    let agrees = bytes == expected && Record::decode_canonical(&expected) == Ok(value);
    writeln!(std::io::stdout(), "{}", u64::from(agrees))
}
"#;

pub(super) fn surface() -> Result<Surface, String> {
    let path = |name: &str| {
        CodecTypePath::spelled(PathRooting::InScope, vec![name.to_owned()])
            .map_err(|error| error.to_string())
    };
    let members = ["first", "second"]
        .into_iter()
        .map(|name| {
            CodecMember::declared(
                name,
                path("u64")?,
                CodecMemberShape::Count,
                Cardinality::Required,
            )
            .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let content = CodecContent {
        shape: CodecShape::declared(
            path("Record")?,
            "DecodeRefusal",
            CodecAssembly::stated("assembled", AssemblyPosture::Total)
                .map_err(|error| error.to_string())?,
            members,
        )
        .map_err(|error| error.to_string())?,
        direction: CodecDirection::RoundTrip,
        placement: CodecPlacement::AtDeclarationSite,
        schema: None,
        byte_role: None,
        assumptions: Bounded::empty(),
    };
    completed_from_codec_order(
        super::declaration::declared(DECLARED_ORDER_FAMILY)?,
        &content,
    )
    .map_err(|error| error.to_string())
}
