//! Reading one authored trial declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! #[threadpak_trials(
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
//! - `support` is the exported name a CONSUMPTION target invokes this
//!   declaration's carrier by. The physical carrier wears the plan's identity at
//!   full width and nobody can know that spelling before the expansion runs, so
//!   the author states the address and rustc collision-checks it.
//! - `module` is the module the stamp writes the trial table into, at the target
//!   that invokes the carrier.
//! - `table` is the authored table's own namespaced name.
//! - `suite <seat> = named(…) { … }` declares ONE aggregate seat: the ordinary
//!   test function that runs by default, and the execution suite it selects on.
//!   Every row declared under it runs under that suite, structurally.
//! - `<lens> { … }` declares one row: the ignored-by-default test function a
//!   person runs by name, and the references the row states about itself.
//!   `roles` and `tags` are rosters and may be left out; the other four are
//!   required.
//!
//! # What has no clause, and why
//!
//! The PRODUCER's own act — the declaration door, the producer's name, the
//! projection that emitted the rows, and the generated-support schema identity a
//! produced table pins against — is fixed by construction or composed inside the
//! rendering. An author who could state one would be signing an act these
//! services performed.
//!
//! The CONSUMPTION target's host facts — the subject and check revision
//! commitments, the callable that reaches a row's conclusion, the declared
//! budgets, the target and toolchain the runs stand on, and the clock — arrive as
//! expressions at the carrier's own invocation, inside the test target that owns
//! them. A declaration that stated a callable would be naming an item in a crate
//! it is not written in.
//!
//! Every one of those keys reaches
//! [`TrialDeclarationCause::NotADeclarableClause`], whose sentence names them, so
//! an author who reaches for one is told which side of the wall it lives on.
//!
//! # Order
//!
//! Clause order inside a body is free and is read by key, exactly as the
//! refusal-family attribute's is. Order between ROSTER members is meaning and is
//! preserved: the suites in the order they were written, the rows under each seat
//! in the order they were written, and each row's roles and tags in the order
//! they were written — the stamp writes one lens function per row in that order,
//! and the attachment roster the carrier's matcher declares follows it.
//!
//! # Tokens, not text
//!
//! Everything below walks [`CapturedTokenTree`] values. Groups are already
//! groups, so nothing here re-discovers balance, and every refusal names the
//! exact token it was established at rather than a byte somewhere near it.

use super::types::{
    DeclarationDoor, DescriptorRow, RowReferences, ShellDeclarationRefusal, SuiteGroup,
    SupportMacroName, TrialDeclarationCause, TrialDeclarationRefusal, TrialLensName,
    TrialModuleName, TrialSeatName, TrialTablePayload, WallName,
};
use crate::token::{CapturedDelimiter, CapturedTokenTree, SpanHandle};

/// The helper attribute one trial declaration is written in.
///
/// The derive declares it beside `refusal` and this home reads it: the ATTRIBUTE
/// is the door's, and the vocabulary inside it is this home's.
pub const TRIAL_ATTRIBUTE: &str = "threadpak_trials";

/// The clause naming the exported support name.
const SUPPORT_CLAUSE: &str = "support";

/// The clause naming the stamped module.
const MODULE_CLAUSE: &str = "module";

/// The clause naming the authored table.
const TABLE_CLAUSE: &str = "table";

/// The word one aggregate seat's group opens with.
const SUITE_WORD: &str = "suite";

/// The road every namespaced reference in this grammar is spelled by.
const NAMED_ROAD: &str = "named";

/// The row clause naming the claim a row serves.
const CLAIM_CLAUSE: &str = "claim";

/// The row clause naming the roles a row carries.
const ROLES_CLAUSE: &str = "roles";

/// The row clause naming the tags a row carries.
const TAGS_CLAUSE: &str = "tags";

/// The row clause naming what a row exercises.
const SUBJECT_CLAUSE: &str = "subject";

/// The row clause naming the check that judges the subject.
const CHECK_CLAUSE: &str = "check";

/// The row clause naming the population that supplies a row's inputs.
const POPULATION_CLAUSE: &str = "population";

/// The clause keys this grammar declares at a declaration's own level.
///
/// A key outside this roster reaches
/// [`TrialDeclarationCause::NotADeclarableClause`], whose sentence names every
/// seat an author might reach for and says which side of the wall it lives on.
const DECLARABLE_CLAUSES: [&str; 3] = [SUPPORT_CLAUSE, MODULE_CLAUSE, TABLE_CLAUSE];

/// The clause keys one ROW admits.
///
/// Its own roster rather than the declaration level's, because the two levels
/// admit different keys and one roster standing for both would let a table's
/// clause be written inside a row and read as lawful.
const DECLARABLE_ROW_CLAUSES: [&str; 6] = [
    CLAIM_CLAUSE,
    ROLES_CLAUSE,
    TAGS_CLAUSE,
    SUBJECT_CLAUSE,
    CHECK_CLAUSE,
    POPULATION_CLAUSE,
];

/// Read one trial declaration out of the trial attribute's body.
///
/// # Errors
///
/// Returns [`TrialDeclarationRefusal::Grammar`] where the tokens do not say a
/// trial declaration, and [`TrialDeclarationRefusal::Carrier`] where the values
/// they say are not a lawful carrier declaration — each at the token the clause
/// it was established at sits at.
pub fn captured_trials(
    body: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<TrialTablePayload, TrialDeclarationRefusal> {
    let clauses = declaration_clauses(body)?;
    let support = SupportMacroName::declared(identifier(&clauses, SUPPORT_CLAUSE, at)?)
        .map_err(|refusal| carrier(refusal, at))?;
    let module = TrialModuleName::declared(identifier(&clauses, MODULE_CLAUSE, at)?)
        .map_err(|refusal| carrier(refusal, at))?;
    let table = named_reference(&clauses, TABLE_CLAUSE, at)?;

    let mut groups: Vec<SuiteGroup> = Vec::new();
    for clause in &clauses {
        if let Clause::Suite { seat, suite, rows } = clause {
            groups.push(suite_group(seat, suite, rows)?);
        }
    }

    TrialTablePayload::declared(
        support,
        module,
        table,
        DeclarationDoor::RefusalFamilyDerive,
        groups,
    )
    .map_err(|refusal| carrier(refusal, at))
}

/// One established grammar refusal at one token.
const fn grammar(cause: TrialDeclarationCause, at: SpanHandle) -> TrialDeclarationRefusal {
    TrialDeclarationRefusal::Grammar { cause, at }
}

/// One carrier-vocabulary refusal carried whole, at the token the value was read
/// from.
const fn carrier(refusal: ShellDeclarationRefusal, at: SpanHandle) -> TrialDeclarationRefusal {
    TrialDeclarationRefusal::Carrier { refusal, at }
}

/// One clause of a trial declaration's body, as the split read it.
///
/// Two shapes rather than one, because the grammar has two: an assignment states
/// one key and one value, and a suite group states a seat, a reference, and a
/// body of rows. A single shape covering both would make "which of the two is
/// this" a question every reader answers again.
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

/// Cut one declaration body into its comma-separated clauses.
///
/// The walk is one pass and the comma is the separator, exactly as the enum
/// body's own reader cuts variants: a suite group ends at the comma after its
/// braced body, and an assignment ends at the comma after its value.
fn declaration_clauses<'trees>(
    body: &[&'trees CapturedTokenTree],
) -> Result<Vec<Clause<'trees>>, TrialDeclarationRefusal> {
    let mut clauses: Vec<Clause<'trees>> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in body {
        if tree.punct() == Some(',') {
            close_declaration_clause(&group, &mut clauses)?;
            group.clear();
        } else {
            group.push(tree);
        }
    }
    close_declaration_clause(&group, &mut clauses)?;
    distinct(&clauses)?;
    Ok(clauses)
}

/// Close one of a declaration body's comma-separated groups.
///
/// An empty group is a trailing comma and is lawful. Everything else is one of
/// the two shapes, and a group that is neither refuses at its own first token.
fn close_declaration_clause<'trees>(
    group: &[&'trees CapturedTokenTree],
    clauses: &mut Vec<Clause<'trees>>,
) -> Result<(), TrialDeclarationRefusal> {
    let Some((head, rest)) = group.split_first() else {
        return Ok(());
    };
    if head.word() == Some(SUITE_WORD) {
        clauses.push(suite_clause(rest, head.span())?);
        return Ok(());
    }
    clauses.push(assignment(head, rest, &DECLARABLE_CLAUSES)?);
    Ok(())
}

/// Read one `<key> = <value>` assignment off one comma-separated group, admitted
/// against the roster its own level declares.
///
/// The group's first tree arrives separately from the rest, so there is no empty
/// case to answer for and no position to invent for one: a caller that has no
/// first tree has no clause to read.
fn assignment<'trees>(
    head: &'trees CapturedTokenTree,
    rest: &[&'trees CapturedTokenTree],
    declarable: &[&str],
) -> Result<Clause<'trees>, TrialDeclarationRefusal> {
    let opening = head.span();
    let Some(key) = head.word() else {
        return Err(grammar(TrialDeclarationCause::NotAClause, opening));
    };
    let Some((assigned_by, value)) = rest.split_first() else {
        return Err(grammar(TrialDeclarationCause::NotAClause, opening));
    };
    if assigned_by.punct() != Some('=') || value.is_empty() {
        return Err(grammar(
            TrialDeclarationCause::NotAClause,
            assigned_by.span(),
        ));
    }
    if !declarable.contains(&key) {
        return Err(grammar(
            TrialDeclarationCause::NotADeclarableClause,
            opening,
        ));
    }
    Ok(Clause::Assigned {
        key,
        value: value.to_vec(),
        at: opening,
    })
}

/// Read one `suite <seat> = named(…) { <rows> }` clause off the trees after its
/// opening word.
fn suite_clause<'trees>(
    rest: &[&'trees CapturedTokenTree],
    opening: SpanHandle,
) -> Result<Clause<'trees>, TrialDeclarationRefusal> {
    let malformed = || grammar(TrialDeclarationCause::NotASuiteGroup, opening);
    let (seat, after_seat) = rest.split_first().ok_or_else(malformed)?;
    if seat.word().is_none() {
        return Err(grammar(TrialDeclarationCause::NotASuiteGroup, seat.span()));
    }
    let (assigned_by, after_assignment) = after_seat.split_first().ok_or_else(malformed)?;
    if assigned_by.punct() != Some('=') {
        return Err(grammar(
            TrialDeclarationCause::NotASuiteGroup,
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
        Some(_) | None => Err(grammar(TrialDeclarationCause::NotASuiteGroup, body.span())),
    }
}

/// Refuse where one clause key is stated twice.
///
/// Assigned clauses alone: two suite groups are two seats, and the stamped
/// module's own namespace law is what tells one seat from another — stated once
/// at the payload rather than a second time here.
fn distinct(clauses: &[Clause<'_>]) -> Result<(), TrialDeclarationRefusal> {
    for (position, clause) in clauses.iter().enumerate() {
        let Clause::Assigned { key, at, .. } = clause else {
            continue;
        };
        let earlier = clauses.iter().take(position).any(|other| match other {
            Clause::Assigned { key: seen, .. } => seen == key,
            Clause::Suite { .. } => false,
        });
        if earlier {
            return Err(grammar(TrialDeclarationCause::NotDistinct, *at));
        }
    }
    Ok(())
}

/// The value tokens one assigned clause carries, and the token its key sits at.
fn assigned<'trees, 'clauses>(
    clauses: &'clauses [Clause<'trees>],
    key: &str,
) -> Option<(&'clauses [&'trees CapturedTokenTree], SpanHandle)> {
    clauses.iter().find_map(|clause| match clause {
        Clause::Assigned {
            key: named,
            value,
            at,
        } if *named == key => Some((value.as_slice(), *at)),
        Clause::Assigned { .. } | Clause::Suite { .. } => None,
    })
}

/// One identifier a clause assigns.
fn identifier<'trees>(
    clauses: &[Clause<'trees>],
    key: &str,
    at: SpanHandle,
) -> Result<&'trees str, TrialDeclarationRefusal> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| grammar(TrialDeclarationCause::NotCovered, at))?;
    let [only] = value else {
        return Err(grammar(TrialDeclarationCause::NotAClause, clause));
    };
    only.word()
        .ok_or_else(|| grammar(TrialDeclarationCause::NotAClause, only.span()))
}

/// One `named(<namespace>, <stem>)` reference a clause assigns.
fn named_reference(
    clauses: &[Clause<'_>],
    key: &str,
    at: SpanHandle,
) -> Result<WallName, TrialDeclarationRefusal> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| grammar(TrialDeclarationCause::NotCovered, at))?;
    named_value(value, clause)
}

/// One `named(<namespace>, <stem>)` reference, read off the tokens that spell it.
///
/// Exactly that shape and no other: the word, a parenthesized group, and inside
/// it two text literals with one comma between them. A reader that admitted a
/// looser shape would be deciding what an author meant by a value it could not
/// read.
fn named_value(
    value: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<WallName, TrialDeclarationRefusal> {
    let [word, arguments] = value else {
        return Err(grammar(TrialDeclarationCause::NotANamedReference, at));
    };
    if word.word() != Some(NAMED_ROAD) {
        return Err(grammar(
            TrialDeclarationCause::NotANamedReference,
            word.span(),
        ));
    }
    let Some((CapturedDelimiter::Parenthesis, inner)) = arguments.group() else {
        return Err(grammar(
            TrialDeclarationCause::NotANamedReference,
            arguments.span(),
        ));
    };
    let parts: Vec<&CapturedTokenTree> = inner.iter().collect();
    let [namespace, separator, stem] = parts.as_slice() else {
        return Err(grammar(
            TrialDeclarationCause::NotANamedReference,
            arguments.span(),
        ));
    };
    if separator.punct() != Some(',') {
        return Err(grammar(
            TrialDeclarationCause::NotANamedReference,
            separator.span(),
        ));
    }
    let (Some(owner), Some(spelling)) = (namespace.text(), stem.text()) else {
        return Err(grammar(
            TrialDeclarationCause::NotANamedReference,
            arguments.span(),
        ));
    };
    WallName::named(owner, spelling).map_err(|refusal| carrier(refusal, arguments.span()))
}

/// One bracketed roster of namespaced references a row clause assigns, or an
/// empty roster where the clause is absent.
///
/// The roles and the tags are the two clauses this grammar admits EMPTY, and
/// admitting the absent clause as the empty roster is the same statement: a row
/// that classifies itself with nothing is a lawful row, and requiring an author
/// to write `roles = []` would be requiring a sentence that says what silence
/// already says.
fn roster(clauses: &[Clause<'_>], key: &str) -> Result<Vec<WallName>, TrialDeclarationRefusal> {
    let Some((value, at)) = assigned(clauses, key) else {
        return Ok(Vec::new());
    };
    let [bracketed] = value else {
        return Err(grammar(TrialDeclarationCause::NotARoster, at));
    };
    let Some((CapturedDelimiter::Bracket, inner)) = bracketed.group() else {
        return Err(grammar(TrialDeclarationCause::NotARoster, bracketed.span()));
    };
    let mut named: Vec<WallName> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in inner.iter() {
        if tree.punct() == Some(',') {
            if !group.is_empty() {
                named.push(named_value(&group, bracketed.span())?);
            }
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        named.push(named_value(&group, bracketed.span())?);
    }
    Ok(named)
}

/// One aggregate seat's group: the seat, the suite it selects on, and the rows
/// declared under it.
fn suite_group(
    seat: &CapturedTokenTree,
    suite: &[&CapturedTokenTree],
    rows: &[&CapturedTokenTree],
) -> Result<SuiteGroup, TrialDeclarationRefusal> {
    let at = seat.span();
    let spelling = seat
        .word()
        .ok_or_else(|| grammar(TrialDeclarationCause::NotASuiteGroup, at))?;
    let named = TrialSeatName::declared(spelling).map_err(|refusal| carrier(refusal, at))?;
    let selected = named_value(suite, at)?;
    let mut declared: Vec<DescriptorRow> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in rows {
        if tree.punct() == Some(',') {
            if !group.is_empty() {
                declared.push(row(&group, at)?);
            }
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        declared.push(row(&group, at)?);
    }
    SuiteGroup::declared(named, selected, declared).map_err(|refusal| carrier(refusal, at))
}

/// One row: the lens it is declared under, and the references it states about
/// itself.
///
/// The seat's own token is the fallback site rather than an invented position: a
/// group that spells no row is a fact about the seat that declared it, and the
/// reader is sent to the seat rather than to a token this walk never read.
fn row(
    group: &[&CapturedTokenTree],
    seat: SpanHandle,
) -> Result<DescriptorRow, TrialDeclarationRefusal> {
    let [named, body] = group else {
        let at = group.first().map_or(seat, |tree| tree.span());
        return Err(grammar(TrialDeclarationCause::NotARow, at));
    };
    let at = named.span();
    let spelling = named
        .word()
        .ok_or_else(|| grammar(TrialDeclarationCause::NotARow, at))?;
    let lens = TrialLensName::declared(spelling).map_err(|refusal| carrier(refusal, at))?;
    let Some((CapturedDelimiter::Brace, inner)) = body.group() else {
        return Err(grammar(TrialDeclarationCause::NotARow, body.span()));
    };
    let trees: Vec<&CapturedTokenTree> = inner.iter().collect();
    let clauses = row_clauses(&trees)?;
    let references = RowReferences {
        claim: named_reference(&clauses, CLAIM_CLAUSE, at)?,
        subject: named_reference(&clauses, SUBJECT_CLAUSE, at)?,
        check: named_reference(&clauses, CHECK_CLAUSE, at)?,
        population: named_reference(&clauses, POPULATION_CLAUSE, at)?,
    };
    let roles = roster(&clauses, ROLES_CLAUSE)?;
    let tags = roster(&clauses, TAGS_CLAUSE)?;
    DescriptorRow::declared(lens, references, roles, tags).map_err(|refusal| carrier(refusal, at))
}

/// Cut one row body into its comma-separated assignments.
///
/// A row admits no suite group, so the walk reads assignments alone and a `suite`
/// written inside a row reaches the undeclarable-clause cause with every other
/// key this level does not admit.
fn row_clauses<'trees>(
    body: &[&'trees CapturedTokenTree],
) -> Result<Vec<Clause<'trees>>, TrialDeclarationRefusal> {
    let mut clauses: Vec<Clause<'trees>> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in body {
        if tree.punct() == Some(',') {
            if let Some((head, rest)) = group.split_first() {
                clauses.push(assignment(head, rest, &DECLARABLE_ROW_CLAUSES)?);
            }
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if let Some((head, rest)) = group.split_first() {
        clauses.push(assignment(head, rest, &DECLARABLE_ROW_CLAUSES)?);
    }
    distinct(&clauses)?;
    Ok(clauses)
}
