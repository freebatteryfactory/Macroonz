//! Reading one authored bench declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! #[<helper>(
//!     support = <exported name>,
//!     module = <stamped module name>,
//!     table = named("<namespace>", "<stem>"),
//!     adapter = <adapter module name>,
//!     backend = <backend identifier>,
//!
//!     <lens> {
//!         workload = named("<namespace>", "<stem>"),
//!         preflight = named("<namespace>", "<stem>"),
//!         planted_worse = named("<namespace>", "<stem>"),
//!         complexity = named("<namespace>", "<stem>"),
//!         axis = [<size>, <size>, ...],
//!         samples = <count>,
//!         warmup = <count>,
//!         ratio = <count>,
//!         formula = "<work formula>",
//!         run = <binding>::<segment>::<segment>,
//!         run_worse = <binding>::<segment>,
//!         run_preflight = <binding>::<segment>,
//!         observe = [<binding>::<segment>, ...],
//!     },
//! )]
//! ```
//!
//! The helper's own spelling is the caller's, which is why `<helper>` stands where a word would: a door registers the attribute it wants and hands the same [`Grammar`] to this reading, so a refusal names the word an author actually wrote.
//!
//! `formula` and `observe` may be left out; every other row clause is required.
//! An operation that declares no work formula states that by carrying none, and a row that observes nothing observes nothing.
//!
//! Every count is one unsuffixed decimal literal, because a count that arrives typed, based, or separated is a spelling this reading would have to interpret, and interpreting a spelling is deciding what an author meant by a value it could not read.
//!
//! A callable path opens with the crate binding it is rooted at — `declaring` or `harness` — and the carrier's invocation supplies the crate's real name once, so a consumer that renamed either dependency gets its own name back.
//!
//! # What has no clause, and why
//!
//! The contention posture has none, because one arm is all the declared facts support and a clause with one lawful value is a sentence that says what silence already says.
//! The producer's own act and the consumption target's host facts have none, on the trial grammar's own terms: the first is composed inside the rendering from the emitter the caller declares, and the second arrives as expressions at the carrier's invocation.
//!
//! # Order
//!
//! Clause order inside a body is free and is read by key.
//! Order between ROSTER members is meaning and is preserved: the rows in the order they were written, each axis in the order its sizes were written, and each observation roster in the order its paths were written.

use super::{
    Adapter, Attachment, Backend, BenchCaptureError, Benches, Budgets, ContentionPosture,
    Measurement, References, Row, WorkFormula,
};
use crate::descriptor::{
    Binding, BoundPath, CaptureCause, DeclarationError, FunctionName, Grammar, ModuleName, Name,
    SupportName,
};
use crate::token::{CapturedDelimiter, CapturedTokenTree, SpanHandle};

/// The clause naming the exported support name.
const SUPPORT: &str = "support";

/// The clause naming the stamped module.
const MODULE: &str = "module";

/// The clause naming the authored table.
const TABLE: &str = "table";

/// The clause naming the adapter module.
const ADAPTER: &str = "adapter";

/// The clause naming the measurement backend.
const BACKEND: &str = "backend";

/// The road every namespaced reference in this grammar is spelled by.
const NAMED: &str = "named";

/// The row clause naming what is measured.
const WORKLOAD: &str = "workload";

/// The row clause naming the correctness preflight's reference.
const PREFLIGHT: &str = "preflight";

/// The row clause naming the planted-worse falsifier's reference.
const PLANTED_WORSE: &str = "planted_worse";

/// The row clause naming the neutral complexity claim.
const COMPLEXITY: &str = "complexity";

/// The row clause stating the input-size axis.
const AXIS: &str = "axis";

/// The row clause stating how many samples the gate takes at each point.
const SAMPLES: &str = "samples";

/// The row clause stating how many warmup iterations run before sampling.
const WARMUP: &str = "warmup";

/// The row clause stating the ratio the planted-worse gap must clear.
const RATIO: &str = "ratio";

/// The row clause stating the declared work formula.
const FORMULA: &str = "formula";

/// The row clause naming the callable under measurement.
const RUN: &str = "run";

/// The row clause naming the deliberately worse realization.
const RUN_WORSE: &str = "run_worse";

/// The row clause naming the correctness preflight's own callable.
const RUN_PREFLIGHT: &str = "run_preflight";

/// The row clause stating the work observations the gate reads.
const OBSERVE: &str = "observe";

/// The clause keys this grammar declares at a declaration's own level.
const DECLARABLE: [&str; 5] = [SUPPORT, MODULE, TABLE, ADAPTER, BACKEND];

/// The clause keys one row admits.
///
/// Its own roster rather than the declaration level's, because the two levels admit different keys and one roster standing for both would let a table's clause be written inside a row and read as lawful.
const DECLARABLE_ROW: [&str; 13] = [
    WORKLOAD,
    PREFLIGHT,
    PLANTED_WORSE,
    COMPLEXITY,
    AXIS,
    SAMPLES,
    WARMUP,
    RATIO,
    FORMULA,
    RUN,
    RUN_WORSE,
    RUN_PREFLIGHT,
    OBSERVE,
];

/// Read one bench payload out of the helper attribute's body.
///
/// # Errors
///
/// Returns [`BenchCaptureError`] where the tokens do not say a bench declaration, and where the values they say are not a lawful declaration — each at the token the clause it was established at sits at.
pub fn captured(
    body: &[&CapturedTokenTree],
    at: SpanHandle,
    grammar: Grammar,
) -> Result<Benches, BenchCaptureError> {
    let clauses = declaration_clauses(grammar, body)?;
    let support = SupportName::declared(identifier(grammar, &clauses, SUPPORT, at)?)
        .map_err(|refusal| carried(grammar, refusal, at))?;
    let module = ModuleName::declared(identifier(grammar, &clauses, MODULE, at)?)
        .map_err(|refusal| carried(grammar, refusal, at))?;
    let table = named_reference(grammar, &clauses, TABLE, at)?;
    let adapter_module = ModuleName::declared(identifier(grammar, &clauses, ADAPTER, at)?)
        .map_err(|refusal| carried(grammar, refusal, at))?;
    let backend = Backend::named(identifier(grammar, &clauses, BACKEND, at)?)
        .map_err(|refusal| carried(grammar, refusal, at))?;

    let mut rows: Vec<Row> = Vec::new();
    for clause in &clauses {
        if let Clause::Row {
            lens,
            body: stated,
            at: site,
        } = clause
        {
            rows.push(row(grammar, lens, stated, *site)?);
        }
    }
    Benches::declared(
        support,
        module,
        table,
        rows,
        Adapter::declared(adapter_module, backend),
    )
    .map_err(|refusal| carried(grammar, refusal, at))
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> BenchCaptureError {
    BenchCaptureError::grammar_refused(grammar, cause, at)
}

/// One vocabulary refusal carried whole, at the token the value was read from.
const fn carried(grammar: Grammar, refusal: DeclarationError, at: SpanHandle) -> BenchCaptureError {
    BenchCaptureError::vocabulary_refused(grammar, refusal, at)
}

/// One clause of a bench declaration's body, as the split read it.
///
/// Two shapes rather than one, because the grammar has two: an assignment states one key and one value, and a row states a lens and a body of row clauses.
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
    /// `<lens> { <row clauses> }`.
    Row {
        /// The lens the row is declared under.
        lens: &'trees str,
        /// The trees inside the row body.
        body: Vec<&'trees CapturedTokenTree>,
        /// The token the lens sits at.
        at: SpanHandle,
    },
}

/// Cut one declaration body into its comma-separated clauses, refusing a separator that separates nothing.
///
/// A trailing comma after the last clause is ordinary Rust and lawful; a leading or doubled comma makes an empty group this reader would otherwise silently drop, so it refuses at the comma's own token.
fn declaration_clauses<'trees>(
    grammar: Grammar,
    body: &[&'trees CapturedTokenTree],
) -> Result<Vec<Clause<'trees>>, BenchCaptureError> {
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
/// A group of one word and one brace body is a row; anything else is read as an assignment against the declaration level's keys.
fn close<'trees>(
    grammar: Grammar,
    group: &[&'trees CapturedTokenTree],
    clauses: &mut Vec<Clause<'trees>>,
) -> Result<(), BenchCaptureError> {
    let Some((head, rest)) = group.split_first() else {
        return Ok(());
    };
    if let [body] = rest
        && let Some(lens) = head.word()
        && let Some((CapturedDelimiter::Brace, inner)) = body.group()
    {
        clauses.push(Clause::Row {
            lens,
            body: inner.iter().collect(),
            at: head.span(),
        });
        return Ok(());
    }
    clauses.push(assignment(grammar, head, rest, &DECLARABLE)?);
    Ok(())
}

/// Read one `<key> = <value>` assignment, admitted against the roster its own level declares.
fn assignment<'trees>(
    grammar: Grammar,
    head: &'trees CapturedTokenTree,
    rest: &[&'trees CapturedTokenTree],
    declarable: &[&str],
) -> Result<Clause<'trees>, BenchCaptureError> {
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

/// Refuse where one clause key is stated twice.
///
/// Assigned clauses alone: two rows are two lenses, and the adapter's own lens-namespace law is what tells one from another — stated once at the payload rather than a second time here.
fn distinct(grammar: Grammar, clauses: &[Clause<'_>]) -> Result<(), BenchCaptureError> {
    for (position, clause) in clauses.iter().enumerate() {
        let Clause::Assigned { key, at, .. } = clause else {
            continue;
        };
        let earlier = clauses.iter().take(position).any(|other| match *other {
            Clause::Assigned { key: seen, .. } => seen == *key,
            Clause::Row { .. } => false,
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
        Clause::Assigned { .. } | Clause::Row { .. } => None,
    })
}

/// One identifier a clause assigns.
fn identifier<'trees>(
    grammar: Grammar,
    clauses: &[Clause<'trees>],
    key: &str,
    at: SpanHandle,
) -> Result<&'trees str, BenchCaptureError> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    let [only] = value else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, clause));
    };
    only.word()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, only.span()))
}

/// One unsuffixed decimal count a clause assigns.
fn count(
    grammar: Grammar,
    clauses: &[Clause<'_>],
    key: &str,
    at: SpanHandle,
) -> Result<u64, BenchCaptureError> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    let [only] = value else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, clause));
    };
    number(grammar, only)
}

/// One unsuffixed decimal literal, read as the value it spells.
fn number(grammar: Grammar, tree: &CapturedTokenTree) -> Result<u64, BenchCaptureError> {
    let spelling = tree
        .number()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, tree.span()))?;
    spelling
        .parse::<u64>()
        .map_err(|_| refused(grammar, CaptureCause::ClauseUnread, tree.span()))
}

/// One `named(<namespace>, <stem>)` reference a clause assigns.
fn named_reference(
    grammar: Grammar,
    clauses: &[Clause<'_>],
    key: &str,
    at: SpanHandle,
) -> Result<Name, BenchCaptureError> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    named_value(grammar, value, clause)
}

/// One `named(<namespace>, <stem>)` reference, read off the tokens that spell it.
fn named_value(
    grammar: Grammar,
    value: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<Name, BenchCaptureError> {
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

/// One path rooted at a crate binding, read off the tokens that spell it.
///
/// The shape is a binding word, then one or more `::`-joined segments.
fn bound_path(
    grammar: Grammar,
    value: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<BoundPath, BenchCaptureError> {
    let Some((root, rest)) = value.split_first() else {
        return Err(refused(grammar, CaptureCause::PathUnread, at));
    };
    let binding = match root.word() {
        Some(word) if word == Binding::Declaring.name() => Binding::Declaring,
        Some(word) if word == Binding::Harness.name() => Binding::Harness,
        Some(_) | None => {
            return Err(refused(grammar, CaptureCause::PathUnread, root.span()));
        }
    };
    let mut segments: Vec<String> = Vec::new();
    let mut trees = rest.iter();
    while let Some(first_colon) = trees.next() {
        let Some(second_colon) = trees.next() else {
            return Err(refused(
                grammar,
                CaptureCause::PathUnread,
                first_colon.span(),
            ));
        };
        if first_colon.punct() != Some(':') || second_colon.punct() != Some(':') {
            return Err(refused(
                grammar,
                CaptureCause::PathUnread,
                first_colon.span(),
            ));
        }
        let Some(segment) = trees.next() else {
            return Err(refused(
                grammar,
                CaptureCause::PathUnread,
                second_colon.span(),
            ));
        };
        let Some(spelling) = segment.word() else {
            return Err(refused(grammar, CaptureCause::PathUnread, segment.span()));
        };
        segments.push(spelling.to_owned());
    }
    BoundPath::rooted(binding, segments).map_err(|refusal| carried(grammar, refusal, at))
}

/// One bound path a clause assigns.
fn path_reference(
    grammar: Grammar,
    clauses: &[Clause<'_>],
    key: &str,
    at: SpanHandle,
) -> Result<BoundPath, BenchCaptureError> {
    let (value, clause) =
        assigned(clauses, key).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    bound_path(grammar, value, clause)
}

/// The bracketed axis of input sizes a row states.
fn axis(
    grammar: Grammar,
    clauses: &[Clause<'_>],
    at: SpanHandle,
) -> Result<Vec<u64>, BenchCaptureError> {
    let (value, clause) =
        assigned(clauses, AXIS).ok_or_else(|| refused(grammar, CaptureCause::ClauseAbsent, at))?;
    let [bracketed] = value else {
        return Err(refused(grammar, CaptureCause::RosterUnread, clause));
    };
    let Some((CapturedDelimiter::Bracket, inner)) = bracketed.group() else {
        return Err(refused(
            grammar,
            CaptureCause::RosterUnread,
            bracketed.span(),
        ));
    };
    let mut sizes: Vec<u64> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in inner {
        if tree.punct() == Some(',') {
            match group.as_slice() {
                [] => {
                    return Err(refused(
                        grammar,
                        CaptureCause::SeparatorDangling,
                        tree.span(),
                    ));
                }
                [only] => sizes.push(number(grammar, only)?),
                [first, ..] => {
                    return Err(refused(grammar, CaptureCause::RosterUnread, first.span()));
                }
            }
            group.clear();
        } else {
            group.push(tree);
        }
    }
    match group.as_slice() {
        [] => {}
        [only] => sizes.push(number(grammar, only)?),
        [first, ..] => return Err(refused(grammar, CaptureCause::RosterUnread, first.span())),
    }
    Ok(sizes)
}

/// The declared work formula, where the row states one.
fn formula(
    grammar: Grammar,
    clauses: &[Clause<'_>],
) -> Result<Option<WorkFormula>, BenchCaptureError> {
    let Some((value, at)) = assigned(clauses, FORMULA) else {
        return Ok(None);
    };
    let [only] = value else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, at));
    };
    let text = only
        .text()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, only.span()))?;
    WorkFormula::encoded(text.as_bytes().to_vec())
        .map(Some)
        .map_err(|refusal| carried(grammar, refusal, only.span()))
}

/// The bracketed roster of work-observation paths a row states, or an empty roster where the clause is absent.
fn observations(
    grammar: Grammar,
    clauses: &[Clause<'_>],
) -> Result<Vec<BoundPath>, BenchCaptureError> {
    let Some((value, at)) = assigned(clauses, OBSERVE) else {
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
    let mut observed: Vec<BoundPath> = Vec::new();
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
            observed.push(bound_path(grammar, &group, bracketed.span())?);
            group.clear();
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        observed.push(bound_path(grammar, &group, bracketed.span())?);
    }
    Ok(observed)
}

/// One row: the lens it is declared under, and everything it states about how one workload is measured.
fn row(
    grammar: Grammar,
    lens: &str,
    body: &[&CapturedTokenTree],
    at: SpanHandle,
) -> Result<Row, BenchCaptureError> {
    let named = FunctionName::declared(lens).map_err(|refusal| carried(grammar, refusal, at))?;
    let clauses = row_clauses(grammar, body)?;
    let references = References {
        workload: named_reference(grammar, &clauses, WORKLOAD, at)?,
        correctness_preflight: named_reference(grammar, &clauses, PREFLIGHT, at)?,
        planted_worse: named_reference(grammar, &clauses, PLANTED_WORSE, at)?,
        complexity_claim: named_reference(grammar, &clauses, COMPLEXITY, at)?,
    };
    let sizes = axis(grammar, &clauses, at)?;
    let measurement = Measurement {
        budgets: Budgets {
            samples: count(grammar, &clauses, SAMPLES, at)?,
            warmup: count(grammar, &clauses, WARMUP, at)?,
            ratio_threshold: count(grammar, &clauses, RATIO, at)?,
        },
        contention: ContentionPosture::NoDeclaredContention,
        work_formula: formula(grammar, &clauses)?,
    };
    let attachment = Attachment::measuring(
        path_reference(grammar, &clauses, RUN, at)?,
        path_reference(grammar, &clauses, RUN_WORSE, at)?,
        path_reference(grammar, &clauses, RUN_PREFLIGHT, at)?,
        observations(grammar, &clauses)?,
    )
    .map_err(|refusal| carried(grammar, refusal, at))?;
    Row::declared(named, references, sizes, measurement, attachment)
        .map_err(|refusal| carried(grammar, refusal, at))
}

/// Cut one row body into its comma-separated assignments.
///
/// A row admits no nested row, so the walk reads assignments alone and a lens written inside a row reaches the undeclarable-clause cause with every other key this level does not admit.
fn row_clauses<'trees>(
    grammar: Grammar,
    body: &[&'trees CapturedTokenTree],
) -> Result<Vec<Clause<'trees>>, BenchCaptureError> {
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
