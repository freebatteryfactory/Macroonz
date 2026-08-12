//! Rendering one refusal-family derivation into typed token trees.
//!
//! # What a renderer writes here is TOKENS
//!
//! Not text. Every path is spelled as segments, every literal is a typed literal
//! whose quoting the tree owns, and every brace is a group. A renderer that
//! wrote Rust source text would be composing a string the caller then has to
//! re-parse — and the place a quoting bug lives is exactly that round trip.
//!
//! The Rust source a person reads is [`crate::token::GeneratedTree::inspected`],
//! and it is a projection of what is emitted rather than the thing itself.
//!
//! # The crate binding is honoured, never assumed
//!
//! Every path below starts at the binding the CAPTURE read. A consumer that
//! renamed its dependency gets its own name back; a consumer that did not gets
//! `threadpak`. Nothing here spells a crate name that did not come from the
//! declaration.
//!
//! # The textual order is emitted from the typed rows
//!
//! The caller never writes a selection-order string. It writes variants and
//! local keys; the textual projection and the typed order are BOTH emitted from
//! the same captured rows, which is the whole point of band 00's split between a
//! cause's identity and its spelling.

use super::types::{CapturedCause, RefusalDeriveSurface};
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};
use threadpak::refusal::FamilyShape;

/// The machine's shape variant one body shape spells.
const fn shape_variant(shape: FamilyShape) -> &'static str {
    match shape {
        FamilyShape::SingleCause => "SingleCause",
        FamilyShape::IssueCollection => "IssueCollection",
        FamilyShape::InseparablePair => "InseparablePair",
    }
}

/// The canonical cause identity one captured cause carries: the family's stable
/// identity joined to the cause's local key, under band 00's key grammar.
///
/// Composed here rather than written by the author, so a family's causes cannot
/// drift apart one hand-typed prefix at a time.
#[must_use]
pub fn cause_identity(surface: &RefusalDeriveSurface, cause: &CapturedCause) -> String {
    let mut identity = String::from(surface.family_id());
    identity.push('.');
    identity.push_str(cause.local_key());
    identity
}

/// How one rendering failed to assemble.
#[must_use = "a rendering refusal names the magnitude the tree would have passed"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderRefusal {
    /// The rendered tree exceeds the declared token magnitude.
    Unbounded,
}

/// Render the `RefusalFamily` implementation.
///
/// # Errors
///
/// Returns [`RenderRefusal::Unbounded`] when the tree outgrows the declared
/// token magnitude.
pub fn family_implementation(
    surface: &RefusalDeriveSurface,
) -> Result<GeneratedTree, RenderRefusal> {
    let binding = surface.binding().spelling();
    let mut body: Vec<GeneratedToken> = Vec::new();

    body.push(GeneratedToken::word("const"));
    body.push(GeneratedToken::word("SHAPE"));
    body.push(GeneratedToken::alone(':'));
    body.extend(GeneratedToken::absolute_path(&[
        binding,
        "refusal",
        "FamilyShape",
    ]));
    body.push(GeneratedToken::alone('='));
    body.extend(GeneratedToken::absolute_path(&[
        binding,
        "refusal",
        "FamilyShape",
        shape_variant(surface.shape()),
    ]));
    body.push(GeneratedToken::alone(';'));

    body.push(GeneratedToken::word("const"));
    body.push(GeneratedToken::word("SELECTION_ORDER"));
    body.push(GeneratedToken::alone(':'));
    body.extend(static_str_slice_type()?);
    body.push(GeneratedToken::alone('='));
    body.push(GeneratedToken::alone('&'));
    let mut spellings: Vec<GeneratedToken> = Vec::new();
    for (position, cause) in surface.causes().enumerate() {
        if position > 0 {
            spellings.push(GeneratedToken::alone(','));
        }
        spellings.push(GeneratedToken::text(cause.spelling()));
    }
    body.push(
        GeneratedToken::group(GeneratedDelimiter::Bracket, spellings)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    body.push(GeneratedToken::alone(';'));

    implementation(binding, "RefusalFamily", surface.family_name(), body)
}

/// Render the `CauseOrderDeclaration` implementation.
///
/// # Errors
///
/// Returns [`RenderRefusal::Unbounded`] when the tree outgrows the declared
/// token magnitude.
pub fn cause_order_implementation(
    surface: &RefusalDeriveSurface,
) -> Result<GeneratedTree, RenderRefusal> {
    let binding = surface.binding().spelling();
    let mut rows: Vec<GeneratedToken> = Vec::new();
    for cause in surface.causes() {
        let mut identity_arguments =
            GeneratedToken::absolute_path(&[binding, "refusal", "CauseId", "declared"]);
        identity_arguments.push(
            GeneratedToken::group(
                GeneratedDelimiter::Parenthesis,
                vec![GeneratedToken::text(&cause_identity(surface, cause))],
            )
            .map_err(|_| RenderRefusal::Unbounded)?,
        );
        identity_arguments.push(GeneratedToken::alone(','));
        identity_arguments.push(GeneratedToken::text(cause.spelling()));

        let mut row =
            GeneratedToken::absolute_path(&[binding, "refusal", "DeclaredCause", "declared"]);
        row.push(
            GeneratedToken::group(GeneratedDelimiter::Parenthesis, identity_arguments)
                .map_err(|_| RenderRefusal::Unbounded)?,
        );
        rows.extend(row);
        rows.push(GeneratedToken::alone(','));
    }

    let mut body: Vec<GeneratedToken> = Vec::new();
    body.push(GeneratedToken::word("const"));
    body.push(GeneratedToken::word("DECLARED_ORDER"));
    body.push(GeneratedToken::alone(':'));
    body.extend(GeneratedToken::absolute_path(&[
        binding,
        "refusal",
        "DeclaredCauseOrder",
    ]));
    body.push(GeneratedToken::alone('='));
    body.extend(GeneratedToken::absolute_path(&[
        binding,
        "refusal",
        "DeclaredCauseOrder",
        "declared",
    ]));
    let mut argument: Vec<GeneratedToken> = vec![GeneratedToken::alone('&')];
    argument.push(
        GeneratedToken::group(GeneratedDelimiter::Bracket, rows)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    body.push(
        GeneratedToken::group(GeneratedDelimiter::Parenthesis, argument)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    body.push(GeneratedToken::alone(';'));

    implementation(
        binding,
        "CauseOrderDeclaration",
        surface.family_name(),
        body,
    )
}

/// `impl ::<binding>::refusal::<Contract> for <Type> { <body> }`.
fn implementation(
    binding: &str,
    contract: &str,
    target: &str,
    body: Vec<GeneratedToken>,
) -> Result<GeneratedTree, RenderRefusal> {
    let mut tokens = vec![GeneratedToken::word("impl")];
    tokens.extend(GeneratedToken::absolute_path(&[
        binding, "refusal", contract,
    ]));
    tokens.push(GeneratedToken::word("for"));
    tokens.push(GeneratedToken::word(target));
    tokens.push(
        GeneratedToken::group(GeneratedDelimiter::Brace, body)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    GeneratedTree::assembled(tokens).map_err(|_| RenderRefusal::Unbounded)
}

/// The type `&'static [&'static str]`, as the tokens that spell it.
fn static_str_slice_type() -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let inner = vec![
        GeneratedToken::alone('&'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("static"),
        GeneratedToken::word("str"),
    ];
    Ok(vec![
        GeneratedToken::alone('&'),
        GeneratedToken::joint('\''),
        GeneratedToken::word("static"),
        GeneratedToken::group(GeneratedDelimiter::Bracket, inner)
            .map_err(|_| RenderRefusal::Unbounded)?,
    ])
}
