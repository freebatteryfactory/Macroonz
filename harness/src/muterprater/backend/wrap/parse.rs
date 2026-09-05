//! The parse role: the grammar spellings, the outcome-word table, and the readers from one console line to what it states.

use crate::muterprater::backend::types::WrapOutcomeWord;
use crate::muterprater::types::{BaselineAxis, SourceCoordinate};

/// The prefix a baseline line's remainder begins with.
const BASELINE_MARKER: &str = "Unmutated baseline";

/// The word a roster line opens with.
const ROSTER_MARKER: &str = "Found";

/// The prefix a roster line's third word begins with.
const ROSTER_SUBJECT: &str = "mutant";

/// The outcome word a qualified baseline is stated under.
const BASELINE_QUALIFIED_WORD: &str = "ok";

/// The backend's outcome words, and what each one states about one mutant.
const OUTCOME_WORDS: [(&str, WrapOutcomeWord); 5] = [
    ("caught", WrapOutcomeWord::Caught),
    ("missed", WrapOutcomeWord::Missed),
    ("unviable", WrapOutcomeWord::Unviable),
    ("timeout", WrapOutcomeWord::TimedOut),
    ("failed", WrapOutcomeWord::ToolFailed),
];

/// What one line of the backend's output states.
///
/// Private to this reading: the public record is [`WrapReading`], and a shape the parser uses on the way there is not vocabulary anybody else writes against.
pub(super) enum LineReading<'line> {
    /// The backend announced its roster count.
    Roster(u32),
    /// The backend reported the unmutated baseline.
    Baseline(BaselineAxis),
    /// The backend reported one mutant.
    Mutant {
        /// The outcome word the line opens with.
        word: WrapOutcomeWord,
        /// Where the damage was placed.
        coordinate: SourceCoordinate,
        /// The backend's own damage text.
        damage: String,
        /// The whole line, for the rejection a kill carries.
        line: &'line str,
    },
    /// The parser does not read this line.
    Unread,
}

/// What one line states, under the grammar this file's page declares.
pub(super) fn read_line(line: &str) -> LineReading<'_> {
    let mut tokens = line.split_whitespace();
    let Some(word) = tokens.next() else {
        return LineReading::Unread;
    };
    let rest = tokens.collect::<Vec<&str>>();
    if let Some(found) = roster_count(word, &rest) {
        return LineReading::Roster(found);
    }
    if rest.join(" ").starts_with(BASELINE_MARKER) {
        return LineReading::Baseline(baseline_axis(word));
    }
    mutant_reading(word, &rest, line)
}

/// The roster count a `Found <count> mutant…` line states.
fn roster_count(word: &str, rest: &[&str]) -> Option<u32> {
    if word != ROSTER_MARKER {
        return None;
    }
    let count = rest.first()?;
    let subject = rest.get(1)?;
    if !subject.starts_with(ROSTER_SUBJECT) {
        return None;
    }
    count.parse::<u32>().ok()
}

/// The baseline axis one outcome word states.
///
/// Only a clean pass qualifies, because no other word is the unchanged passing suite a kill stands on.
fn baseline_axis(word: &str) -> BaselineAxis {
    if word.eq_ignore_ascii_case(BASELINE_QUALIFIED_WORD) {
        BaselineAxis::Qualified
    } else {
        BaselineAxis::Failed
    }
}

/// The mutant reading a line carries, where it carries one.
fn mutant_reading<'line>(word: &str, rest: &[&str], line: &'line str) -> LineReading<'line> {
    let Some(outcome) = outcome_word(word) else {
        return LineReading::Unread;
    };
    let Some(coordinate_text) = rest.first() else {
        return LineReading::Unread;
    };
    let Some(coordinate) = read_coordinate(coordinate_text) else {
        return LineReading::Unread;
    };
    let damage = rest.get(1..).unwrap_or_default().join(" ");
    if damage.is_empty() {
        return LineReading::Unread;
    }
    LineReading::Mutant {
        word: outcome,
        coordinate,
        damage,
        line,
    }
}

/// The outcome word one token states, where it states one.
fn outcome_word(token: &str) -> Option<WrapOutcomeWord> {
    OUTCOME_WORDS
        .iter()
        .find(|(spelling, _)| token.eq_ignore_ascii_case(spelling))
        .map(|(_, word)| *word)
}

/// The coordinate one `<file>:<line>:<column>:` token states.
///
/// Cut from the right, so a path carrying its own colons stays whole in the file part.
fn read_coordinate(token: &str) -> Option<SourceCoordinate> {
    let body = token.strip_suffix(':').unwrap_or(token);
    let mut fields = body.rsplitn(3, ':');
    let column_text = fields.next()?;
    let line_text = fields.next()?;
    let file = fields.next()?;
    let column = column_text.parse::<u32>().ok()?;
    let line = line_text.parse::<u32>().ok()?;
    SourceCoordinate::reported(file, line, column).ok()
}
