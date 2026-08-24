//! The one grammar every line a compiler is handed is composed in.
//!
//! `<prefix>: <class>: <first established issue>[<body>][<site>]`, and there is no second composition anywhere in this crate.
//! Two grammars for one compiler is two shapes a reader has to learn, and the second one is always the one that drifts — it is the one no law was written against.
//!
//! The line is a SUMMARY and says so: the first established issue in full, then how many others there were, then whether the set that names them kept all of them, then where the refusal sits if it sits anywhere narrower than the declaration.
//! The remainder is not lost — every issue has its own identity in the related set, and the typed body is the value the caller of the refusing step holds.

use super::{Door, Line, LineBody, LineSite, SiteCoordinate};
use crate::bounded::Capping;
use crate::token::CoordinateRole;

/// The word one coordinate role counts its positions in.
///
/// Exhaustive on purpose, so a third role stops compiling here rather than being shown under a second role's word.
const fn coordinate_role_word(role: CoordinateRole) -> &'static str {
    match role {
        CoordinateRole::Byte => "byte",
        CoordinateRole::SemanticOrigin => "semantic-origin position",
    }
}

/// How much of a body one line is not carrying, composed from the typed body.
///
/// A capped body examined every issue it established and knows exactly how many it has no room for, so the clause says that rather than telling a reader to re-run a pass that already covered everything.
fn body_clause(body: LineBody) -> String {
    let (further, capping) = match body {
        LineBody::SingleCause => return String::new(),
        LineBody::Body { further, capping } => (further, capping),
    };
    let more = if further > 0 {
        format!(" (and {further} further established issues)")
    } else {
        String::new()
    };
    let kept = match capping {
        Capping::Complete => String::new(),
        Capping::Truncated { omitted } => format!(
            " (every issue was established; {omitted} of them do not fit the declared issue bound)"
        ),
    };
    format!("{more}{kept}")
}

/// Where one line says the refusal sits, composed from the typed coordinate.
///
/// The role travels with the position, so a byte offset never reads as a token ordinal and an ordinal never reads as a byte.
/// Where the producer's table does not reach the handle the clause is that refusal's own rendering: the locating half is missing and the reader is told so, rather than handed a number that means nothing.
fn site_clause(site: LineSite) -> String {
    match site {
        LineSite::WholeDeclaration => String::new(),
        LineSite::At(SiteCoordinate::Resolved(coordinate)) => {
            let word = coordinate_role_word(coordinate.role);
            let position = coordinate.position;
            format!(" (at {word} {position})")
        }
        LineSite::At(SiteCoordinate::NotReached(refusal)) => format!(" ({refusal})"),
    }
}

/// Compose the one line a compiler is handed.
///
/// Every part is read off a typed value — the door's prefix, the class's own sentence, the first issue as the refusing home stated it, and the clauses the body and the site compose — and no phrase here restates any of them in other words.
#[must_use]
pub fn composed(door: &Door, line: &Line<'_>, site: LineSite) -> String {
    let prefix = door.prefix();
    let class = line.class.described();
    let first = line.first;
    let stated_body = body_clause(line.body);
    let stated_site = site_clause(site);
    format!("{prefix}: {class}: {first}{stated_body}{stated_site}")
}

/// One composed line, with the related set's own capping written into it.
///
/// A complete set adds nothing: the line already reads as a summary of a complete body.
/// A capped set says so and says by how much, because the typed capping beside it is not something a compiler shows, and a reader given only the body's own identity would otherwise take the coarser commitment for the full one.
pub(crate) fn witnessed(line: &str, capping: Capping) -> String {
    match capping {
        Capping::Complete => line.to_owned(),
        Capping::Truncated { omitted } => format!(
            "{line} (the related set was capped at the declared issue bound: one identity over the \
             complete body is carried and {omitted} per-issue identities are not)"
        ),
    }
}
