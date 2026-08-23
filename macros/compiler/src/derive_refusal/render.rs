//! Rendering one refusal-family derivation into typed token trees.
//!
//! # Tokens, not text
//!
//! Every path is spelled as segments, every literal is a typed literal whose
//! quoting the tree owns, and every brace is a group.
//! A renderer that wrote Rust source text would be composing a string the caller
//! then has to re-parse — and the place a quoting bug lives is exactly that
//! round trip.
//!
//! The Rust source a person reads is [`crate::token::GeneratedTree::inspected`],
//! and it is a projection of what is emitted rather than the thing itself.
//!
//! # The crate binding
//!
//! Every path below starts at the binding the CAPTURE read.
//! A consumer that renamed its dependency gets its own name back; a consumer
//! that did not gets `threadpak`.
//! Nothing here spells a crate name that did not come from the declaration.
//!
//! # The production subject
//!
//! Production implementations are rendered only for the type the declaration
//! named. Generated mutation discovery and its evaluation callables belong to
//! [`crate::mutation_descriptor`], which renders one separate TestPak-facing
//! module rather than copying a production implementation onto another type.
//!
//! # The selection order
//!
//! The caller never writes a selection-order string.
//! It writes variants and local keys; the textual projection and the typed order
//! are BOTH emitted from the same captured rows, which is the whole point of
//! band 00's split between a cause's identity and its spelling.

use super::types::{CapturedCause, RefusalDeriveSurface, RenderRefusal};
use crate::mutation_descriptor::{
    MutationOrderCause, MutationOrderDeclaration, MutationProjectionRequest,
};
use crate::test_descriptor::WallName;
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};
use macroonz::FamilyShape;

/// The contract a family implementation realizes.
pub const FAMILY_CONTRACT: &str = "RefusalFamily";

/// The contract a cause-order implementation realizes.
pub const CAUSE_ORDER_CONTRACT: &str = "CauseOrderDeclaration";

/// The machine's shape variant one body shape spells.
const fn shape_variant(shape: FamilyShape) -> &'static str {
    match shape {
        FamilyShape::SingleCause => "SingleCause",
        FamilyShape::IssueCollection => "IssueCollection",
        FamilyShape::InseparablePair => "InseparablePair",
    }
}

/// The path one rendering realizes the family contract under, as canonical
/// bytes.
///
/// # Why the bytes live here
///
/// A path is a rendering fact.
/// This home is the one that knows how a contract is spelled under a binding —
/// it spells exactly that path into the tokens it emits — so the bytes an
/// identity is derived over are composed here, out of the same three spellings
/// the rendering uses. Composed anywhere else, the identity would stand over a
/// path assembled from a second copy of those spellings, and a rename that moved
/// the rendering would leave the identity naming a path nothing emits.
///
/// The crate binding travels into it, because a rendering against a renamed
/// dependency realizes the contract under a different path and is a different
/// generated unit.
#[must_use]
pub fn contract_path_bytes(surface: &RefusalDeriveSurface) -> Vec<u8> {
    let mut material = Vec::new();
    material.extend_from_slice(surface.binding().spelling().as_bytes());
    material.push(b'.');
    material.extend_from_slice(FAMILY_CONTRACT.as_bytes());
    material
}

/// The tokens one captured cause's stable identity is minted through: the
/// family's declared identity and the cause's local key, each through its own
/// constructor on the binding the capture read.
///
/// # Two seats, and no join
///
/// A cause identity IS the pair, so the rendering emits the pair and composes no
/// text at all here.
/// Joining the family identity and the local key into a single literal would put
/// a family's ownership of its own cause back into a spelling, exactly where
/// band 00's shape takes it out of one.
/// The author writes the local key and never the prefix, so a family's causes
/// cannot drift apart one hand-typed prefix at a time.
fn cause_identity(
    surface: &RefusalDeriveSurface,
    cause: &CapturedCause,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let binding = surface.binding().spelling();
    let mut seats = GeneratedToken::absolute_path(&[binding, "RefusalFamilyId", "declared"]);
    seats.push(
        GeneratedToken::group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::text(surface.family_id())],
        )
        .map_err(|_| RenderRefusal::Unbounded)?,
    );
    seats.push(GeneratedToken::alone(','));
    seats.extend(GeneratedToken::absolute_path(&[
        binding,
        "LocalCauseKey",
        "declared",
    ]));
    seats.push(
        GeneratedToken::group(
            GeneratedDelimiter::Parenthesis,
            vec![GeneratedToken::text(cause.local_key())],
        )
        .map_err(|_| RenderRefusal::Unbounded)?,
    );

    let mut minted = GeneratedToken::absolute_path(&[binding, "CauseId", "declared"]);
    minted.push(
        GeneratedToken::group(GeneratedDelimiter::Parenthesis, seats)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    Ok(minted)
}

/// Render the `RefusalFamily` implementation, for the type the author declared.
///
/// # Errors
///
/// Returns [`RenderRefusal::Unbounded`] when the tree outgrows the declared
/// token magnitude.
pub fn family_implementation(
    surface: &RefusalDeriveSurface,
) -> Result<GeneratedTree, RenderRefusal> {
    let binding = surface.binding().spelling();
    let mut tokens = implementation_tokens(
        binding,
        FAMILY_CONTRACT,
        surface.family_name(),
        family_body(surface),
    )?;
    tokens.extend(inherent_implementation_tokens(
        surface.family_name(),
        selection_order_body(surface)?,
    )?);
    GeneratedTree::assembled(tokens).map_err(|_| RenderRefusal::Unbounded)
}

/// The `RefusalFamily` implementation's body: the declared shape.
fn family_body(surface: &RefusalDeriveSurface) -> Vec<GeneratedToken> {
    let binding = surface.binding().spelling();
    let mut body: Vec<GeneratedToken> = Vec::new();
    body.push(GeneratedToken::word("const"));
    body.push(GeneratedToken::word("SHAPE"));
    body.push(GeneratedToken::alone(':'));
    body.extend(GeneratedToken::absolute_path(&[binding, "FamilyShape"]));
    body.push(GeneratedToken::alone('='));
    body.extend(GeneratedToken::absolute_path(&[
        binding,
        "FamilyShape",
        shape_variant(surface.shape()),
    ]));
    body.push(GeneratedToken::alone(';'));

    body
}

/// The generated inherent textual projection of the typed cause order.
fn selection_order_body(
    surface: &RefusalDeriveSurface,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut body: Vec<GeneratedToken> = vec![
        GeneratedToken::joint('#'),
        GeneratedToken::group(
            GeneratedDelimiter::Bracket,
            vec![
                GeneratedToken::word("doc"),
                GeneratedToken::alone('='),
                GeneratedToken::text("The generated textual projection of the typed cause order."),
            ],
        )
        .map_err(|_| RenderRefusal::Unbounded)?,
        GeneratedToken::word("pub"),
        GeneratedToken::word("const"),
        GeneratedToken::word("SELECTION_ORDER"),
        GeneratedToken::alone(':'),
    ];
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

    Ok(body)
}

/// Render the `CauseOrderDeclaration` implementation, for the type the author
/// declared.
///
/// # Errors
///
/// Returns [`RenderRefusal::Unbounded`] when the tree outgrows the declared
/// token magnitude.
pub fn cause_order_implementation(
    surface: &RefusalDeriveSurface,
) -> Result<GeneratedTree, RenderRefusal> {
    let binding = surface.binding().spelling();
    let body = cause_order_body(surface)?;
    implementation(binding, CAUSE_ORDER_CONTRACT, surface.family_name(), body)
}

/// Build the complete one-way request the mechanical mutation renderer consumes.
pub(crate) fn mutation_projection_request(
    surface: &RefusalDeriveSurface,
) -> Result<MutationProjectionRequest, RenderRefusal> {
    let super::MutationDeclarationPosture::Declared(mutations) = surface.mutations() else {
        return Err(RenderRefusal::MutationRequestAbsent);
    };
    let Some((namespace, family)) = surface.family_id().split_once('.') else {
        return Err(RenderRefusal::MutationPointNotInformed(
            crate::test_descriptor::ShellDeclarationRefusal::EmptyNamespace,
        ));
    };
    let point =
        WallName::named(namespace, family).map_err(RenderRefusal::MutationPointNotInformed)?;
    let binding = surface.binding().spelling();
    let order_type = GeneratedToken::absolute_path(&[binding, "DeclaredCauseOrder"]);
    let order_constructor =
        GeneratedToken::absolute_path(&[binding, "DeclaredCauseOrder", "declared"]);

    let mut production_expression = vec![
        GeneratedToken::alone('<'),
        GeneratedToken::joint('$'),
        GeneratedToken::word("crate"),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word(surface.family_name()),
        GeneratedToken::word("as"),
    ];
    production_expression.extend(GeneratedToken::absolute_path(&[
        binding,
        CAUSE_ORDER_CONTRACT,
    ]));
    production_expression.extend([
        GeneratedToken::alone('>'),
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word("DECLARED_ORDER"),
    ]);

    let order = match surface.shape() {
        FamilyShape::SingleCause => {
            let causes = surface
                .causes()
                .map(|cause| {
                    declared_cause_row(surface, cause).map(|row| {
                        MutationOrderCause::informed(cause.local_key(), cause.spelling(), row)
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            MutationOrderDeclaration::Declared(causes)
        }
        FamilyShape::IssueCollection | FamilyShape::InseparablePair => {
            let mut expression =
                GeneratedToken::absolute_path(&[binding, "DeclaredCauseOrder", "none"]);
            expression.push(
                GeneratedToken::group(GeneratedDelimiter::Parenthesis, Vec::new())
                    .map_err(|_| RenderRefusal::Unbounded)?,
            );
            MutationOrderDeclaration::NotApplicable { expression }
        }
    };
    let observed = match &order {
        MutationOrderDeclaration::NotApplicable { .. } => 0,
        MutationOrderDeclaration::Declared(causes) => causes.len().saturating_sub(1),
    };
    let alternative_count = u64::try_from(observed)
        .map_err(|_| RenderRefusal::MutationAlternativesUnbounded { observed })?;

    Ok(MutationProjectionRequest::informed(
        mutations.declaration(),
        point,
        order_type,
        production_expression,
        order_constructor,
        order,
        alternative_count,
    ))
}

/// The `CauseOrderDeclaration` implementation's body: the typed order, cause by
/// cause.
fn cause_order_body(surface: &RefusalDeriveSurface) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let binding = surface.binding().spelling();
    let mut body: Vec<GeneratedToken> = Vec::new();
    body.push(GeneratedToken::word("const"));
    body.push(GeneratedToken::word("DECLARED_ORDER"));
    body.push(GeneratedToken::alone(':'));
    body.extend(GeneratedToken::absolute_path(&[
        binding,
        "DeclaredCauseOrder",
    ]));
    body.push(GeneratedToken::alone('='));
    body.extend(declared_order_expression(surface, surface.causes())?);
    body.push(GeneratedToken::alone(';'));

    Ok(body)
}

/// One `DeclaredCauseOrder` expression over captured causes in the supplied order.
pub(crate) fn declared_order_expression<'causes>(
    surface: &RefusalDeriveSurface,
    causes: impl IntoIterator<Item = &'causes CapturedCause>,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let binding = surface.binding().spelling();
    let mut rows: Vec<GeneratedToken> = Vec::new();
    for cause in causes {
        rows.extend(declared_cause_row(surface, cause)?);
        rows.push(GeneratedToken::alone(','));
    }

    let mut expression =
        GeneratedToken::absolute_path(&[binding, "DeclaredCauseOrder", "declared"]);
    let mut argument: Vec<GeneratedToken> = vec![GeneratedToken::alone('&')];
    argument.push(
        GeneratedToken::group(GeneratedDelimiter::Bracket, rows)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    expression.push(
        GeneratedToken::group(GeneratedDelimiter::Parenthesis, argument)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    Ok(expression)
}

fn declared_cause_row(
    surface: &RefusalDeriveSurface,
    cause: &CapturedCause,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut row_arguments = cause_identity(surface, cause)?;
    row_arguments.push(GeneratedToken::alone(','));
    row_arguments.push(GeneratedToken::text(cause.spelling()));
    let mut row =
        GeneratedToken::absolute_path(&[surface.binding().spelling(), "DeclaredCause", "declared"]);
    row.push(
        GeneratedToken::group(GeneratedDelimiter::Parenthesis, row_arguments)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    Ok(row)
}

/// Render one implementation of a contract at the captured dependency binding.
///
/// The target is a parameter shared by the two production contracts this home renders.
fn implementation(
    binding: &str,
    contract: &str,
    target: &str,
    body: Vec<GeneratedToken>,
) -> Result<GeneratedTree, RenderRefusal> {
    GeneratedTree::assembled(implementation_tokens(binding, contract, target, body)?)
        .map_err(|_| RenderRefusal::Unbounded)
}

fn implementation_tokens(
    binding: &str,
    contract: &str,
    target: &str,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut tokens = vec![GeneratedToken::word("impl")];
    tokens.extend(GeneratedToken::absolute_path(&[binding, contract]));
    tokens.push(GeneratedToken::word("for"));
    tokens.push(GeneratedToken::word(target));
    tokens.push(
        GeneratedToken::group(GeneratedDelimiter::Brace, body)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    Ok(tokens)
}

fn inherent_implementation_tokens(
    target: &str,
    body: Vec<GeneratedToken>,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let mut tokens = vec![GeneratedToken::word("impl"), GeneratedToken::word(target)];
    tokens.push(
        GeneratedToken::group(GeneratedDelimiter::Brace, body)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    Ok(tokens)
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
