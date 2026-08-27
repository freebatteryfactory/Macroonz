//! The callable text route into the normalized captured tree.
//!
//! `ra-ap-rustc_lexer` owns low-level Rust token boundaries and lengths.
//! This file slices the original declared source by those byte lengths, translates every lexer row into this crate's typed capture vocabulary, and sends every retained token through the checked builder.
//! Text never synthesizes an invisible group; only a compiler producer can carry that distinction.

use super::{
    CaptureBound, CaptureBuildRefusal, CaptureBuilder, CaptureLevel, CapturedAtom,
    CapturedDelimiter, CapturedInput, LiteralReadCause, SpanHandle, SpanTable,
    TEXT_SOURCE_BYTE_LIMIT, TextCapture, TextLexicalCause, TextReadCause, TextReadRefusal,
    TokenPath, capture_literal,
};
use crate::bounded::Bounded;
use ra_ap_rustc_lexer::{DocStyle, FrontmatterAllowed, LiteralKind, TokenKind, tokenize};

impl TextCapture {
    /// Read one declared Rust token input from source text.
    ///
    /// # Errors
    ///
    /// Returns [`TextReadRefusal`] naming the established cause and source byte.
    /// The source-byte magnitude is checked before tokenization and is independent of every structural capture magnitude.
    pub fn read(source: &str) -> Result<Self, TextReadRefusal> {
        if source.len() > TEXT_SOURCE_BYTE_LIMIT {
            return Err(TextReadRefusal {
                cause: TextReadCause::SourceBytesUnbounded,
                at: source_offset(TEXT_SOURCE_BYTE_LIMIT),
            });
        }
        let lexemes = lex(source)?;
        let mut builder = CaptureBuilder::declared();
        let mut cursor = 0;
        let level = builder.open();
        let level = capture_sequence(&lexemes, &mut cursor, None, level)
            .map_err(|refusal| text_refusal(&refusal))?;
        let input = level.finish();
        let offsets = Bounded::new(builder.positions().to_vec()).map_err(|_| TextReadRefusal {
            cause: TextReadCause::Unbounded(CaptureBound::Tree),
            at: source_offset(source.len()),
        })?;
        Ok(Self {
            input,
            spans: SpanTable::ByteOffsets(offsets),
        })
    }

    /// The normalized captured input shared with the compiler-token producer.
    #[must_use]
    pub const fn input(&self) -> &CapturedInput {
        &self.input
    }

    /// The byte-offset table that resolves this read's handles.
    #[must_use]
    pub const fn spans(&self) -> &SpanTable {
        &self.spans
    }
}

/// One low-level token together with its exact original source slice.
#[derive(Debug, Clone, Copy)]
struct Lexeme<'source> {
    kind: TokenKind,
    spelling: &'source str,
    at: u64,
}

/// One delimiter expected by an open checked-builder group.
struct Closing {
    kind: ClosingKind,
    at: u64,
    path: TokenPath,
    span: SpanHandle,
}

/// The written delimiter that closes an open group.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ClosingKind {
    Parenthesis,
    Brace,
    Bracket,
}

/// Whether one retained lifetime identifier was written in its ordinary or raw form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LifetimeForm {
    Ordinary,
    Raw,
}

/// Whether one doc comment occupies a line or block spelling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DocForm {
    Line,
    Block,
}

/// Slice the declared source by the exact byte lengths produced by the low-level lexer.
fn lex(source: &str) -> Result<Vec<Lexeme<'_>>, TextReadRefusal> {
    let mut lexemes = Vec::new();
    let mut offset = 0usize;
    for token in tokenize(source, FrontmatterAllowed::No) {
        let length = usize::try_from(token.len).map_err(|_| TextReadRefusal {
            cause: TextReadCause::SourceBytesUnbounded,
            at: source_offset(offset),
        })?;
        let end = offset.checked_add(length).ok_or(TextReadRefusal {
            cause: TextReadCause::SourceBytesUnbounded,
            at: source_offset(offset),
        })?;
        let spelling = source.get(offset..end).ok_or(TextReadRefusal {
            cause: TextReadCause::Lexical(TextLexicalCause::UnknownToken),
            at: source_offset(offset),
        })?;
        lexemes.push(Lexeme {
            kind: token.kind,
            spelling,
            at: source_offset(offset),
        });
        offset = end;
    }
    if offset != source.len() {
        return Err(TextReadRefusal {
            cause: TextReadCause::Lexical(TextLexicalCause::UnknownToken),
            at: source_offset(offset),
        });
    }
    Ok(lexemes)
}

/// Capture one root or delimited sequence.
fn capture_sequence<'capture>(
    lexemes: &[Lexeme<'_>],
    cursor: &mut usize,
    closing: Option<Closing>,
    mut level: CaptureLevel<'capture, u64>,
) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    loop {
        let Some(lexeme) = lexemes.get(*cursor) else {
            return match closing {
                Some(held) => Err(CaptureBuildRefusal::ProducerRefused {
                    cause: refusal(TextReadCause::NotBalanced, held.at),
                    path: held.path,
                    at: held.span,
                }),
                None => Ok(level),
            };
        };
        if closes(lexeme.kind, closing.as_ref()) {
            advance(cursor);
            return Ok(level);
        }
        if is_closing(lexeme.kind) {
            return refuse(level, lexeme.at, TextReadCause::NotOpened);
        }
        if let Some((delimiter, expected)) = opening(lexeme.kind) {
            advance(cursor);
            level = level.group(lexeme.at, delimiter, |span, inner| {
                let path = inner.path().clone();
                capture_sequence(
                    lexemes,
                    cursor,
                    Some(Closing {
                        kind: expected,
                        at: lexeme.at,
                        path,
                        span,
                    }),
                    inner,
                )
            })?;
            continue;
        }
        level = capture_lexeme(lexemes, cursor, level)?;
    }
}

/// Translate one non-delimiter lexeme through the checked capture builder.
fn capture_lexeme<'capture>(
    lexemes: &[Lexeme<'_>],
    cursor: &mut usize,
    level: CaptureLevel<'capture, u64>,
) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    let Some(lexeme) = lexemes.get(*cursor) else {
        return Ok(level);
    };
    let index = *cursor;
    advance(cursor);
    match lexeme.kind {
        TokenKind::Whitespace | TokenKind::LineComment { doc_style: None } => {
            level.examined(lexeme.at)
        }
        TokenKind::BlockComment {
            doc_style: None,
            terminated: true,
        } => level.examined(lexeme.at),
        TokenKind::BlockComment {
            doc_style: _,
            terminated: false,
        } => refuse(
            level,
            lexeme.at,
            TextReadCause::Lexical(TextLexicalCause::BlockCommentNotTerminated),
        ),
        TokenKind::LineComment {
            doc_style: Some(style),
        } => capture_doc(level, lexeme, style, DocForm::Line),
        TokenKind::BlockComment {
            doc_style: Some(style),
            terminated: true,
        } => capture_doc(level, lexeme, style, DocForm::Block),
        TokenKind::Ident => capture_atom(
            level,
            lexeme.at,
            CapturedAtom::Word(lexeme.spelling.to_owned()),
        ),
        TokenKind::RawIdent => capture_prefixed_identifier(level, lexeme, "r#"),
        TokenKind::Lifetime {
            starts_with_number: false,
        } => capture_lifetime(level, lexeme, LifetimeForm::Ordinary),
        TokenKind::RawLifetime => capture_lifetime(level, lexeme, LifetimeForm::Raw),
        TokenKind::Literal { kind, suffix_start } => {
            capture_lexed_literal(level, lexeme, kind, suffix_start)
        }
        TokenKind::Semi => capture_punctuation(level, lexemes, index, ';'),
        TokenKind::Comma => capture_punctuation(level, lexemes, index, ','),
        TokenKind::Dot => capture_punctuation(level, lexemes, index, '.'),
        TokenKind::At => capture_punctuation(level, lexemes, index, '@'),
        TokenKind::Pound => capture_punctuation(level, lexemes, index, '#'),
        TokenKind::Tilde => capture_punctuation(level, lexemes, index, '~'),
        TokenKind::Question => capture_punctuation(level, lexemes, index, '?'),
        TokenKind::Colon => capture_punctuation(level, lexemes, index, ':'),
        TokenKind::Dollar => capture_punctuation(level, lexemes, index, '$'),
        TokenKind::Eq => capture_punctuation(level, lexemes, index, '='),
        TokenKind::Bang => capture_punctuation(level, lexemes, index, '!'),
        TokenKind::Lt => capture_punctuation(level, lexemes, index, '<'),
        TokenKind::Gt => capture_punctuation(level, lexemes, index, '>'),
        TokenKind::Minus => capture_punctuation(level, lexemes, index, '-'),
        TokenKind::And => capture_punctuation(level, lexemes, index, '&'),
        TokenKind::Or => capture_punctuation(level, lexemes, index, '|'),
        TokenKind::Plus => capture_punctuation(level, lexemes, index, '+'),
        TokenKind::Star => capture_punctuation(level, lexemes, index, '*'),
        TokenKind::Slash => capture_punctuation(level, lexemes, index, '/'),
        TokenKind::Caret => capture_punctuation(level, lexemes, index, '^'),
        TokenKind::Percent => capture_punctuation(level, lexemes, index, '%'),
        TokenKind::InvalidIdent => {
            lexical_refusal(level, lexeme, TextLexicalCause::InvalidIdentifier)
        }
        TokenKind::UnknownPrefix => lexical_refusal(level, lexeme, TextLexicalCause::UnknownPrefix),
        TokenKind::UnknownPrefixLifetime => {
            lexical_refusal(level, lexeme, TextLexicalCause::UnknownLifetimePrefix)
        }
        TokenKind::GuardedStrPrefix => {
            lexical_refusal(level, lexeme, TextLexicalCause::GuardedStringPrefix)
        }
        TokenKind::Lifetime {
            starts_with_number: true,
        } => lexical_refusal(level, lexeme, TextLexicalCause::LifetimeStartsWithNumber),
        TokenKind::Frontmatter {
            has_invalid_preceding_whitespace: _,
            invalid_infostring: _,
        } => lexical_refusal(level, lexeme, TextLexicalCause::Frontmatter),
        TokenKind::Unknown
        | TokenKind::OpenParen
        | TokenKind::CloseParen
        | TokenKind::OpenBrace
        | TokenKind::CloseBrace
        | TokenKind::OpenBracket
        | TokenKind::CloseBracket => lexical_refusal(level, lexeme, TextLexicalCause::UnknownToken),
        TokenKind::Eof => Ok(level),
    }
}

/// Capture a literal after checking every malformed flag the low-level lexer exposes.
fn capture_lexed_literal<'capture>(
    level: CaptureLevel<'capture, u64>,
    lexeme: &Lexeme<'_>,
    kind: LiteralKind,
    suffix_start: u32,
) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    let suffix = usize::try_from(suffix_start)
        .ok()
        .and_then(|start| lexeme.spelling.get(start..));
    if let Some(cause) = malformed_literal(kind) {
        return refuse(level, lexeme.at, cause);
    }
    if suffix == Some("_") {
        return lexical_refusal(level, lexeme, TextLexicalCause::MalformedLiteral);
    }
    level.atom(lexeme.at, |_| {
        capture_literal(lexeme.spelling).map_err(|cause| match cause {
            LiteralReadCause::NotReadable if lexeme.spelling.contains('\\') => {
                refusal(TextReadCause::NotEscapeFree, lexeme.at)
            }
            LiteralReadCause::NotAKnownForm | LiteralReadCause::NotReadable => refusal(
                TextReadCause::Lexical(TextLexicalCause::MalformedLiteral),
                lexeme.at,
            ),
        })
    })
}

/// The text refusal established by one malformed low-level literal flag.
const fn malformed_literal(kind: LiteralKind) -> Option<TextReadCause> {
    match kind {
        LiteralKind::Int { base: _, empty_int } => {
            if empty_int {
                Some(TextReadCause::Lexical(TextLexicalCause::MalformedLiteral))
            } else {
                None
            }
        }
        LiteralKind::Float {
            base: _,
            empty_exponent,
        } => {
            if empty_exponent {
                Some(TextReadCause::Lexical(TextLexicalCause::MalformedLiteral))
            } else {
                None
            }
        }
        LiteralKind::Char { terminated }
        | LiteralKind::Byte { terminated }
        | LiteralKind::Str { terminated }
        | LiteralKind::ByteStr { terminated }
        | LiteralKind::CStr { terminated } => {
            if terminated {
                None
            } else {
                Some(TextReadCause::NotTerminated)
            }
        }
        LiteralKind::RawStr { n_hashes }
        | LiteralKind::RawByteStr { n_hashes }
        | LiteralKind::RawCStr { n_hashes } => {
            if n_hashes.is_none() {
                Some(TextReadCause::Lexical(TextLexicalCause::MalformedLiteral))
            } else {
                None
            }
        }
    }
}

/// Capture an ordinary or raw lifetime as the punctuation and identifier tokens a proc macro receives.
fn capture_lifetime<'capture>(
    level: CaptureLevel<'capture, u64>,
    lexeme: &Lexeme<'_>,
    form: LifetimeForm,
) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    let prefix = match form {
        LifetimeForm::Ordinary => "'",
        LifetimeForm::Raw => "'r#",
    };
    let Some(name) = lexeme.spelling.strip_prefix(prefix) else {
        return lexical_refusal(level, lexeme, TextLexicalCause::UnknownLifetimePrefix);
    };
    if form == LifetimeForm::Raw && raw_identifier_is_reserved(name) {
        return lexical_refusal(level, lexeme, TextLexicalCause::InvalidIdentifier);
    }
    let level = capture_atom(level, lexeme.at, CapturedAtom::JointPunct('\''))?;
    let atom = match form {
        LifetimeForm::Ordinary => CapturedAtom::Word(name.to_owned()),
        LifetimeForm::Raw => CapturedAtom::RawIdentifier(name.to_owned()),
    };
    capture_atom(level, lexeme.at, atom)
}

/// Capture a raw identifier after removing its syntax-only prefix.
fn capture_prefixed_identifier<'capture>(
    level: CaptureLevel<'capture, u64>,
    lexeme: &Lexeme<'_>,
    prefix: &str,
) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    let Some(name) = lexeme.spelling.strip_prefix(prefix) else {
        return lexical_refusal(level, lexeme, TextLexicalCause::InvalidIdentifier);
    };
    if raw_identifier_is_reserved(name) {
        return lexical_refusal(level, lexeme, TextLexicalCause::InvalidIdentifier);
    }
    capture_atom(
        level,
        lexeme.at,
        CapturedAtom::RawIdentifier(name.to_owned()),
    )
}

/// Whether one lexer-admitted raw name is forbidden by Rust's raw-identifier grammar.
fn raw_identifier_is_reserved(name: &str) -> bool {
    matches!(name, "" | "_" | "crate" | "self" | "Self" | "super")
}

/// Lower one doc comment into the attribute tokens a proc macro receives.
fn capture_doc<'capture>(
    level: CaptureLevel<'capture, u64>,
    lexeme: &Lexeme<'_>,
    style: DocStyle,
    form: DocForm,
) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    let Some(body) = doc_body(lexeme.spelling, style, form) else {
        return lexical_refusal(level, lexeme, TextLexicalCause::UnknownToken);
    };
    let level = capture_atom(level, lexeme.at, CapturedAtom::Punct('#'))?;
    let level = match style {
        DocStyle::Outer => level,
        DocStyle::Inner => capture_atom(level, lexeme.at, CapturedAtom::Punct('!'))?,
    };
    level.group(lexeme.at, CapturedDelimiter::Bracket, |_span, inner| {
        let inner = capture_atom(inner, lexeme.at, CapturedAtom::Word("doc".to_owned()))?;
        let inner = capture_atom(inner, lexeme.at, CapturedAtom::Punct('='))?;
        capture_atom(inner, lexeme.at, CapturedAtom::Text(body.to_owned()))
    })
}

/// Extract the exact doc-comment body the compiler's implicit `doc` attribute carries.
fn doc_body(spelling: &str, style: DocStyle, form: DocForm) -> Option<&str> {
    let prefix = match (style, form) {
        (DocStyle::Outer, DocForm::Line) => "///",
        (DocStyle::Inner, DocForm::Line) => "//!",
        (DocStyle::Outer, DocForm::Block) => "/**",
        (DocStyle::Inner, DocForm::Block) => "/*!",
    };
    let body = spelling.strip_prefix(prefix)?;
    match form {
        DocForm::Line => Some(body),
        DocForm::Block => body.strip_suffix("*/"),
    }
}

/// Capture one punctuation token with the same adjacency a proc macro observes.
fn capture_punctuation<'capture>(
    level: CaptureLevel<'capture, u64>,
    lexemes: &[Lexeme<'_>],
    index: usize,
    mark: char,
) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    let at = lexemes.get(index).map_or(u64::MAX, |lexeme| lexeme.at);
    let atom = if punctuation_is_joint(lexemes, index) {
        CapturedAtom::JointPunct(mark)
    } else {
        CapturedAtom::Punct(mark)
    };
    capture_atom(level, at, atom)
}

/// Whether one punctuation character is immediately followed by another punctuation token without whitespace.
fn punctuation_is_joint(lexemes: &[Lexeme<'_>], index: usize) -> bool {
    lexemes
        .get(index.saturating_add(1))
        .is_some_and(|lexeme| is_punctuation(lexeme.kind))
}

/// Whether one low-level token becomes a procedural-macro punctuation token.
const fn is_punctuation(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Semi
            | TokenKind::Comma
            | TokenKind::Dot
            | TokenKind::At
            | TokenKind::Pound
            | TokenKind::Tilde
            | TokenKind::Question
            | TokenKind::Colon
            | TokenKind::Dollar
            | TokenKind::Eq
            | TokenKind::Bang
            | TokenKind::Lt
            | TokenKind::Gt
            | TokenKind::Minus
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Plus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Caret
            | TokenKind::Percent
    )
}

/// Capture one already-read atom.
fn capture_atom(
    level: CaptureLevel<'_, u64>,
    at: u64,
    atom: CapturedAtom,
) -> Result<CaptureLevel<'_, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    level.atom(at, |_| Ok(atom))
}

/// Establish one lexical refusal at a lexeme's first byte.
fn lexical_refusal<'capture>(
    level: CaptureLevel<'capture, u64>,
    lexeme: &Lexeme<'_>,
    cause: TextLexicalCause,
) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    refuse(level, lexeme.at, TextReadCause::Lexical(cause))
}

/// Establish one producer refusal through the checked builder's refusal seat.
fn refuse(
    level: CaptureLevel<'_, u64>,
    at: u64,
    cause: TextReadCause,
) -> Result<CaptureLevel<'_, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
    level.atom(at, |_| Err(refusal(cause, at)))
}

/// One byte-positioned text refusal.
const fn refusal(cause: TextReadCause, at: u64) -> TextReadRefusal {
    TextReadRefusal { cause, at }
}

/// Lower the checked builder's refusal into the text route's own byte-positioned refusal.
fn text_refusal(refusal: &CaptureBuildRefusal<u64, TextReadRefusal>) -> TextReadRefusal {
    match refusal {
        CaptureBuildRefusal::Unbounded { bound, at } => TextReadRefusal {
            cause: TextReadCause::Unbounded(*bound),
            at: *at,
        },
        CaptureBuildRefusal::ProducerRefused {
            cause,
            path: _,
            at: _,
        } => *cause,
    }
}

/// Advance one bounded lexeme cursor without arithmetic overflow.
fn advance(cursor: &mut usize) {
    *cursor = cursor.saturating_add(1);
}

/// Convert one source index into the public coordinate width without a panic path.
fn source_offset(offset: usize) -> u64 {
    u64::try_from(offset).unwrap_or(u64::MAX)
}

/// The captured delimiter and closer written by one opening token.
const fn opening(kind: TokenKind) -> Option<(CapturedDelimiter, ClosingKind)> {
    match kind {
        TokenKind::OpenParen => Some((CapturedDelimiter::Parenthesis, ClosingKind::Parenthesis)),
        TokenKind::OpenBrace => Some((CapturedDelimiter::Brace, ClosingKind::Brace)),
        TokenKind::OpenBracket => Some((CapturedDelimiter::Bracket, ClosingKind::Bracket)),
        TokenKind::LineComment { .. }
        | TokenKind::BlockComment { .. }
        | TokenKind::Whitespace
        | TokenKind::Frontmatter { .. }
        | TokenKind::Ident
        | TokenKind::InvalidIdent
        | TokenKind::RawIdent
        | TokenKind::UnknownPrefix
        | TokenKind::UnknownPrefixLifetime
        | TokenKind::RawLifetime
        | TokenKind::GuardedStrPrefix
        | TokenKind::Literal { .. }
        | TokenKind::Lifetime { .. }
        | TokenKind::Semi
        | TokenKind::Comma
        | TokenKind::Dot
        | TokenKind::CloseParen
        | TokenKind::CloseBrace
        | TokenKind::CloseBracket
        | TokenKind::At
        | TokenKind::Pound
        | TokenKind::Tilde
        | TokenKind::Question
        | TokenKind::Colon
        | TokenKind::Dollar
        | TokenKind::Eq
        | TokenKind::Bang
        | TokenKind::Lt
        | TokenKind::Gt
        | TokenKind::Minus
        | TokenKind::And
        | TokenKind::Or
        | TokenKind::Plus
        | TokenKind::Star
        | TokenKind::Slash
        | TokenKind::Caret
        | TokenKind::Percent
        | TokenKind::Unknown
        | TokenKind::Eof => None,
    }
}

/// Whether one token is any written group closer.
const fn is_closing(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::CloseParen | TokenKind::CloseBrace | TokenKind::CloseBracket
    )
}

/// Whether one token closes the group currently being read.
fn closes(kind: TokenKind, closing: Option<&Closing>) -> bool {
    match (kind, closing.map(|held| held.kind)) {
        (TokenKind::CloseParen, Some(ClosingKind::Parenthesis))
        | (TokenKind::CloseBrace, Some(ClosingKind::Brace))
        | (TokenKind::CloseBracket, Some(ClosingKind::Bracket)) => true,
        (
            TokenKind::LineComment { .. }
            | TokenKind::BlockComment { .. }
            | TokenKind::Whitespace
            | TokenKind::Frontmatter { .. }
            | TokenKind::Ident
            | TokenKind::InvalidIdent
            | TokenKind::RawIdent
            | TokenKind::UnknownPrefix
            | TokenKind::UnknownPrefixLifetime
            | TokenKind::RawLifetime
            | TokenKind::GuardedStrPrefix
            | TokenKind::Literal { .. }
            | TokenKind::Lifetime { .. }
            | TokenKind::Semi
            | TokenKind::Comma
            | TokenKind::Dot
            | TokenKind::OpenParen
            | TokenKind::OpenBrace
            | TokenKind::OpenBracket
            | TokenKind::CloseParen
            | TokenKind::CloseBrace
            | TokenKind::CloseBracket
            | TokenKind::At
            | TokenKind::Pound
            | TokenKind::Tilde
            | TokenKind::Question
            | TokenKind::Colon
            | TokenKind::Dollar
            | TokenKind::Eq
            | TokenKind::Bang
            | TokenKind::Lt
            | TokenKind::Gt
            | TokenKind::Minus
            | TokenKind::And
            | TokenKind::Or
            | TokenKind::Plus
            | TokenKind::Star
            | TokenKind::Slash
            | TokenKind::Caret
            | TokenKind::Percent
            | TokenKind::Unknown
            | TokenKind::Eof,
            None | Some(ClosingKind::Parenthesis | ClosingKind::Brace | ClosingKind::Bracket),
        ) => false,
    }
}
