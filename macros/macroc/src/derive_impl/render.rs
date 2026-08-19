//! The token half of the road: the active-point enum, one point's selection,
//! and the single walk that turns the production tree into the evaluation copy.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every brace is a group, and no function
//! here composes Rust source. The Rust a person reads is
//! [`GeneratedTree::inspected`], a projection of what is emitted rather than the
//! thing itself.
//!
//! # Selection, and never interpretation
//!
//! A point renders as an ordinary `match` over a closed enum. Every variant the
//! whole table declares gets an arm — the point's own alternatives get their
//! alternative's tokens, every other variant gets this point's ORIGINAL tokens,
//! and so does the control. There is no wildcard arm anywhere: a variant added
//! without an arm stops the consumer's compiler rather than falling through to
//! whatever the last arm said, and a selection with no `_` cannot quietly become
//! a default.
//!
//! No `cfg`, no feature gate, and no test switch is written by any function
//! here. The selector exists on the evaluation surface and on nothing else.
//!
//! # One walk, and what it establishes
//!
//! The transform is a SINGLE walk over the production tree. Every point is
//! offered each position at once, and the first point whose original operation
//! starts there consumes it — so no point can be substituted into tokens another
//! point's selection just inserted, and the copy is a function of the production
//! tree rather than of the order the points happened to run in.
//!
//! Two facts come off that walk and both are refusals rather than repairs: a
//! point whose operation the production tree does not contain exactly once
//! ([`occurrences`] answers before anything is substituted), and a point whose
//! one occurrence sat inside another point's operation, which the walk reports
//! by having substituted it nowhere.

use super::types::{
    EvaluationBinding, ImplementationSurfaceIssue, MutationOperation, MutationPoint,
    MutationPointTable, NO_MUTATION_VARIANT,
};
use crate::plane::GeneratedTokenLimit;
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};
use threadpak::types::{BoundedConstruction, ConstLimit};

/// One point's rendered halves, and how many times the walk stood it in.
///
/// The count is written by the walk itself rather than supplied to it, which is
/// what makes "this point was substituted exactly once" a fact about what
/// happened instead of a claim about what should have.
struct PointRendering {
    operation: Vec<GeneratedToken>,
    selection: Vec<GeneratedToken>,
    performed: usize,
}

impl PointRendering {
    /// Whether this point's operation starts at the position the tail begins at.
    ///
    /// An empty operation starts nowhere: it names no tokens, so it can neither
    /// be found nor consumed, and admitting it would match at every position.
    fn starts(&self, tail: Option<&[GeneratedToken]>) -> bool {
        !self.operation.is_empty()
            && tail.is_some_and(|material| material.starts_with(&self.operation))
    }
}

/// The Rust variant one alternative is rendered as.
///
/// Positional, and deliberately so: a variant spelling derived from a point's
/// declared name would be this home deciding how an owner's namespaced text
/// becomes a Rust identifier, which is a spelling law nobody gave it. The
/// declared names travel as the table's own data; the enum carries positions,
/// and the two are joined by the position rather than by a transliteration.
#[must_use]
pub fn variant_spelling(point: usize, alternative: usize) -> String {
    format!("Point{point}Alternative{alternative}")
}

/// The active-point enum the evaluation copy selects over: the control at the
/// first position, then one variant per admitted alternative in table order.
///
/// # Errors
///
/// Returns [`BoundedConstruction::OverLimit`] where the variant roster outgrows
/// the declared token magnitude.
pub fn active_point_enum(
    binding: &EvaluationBinding,
    table: &MutationPointTable,
) -> Result<Vec<GeneratedToken>, BoundedConstruction> {
    let mut variants: Vec<GeneratedToken> = vec![
        GeneratedToken::word(NO_MUTATION_VARIANT),
        GeneratedToken::alone(','),
    ];
    for (position, point) in table.admitted().enumerate() {
        for (alternative, _operation) in point.alternatives().enumerate() {
            variants.push(GeneratedToken::word(
                variant_spelling(position, alternative).as_str(),
            ));
            variants.push(GeneratedToken::alone(','));
        }
    }
    Ok(vec![
        GeneratedToken::alone('#'),
        derive_attribute()?,
        GeneratedToken::word("enum"),
        GeneratedToken::word(binding.active_enum()),
        GeneratedToken::group(GeneratedDelimiter::Brace, variants)?,
    ])
}

/// The selection one point stands under: a `match` over the whole active-point
/// roster, with an arm for every variant and no wildcard.
///
/// The subject point's own alternatives render their alternative's tokens; every
/// other variant, and the control, render the subject point's ORIGINAL tokens.
/// That is what makes the control's meaning uniform across a table of any size:
/// under the control every point is original, so the evaluation copy emits the
/// production surface's own operations.
///
/// # Errors
///
/// Returns [`BoundedConstruction::OverLimit`] where the arms outgrow the
/// declared token magnitude.
pub fn selection(
    binding: &EvaluationBinding,
    table: &MutationPointTable,
    at: usize,
    point: &MutationPoint,
) -> Result<Vec<GeneratedToken>, BoundedConstruction> {
    let original: Vec<GeneratedToken> = point.original().tree().tokens().cloned().collect();
    let mut arms: Vec<GeneratedToken> = arm(binding.active_enum(), NO_MUTATION_VARIANT, &original)?;
    for (position, other) in table.admitted().enumerate() {
        for (alternative, operation) in other.alternatives().enumerate() {
            let body = arm_body((position == at).then_some(operation), &original);
            arms.extend(arm(
                binding.active_enum(),
                variant_spelling(position, alternative).as_str(),
                &body,
            )?);
        }
    }
    Ok(vec![
        GeneratedToken::word("match"),
        GeneratedToken::word(binding.selector()),
        GeneratedToken::group(GeneratedDelimiter::Brace, arms)?,
    ])
}

/// How many times one operation occurs in one token tree, counted through every
/// nesting level and never overlapping itself.
///
/// An empty operation occurs nowhere: it names no tokens, so there is nothing
/// for a walk to find and nothing a substitution could consume.
#[must_use]
pub fn occurrences(tokens: &[GeneratedToken], operation: &[GeneratedToken]) -> usize {
    if operation.is_empty() {
        return 0;
    }
    let mut found: usize = 0;
    let mut skip: usize = 0;
    for (position, token) in tokens.iter().enumerate() {
        if skip > 0 {
            skip = skip.saturating_sub(1);
            continue;
        }
        if starts_here(tokens.get(position..), operation) {
            found = found.saturating_add(1);
            skip = operation.len().saturating_sub(1);
            continue;
        }
        if let GeneratedToken::Group { tokens: inner, .. } = token {
            let material: Vec<GeneratedToken> = inner.iter().cloned().collect();
            found = found.saturating_add(occurrences(&material, operation));
        }
    }
    found
}

/// Render the evaluation copy: the active-point enum, then the production tree
/// with every admitted point's operation standing under its selection.
///
/// # Errors
///
/// Returns the established issues, first ahead of the rest, where a point's
/// operation is absent from the production tree, occurs there more than once,
/// or sits inside another point's operation — and where the copy outgrows the
/// declared token magnitude. The site pass runs before anything is substituted,
/// so a copy is never half-transformed.
pub fn evaluation_copy(
    binding: &EvaluationBinding,
    table: &MutationPointTable,
    production: &GeneratedTree,
) -> Result<GeneratedTree, (ImplementationSurfaceIssue, Vec<ImplementationSurfaceIssue>)> {
    let material: Vec<GeneratedToken> = production.tokens().cloned().collect();
    if let Some(established) = ImplementationSurfaceIssue::established(site_issues(table, &material))
    {
        return Err(established);
    }
    let mut renderings = point_renderings(binding, table).map_err(sole)?;
    let transformed = substituted(&material, &mut renderings).map_err(|_| sole(unbounded()))?;
    if let Some(established) =
        ImplementationSurfaceIssue::established(overlap_issues(table, &renderings))
    {
        return Err(established);
    }
    let mut tokens = active_point_enum(binding, table).map_err(|_| sole(unbounded()))?;
    tokens.extend(transformed);
    GeneratedTree::assembled(tokens).map_err(|_| sole(unbounded()))
}

/// `#[derive(Clone, Copy, Debug, PartialEq, Eq)]`, as the tokens that spell it.
///
/// Five derives and no more: the selector is compared, copied into a call, and
/// shown in a failure report, and nothing about an evaluation copy needs
/// ordering or hashing.
fn derive_attribute() -> Result<GeneratedToken, BoundedConstruction> {
    let named = GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::word("Clone"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Copy"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Debug"),
            GeneratedToken::alone(','),
            GeneratedToken::word("PartialEq"),
            GeneratedToken::alone(','),
            GeneratedToken::word("Eq"),
        ],
    )?;
    GeneratedToken::group(
        GeneratedDelimiter::Bracket,
        vec![GeneratedToken::word("derive"), named],
    )
}

/// One arm: `<Enum>::<Variant> => { <body> },`.
///
/// The body is a block, so an operation of any token length is one expression
/// and no arm depends on what the operation's last token happened to be.
fn arm(
    active_enum: &str,
    variant: &str,
    body: &[GeneratedToken],
) -> Result<Vec<GeneratedToken>, BoundedConstruction> {
    let mut tokens = vec![
        GeneratedToken::word(active_enum),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(variant),
        GeneratedToken::joint('='),
        GeneratedToken::alone('>'),
    ];
    tokens.push(GeneratedToken::group(
        GeneratedDelimiter::Brace,
        body.to_vec(),
    )?);
    tokens.push(GeneratedToken::alone(','));
    Ok(tokens)
}

/// The tokens one arm stands over: the alternative where this arm belongs to the
/// subject point, and the subject's own original where it does not.
///
/// The alternative arrives as an OPTION rather than beside a flag, because the
/// two answers are two different things a caller has — an alternative it is
/// rendering, or none — and a flag beside a value is a second thing to keep
/// true.
fn arm_body(
    alternative: Option<&MutationOperation>,
    original: &[GeneratedToken],
) -> Vec<GeneratedToken> {
    match alternative {
        Some(operation) => operation.tree().tokens().cloned().collect(),
        None => original.to_vec(),
    }
}

/// Whether one operation starts at the beginning of one tail.
fn starts_here(tail: Option<&[GeneratedToken]>, operation: &[GeneratedToken]) -> bool {
    tail.is_some_and(|material| material.starts_with(operation))
}

/// The site pass: what the production tree says about each point's original
/// operation, before anything is substituted.
fn site_issues(
    table: &MutationPointTable,
    tokens: &[GeneratedToken],
) -> Vec<ImplementationSurfaceIssue> {
    let mut issues: Vec<ImplementationSurfaceIssue> = Vec::new();
    for point in table.admitted() {
        let operation: Vec<GeneratedToken> = point.original().tree().tokens().cloned().collect();
        let found = occurrences(tokens, &operation);
        if found == 0 {
            issues.push(ImplementationSurfaceIssue::OriginalOperationAbsent {
                point: point.name().clone(),
            });
        } else if found > 1 {
            issues.push(ImplementationSurfaceIssue::OriginalOperationNotUnique {
                point: point.name().clone(),
                observed: u32::try_from(found).unwrap_or(u32::MAX),
            });
        }
    }
    issues
}

/// Render every admitted point's halves, in table order.
fn point_renderings(
    binding: &EvaluationBinding,
    table: &MutationPointTable,
) -> Result<Vec<PointRendering>, ImplementationSurfaceIssue> {
    let mut renderings: Vec<PointRendering> = Vec::new();
    for (position, point) in table.admitted().enumerate() {
        let selected = selection(binding, table, position, point).map_err(|_| unbounded())?;
        renderings.push(PointRendering {
            operation: point.original().tree().tokens().cloned().collect(),
            selection: selected,
            performed: 0,
        });
    }
    Ok(renderings)
}

/// What the walk did with each point, read back off the walk's own counts.
fn overlap_issues(
    table: &MutationPointTable,
    renderings: &[PointRendering],
) -> Vec<ImplementationSurfaceIssue> {
    let mut issues: Vec<ImplementationSurfaceIssue> = Vec::new();
    for (point, rendering) in table.admitted().zip(renderings.iter()) {
        if rendering.performed == 0 {
            issues.push(ImplementationSurfaceIssue::OriginalOperationOverlapped {
                point: point.name().clone(),
            });
        } else if rendering.performed > 1 {
            issues.push(ImplementationSurfaceIssue::OriginalOperationNotUnique {
                point: point.name().clone(),
                observed: u32::try_from(rendering.performed).unwrap_or(u32::MAX),
            });
        }
    }
    issues
}

/// The single walk: every point offered every position at once, the first match
/// consuming it, and the rest of the tree descended into unchanged.
fn substituted(
    tokens: &[GeneratedToken],
    renderings: &mut [PointRendering],
) -> Result<Vec<GeneratedToken>, BoundedConstruction> {
    let mut rendered: Vec<GeneratedToken> = Vec::new();
    let mut skip: usize = 0;
    for (position, token) in tokens.iter().enumerate() {
        if skip > 0 {
            skip = skip.saturating_sub(1);
            continue;
        }
        let tail = tokens.get(position..);
        if let Some(rendering) = renderings.iter_mut().find(|point| point.starts(tail)) {
            rendered.extend(rendering.selection.iter().cloned());
            skip = rendering.operation.len().saturating_sub(1);
            rendering.performed = rendering.performed.saturating_add(1);
            continue;
        }
        rendered.push(descended(token, renderings)?);
    }
    Ok(rendered)
}

/// One token the walk did not substitute at: a group is descended into, and
/// everything else crosses unchanged.
fn descended(
    token: &GeneratedToken,
    renderings: &mut [PointRendering],
) -> Result<GeneratedToken, BoundedConstruction> {
    match token {
        GeneratedToken::Group { delimiter, tokens } => {
            let material: Vec<GeneratedToken> = tokens.iter().cloned().collect();
            GeneratedToken::group(*delimiter, substituted(&material, renderings)?)
        }
        GeneratedToken::Word(_) | GeneratedToken::Punct { .. } | GeneratedToken::Text(_) => {
            Ok(token.clone())
        }
    }
}

/// The issue a tree that outgrew the declared token magnitude amounts to.
fn unbounded() -> ImplementationSurfaceIssue {
    ImplementationSurfaceIssue::EvaluationTreeUnbounded {
        bound: u64::try_from(GeneratedTokenLimit::MAX).unwrap_or(u64::MAX),
    }
}

/// One established issue as the pair a refusal body is built from.
fn sole(
    issue: ImplementationSurfaceIssue,
) -> (ImplementationSurfaceIssue, Vec<ImplementationSurfaceIssue>) {
    (issue, Vec::new())
}
