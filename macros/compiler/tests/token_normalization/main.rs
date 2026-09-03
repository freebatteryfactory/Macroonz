//! The shared captured-input normalization boundary observed from outside the token home.
//!
//! One source read is compared with a second producer that states the exact proc-macro token distinctions through the checked public builder.
//! This target fixes the normalized value and its identity bytes; the standalone proc witness remains the runtime observer for compiler span custody and invisible groups.

use core::convert::Infallible;
use macroonz_compiler::{
    CaptureBuildRefusal, CaptureBuilder, CaptureLevel, CapturedAtom, CapturedDelimiter,
    CapturedInput, TextCapture, TextLexicalCause, TextReadCause, TextReadRefusal, capture_literal,
};

/// One source spelling carrying every structural distinction this lane compares.
const SOURCE: &str = "r#type::Item<'a, 'r#kind> /* gone */ /// docs\n";

/// Literal spellings covering every value row the text lexer delegates to the literal owner.
const LITERAL_SPELLINGS: &[&str] = &[
    r#""a\nb""#,
    "r#\"raw\"#",
    r#"b"\xff""#,
    r#"br"bytes""#,
    r#"c"nul""#,
    "'é'",
    r"b'\xff'",
    "0xFFu8",
];

/// The same literal roster as one source input.
const LITERAL_SOURCE: &str = r##""a\nb" r#"raw"# b"\xff" br"bytes" c"nul" 'é' b'\xff' 0xFFu8"##;

/// Append one infallible producer atom.
fn atom(
    level: CaptureLevel<'_, u64>,
    position: u64,
    value: CapturedAtom,
) -> Result<CaptureLevel<'_, u64>, CaptureBuildRefusal<u64, Infallible>> {
    level.atom(position, |_| Ok(value))
}

/// Adjacent punctuation keeps every joint position whether or not the pair is a Rust operator.
#[test]
fn compound_punctuation_keeps_proc_macro_spacing() -> Result<(), ()> {
    let text = TextCapture::read(":: ..= <<= -> ++ ?# ,, <-").map_err(|_| ())?;
    let trees = text.input().trees();
    let observed: Vec<(Option<char>, Option<char>)> = trees
        .iter()
        .map(|tree| (tree.joint_punct(), tree.punct()))
        .collect();
    assert_eq!(
        observed,
        [
            (Some(':'), Some(':')),
            (None, Some(':')),
            (Some('.'), Some('.')),
            (Some('.'), Some('.')),
            (None, Some('=')),
            (Some('<'), Some('<')),
            (Some('<'), Some('<')),
            (None, Some('=')),
            (Some('-'), Some('-')),
            (None, Some('>')),
            (Some('+'), Some('+')),
            (None, Some('+')),
            (Some('?'), Some('?')),
            (None, Some('#')),
            (Some(','), Some(',')),
            (None, Some(',')),
            (Some('<'), Some('<')),
            (None, Some('-')),
        ]
    );
    Ok(())
}

/// A lexer-admitted spelling forbidden by raw-identifier grammar refuses before host emission can reach a panicking constructor.
#[test]
fn forbidden_raw_identifier_spelling_refuses_typed() {
    for source in ["r#_", "r#crate", "r#self", "r#Self", "r#super"] {
        assert_eq!(
            TextCapture::read(source),
            Err(TextReadRefusal {
                cause: TextReadCause::Lexical(TextLexicalCause::InvalidIdentifier),
                at: 0,
            })
        );
    }
}

/// Ordinary comments disappear while an inner doc comment becomes the compiler's attribute-shaped token sequence.
#[test]
fn comment_dispositions_match_the_proc_macro_boundary() -> Result<(), ()> {
    let text = TextCapture::read("// ordinary\n/*! inner */").map_err(|_| ())?;
    let [pound, bang, attribute] = text.input().trees() else {
        return Err(());
    };
    assert_eq!(pound.punct(), Some('#'));
    assert_eq!(bang.punct(), Some('!'));
    let Some((CapturedDelimiter::Bracket, [doc, equals, body])) = attribute.group() else {
        return Err(());
    };
    assert_eq!(doc.word(), Some("doc"));
    assert_eq!(equals.punct(), Some('='));
    assert_eq!(body.text(), Some(" inner "));
    assert_eq!(text.input().issued(), 6);
    Ok(())
}

/// The compiler-shaped token stream stated independently through the public checked builder.
fn compiler_shaped_capture() -> Result<CapturedInput, CaptureBuildRefusal<u64, Infallible>> {
    let mut builder = CaptureBuilder::declared();
    let level = builder.open();
    let level = atom(
        level,
        100,
        CapturedAtom::RawIdentifier(String::from("type")),
    )?;
    let level = atom(level, 101, CapturedAtom::JointPunct(':'))?;
    let level = atom(level, 102, CapturedAtom::Punct(':'))?;
    let level = atom(level, 103, CapturedAtom::Word(String::from("Item")))?;
    let level = atom(level, 104, CapturedAtom::Punct('<'))?;
    let level = atom(level, 105, CapturedAtom::JointPunct('\''))?;
    let level = atom(level, 106, CapturedAtom::Word(String::from("a")))?;
    let level = atom(level, 107, CapturedAtom::Punct(','))?;
    let level = atom(level, 108, CapturedAtom::JointPunct('\''))?;
    let level = atom(
        level,
        109,
        CapturedAtom::RawIdentifier(String::from("kind")),
    )?;
    let level = atom(level, 110, CapturedAtom::Punct('>'))?;
    let level = atom(level, 111, CapturedAtom::Punct('#'))?;
    let level = level.group(112, CapturedDelimiter::Bracket, |_span, inner| {
        let inner = atom(inner, 113, CapturedAtom::Word(String::from("doc")))?;
        let inner = atom(inner, 114, CapturedAtom::Punct('='))?;
        atom(inner, 115, CapturedAtom::Text(String::from(" docs")))
    })?;
    Ok(level.finish())
}

/// State the literal roster through the literal owner and checked builder, independently of text lexing.
fn literal_owner_capture() -> Result<CapturedInput, ()> {
    let mut builder = CaptureBuilder::declared();
    let mut level = builder.open();
    for (index, spelling) in LITERAL_SPELLINGS.iter().enumerate() {
        let position = u64::try_from(index).map_err(|_| ())?;
        level = level
            .atom(position, |_| capture_literal(spelling))
            .map_err(|_| ())?;
    }
    Ok(level.finish())
}

/// Source text and a compiler-shaped producer terminate at one normalized value.
#[test]
fn text_and_compiler_shaped_tokens_have_one_normalized_identity() -> Result<(), ()> {
    let text = TextCapture::read(SOURCE).map_err(|_| ())?;
    let compiler = compiler_shaped_capture().map_err(|_| ())?;
    assert_eq!(text.input().canonical_bytes(), compiler.canonical_bytes());
    assert_eq!(text.input().issued(), compiler.issued());
    assert_eq!(text.input().issued(), 16);
    Ok(())
}

/// The low-level lexer delegates every literal value to the existing literal owner without reinterpreting it.
#[test]
fn text_literals_match_the_literal_owner() -> Result<(), ()> {
    let text = TextCapture::read(LITERAL_SOURCE).map_err(|_| ())?;
    let owned = literal_owner_capture()?;
    assert_eq!(text.input().canonical_bytes(), owned.canonical_bytes());
    assert_eq!(text.input().issued(), LITERAL_SPELLINGS.len());
    Ok(())
}

/// Raw identifiers and punctuation adjacency remain directly observable after normalization.
#[test]
fn normalized_tokens_keep_rawness_and_adjacency() -> Result<(), ()> {
    let text = TextCapture::read(SOURCE).map_err(|_| ())?;
    let trees = text.input().trees();
    assert_eq!(
        trees.first().and_then(|tree| tree.raw_identifier()),
        Some("type")
    );
    assert_eq!(
        trees
            .get(1)
            .and_then(macroonz_compiler::CapturedTokenTree::joint_punct),
        Some(':')
    );
    assert_eq!(
        trees
            .get(2)
            .and_then(macroonz_compiler::CapturedTokenTree::punct),
        Some(':')
    );
    assert_eq!(
        trees
            .get(5)
            .and_then(macroonz_compiler::CapturedTokenTree::joint_punct),
        Some('\'')
    );
    assert_eq!(
        trees
            .get(8)
            .and_then(macroonz_compiler::CapturedTokenTree::joint_punct),
        Some('\'')
    );
    assert_eq!(
        trees.get(9).and_then(|tree| tree.raw_identifier()),
        Some("kind")
    );
    Ok(())
}
