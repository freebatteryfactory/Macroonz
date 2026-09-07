//! Outside observations of generated-tree lexical admission and preserved refusal custody.

use core::convert::Infallible;
use macroonz_compiler::bounded::Bounded;
use macroonz_compiler::token::{
    CaptureBuilder, CapturedAtom, CapturedDelimiter, FragmentGenerationIssue, GeneratedDelimiter,
    GeneratedSpacing, GeneratedToken, GeneratedTokenIssue, GeneratedTree, GeneratedTreeRefusal,
};

/// Helper and public-variant spellings both refuse before a completed tree can expose identity bytes.
#[test]
fn malformed_words_and_raw_names_cannot_enter_completed_output() {
    for spelling in ["#", "", "1x", "a b", "a-b", "r#name", "'life", "a\0b", "🦀"] {
        for token in [
            GeneratedToken::word(spelling),
            GeneratedToken::Word(spelling.to_owned()),
        ] {
            assert_eq!(
                GeneratedTree::assembled(vec![token]),
                Err(GeneratedTreeRefusal::Token {
                    position: 0,
                    issue: GeneratedTokenIssue::Word
                }),
                "{spelling:?}",
            );
        }
    }
    for spelling in [
        "#", "", "1x", "a b", "r#name", "_", "self", "Self", "crate", "super",
    ] {
        for token in [
            GeneratedToken::raw_identifier(spelling),
            GeneratedToken::RawIdentifier(spelling.to_owned()),
        ] {
            assert_eq!(
                GeneratedTree::assembled(vec![token]),
                Err(GeneratedTreeRefusal::Token {
                    position: 0,
                    issue: GeneratedTokenIssue::RawIdentifier
                }),
                "{spelling:?}",
            );
        }
    }
}

/// Written and invisible groups cannot hide malformed nested tokens, including through direct public variants.
#[test]
fn nested_punctuation_refusals_retain_the_first_preorder_position()
-> Result<(), Box<dyn core::error::Error>> {
    for mark in ['a', ' ', '(', ')', '[', ']', '{', '}', '"', '\\', '\0', 'é'] {
        for delimiter in [
            GeneratedDelimiter::Parenthesis,
            GeneratedDelimiter::Brace,
            GeneratedDelimiter::Bracket,
            GeneratedDelimiter::Bare,
        ] {
            let members = vec![
                GeneratedToken::word("lawful"),
                GeneratedToken::Punct {
                    mark,
                    spacing: GeneratedSpacing::Joint,
                },
            ];
            let groups = [
                GeneratedToken::group(delimiter, members.clone())?,
                GeneratedToken::Group {
                    delimiter,
                    tokens: Bounded::new(members)?,
                },
            ];
            for group in groups {
                assert_eq!(
                    GeneratedTree::assembled(vec![
                        GeneratedToken::number(7),
                        group,
                        GeneratedToken::word("also invalid")
                    ]),
                    Err(GeneratedTreeRefusal::Token {
                        position: 3,
                        issue: GeneratedTokenIssue::Punctuation
                    }),
                );
            }
        }
    }
    Ok(())
}

/// Programmatic captures cannot bypass output admission and the refusal retains the offending nested producer handle.
#[test]
fn preserved_hostile_atoms_refuse_at_their_own_spans() -> Result<(), ()> {
    for (atom, expected) in [
        (
            CapturedAtom::Word("a b".to_owned()),
            GeneratedTokenIssue::Word,
        ),
        (
            CapturedAtom::RawIdentifier("self".to_owned()),
            GeneratedTokenIssue::RawIdentifier,
        ),
        (CapturedAtom::Punct('x'), GeneratedTokenIssue::Punctuation),
        (
            CapturedAtom::JointPunct(' '),
            GeneratedTokenIssue::Punctuation,
        ),
    ] {
        let mut builder = CaptureBuilder::declared();
        let level = builder
            .open()
            .atom(80u64, |_| {
                Ok::<_, Infallible>(CapturedAtom::Word("lawful".to_owned()))
            })
            .map_err(|_| ())?;
        let level = level
            .group(90, CapturedDelimiter::Bare, |_span, inner| {
                inner.atom(100, |_| Ok::<_, Infallible>(atom))
            })
            .map_err(|_| ())?;
        let capture = level.finish();
        let Err(refusal) = capture.fragment().generated() else {
            return Err(());
        };
        assert_eq!(refusal.issue(), FragmentGenerationIssue::Token(expected));
        assert_eq!(
            refusal.token().map(macroonz_compiler::SpanHandle::index),
            Some(2)
        );
        assert_eq!(builder.positions(), [80, 90, 100]);
    }
    Ok(())
}

/// Lexical words include keywords and underscore, Unicode names stay exact, and punctuation retains both spacing rows.
#[test]
fn lawful_token_roles_are_not_confused_with_item_names() -> Result<(), Box<dyn core::error::Error>>
{
    let words = [
        "pub", "fn", "self", "Self", "super", "crate", "_", "_value", "é", "变量", "e\u{301}",
    ];
    let raw = ["type", "async", "gen", "ordinary", "é", "变量"];
    let mut tokens = words.map(GeneratedToken::word).to_vec();
    tokens.extend(raw.map(GeneratedToken::raw_identifier));
    for mark in "=<>!~+-*/%^&|@.,;:#$?'".chars() {
        tokens.extend([GeneratedToken::joint(mark), GeneratedToken::alone(mark)]);
    }
    tokens.push(GeneratedToken::text("# 1x a b"));
    let tree = GeneratedTree::assembled(tokens.clone())?;
    assert_eq!(tree.tokens(), tokens);
    let copy = GeneratedTree::assembled(tree.tokens().to_vec())?;
    assert_eq!(tree.canonical_bytes(), copy.canonical_bytes());
    Ok(())
}
