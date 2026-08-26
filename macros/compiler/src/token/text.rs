//! The callable text route into the captured tree.
//!
//! The reader is hand-rolled and bounded: it spends the same declared walk every producer spends, and every refusal it establishes names the byte it sits at.
//! The whole route lives here, [`TextCapture::read`] included, because the relationship between a capture and the offsets that resolve its handles is established in exactly one place — the read that issued both.

use super::{
    CaptureBound, CaptureBuildRefusal, CaptureBuilder, CaptureLevel, CapturedAtom,
    CapturedDelimiter, CapturedInput, SpanHandle, SpanTable, TextCapture, TextReadCause,
    TextReadRefusal, TokenPath,
};
use crate::bounded::Bounded;

impl TextCapture {
    /// Read one declared input from source text.
    ///
    /// # Errors
    ///
    /// Returns [`TextReadRefusal`] naming the established cause and the byte it sits at.
    /// A cause established with the source read to its end sits at the source's own length: that is where the read stood when it refused, and it is a fact the caller can measure against the text it supplied rather than the zero an absent position renders as.
    pub fn read(source: &str) -> Result<Self, TextReadRefusal> {
        let mut reader = TextReader;
        let mut builder = CaptureBuilder::declared();
        let mut characters = source.char_indices().peekable();
        let level = builder.open();
        let level = reader
            .read_group(&mut characters, None, level)
            .map_err(|refusal| text_refusal(&refusal))?;
        let input = level.finish();
        let end = u64::try_from(source.len()).unwrap_or(u64::MAX);
        // The table carries one offset per token the walk kept, so it stands under the whole-tree magnitude the walk counted against rather than the width of any one level.
        let offsets = Bounded::new(builder.positions().to_vec()).map_err(|_| TextReadRefusal {
            cause: TextReadCause::Unbounded(CaptureBound::Tree),
            at: end,
        })?;
        Ok(Self {
            input,
            spans: SpanTable::ByteOffsets(offsets),
        })
    }

    /// The captured input.
    #[must_use]
    pub const fn input(&self) -> &CapturedInput {
        &self.input
    }

    /// The table that resolves this read's handles.
    #[must_use]
    pub const fn spans(&self) -> &SpanTable {
        &self.spans
    }
}

/// The stateless source reader that feeds the checked capture builder.
struct TextReader;

/// One character stream over source text, with lookahead.
type Characters<'source> = core::iter::Peekable<core::str::CharIndices<'source>>;

/// The facts retained while reading one open source group.
struct GroupClosing {
    /// The character that closes the group.
    delimiter: char,
    /// The source byte where the group opened.
    at: u64,
    /// The declaration-local route to the group token.
    path: TokenPath,
    /// The producer-local handle for the group's opening span.
    span: SpanHandle,
}

impl TextReader {
    /// Read the tokens of one group, stopping at `closing` where one is given.
    ///
    /// The builder level owns every route, handle, and magnitude while this reader owns only the character grammar.
    fn read_group<'capture>(
        &mut self,
        characters: &mut Characters<'_>,
        closing: Option<GroupClosing>,
        mut level: CaptureLevel<'capture, u64>,
    ) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
        loop {
            let Some(&(offset, character)) = characters.peek() else {
                return match closing {
                    Some(GroupClosing {
                        delimiter: _,
                        at,
                        path,
                        span,
                    }) => Err(CaptureBuildRefusal::ProducerRefused {
                        cause: TextReadRefusal {
                            cause: TextReadCause::NotBalanced,
                            at,
                        },
                        path,
                        at: span,
                    }),
                    None => Ok(level),
                };
            };
            let at = u64::try_from(offset).unwrap_or(u64::MAX);
            if character.is_whitespace() {
                let _consumed = characters.next();
                continue;
            }
            match group_boundary(character, closing.as_ref()) {
                GroupBoundary::Interior => {}
                GroupBoundary::Closes => {
                    let _consumed = characters.next();
                    return Ok(level);
                }
                GroupBoundary::NotOpened => {
                    let refusal = TextReadRefusal {
                        cause: TextReadCause::NotOpened,
                        at,
                    };
                    return level.atom(at, |_| Err(refusal));
                }
            }
            level = self.read_token(characters, at, character, level)?;
        }
    }

    /// Read one token, whatever kind it is.
    fn read_token<'capture>(
        &mut self,
        characters: &mut Characters<'_>,
        at: u64,
        character: char,
        level: CaptureLevel<'capture, u64>,
    ) -> Result<CaptureLevel<'capture, u64>, CaptureBuildRefusal<u64, TextReadRefusal>> {
        if let Some((delimiter, closes)) = opening(character) {
            let _consumed = characters.next();
            return level.group(at, delimiter, |handle, inner| {
                let path = inner.path().clone();
                self.read_group(
                    characters,
                    Some(GroupClosing {
                        delimiter: closes,
                        at,
                        path,
                        span: handle,
                    }),
                    inner,
                )
            });
        }
        if character.is_alphabetic() || character == '_' {
            let word = read_run(characters, |next| next.is_alphanumeric() || next == '_');
            return level.atom(at, |_| Ok(CapturedAtom::Word(word)));
        }
        if character.is_ascii_digit() {
            let number = read_run(characters, |next| {
                next.is_alphanumeric() || next == '_' || next == '.'
            });
            return level.atom(at, |_| Ok(CapturedAtom::Number(number)));
        }
        if character == '"' {
            let _consumed = characters.next();
            return level.atom(at, |_| read_quoted(characters, at).map(CapturedAtom::Text));
        }
        let _consumed = characters.next();
        level.atom(at, |_| Ok(CapturedAtom::Punct(character)))
    }
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

/// What one character is to the group currently being read.
///
/// One question with three answers rather than two nested yes-or-nos, which is what makes the unmatched closer a stated outcome rather than the fall-through of a branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GroupBoundary {
    /// Not a closing character: it belongs to a token inside the group.
    Interior,
    /// The closing character this group was opened with; the group ends here.
    Closes,
    /// A closing character no open group asked for.
    NotOpened,
}

/// The boundary answer for one character, given the closer the group expects.
fn group_boundary(character: char, closing: Option<&GroupClosing>) -> GroupBoundary {
    if !matches!(character, ')' | ']' | '}') {
        return GroupBoundary::Interior;
    }
    if closing.map(|held| held.delimiter) == Some(character) {
        return GroupBoundary::Closes;
    }
    GroupBoundary::NotOpened
}

/// The run of characters one atom is spelled with, consumed from the stream.
///
/// A word and a number are one operation over two admitted alphabets: take characters while the alphabet admits them, stop at the first it does not.
fn read_run(characters: &mut Characters<'_>, admits: fn(char) -> bool) -> String {
    let mut spelled = String::new();
    while let Some(&(_, next)) = characters.peek() {
        if !admits(next) {
            break;
        }
        spelled.push(next);
        let _consumed = characters.next();
    }
    spelled
}

/// The body of one quoted text, with the opening quote already consumed.
///
/// This stage owns both refusals a quoted text can establish — running off the end of the source, and carrying an escape this reader does not interpret — and it is the only place either is decided.
/// The caller supplies the offset the text opened at, so a refusal points at the quote rather than at the byte the reader happened to reach.
fn read_quoted(characters: &mut Characters<'_>, at: u64) -> Result<String, TextReadRefusal> {
    let mut text = String::new();
    loop {
        let Some((_, next)) = characters.next() else {
            return Err(TextReadRefusal {
                cause: TextReadCause::NotTerminated,
                at,
            });
        };
        if next == '"' {
            return Ok(text);
        }
        if next == '\\' {
            return Err(TextReadRefusal {
                cause: TextReadCause::NotEscapeFree,
                at,
            });
        }
        text.push(next);
    }
}

/// The delimiter one opening character writes and the character that closes it, where it opens a group.
///
/// One row per delimiter this route can write, opener and closer together, so the two cannot drift apart and no delimiter outside the alphabet text spells can be named here at all.
/// The invisible grouping a compiler inserts around a captured fragment has no written characters in either column, which is why it has no row.
const fn opening(character: char) -> Option<(CapturedDelimiter, char)> {
    match character {
        '(' => Some((CapturedDelimiter::Parenthesis, ')')),
        '[' => Some((CapturedDelimiter::Bracket, ']')),
        '{' => Some((CapturedDelimiter::Brace, '}')),
        _ => None,
    }
}
