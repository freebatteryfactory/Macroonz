//! The callable text route into the captured tree.
//!
//! The reader is hand-rolled and bounded: it spends the same declared walk
//! every producer spends, and every refusal it establishes names the byte it
//! sits at.
//!
//! The whole route lives here, [`TextCapture::read`] included, because the
//! relationship between a capture and the offsets that resolve its handles is
//! established in exactly one place — the read that issued both.

use super::{
    CaptureBound, CaptureWalk, CapturedDelimiter, CapturedInput, CapturedPayload,
    CapturedTokenTree, SpanHandle, SpanTable, TextCapture, TextReadCause, TextReadRefusal,
    TokenPath,
};
use crate::plane::AuthoringLimitProfile;
use threadpak::types::{AdmittedLimit, Bounded};

impl TextCapture {
    /// Read one declared input from source text.
    ///
    /// # Errors
    ///
    /// Returns [`TextReadRefusal`] naming the established cause and the byte it
    /// sits at.
    /// A cause established with the source read to its end sits at the source's
    /// own length: that is where the read stood when it refused, and it is a
    /// fact the caller can measure against the text it supplied rather than the
    /// zero an absent position renders as.
    pub fn read(source: &str) -> Result<Self, TextReadRefusal> {
        let mut reader = TextReader {
            offsets: Vec::new(),
            walk: CaptureWalk::declared(),
        };
        let mut characters = source.char_indices().peekable();
        let trees = reader.read_group(&mut characters, None, &TokenPath::root())?;
        // Every cause below is established with the source read to its end, so
        // the byte each sits at is the source's own length — the one position an
        // end-of-input refusal has, and a length the caller already holds.
        let end = u64::try_from(source.len()).unwrap_or(u64::MAX);
        let issued = u32::try_from(reader.offsets.len()).unwrap_or(u32::MAX);
        // The table carries one offset per token the walk kept, so it stands
        // under the whole-tree magnitude the walk counted against, and names
        // that magnitude rather than the width of any one level.
        let offsets = Bounded::admitted_const(
            reader.offsets,
            &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
        )
        .map_err(|_| TextReadRefusal {
            cause: TextReadCause::Unbounded(CaptureBound::TreeUnbounded),
            at: end,
        })?;
        // The top level is complete only once the source is, so this is where
        // the per-level magnitude bites on the text route, and it names that
        // magnitude at the byte the level closed at.
        let input = CapturedInput::taken(trees, issued).map_err(|bound| TextReadRefusal {
            cause: TextReadCause::Unbounded(bound),
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

/// The bounded hand-rolled text reader's running state: the byte offset issued
/// for each handle, in handle order, and the declared walk this read spends.
struct TextReader {
    offsets: Vec<u64>,
    walk: CaptureWalk,
}

/// One character stream over source text, with lookahead.
type Characters<'source> = core::iter::Peekable<core::str::CharIndices<'source>>;

impl TextReader {
    /// Issue the next handle for a token starting at one byte offset.
    fn issue(&mut self, at: u64) -> SpanHandle {
        let index = u32::try_from(self.offsets.len()).unwrap_or(u32::MAX);
        self.offsets.push(at);
        SpanHandle::at(index)
    }

    /// Read the tokens of one group, stopping at `closing` where one is given.
    ///
    /// The route this group sits at is carried in, and each token's own route
    /// is that route stepped by the token's position — so a route is built the
    /// same way at every level and no two tokens can share one.
    fn read_group(
        &mut self,
        characters: &mut Characters<'_>,
        closing: Option<(char, u64)>,
        path: &TokenPath,
    ) -> Result<Vec<CapturedTokenTree>, TextReadRefusal> {
        let mut trees: Vec<CapturedTokenTree> = Vec::new();
        loop {
            let Some(&(offset, character)) = characters.peek() else {
                return match closing {
                    Some((_, at)) => Err(TextReadRefusal {
                        cause: TextReadCause::NotBalanced,
                        at,
                    }),
                    None => Ok(trees),
                };
            };
            let at = u64::try_from(offset).unwrap_or(u64::MAX);
            if character.is_whitespace() {
                let _consumed = characters.next();
                continue;
            }
            match group_boundary(character, closing) {
                GroupBoundary::Interior => {}
                GroupBoundary::Closes => {
                    let _consumed = characters.next();
                    return Ok(trees);
                }
                GroupBoundary::NotOpened => {
                    return Err(TextReadRefusal {
                        cause: TextReadCause::NotOpened,
                        at,
                    });
                }
            }
            self.walk.examined().map_err(|bound| TextReadRefusal {
                cause: TextReadCause::Unbounded(bound),
                at,
            })?;
            self.walk.took().map_err(|bound| TextReadRefusal {
                cause: TextReadCause::Unbounded(bound),
                at,
            })?;
            let index = u32::try_from(trees.len()).map_err(|_| TextReadRefusal {
                cause: TextReadCause::Unbounded(CaptureBound::LevelUnbounded),
                at,
            })?;
            let stepped = path.stepped(index).map_err(|bound| TextReadRefusal {
                cause: TextReadCause::Unbounded(bound),
                at,
            })?;
            let tree = self.read_token(characters, at, character, stepped)?;
            trees.push(tree);
        }
    }

    /// Read one token, whatever kind it is.
    fn read_token(
        &mut self,
        characters: &mut Characters<'_>,
        at: u64,
        character: char,
        path: TokenPath,
    ) -> Result<CapturedTokenTree, TextReadRefusal> {
        if let Some((delimiter, closes)) = opening(character) {
            let span = self.issue(at);
            let _consumed = characters.next();
            let inner = self.read_group(characters, Some((closes, at)), &path)?;
            let trees = Bounded::admitted_const(
                inner,
                &AdmittedLimit::<_, AuthoringLimitProfile>::under_profile(),
            )
            .map_err(|_| TextReadRefusal {
                cause: TextReadCause::Unbounded(CaptureBound::LevelUnbounded),
                at,
            })?;
            return Ok(CapturedTokenTree::captured(
                CapturedPayload::Group { delimiter, trees },
                path,
                span,
            ));
        }
        if character.is_alphabetic() || character == '_' {
            let span = self.issue(at);
            let word = read_run(characters, |next| next.is_alphanumeric() || next == '_');
            return Ok(CapturedTokenTree::captured(
                CapturedPayload::Word(word),
                path,
                span,
            ));
        }
        if character.is_ascii_digit() {
            let span = self.issue(at);
            let number = read_run(characters, |next| {
                next.is_alphanumeric() || next == '_' || next == '.'
            });
            return Ok(CapturedTokenTree::captured(
                CapturedPayload::Number(number),
                path,
                span,
            ));
        }
        if character == '"' {
            let span = self.issue(at);
            let _consumed = characters.next();
            let text = read_quoted(characters, at)?;
            return Ok(CapturedTokenTree::captured(
                CapturedPayload::Text(text),
                path,
                span,
            ));
        }
        let span = self.issue(at);
        let _consumed = characters.next();
        Ok(CapturedTokenTree::captured(
            CapturedPayload::Punct(character),
            path,
            span,
        ))
    }
}

/// What one character is to the group currently being read.
///
/// One question with three answers rather than two nested yes-or-nos, which is
/// what makes the unmatched closer a stated outcome rather than the
/// fall-through of a branch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GroupBoundary {
    /// Not a closing character: it belongs to a token inside the group.
    Interior,
    /// The closing character this group was opened with. The group ends here.
    Closes,
    /// A closing character no open group asked for.
    NotOpened,
}

/// The boundary answer for one character, given the closer the group expects.
fn group_boundary(character: char, closing: Option<(char, u64)>) -> GroupBoundary {
    if !matches!(character, ')' | ']' | '}') {
        return GroupBoundary::Interior;
    }
    if closing.map(|(close, _)| close) == Some(character) {
        return GroupBoundary::Closes;
    }
    GroupBoundary::NotOpened
}

/// The run of characters one atom is spelled with, consumed from the stream.
///
/// A word and a number are one operation over two admitted alphabets: take
/// characters while the alphabet admits them, stop at the first it does not.
/// Spelled once, the alphabet is the only thing that differs between them.
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
/// This stage owns both refusals a quoted text can establish — running off the
/// end of the source, and carrying an escape this reader does not interpret —
/// and it is the only place either is decided.
/// The caller supplies the offset the text opened at, so a refusal points at
/// the quote rather than at the byte the reader happened to reach.
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

/// The delimiter one opening character writes and the character that closes it,
/// where it opens a group.
///
/// One row per delimiter this route can write, opener and closer together, so
/// the two cannot drift apart and no delimiter outside the alphabet text spells
/// can be named here at all.
/// The invisible grouping a compiler inserts around a captured fragment has no
/// written characters in either column, which is why it has no row: a reader of
/// text can neither open one nor be looking for the character that closes one.
const fn opening(character: char) -> Option<(CapturedDelimiter, char)> {
    match character {
        '(' => Some((CapturedDelimiter::Parenthesis, ')')),
        '[' => Some((CapturedDelimiter::Bracket, ']')),
        '{' => Some((CapturedDelimiter::Brace, '}')),
        _ => None,
    }
}
