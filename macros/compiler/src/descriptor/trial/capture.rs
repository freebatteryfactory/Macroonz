//! Reading one authored trial declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! #[<helper>(
//!     support = <exported name>,
//!     module = <stamped module name>,
//!     table = named("<namespace>", "<stem>"),
//!
//!     suite <seat> = named("<namespace>", "<stem>") {
//!         <lens> {
//!             claim = named("<namespace>", "<stem>"),
//!             roles = [named("<namespace>", "<stem>"), ...],
//!             tags = [named("<namespace>", "<stem>"), ...],
//!             subject = named("<namespace>", "<stem>"),
//!             check = named("<namespace>", "<stem>"),
//!             population = named("<namespace>", "<stem>"),
//!         },
//!     },
//! )]
//! ```
//!
//! The helper's own spelling is the caller's, which is why `<helper>` stands where a word would: a door registers the attribute it wants and hands the same [`Grammar`] to this reading, so a refusal names the word an author actually wrote.
//!
//! `roles` and `tags` are rosters and may be left out; the other four row clauses are required.
//! A row that classifies itself with nothing is a lawful row, and requiring an author to write `roles = []` would be requiring a sentence that says what silence already says.
//!
//! # What has no clause, and why
//!
//! The producer's own act — the door, the producer's name, and the projection that emitted the rows — is composed inside the rendering from the emitter the caller declares. An author who could state one would be signing an act these services performed.
//!
//! The consumption target's host facts — the two revision commitments, the callable that reaches a row's conclusion, the declared budgets, the target and toolchain, and the clock — arrive as expressions at the carrier's own invocation, inside the test target that owns them.
//!
//! Every one of those keys reaches [`CaptureCause::ClauseUndeclared`].
//!
//! # Order
//!
//! Clause order inside a body is free and is read by key.
//! Order between ROSTER members is meaning and is preserved: the suites in the order they were written, the rows under each seat in the order they were written, and each row's roles and tags in the order they were written.

use super::{References, Row, SuiteGroup, TrialCaptureError, Trials};
use crate::descriptor::{
    CaptureCause, DeclarationError, FunctionName, Grammar, ModuleName, Name, SupportName,
};
use crate::token::{CapturedDelimiter, CapturedTokenTree, SpanHandle};

/// The clause naming the exported support name.
const SUPPORT: &str = "support";

/// The clause naming the stamped module.
const MODULE: &str = "module";

/// The clause naming the authored table.
const TABLE: &str = "table";

/// The word one aggregate seat's group opens with.
const SUITE: &str = "suite";

/// The road every namespaced reference in this grammar is spelled by.
const NAMED: &str = "named";

/// The row clause naming the claim a row serves.
const CLAIM: &str = "claim";

/// The row clause naming the roles a row carries.
const ROLES: &str = "roles";

/// The row clause naming the tags a row carries.
const TAGS: &str = "tags";

/// The row clause naming what a row exercises.
const SUBJECT: &str = "subject";

/// The row clause naming the check that judges the subject.
const CHECK: &str = "check";

/// The row clause naming the population that supplies a row's inputs.
const POPULATION: &str = "population";

/// The clause keys this grammar declares at a declaration's own level.
const DECLARABLE: [&str; 3] = [SUPPORT, MODULE, TABLE];

/// The clause keys one row admits.
///
/// Its own roster rather than the declaration level's, because the two levels admit different keys and one roster standing for both would let a table's clause be written inside a row and read as lawful.
const DECLARABLE_ROW: [&str; 6] = [CLAIM, ROLES, TAGS, SUBJECT, CHECK, POPULATION];

/// Read one trial payload out of the helper attribute's body.
///
/// # Errors
///
/// Returns [`TrialCaptureError`] where the tokens do not say a trial declaration, and where the values they say are not a lawful declaration — each at the token the clause it was established at sits at.
pub fn captured(
    body: &[&CapturedTokenTree],
    at: SpanHandle,
    grammar: Grammar,
) -> Result<Trials, TrialCaptureError> {
    let clauses = declaration_clauses(grammar, body)?;
    let support = SupportName::declared(identifier(grammar, &clauses, SUPPORT, at)?)
        .map_err(|refusal| carried(grammar, refusal, at))?;
    let module = ModuleName::declared(identifier(grammar, &clauses, MODULE, at)?)
        .map_err(|refusal| carried(grammar, refusal, at))?;
    let table = named_reference(grammar, &clauses, TABLE, at)?;

    let mut groups: Vec<SuiteGroup> = Vec::new();
    for clause in &clauses {
        if let Clause::Suite { seat, suite, rows } = clause {
            groups.push(suite_group(grammar, seat, suite, rows)?);
        }
    }
    Trials::declared(support, module, table, groups)
        .map_err(|refusal| carried(grammar, refusal, at))
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> TrialCaptureError {
    TrialCaptureError::grammar_refused(grammar, cause, at)
}

/// One vocabulary refusal carried whole, at the token the value was read from.
const fn carried(grammar: Grammar, refusal: DeclarationError, at: SpanHandle) -> TrialCaptureError {
    TrialCaptureError::vocabulary_refused(grammar, refusal, at)
}

/// One clause of a trial declaration's body, as the split read it.
///
/// Two shapes rather than one, because the grammar has two: an assignment states one key and one value, and a suite group states a seat, a reference, and a body of rows.
enum Clause<'trees> {
    /// `<key> = <value tokens>`.
    Assigned {
        /// The key the clause names.
        key: &'trees str,
        /// The tokens the value is spelled from.
        value: Vec<&'trees CapturedTokenTree>,
        /// The token the key sits at.
        at: SpanHandle,
    },
    /// `suite <seat> = named(…) { <rows> }`.
    Suite {
        /// The seat the group declares.
        seat: &'trees CapturedTokenTree,
        /// The tokens the suite reference is spelled from.
        suite: Vec<&'trees CapturedTokenTree>,
        /// The trees inside the row body.
        rows: Vec<&'trees CapturedTokenTree>,
    },
}

/// Cut one declaration body into its comma-separated clauses, refusing a separator that separates nothing.
///
/// A trailing comma after the last clause is ordinary Rust and lawful; a leading or doubled comma makes an empty group this reader would otherwise silently drop, so it refuses at the comma's own token.
fn declaration_clauses<'trees>(
    grammar: Grammar,
    body: &[&'trees CapturedTokenTree],
) -> Result<Vec<Clause<'trees>>, TrialCaptureError> {
    let mut clauses: Vec<Clause<'trees>> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in body {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            close(grammar, &group, &mut clauses)?;
            group.clear();
        } else {
            group.push(tree);
        }
    }
    close(grammar, &group, &mut clauses)?;
    distinct(grammar, &clauses)?;
    Ok(clauses)
}

/// Close one of a declaration body's comma-separated groups.
///
/// An empty group is a trailing comma and is lawful; an empty group standing at a comma was refused before this road is reached.
fn close<'trees>(
    grammar: Grammar,
    group: &[&'trees CapturedTokenTree],
    clauses: &mut Vec<Clause<'trees>>,
) -> Result<(), TrialCaptureError> {
    let Some((head, rest)) = group.split_first() else {
        return Ok(());
    };
    if head.word() == Some(SUITE) {
        clauses.push(suite_clause(grammar, rest, head.span())?);
        return Ok(());
    }
    clauses.push(assignment(grammar, head, rest, &DECLARABLE)?);
    Ok(())
}

/// Read one `<key> = <value>` assignment, admitted against the roster its own level declares.
///
/// The group's first tree arrives separately from the rest, so there is no empty case to answer for: a caller with no first tree has no clause to read.
fn assignment<'trees>(
    grammar: Grammar,
    head: &'trees CapturedTokenTree,
    rest: &[&'trees CapturedTokenTree],
    declarable: &[&str],
) -> Result<Clause<'trees>, TrialCaptureError> {
    let opening = head.span();
    let Some(key) = head.word() else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening));
    };
    let Some((assigned_by, value)) = rest.split_first() else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening));
    };
    if assigned_by.punct() != Some('=') || value.is_empty() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseUnread,
            assigned_by.span(),
        ));
    }
    if !declarable.contains(&key) {
        return Err(refused(grammar, CaptureCause::ClauseUndeclared, opening));
    }
    Ok(Clause::Assigned {
        key,
        value: value.to_vec(),
        at: opening,
    })
}

/// Read one `suite <seat> = named(…) { <rows> }` clause off the trees after its opening word.
fn suite_clause<'trees>(
    grammar: Grammar,
    rest: &[&'trees CapturedTokenTree],
    opening: SpanHandle,
) -> Result<Clause<'trees>, TrialCaptureError> {
    let malformed = || refused(grammar, CaptureCause::GroupUnread, opening);
    let (seat, after_seat) = rest.split_first().ok_or_else(malformed)?;
    if seat.word().is_none() {
        return Err(refused(grammar, CaptureCause::GroupUnread, seat.span()));
    }
    let (assigned_by, after_assignment) = after_seat.split_first().ok_or_else(malformed)?;
    if assigned_by.punct() != Some('=') {
        return Err(refused(
            grammar,
            CaptureCause::GroupUnread,
            assigned_by.span(),
        ));
    }
    let (body, suite) = after_assignment.split_last().ok_or_else(malformed)?;
    match body.group() {
        Some((CapturedDelimiter::Brace, inner)) => Ok(Clause::Suite {
            seat,
            suite: suite.to_vec(),
            rows: inner.iter().collect(),
        }),
        Some(_) | None => Err(refused(grammar, CaptureCause::GroupUnread, body.span())),
    }
}

/// Refuse where one clause key is stated twice.
///
/// Assigned clauses alone: two suite groups are two seats, and the stamped module's own namespace law is what tells one seat from another — stated once at the payload rather than a second time here.
fn distinct(grammar: Grammar, clauses: &[Clause<'_>]) -> Result<(), TrialCaptureError> {
    for (position, clause) in clauses.iter().enumerate() {
        let Clause::Assigned { key, at, .. } = clause else {
            continue;
        };
        let earlier = clauses.iter().take(position).any(|other| match *other {
            Clause::Assigned { key: seen, .. } => seen == *key,
            Clause::Suite { .. } => false,
        });
        if earlier {
            return Err(refused(grammar, CaptureCause::ClauseDoubled, *at));
        }
    }
    Ok(())
}

/// The value tokens one assigned clause carries, and the token its key sits at.
fn assigned<'trees, 'clauses>(
    clauses: &'clauses [Clause<'trees>],
    key: &str,
) -> Option<(&'clauses [&'trees CapturedTokenTree], SpanHandle)> {
    clauses.iter().find_map(|clause| match *clause {
        Clause::Assigned {
            key: named,
            ref value,
            at,
        } if named == key => Some((value.as_slice(), at)),
        Clause::Assigned { .. } | Clause::Suite { .. } => None,
    })
}

/// One identifier a clause assigns.
fn identifier<'trees>(
    grammar: Grammar,
    clauses: &[Clause<'trees>],
    key: &str,
    at: SpanHandle,
) -> Result<&'trees str, TrialCaptureError> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    let [only] = value else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, clause));
    };
    only.word()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, only.span()))
}

/// One `named(<namespace>, <stem>)` reference a clause assigns.
fn named_reference(
    grammar: Grammar,
    clauses: &[Clause<'_>],
    key: &str,
    at: SpanHandle,
) -> Result<Name, TrialCaptureError> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    named_value(grammar, value, clause)
}

/// One `named(<namespace>, <stem>)` reference, read off the tokens that spell it.
///
/// Exactly that shape and no other: the word, a parenthesized group, and inside it two text literals with one comma between them.
/// A reader that admitted a looser shape would be deciding what an author meant by a value it could not read.
fn named_value(
    grammar: Grammar,
    value: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<Name, TrialCaptureError> {
    let [word, arguments] = value else {
        return Err(refused(grammar, CaptureCause::ReferenceUnread, at));
    };
    if word.word() != Some(NAMED) {
        return Err(refused(grammar, CaptureCause::ReferenceUnread, word.span()));
    }
    let Some((CapturedDelimiter::Parenthesis, inner)) = arguments.group() else {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            arguments.span(),
        ));
    };
    let parts: Vec<&CapturedTokenTree> = inner.iter().collect();
    let [namespace, separator, stem] = parts.as_slice() else {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            arguments.span(),
        ));
    };
    if separator.punct() != Some(',') {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            separator.span(),
        ));
    }
    let (Some(owner), Some(spelling)) = (namespace.text(), stem.text()) else {
        return Err(refused(
            grammar,
            CaptureCause::ReferenceUnread,
            arguments.span(),
        ));
    };
    Name::named(owner, spelling).map_err(|refusal| carried(grammar, refusal, arguments.span()))
}

/// One bracketed roster of namespaced references a row clause assigns, or an empty roster where the clause is absent.
fn roster(
    grammar: Grammar,
    clauses: &[Clause<'_>],
    key: &str,
) -> Result<Vec<Name>, TrialCaptureError> {
    let Some((value, at)) = assigned(clauses, key) else {
        return Ok(Vec::new());
    };
    let [bracketed] = value else {
        return Err(refused(grammar, CaptureCause::RosterUnread, at));
    };
    let Some((CapturedDelimiter::Bracket, inner)) = bracketed.group() else {
        return Err(refused(
            grammar,
            CaptureCause::RosterUnread,
            bracketed.span(),
        ));
    };
    let mut named: Vec<Name> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in inner {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            named.push(named_value(grammar, &group, bracketed.span())?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        named.push(named_value(grammar, &group, bracketed.span())?);
    }
    Ok(named)
}

/// One aggregate seat's group: the seat, the suite it selects on, and the rows declared under it.
fn suite_group(
    grammar: Grammar,
    seat: &CapturedTokenTree,
    suite: &[&CapturedTokenTree],
    rows: &[&CapturedTokenTree],
) -> Result<SuiteGroup, TrialCaptureError> {
    let at = seat.span();
    let spelling = seat
        .word()
        .ok_or_else(|| refused(grammar, CaptureCause::GroupUnread, at))?;
    let named =
        FunctionName::declared(spelling).map_err(|refusal| carried(grammar, refusal, at))?;
    let selected = named_value(grammar, suite, at)?;
    let mut declared: Vec<Row> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in rows {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            declared.push(row(grammar, &group, at)?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        declared.push(row(grammar, &group, at)?);
    }
    SuiteGroup::declared(named, selected, declared).map_err(|refusal| carried(grammar, refusal, at))
}

/// One row: the lens it is declared under, and the references it states about itself.
///
/// The seat's own token is the fallback site rather than an invented position: a group that spells no row is a fact about the seat that declared it.
fn row(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    seat: SpanHandle,
) -> Result<Row, TrialCaptureError> {
    let [named, body] = group else {
        let at = group.first().map_or(seat, |tree| tree.span());
        return Err(refused(grammar, CaptureCause::RowUnread, at));
    };
    let at = named.span();
    let spelling = named
        .word()
        .ok_or_else(|| refused(grammar, CaptureCause::RowUnread, at))?;
    let lens = FunctionName::declared(spelling).map_err(|refusal| carried(grammar, refusal, at))?;
    let Some((CapturedDelimiter::Brace, inner)) = body.group() else {
        return Err(refused(grammar, CaptureCause::RowUnread, body.span()));
    };
    let trees: Vec<&CapturedTokenTree> = inner.iter().collect();
    let clauses = row_clauses(grammar, &trees)?;
    let references = References {
        claim: named_reference(grammar, &clauses, CLAIM, at)?,
        subject: named_reference(grammar, &clauses, SUBJECT, at)?,
        check: named_reference(grammar, &clauses, CHECK, at)?,
        population: named_reference(grammar, &clauses, POPULATION, at)?,
    };
    let roles = roster(grammar, &clauses, ROLES)?;
    let tags = roster(grammar, &clauses, TAGS)?;
    Row::declared(lens, references, roles, tags).map_err(|refusal| carried(grammar, refusal, at))
}

/// Cut one row body into its comma-separated assignments.
///
/// A row admits no suite group, so the walk reads assignments alone and a `suite` written inside a row reaches the undeclarable-clause cause with every other key this level does not admit.
fn row_clauses<'trees>(
    grammar: Grammar,
    body: &[&'trees CapturedTokenTree],
) -> Result<Vec<Clause<'trees>>, TrialCaptureError> {
    let mut clauses: Vec<Clause<'trees>> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in body {
        if tree.punct() == Some(',') {
            let Some((head, rest)) = group.split_first() else {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            };
            clauses.push(assignment(grammar, head, rest, &DECLARABLE_ROW)?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if let Some((head, rest)) = group.split_first() {
        clauses.push(assignment(grammar, head, rest, &DECLARABLE_ROW)?);
    }
    distinct(grammar, &clauses)?;
    Ok(clauses)
}
