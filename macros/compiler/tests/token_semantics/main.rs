//! The token seam's public paths, canonical bytes, and readable projection observed from outside its crate.
//!
//! The receipts below are rebuilt without the compiler's framing helpers, so a moved slot, delimiter, spacing row, framing boundary, or producer coordinate changes the observation.
//! The generated projection is fixed beside the byte receipt because the projection and the identity bytes are different contracts and neither substitutes for the other.

use core::convert::Infallible;
use macroonz_compiler::token::{
    CaptureBuildRefusal as HomeCaptureBuildRefusal, CaptureBuilder as HomeCaptureBuilder,
    CapturedAtom, CapturedDelimiter, CapturedInput, GeneratedDelimiter, GeneratedLiteral,
    GeneratedToken as HomeGeneratedToken, GeneratedTree as HomeGeneratedTree, rust_keyword,
};
use macroonz_compiler::{CaptureBuildRefusal, CaptureBuilder, GeneratedToken, GeneratedTree};

/// Append one independently framed variable-width token payload.
fn framed(slot: u8, material: &[u8], into: &mut Vec<u8>) {
    into.push(slot);
    into.extend_from_slice(
        &u64::try_from(material.len())
            .unwrap_or(u64::MAX)
            .to_be_bytes(),
    );
    into.extend_from_slice(material);
}

/// The complete edition-2024 keyword roster and neighbouring ordinary names are classified at the public token boundary.
#[test]
fn rust_keywords_are_one_exact_language_roster() {
    let keywords = [
        "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
        "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl",
        "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub",
        "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "try",
        "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
    ];
    assert!(keywords.into_iter().all(rust_keyword));
    assert!(
        [
            "",
            "_",
            "State",
            "async_task",
            "selfish",
            "yielded",
            "r#type"
        ]
        .into_iter()
        .all(|spelling| !rust_keyword(spelling))
    );
}

/// One capture carrying every payload slot and every captured delimiter slot.
fn captured(offset: u64) -> Option<CapturedInput> {
    let mut builder = CaptureBuilder::declared();
    let level = builder.open();
    let level = level
        .atom(offset, |_| {
            Ok::<_, Infallible>(CapturedAtom::Word(String::from("w")))
        })
        .ok()?;
    let level = level
        .atom(offset.saturating_add(1), |_| {
            Ok::<_, Infallible>(CapturedAtom::Punct(':'))
        })
        .ok()?;
    let level = level
        .atom(offset.saturating_add(2), |_| {
            Ok::<_, Infallible>(CapturedAtom::Text(String::from("t")))
        })
        .ok()?;
    let level = level
        .atom(offset.saturating_add(3), |_| {
            Ok::<_, Infallible>(CapturedAtom::Number(String::from("01")))
        })
        .ok()?;
    let level = empty_group(
        level,
        offset.saturating_add(4),
        CapturedDelimiter::Parenthesis,
    )?;
    let level = empty_group(level, offset.saturating_add(5), CapturedDelimiter::Brace)?;
    let level = empty_group(level, offset.saturating_add(6), CapturedDelimiter::Bracket)?;
    let level = empty_group(level, offset.saturating_add(7), CapturedDelimiter::Bare)?;
    let level = level
        .atom(offset.saturating_add(8), |_| {
            Ok::<_, Infallible>(CapturedAtom::ByteText(vec![0, 0xff]))
        })
        .ok()?;
    let level = level
        .atom(offset.saturating_add(9), |_| {
            Ok::<_, Infallible>(CapturedAtom::Character('é'))
        })
        .ok()?;
    let level = level
        .atom(offset.saturating_add(10), |_| {
            Ok::<_, Infallible>(CapturedAtom::Byte(0xff))
        })
        .ok()?;
    let level = level
        .atom(offset.saturating_add(11), |_| {
            Ok::<_, Infallible>(CapturedAtom::NulTerminatedText(vec![b'c']))
        })
        .ok()?;
    let level = level
        .atom(offset.saturating_add(12), |_| {
            Ok::<_, Infallible>(CapturedAtom::RawIdentifier(String::from("type")))
        })
        .ok()?;
    let level = level
        .atom(offset.saturating_add(13), |_| {
            Ok::<_, Infallible>(CapturedAtom::JointPunct('+'))
        })
        .ok()?;
    Some(level.finish())
}

/// Append one empty captured group through the checked builder.
fn empty_group(
    level: macroonz_compiler::CaptureLevel<'_, u64>,
    position: u64,
    delimiter: CapturedDelimiter,
) -> Option<macroonz_compiler::CaptureLevel<'_, u64>> {
    level
        .group(position, delimiter, |_span, inner| {
            Ok::<_, CaptureBuildRefusal<u64, Infallible>>(inner)
        })
        .ok()
}

/// The independently rebuilt bytes for [`captured`].
fn captured_receipt() -> Vec<u8> {
    let mut bytes = Vec::new();
    framed(1, b"w", &mut bytes);
    framed(2, b":", &mut bytes);
    framed(3, b"t", &mut bytes);
    framed(4, b"01", &mut bytes);
    for delimiter in 0u8..=3u8 {
        bytes.extend_from_slice(&[5, delimiter]);
        bytes.extend_from_slice(&0u64.to_be_bytes());
    }
    framed(6, &[0, 0xff], &mut bytes);
    framed(7, "é".as_bytes(), &mut bytes);
    bytes.extend_from_slice(&[8, 0xff]);
    framed(9, b"c", &mut bytes);
    framed(10, b"type", &mut bytes);
    framed(11, b"+", &mut bytes);
    bytes
}

/// One generated tree carrying every token slot, both spacing rows, every generated delimiter slot, and every exact-literal row.
fn generated() -> Option<GeneratedTree> {
    let tokens = vec![
        GeneratedToken::word("word"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::text("a\"\\"),
        GeneratedToken::group(GeneratedDelimiter::Parenthesis, Vec::new()).ok()?,
        GeneratedToken::group(GeneratedDelimiter::Brace, Vec::new()).ok()?,
        GeneratedToken::group(GeneratedDelimiter::Bracket, Vec::new()).ok()?,
        GeneratedToken::group(
            GeneratedDelimiter::Bare,
            vec![GeneratedToken::word("inside")],
        )
        .ok()?,
        GeneratedToken::byte_text(&[0, 0xff]),
        GeneratedToken::number(0x0102_0304_0506_0708),
        GeneratedToken::raw_identifier("type"),
        GeneratedToken::literal(GeneratedLiteral::number("0xFFu8").ok()?),
        GeneratedToken::literal(GeneratedLiteral::character('é')),
        GeneratedToken::literal(GeneratedLiteral::byte(0xff)),
        GeneratedToken::literal(GeneratedLiteral::nul_terminated_text(b"ab").ok()?),
    ];
    GeneratedTree::assembled(tokens).ok()
}

/// The independently rebuilt bytes for [`generated`].
fn generated_receipt() -> Vec<u8> {
    let mut bytes = Vec::new();
    framed(1, b"word", &mut bytes);
    bytes.extend_from_slice(&[2, 0]);
    bytes.extend_from_slice(&1u64.to_be_bytes());
    bytes.push(b':');
    bytes.extend_from_slice(&[2, 1]);
    bytes.extend_from_slice(&1u64.to_be_bytes());
    bytes.push(b':');
    framed(3, b"a\"\\", &mut bytes);
    for delimiter in 0u8..=2u8 {
        bytes.extend_from_slice(&[4, delimiter]);
        bytes.extend_from_slice(&0u64.to_be_bytes());
    }
    bytes.extend_from_slice(&[4, 3]);
    bytes.extend_from_slice(&1u64.to_be_bytes());
    framed(1, b"inside", &mut bytes);
    framed(5, &[0, 0xff], &mut bytes);
    bytes.push(6);
    bytes.extend_from_slice(&0x0102_0304_0506_0708u64.to_be_bytes());
    framed(7, b"type", &mut bytes);
    bytes.push(8);
    framed(0, b"0xFFu8", &mut bytes);
    bytes.push(8);
    framed(1, "é".as_bytes(), &mut bytes);
    bytes.extend_from_slice(&[8, 2, 0xff]);
    bytes.push(8);
    framed(3, b"ab", &mut bytes);
    bytes
}

/// Claim: both established public paths name the same token and capture types.
/// Subject: the crate-root reexports and the public `token` home.
/// Population: the generated token, generated tree, capture builder, and capture refusal types this lane uses.
/// Hostile control: each conversion is typed across the two paths, so a moved or replaced reexport stops compilation.
/// Evidence ceiling: this fixes these established paths and does not claim a facade-root flattening.
#[test]
fn the_root_and_home_paths_name_one_public_surface() {
    let root_token = GeneratedToken::word("same");
    let home_token: HomeGeneratedToken = root_token;
    let root_tree = GeneratedTree::assembled(vec![home_token]);
    let home_tree: Result<HomeGeneratedTree, _> = root_tree;
    let root_builder: CaptureBuilder<u64> = HomeCaptureBuilder::declared();
    let home_builder: HomeCaptureBuilder<u64> = root_builder;
    let refusal: Option<HomeCaptureBuildRefusal<u64, Infallible>> = None;
    assert!(home_tree.is_ok());
    assert!(home_builder.positions().is_empty());
    assert!(refusal.is_none());
}

/// Claim: every captured payload and delimiter retains its exact canonical slot and framing.
/// Subject: two captures with identical token values and different producer coordinates.
/// Population: all eleven captured payload slots and all four captured delimiter slots.
/// Hostile control: the two producer coordinate sequences differ at every token but must reach one receipt.
/// Evidence ceiling: this fixes the current slot table and span exclusion, not future appended rows.
#[test]
fn captured_slots_are_exact_and_producer_coordinates_stay_out() -> Result<(), ()> {
    let first = captured(10).ok_or(())?;
    let moved = captured(1_000).ok_or(())?;
    let expected = captured_receipt();
    assert_eq!(first.canonical_bytes(), expected);
    assert_eq!(moved.canonical_bytes(), expected);
    assert_eq!(first.issued(), 14usize);
    assert_eq!(moved.issued(), 14usize);
    Ok(())
}

/// Claim: every generated token and delimiter retains its exact canonical slot, framing, and readable spelling.
/// Subject: one generated tree carrying all eight token slots, both spacing rows, all four delimiters, and all four exact-literal rows.
/// Population: the complete current generated-token roster.
/// Hostile control: joint and alone punctuation share one mark but encode apart, while the projection makes their spacing difference visible.
/// Evidence ceiling: this fixes canonical bytes and the one-way readable projection, not proc-macro span placement or downstream parsing.
#[test]
fn generated_slots_and_readable_spelling_are_exact() -> Result<(), ()> {
    let tree = generated().ok_or(())?;
    assert_eq!(tree.canonical_bytes(), generated_receipt());
    assert_eq!(
        tree.inspected(),
        r#"word :: "a\"\\" ( ) { } [ ] inside b"\x00\xFF" 72623859790382856 r#type 0xFFu8 '\u{e9}' b'\xFF' c"ab" "#
    );
    Ok(())
}
