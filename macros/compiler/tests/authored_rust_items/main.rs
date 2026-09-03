//! Preserved authored Rust, its source-coupled structural lens, and exact generated-token projection observed outside the compiler crate.
//!
//! The fixtures carry structural pressure from ordinary and advanced Rust without asking Macroonz to interpret what any item means.

use core::convert::Infallible;
use core::hash::{Hash, Hasher};
use macroonz_compiler::{
    AuthoredItemKind, AuthoredItemReadIssue, CaptureBuildRefusal, CaptureBuilder, CapturedAtom,
    CapturedDelimiter, FragmentGenerationIssue, GeneratedLiteralRefusal, SpanHandle, TextCapture,
    encode_bytes,
};
use std::collections::hash_map::DefaultHasher;

/// The generated-token receipt reconstructed from slots, order, and the public framing contract.
fn exact_literal_receipt() -> Vec<u8> {
    let mut bytes = vec![8, 0];
    encode_bytes(b"0xFFu8", &mut bytes);
    bytes.extend_from_slice(&[8, 1]);
    encode_bytes("é".as_bytes(), &mut bytes);
    bytes.extend_from_slice(&[8, 2, 0xff, 8, 3]);
    encode_bytes(b"ab", &mut bytes);
    bytes
}

/// Read one item and return its kind, optional name, and explicit item-level unsafe standing.
fn item_facts(source: &str) -> Result<(AuthoredItemKind, Option<String>, bool), ()> {
    let captured = TextCapture::read(source).map_err(|_| ())?;
    let item = captured.input().authored_item().map_err(|_| ())?;
    assert_eq!(
        item.preserved().canonical_bytes(),
        captured.input().canonical_bytes()
    );
    let name = item.name().map(|(_, spelling)| spelling.to_owned());
    Ok((item.kind(), name, item.unsafe_token().is_some()))
}

/// Claim: one borrowed fragment keeps exact token identity and source handles while projecting structurally into generated Rust.
/// Subject: exact numeric, character, byte, and C-string literals admitted by the text producer.
/// Population: every new guarded exact-literal form.
/// Hostile control: the expected slots, order, and literal material are reconstructed rather than read through the generated-tree encoder.
/// Evidence ceiling: this proves token-level preservation and not downstream type correctness, which remains Rustc's.
#[test]
fn exact_literal_forms_project_without_a_source_string_round_trip() -> Result<(), ()> {
    let captured = TextCapture::read("0xFFu8 'é' b'\\xff' c\"ab\"").map_err(|_| ())?;
    let fragment = captured.input().fragment();
    let generated = fragment.generated().map_err(|_| ())?;
    assert_eq!(generated.canonical_bytes(), exact_literal_receipt());
    assert_eq!(generated.inspected(), "0xFFu8 '\\u{e9}' b'\\xFF' c\"ab\" ");
    assert_eq!(fragment.first_span().map(SpanHandle::index), Some(0));
    assert_eq!(fragment.last_span().map(SpanHandle::index), Some(3));
    Ok(())
}

/// Claim: the item lens covers the required item families and preserves advanced Rust as exact tokens.
/// Subject: authored enum, struct, trait, function, and implementation boundaries.
/// Population: attributes, restricted visibility, lifetimes, type and const generics, bounds, where clauses, GAT syntax, precise capture, and explicit unsafe item qualifiers.
/// Hostile control: each family carries a different name posture and only caller-written unsafe tokens answer as unsafe.
/// Evidence ceiling: the lens recognizes structural envelopes and does not claim to replace Rustc's grammar judgment.
#[test]
fn item_lens_preserves_required_rust_families() -> Result<(), ()> {
    let cases = [
        (
            "pub mod room { pub const WIDTH: usize = 4; }",
            AuthoredItemKind::Module,
            Some("room"),
            false,
        ),
        (
            "#[doc = \"state\"] pub enum State<'a, T, const N: usize> where T: 'a { Ready(&'a T), Bytes([u8; N]) }",
            AuthoredItemKind::Enumeration,
            Some("State"),
            false,
        ),
        (
            "pub union Storage { word: u64, bytes: [u8; 8] }",
            AuthoredItemKind::Union,
            Some("Storage"),
            false,
        ),
        (
            "pub(crate) struct Packet<T, const N: usize>(T, [u8; N]) where T: Copy;",
            AuthoredItemKind::Structure,
            Some("Packet"),
            false,
        ),
        (
            "pub unsafe trait Contract<'a, T> where T: 'a { type Item<'b> where Self: 'b; unsafe fn read(&'a self) -> impl Copy + use<'a, T>; }",
            AuthoredItemKind::Trait,
            Some("Contract"),
            true,
        ),
        (
            "pub const unsafe extern \"C\" fn apply<'a, T: Copy>(value: &'a mut T) -> impl Copy + use<'a, T> where T: 'a { *value }",
            AuthoredItemKind::Function,
            Some("apply"),
            true,
        ),
        (
            "pub extern fn default_abi() {}",
            AuthoredItemKind::Function,
            Some("default_abi"),
            false,
        ),
        (
            "pub const extern fn constant_default_abi() {}",
            AuthoredItemKind::Function,
            Some("constant_default_abi"),
            false,
        ),
        (
            "unsafe impl<'a, T> Contract<'a, T> for Packet<T, 4> where T: Copy + 'a { type Item<'b> = &'b T where Self: 'b; unsafe fn read(&'a self) -> impl Copy + use<'a, T> { &self.0 } }",
            AuthoredItemKind::Implementation,
            None,
            true,
        ),
        (
            "pub type Alias<T> where T: Copy = T;",
            AuthoredItemKind::TypeAlias,
            Some("Alias"),
            false,
        ),
        (
            "pub const WIDTH: usize = 4;",
            AuthoredItemKind::Constant,
            Some("WIDTH"),
            false,
        ),
        (
            "pub static mut COUNT: usize = 0;",
            AuthoredItemKind::Static,
            Some("COUNT"),
            false,
        ),
        (
            "pub static LIMIT: usize = 4;",
            AuthoredItemKind::Static,
            Some("LIMIT"),
            false,
        ),
        (
            "pub use crate::room::WIDTH;",
            AuthoredItemKind::Use,
            None,
            false,
        ),
        (
            "extern crate core;",
            AuthoredItemKind::ExternalCrate,
            Some("core"),
            false,
        ),
    ];
    for (source, expected_kind, expected_name, expected_unsafe) in cases {
        let (kind, name, item_unsafe) = item_facts(source)?;
        assert_eq!(kind, expected_kind);
        assert_eq!(name.as_deref(), expected_name);
        assert_eq!(item_unsafe, expected_unsafe);
    }
    Ok(())
}

/// A structural lens exposes its exact authored fragments without duplicating the complete item.
#[test]
fn one_item_reading_supplies_checked_structural_fragments() -> Result<(), ()> {
    let captured = TextCapture::read(
        "#[doc = \"read\"] pub(crate) async fn read<'a, T, const N: usize>(value: &'a T) -> impl Copy + use<'a, T> where T: Copy + 'a { (value, N) }",
    )
    .map_err(|_| ())?;
    let item = captured.input().authored_item().map_err(|_| ())?;
    assert!(!item.attributes().is_empty());
    assert_eq!(
        item.visibility().generated().map_err(|_| ())?.inspected(),
        "pub ( crate ) "
    );
    assert_eq!(
        item.qualifiers().generated().map_err(|_| ())?.inspected(),
        "async "
    );
    assert_eq!(
        item.generics()
            .ok_or(())?
            .generated()
            .map_err(|_| ())?
            .inspected(),
        "< 'a , T , const N : usize > "
    );
    assert!(
        item.where_clause()
            .ok_or(())?
            .generated()
            .map_err(|_| ())?
            .inspected()
            .starts_with("where T : Copy + 'a ")
    );
    let (delimiter, body) = item.body().ok_or(())?;
    assert_eq!(delimiter, CapturedDelimiter::Brace);
    assert_eq!(
        body.generated().map_err(|_| ())?.inspected(),
        "( value , N ) "
    );
    Ok(())
}

/// A tuple structure keeps its parenthesized fields and following where clause as separate source-coupled fragments.
#[test]
fn tuple_structure_where_clause_remains_visible_after_its_fields() -> Result<(), ()> {
    let captured =
        TextCapture::read("pub struct Packet<T, const N: usize>(T, [u8; N]) where T: Copy;")
            .map_err(|_| ())?;
    let item = captured.input().authored_item().map_err(|_| ())?;
    let (delimiter, fields) = item.body().ok_or(())?;
    assert_eq!(delimiter, CapturedDelimiter::Parenthesis);
    assert_eq!(
        fields.generated().map_err(|_| ())?.inspected(),
        "T , [ u8 ; N ] "
    );
    assert_eq!(
        item.where_clause()
            .ok_or(())?
            .generated()
            .map_err(|_| ())?
            .inspected(),
        "where T : Copy "
    );
    Ok(())
}

/// An explicit unsafe declaration, its safety documentation, and its narrow discharge block remain caller-authored token material.
#[test]
fn explicit_unsafe_custody_moves_the_authored_identity() -> Result<(), ()> {
    let safe =
        TextCapture::read("pub fn read_raw<T: Copy>(pointer: *const T) -> T { pointer.read() }")
            .map_err(|_| ())?;
    let explicit = TextCapture::read(
        "#[doc = \"Read one pointer.\\n\\n# Safety\\n\\nThe pointer must be valid.\"] pub unsafe fn read_raw<T: Copy>(pointer: *const T) -> T { unsafe { pointer.read() } }",
    )
    .map_err(|_| ())?;
    let item = explicit.input().authored_item().map_err(|_| ())?;
    let inspected = item.preserved().generated().map_err(|_| ())?.inspected();
    assert!(!item.attributes().is_empty());
    assert!(item.unsafe_token().is_some());
    assert!(inspected.contains("# Safety"));
    assert!(inspected.contains("unsafe fn read_raw"));
    assert!(inspected.contains("unsafe { pointer . read ( ) }"));
    assert_ne!(
        safe.input().canonical_bytes(),
        explicit.input().canonical_bytes()
    );
    Ok(())
}

/// A caller-owned mechanical read can retain the exact run it consumed, including raw identifiers and lifetimes.
#[test]
fn cursor_read_and_exact_fragment_are_one_operation() -> Result<(), ()> {
    let captured = TextCapture::read("r#type::Item<'a>").map_err(|_| ())?;
    let mut cursor = captured.input().cursor();
    let (fragment, ()) = cursor
        .fragment(|reading| {
            while !reading.is_finished() {
                let _token = reading.token()?;
            }
            Ok(())
        })
        .map_err(|_| ())?;
    cursor.finish().map_err(|_| ())?;
    assert_eq!(
        fragment.canonical_bytes(),
        captured.input().canonical_bytes()
    );
    assert_eq!(
        fragment.generated().map_err(|_| ())?.inspected(),
        "r#type :: Item < 'a > "
    );
    Ok(())
}

/// Producer-coordinate movement changes fragment spans but not generated equality, hashing, canonical bytes, inspection, or debugging.
#[test]
fn span_only_movement_does_not_move_fragment_identity() -> Result<(), ()> {
    let first = one_word(7)?;
    let moved = one_word(700)?;
    let first_fragment = first.fragment();
    let moved_fragment = moved.fragment();
    assert_ne!(first_fragment.first_span(), moved_fragment.first_span());
    assert_eq!(
        first_fragment.canonical_bytes(),
        moved_fragment.canonical_bytes()
    );
    let first_generated = first_fragment.generated().map_err(|_| ())?;
    let moved_generated = moved_fragment.generated().map_err(|_| ())?;
    assert_eq!(first_generated, moved_generated);
    assert_eq!(
        standard_hash(&first_generated),
        standard_hash(&moved_generated)
    );
    assert_eq!(
        first_generated.canonical_bytes(),
        moved_generated.canonical_bytes()
    );
    assert_eq!(first_generated.inspected(), moved_generated.inspected());
    assert_eq!(
        format!("{first_generated:?}"),
        format!("{moved_generated:?}")
    );
    Ok(())
}

/// Manually supplied C-string material with an interior NUL refuses at its captured token.
#[test]
fn invalid_exact_c_string_never_reaches_generation() -> Result<(), ()> {
    let mut builder = CaptureBuilder::declared();
    let level = builder
        .open()
        .atom(44u64, |_| {
            Ok::<_, Infallible>(CapturedAtom::NulTerminatedText(vec![b'a', 0, b'b']))
        })
        .map_err(|_refusal: CaptureBuildRefusal<u64, Infallible>| ())?;
    let input = level.finish();
    let refusal = input.fragment().generated().err().ok_or(())?;
    assert_eq!(
        refusal.issue(),
        FragmentGenerationIssue::Literal(GeneratedLiteralRefusal::InteriorNul)
    );
    assert_eq!(refusal.token().map(SpanHandle::index), Some(0));
    Ok(())
}

/// Every item-envelope refusal retains its exact available site.
#[test]
fn malformed_item_envelopes_refuse_under_typed_causes() -> Result<(), ()> {
    let empty = TextCapture::read("").map_err(|_| ())?;
    let missing = empty.input().authored_item().err().ok_or(())?;
    assert_eq!(missing.issue(), AuthoredItemReadIssue::ItemMissing);
    assert_eq!(missing.token(), None);

    let unknown = TextCapture::read("pub mystery;").map_err(|_| ())?;
    let missing_kind = unknown.input().authored_item().err().ok_or(())?;
    assert_eq!(missing_kind.issue(), AuthoredItemReadIssue::ItemKindMissing);
    assert_eq!(missing_kind.token().map(SpanHandle::index), Some(1));

    for malformed_attribute in ["# pub fn read() {}", "mystery [cfg] pub fn read() {}"] {
        let captured = TextCapture::read(malformed_attribute).map_err(|_| ())?;
        let refusal = captured.input().authored_item().err().ok_or(())?;
        assert_eq!(refusal.issue(), AuthoredItemReadIssue::ItemKindMissing);
        assert_eq!(refusal.token().map(SpanHandle::index), Some(0));
    }

    let unnamed = TextCapture::read("pub struct ;").map_err(|_| ())?;
    let missing_name = unnamed.input().authored_item().err().ok_or(())?;
    assert_eq!(
        missing_name.issue(),
        AuthoredItemReadIssue::ItemNameMissing(AuthoredItemKind::Structure)
    );
    assert_eq!(missing_name.token().map(SpanHandle::index), Some(2));

    let unfinished = TextCapture::read("pub fn read()").map_err(|_| ())?;
    let missing_end = unfinished.input().authored_item().err().ok_or(())?;
    assert_eq!(
        missing_end.issue(),
        AuthoredItemReadIssue::ItemBoundaryUnfinished(AuthoredItemKind::Function)
    );
    assert_eq!(missing_end.token().map(SpanHandle::index), Some(3));
    Ok(())
}

/// Build one captured word with a producer-dependent number of prior handles.
fn one_word(prior: usize) -> Result<macroonz_compiler::CapturedInput, ()> {
    let mut builder = CaptureBuilder::declared();
    for position in 0..prior {
        let position = u64::try_from(position).map_err(|_| ())?;
        let earlier = builder
            .open()
            .atom(position, |_| {
                Ok::<_, Infallible>(CapturedAtom::Word(String::from("earlier")))
            })
            .map_err(|_| ())?;
        let _earlier = earlier.finish();
    }
    let level = builder
        .open()
        .atom(u64::try_from(prior).map_err(|_| ())?, |_| {
            Ok::<_, Infallible>(CapturedAtom::Word(String::from("same")))
        })
        .map_err(|_| ())?;
    Ok(level.finish())
}

/// One process-local observation of a generated tree's ordinary [`Hash`] contract.
fn standard_hash(value: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}
