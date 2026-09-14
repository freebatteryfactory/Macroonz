//! Independent source and byte witnesses for codec-order mutation material.

#[path = "../support/mod.rs"]
mod support;

mod fixture;

use fixture::{content, declaration, source};
use macroonz_compiler::CanonicalContent;
use macroonz_compiler::codec::CodecDirection;
use macroonz_compiler::descriptor::mutation::{
    Alternative, CodecMutationError, completed_from_codec_order,
};
use support::observe_rustc;

const HOMOGENEOUS: &str = r#"
#[derive(Debug, PartialEq, Eq)]
struct Record { first: u64, second: u64, third: u64 }
impl Record {
    fn assembled(first: u64, second: u64, third: u64) -> Self {
        Self { first, second, third }
    }
}
fn main() {
    let value = Record::assembled(1, 2, 3);
    let expected = [
        0, 0, 0, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 2,
        0, 0, 0, 0, 0, 0, 0, 3,
    ];
    let mut encoded = Vec::new();
    value.encode_canonical(&mut encoded);
    assert_eq!(encoded, expected, "independent-codec-wire-order");
    assert_eq!(Record::decode_canonical(&expected).expect("lawful wire"), value);
}
"#;

#[test]
fn every_adjacent_codec_damage_compiles_and_the_independent_wire_witness_objects()
-> Result<(), String> {
    let codec = content(&[("first", "u64"), ("second", "u64"), ("third", "u64")])?;
    let original = codec.canonical_content_bytes();
    let surface = completed_from_codec_order(declaration()?, &codec)
        .map_err(|refusal| refusal.to_string())?;
    let baseline = format!("{HOMOGENEOUS}\n{}", source(surface.site().production())?);
    let compiled = observe_rustc("codec_order_baseline", &baseline, &[])?;
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    assert_eq!(surface.site().alternatives().len(), 2);
    for (position, alternative) in surface.site().alternatives().iter().enumerate() {
        assert_eq!(alternative.family().slug(), "declared-order-permutation");
        assert_ne!(alternative.operation(), surface.site().unchanged());
        let changed = format!("{HOMOGENEOUS}\n{}", source(alternative.meaning())?);
        let result = observe_rustc(&format!("codec_order_changed_{position}"), &changed, &[]);
        assert!(matches!(result, Err(refusal) if refusal.contains("independent-codec-wire-order")));
    }
    let [first, second] = surface.site().alternatives() else {
        return Err("all adjacent pairs must be retained".to_owned());
    };
    assert_ne!(first.operation(), second.operation());
    assert_eq!(codec.canonical_content_bytes(), original);
    Ok(())
}

#[test]
fn a_reordered_assembly_can_be_unviable_without_becoming_a_witness_rejection() -> Result<(), String>
{
    let owner = r"
        struct Record { first: u64, second: u16 }
        impl Record {
            fn assembled(first: u64, second: u16) -> Self { Self { first, second } }
        }
        fn main() {
            let mut encoded = Vec::new();
            Record::assembled(1, 2).encode_canonical(&mut encoded);
        }
    ";
    let codec = content(&[("first", "u64"), ("second", "u16")])?;
    let surface = completed_from_codec_order(declaration()?, &codec)
        .map_err(|refusal| refusal.to_string())?;
    let baseline = format!("{owner}\n{}", source(surface.site().production())?);
    assert!(
        observe_rustc("codec_assembly_baseline", &baseline, &[])?
            .status
            .success()
    );
    let [alternative] = surface.site().alternatives() else {
        return Err("a two-member codec must offer its one adjacent swap".to_owned());
    };
    let changed = format!("{owner}\n{}", source(alternative.meaning())?);
    let compiled = observe_rustc("codec_assembly_changed", &changed, &[])?;
    assert!(!compiled.status.success());
    let diagnostic = String::from_utf8_lossy(&compiled.stderr);
    assert!(diagnostic.contains("E0308"), "{diagnostic}");
    Ok(())
}

#[test]
fn no_pair_refuses_and_completion_preserves_policy_and_direction() -> Result<(), String> {
    let one = content(&[("first", "u64")])?;
    assert_eq!(
        completed_from_codec_order(declaration()?, &one),
        Err(CodecMutationError::NoAdjacentMembers)
    );
    let mut codec = content(&[("first", "u64"), ("second", "u64")])?;
    codec.direction = CodecDirection::Encode;
    let declaration = declaration()?;
    let policy = declaration.policy().clone();
    let surface =
        completed_from_codec_order(declaration, &codec).map_err(|refusal| refusal.to_string())?;
    assert_eq!(surface.policy(), &policy);
    for tokens in core::iter::once(surface.site().production()).chain(
        surface
            .site()
            .alternatives()
            .iter()
            .map(Alternative::meaning),
    ) {
        assert!(source(tokens)?.contains("encode_canonical"));
        assert!(!source(tokens)?.contains("decode_canonical"));
        assert!(!source(tokens)?.contains("enum DecodeRefusal"));
    }
    Ok(())
}
