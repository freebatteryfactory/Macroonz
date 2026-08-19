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
//! # The two subjects
//!
//! One body, two subjects. The PRODUCTION implementation is rendered for the
//! type the declaration named, at the declaration site, where that type is the
//! consumer's own. The mutation-EVALUATION copy is rendered for
//! [`EVALUATION_SUBJECT`], the private type the support shell declares inside
//! its own hex-keyed module, because a copy rendered for the author's type is
//! the same contract implemented twice for one type beside the production
//! implementation, and a foreign trait implemented for a foreign type once it
//! reaches a consumer's test target.
//!
//! The evaluation roads are guarded and the production ones are not, because
//! the substitution is what needs establishing: `relocatable` walks the BODY
//! before it goes under the subject's head and refuses one that observes `Self`
//! or names the declared type, so a copy whose meaning moved with its subject is
//! a typed refusal rather than a rendering.
//!
//! # The selection order
//!
//! The caller never writes a selection-order string.
//! It writes variants and local keys; the textual projection and the typed order
//! are BOTH emitted from the same captured rows, which is the whole point of
//! band 00's split between a cause's identity and its spelling.

use super::types::{CapturedCause, RefusalDeriveSurface};
use crate::token::{GeneratedDelimiter, GeneratedToken, GeneratedTree};
use threadpak::refusal::FamilyShape;

/// The module the machine's refusal contracts live under, inside whatever the
/// consumer calls the machine.
pub const REFUSAL_MODULE: &str = "refusal";

/// The contract a family implementation realizes.
pub const FAMILY_CONTRACT: &str = "RefusalFamily";

/// The contract a cause-order implementation realizes.
pub const CAUSE_ORDER_CONTRACT: &str = "CauseOrderDeclaration";

/// The type the mutation-evaluation copies are implemented for.
///
/// # Why the copy stands over another subject at all
///
/// The copy is the production implementation transformed, so rendered against
/// the author's own type it realizes the SAME contract for the SAME type twice:
/// beside the production implementation that is a duplicate the consumer's
/// compiler refuses outright, and inside a consumer's test target it is a
/// foreign trait implemented for a foreign type, which the language refuses on
/// the orphan rule. Neither is a delivery anybody can repair from the outside —
/// so the copy stands over a subject the target that receives it owns.
///
/// A literal identifier and never a spelling composed from the declaration:
/// composing one would be this home deciding how an author's own type name
/// becomes a Rust identifier, which is a spelling law nobody gave it.
///
/// # Bounds
///
/// The subject is not declared here. This home renders implementations FOR it;
/// the item that declares it — a private type inside the support shell's own
/// hex-keyed module, which never becomes consumer API — is the shell's splice,
/// and the shell reads this spelling as the data it is.
/// Collision-freedom is therefore the shell's: one module per shell, scoped by
/// the shell's own content-addressed name, so one plain spelling stands in each.
pub const EVALUATION_SUBJECT: &str = "EvaluationSubject";

/// The keyword one implementation body observes its own target through.
///
/// Read by the guard and by nothing else. It is the language's, not this
/// home's: a body that spells it means "the type this implementation is for",
/// and that meaning moves the moment the target does.
const SELF_TYPE: &str = "Self";

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
    material.extend_from_slice(REFUSAL_MODULE.as_bytes());
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
    let mut seats =
        GeneratedToken::absolute_path(&[binding, REFUSAL_MODULE, "RefusalFamilyId", "declared"]);
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
        REFUSAL_MODULE,
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

    let mut minted =
        GeneratedToken::absolute_path(&[binding, REFUSAL_MODULE, "CauseId", "declared"]);
    minted.push(
        GeneratedToken::group(GeneratedDelimiter::Parenthesis, seats)
            .map_err(|_| RenderRefusal::Unbounded)?,
    );
    Ok(minted)
}

/// How one rendering failed to assemble.
#[must_use = "a rendering refusal names what the tree could not be rendered under"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RenderRefusal {
    /// The rendered tree exceeds the declared token magnitude.
    Unbounded,
    /// The implementation body observes the target it was derived for, so no
    /// copy of it stands over another subject and the evaluation delivery has
    /// nothing lawful to render.
    ///
    /// A typed answer rather than a silent rendering: a body that means
    /// something different once its target changes, rendered against the
    /// support shell's subject anyway, is an evaluation copy that is not the
    /// production implementation — and the parity the copy exists to prove
    /// would be a statement about two different meanings.
    TargetObserved,
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
    let body = family_body(surface)?;
    implementation(binding, FAMILY_CONTRACT, surface.family_name(), body)
}

/// Render the `RefusalFamily` implementation for the shell's own local
/// evaluation subject.
///
/// The GUARDED road, and one of the only two that spell [`EVALUATION_SUBJECT`]:
/// the body is established relocatable before it is put under the subject's
/// head, so there is no road in this home that moves a body to another subject
/// without establishing that moving it changes nothing.
///
/// # Errors
///
/// Returns [`RenderRefusal::Unbounded`] when the tree outgrows the declared
/// token magnitude, and [`RenderRefusal::TargetObserved`] when the body
/// observes the target the declaration named.
pub fn family_evaluation_implementation(
    surface: &RefusalDeriveSurface,
) -> Result<GeneratedTree, RenderRefusal> {
    let binding = surface.binding().spelling();
    let body = relocatable(family_body(surface)?, surface.family_name())?;
    implementation(binding, FAMILY_CONTRACT, EVALUATION_SUBJECT, body)
}

/// Render the `CauseOrderDeclaration` implementation for the shell's own local
/// evaluation subject, on exactly the terms
/// [`family_evaluation_implementation`] states.
///
/// # Errors
///
/// Returns [`RenderRefusal::Unbounded`] when the tree outgrows the declared
/// token magnitude, and [`RenderRefusal::TargetObserved`] when the body
/// observes the target the declaration named.
pub fn cause_order_evaluation_implementation(
    surface: &RefusalDeriveSurface,
) -> Result<GeneratedTree, RenderRefusal> {
    let binding = surface.binding().spelling();
    let body = relocatable(cause_order_body(surface)?, surface.family_name())?;
    implementation(binding, CAUSE_ORDER_CONTRACT, EVALUATION_SUBJECT, body)
}

/// Hand back one implementation body where standing it under another subject
/// changes nothing, and refuse where it would.
///
/// # The guard, exactly
///
/// The BODY is walked whole — every nesting level, every token — and the walk
/// asks one question of every WORD: is it `Self`, or is it the spelling the
/// declaration named its own type by? Either answer means the body's meaning
/// depends on the nominal identity of the target, and a body like that says
/// something different once the target is the shell's subject.
///
/// The body and never the whole implementation, because the HEAD names the
/// target on purpose and names the contract beside it: a walk over the head
/// would refuse a family whose Rust type happens to be spelled like the
/// contract it realizes, the module that contract lives in, or the binding the
/// consumer reached the machine by — four names that have nothing to do with
/// whether the body observes anything.
///
/// Words alone, and that is the second half of the same precision. A cause's
/// spelling and a family's declared identity are rendered as TEXT literals
/// rather than as identifiers, so a family whose causes are spelled like its own
/// type is not a family this guard refuses — it is one whose body names no type
/// at all.
///
/// # Bounds
///
/// It establishes that the substitution is MEANING-PRESERVING and nothing
/// beyond it. Whether the shell declared the subject, whether the consumer's
/// target compiles the copy, and whether the copy's alternatives are meaningful
/// damages are three other questions, answered by the shell's splice and by the
/// harness's running.
fn relocatable(
    body: Vec<GeneratedToken>,
    declared: &str,
) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    if body.iter().any(|token| observes_target(token, declared)) {
        return Err(RenderRefusal::TargetObserved);
    }
    Ok(body)
}

/// Whether one token, or anything nested inside it, names the target.
fn observes_target(token: &GeneratedToken, declared: &str) -> bool {
    match token {
        GeneratedToken::Word(word) => word.as_str() == SELF_TYPE || word.as_str() == declared,
        GeneratedToken::Group { tokens, .. } => tokens
            .iter()
            .any(|nested| observes_target(nested, declared)),
        GeneratedToken::Punct { .. }
        | GeneratedToken::Text(_)
        | GeneratedToken::ByteText(_)
        | GeneratedToken::Number(_) => false,
    }
}

/// The `RefusalFamily` implementation's body: the declared shape, and the
/// textual selection order.
fn family_body(surface: &RefusalDeriveSurface) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let binding = surface.binding().spelling();
    let mut body: Vec<GeneratedToken> = Vec::new();

    body.push(GeneratedToken::word("const"));
    body.push(GeneratedToken::word("SHAPE"));
    body.push(GeneratedToken::alone(':'));
    body.extend(GeneratedToken::absolute_path(&[
        binding,
        REFUSAL_MODULE,
        "FamilyShape",
    ]));
    body.push(GeneratedToken::alone('='));
    body.extend(GeneratedToken::absolute_path(&[
        binding,
        REFUSAL_MODULE,
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

/// The `CauseOrderDeclaration` implementation's body: the typed order, cause by
/// cause.
fn cause_order_body(surface: &RefusalDeriveSurface) -> Result<Vec<GeneratedToken>, RenderRefusal> {
    let binding = surface.binding().spelling();
    let mut rows: Vec<GeneratedToken> = Vec::new();
    for cause in surface.causes() {
        let mut row_arguments = cause_identity(surface, cause)?;
        row_arguments.push(GeneratedToken::alone(','));
        row_arguments.push(GeneratedToken::text(cause.spelling()));

        let mut row =
            GeneratedToken::absolute_path(&[binding, REFUSAL_MODULE, "DeclaredCause", "declared"]);
        row.push(
            GeneratedToken::group(GeneratedDelimiter::Parenthesis, row_arguments)
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
        REFUSAL_MODULE,
        "DeclaredCauseOrder",
    ]));
    body.push(GeneratedToken::alone('='));
    body.extend(GeneratedToken::absolute_path(&[
        binding,
        REFUSAL_MODULE,
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

    Ok(body)
}

/// `impl ::<binding>::refusal::<Contract> for <Target> { <body> }`.
///
/// The target is a parameter because one body is delivered for two subjects:
/// the type the declaration named, and the support shell's own local evaluation
/// subject. The head is the only place either spelling is written, which is
/// what makes the guard's walk over the body a complete question.
fn implementation(
    binding: &str,
    contract: &str,
    target: &str,
    body: Vec<GeneratedToken>,
) -> Result<GeneratedTree, RenderRefusal> {
    let mut tokens = vec![GeneratedToken::word("impl")];
    tokens.extend(GeneratedToken::absolute_path(&[
        binding, REFUSAL_MODULE, contract,
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
