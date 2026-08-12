//! Deriving a refusal family's declared facts: one road, and it is the
//! receipt-rich one.
//!
//! # Ordinary callable Rust, and nothing else
//!
//! Nothing in this home knows a proc-macro exists. [`compile_refusal`] takes a
//! typed [`CapturedInput`] and returns either a [`ClosedExpansion`] or a
//! [`MacrocDiagnostic`]; everything downstream takes typed values. The
//! Rust-facing shell is one caller of this function; a test is another; a future
//! language frontend would be a third. A diagnostic from here names
//! [`crate::diagnostics::ReproductionRoute::CallableServices`] because that
//! route is real, and
//! [`compile_refusal_text`] is it.
//!
//! # There is exactly one road to emitted tokens
//!
//! There used to be two. A caller could capture a declaration, fix its
//! membership, and take a rendering straight off the draft — no plan, no
//! identities, no origin graph, no trace, no explanation, no closure. That road
//! was shorter than the receipt-rich one, which is another way of saying every
//! receipt on the receipt-rich road was optional.
//!
//! It is closed. The membership-only object is
//! [`RefusalDerivationDraft`], it has no render
//! method, and there is no other public value in this home that carries a token
//! tree. The steps below run in order and each one refuses on its own terms:
//!
//! ```text
//! capture → plan → render → close → explain → bind → emit
//! ```
//!
//! Delete any one of them and no [`ClosedExpansion`] exists, so nothing is
//! emitted. That is the property, and it is structural rather than reviewed.
//!
//! The emit step is not a step on this road any more, and that is the point.
//! Joining the rendered units into the tree a compiler is handed happens INSIDE
//! `close`, which keeps the result and commits to its digest — so there is no
//! act after the proof for a defect to live in.
//!
//! # Every step refuses in its own vocabulary, and it survives the crossing
//!
//! Each `map_err` below is a projection rather than a collapse. A planning body
//! reaches the caller naming its axis and magnitude, a closure body naming its
//! role and the disagreement at it, a coverage body naming every seat, a
//! rendering refusal naming the exact bound. See [`diagnose`].
//!
//! # What this home does not decide
//!
//! It decides no meaning. The three body shapes are band 00's; the canonical key
//! grammar is band 00's; the selection order's *content* is the author's; the
//! local keys are the author's; the `RefusalFamily` and `CauseOrderDeclaration`
//! contracts are band 00's. This home reads a declaration and writes down what
//! it already said.

pub mod capture;
pub mod diagnose;
pub mod explain;
pub mod plan;
pub mod render;
mod type_contract;
pub mod types;

pub use capture::{captured, captured_text};
pub use explain::{ExplanationBindingRefusal, ExplanationSeat};
pub use plan::DerivedPlan;
pub use render::RenderRefusal;
pub use types::{
    CapturedCause, CauseOrderStanding, ClosedExpansion, CrateBinding, DEFAULT_CRATE_BINDING,
    DerivedMembership, RefusalCompileContext, RefusalDerivationDraft, RefusalDeriveCapture,
    RefusalDeriveRefusal, RefusalDeriveSurface, RefusalOwnerFacts, SHAPE_WORD_INSEPARABLE_PAIR,
    SHAPE_WORD_ISSUE_COLLECTION, SHAPE_WORD_SINGLE_CAUSE,
};

use crate::closure::{ProjectionClosure, RenderedProjection, RenderedUnit};
use crate::diagnostics::MacrocDiagnostic;
use crate::planning::{MemberDestination, RenderedImplementation};
use crate::token::{CapturedInput, TextCapture};
use threadpak::types::Bounded;

/// Capture, plan, render, close, and explain one refusal-family declaration —
/// the whole road, in one call, and the only road there is.
///
/// # Errors
///
/// Returns a [`MacrocDiagnostic`] whenever any step refuses: a declaration the
/// grammar does not admit, a plan whose magnitudes are exceeded, a rendering
/// that outgrows its bound, a closure the rendering does not satisfy, or an
/// explanation that does not cover its kind's questions. **Every one of those
/// refusals happens BEFORE a token exists to emit**, because the token tree is
/// reachable only off the value this function returns on success.
#[expect(
    clippy::result_large_err,
    reason = "the diagnostic is seat-complete by law, and the ruled service signature returns it by value: boxing it here would move a required seat behind a pointer to satisfy a size lint"
)]
pub fn compile_refusal(
    input: &CapturedInput,
    context: &RefusalCompileContext,
) -> Result<ClosedExpansion, MacrocDiagnostic> {
    let surface = captured(input)
        .map_err(|refusal| refusal.diagnosed(&context.spans, context.machine.clone()))?;
    let draft = surface.planned();

    let planned = plan::planned(&draft, context.owner_facts, context.nonclaims.clone())
        .map_err(|refusal| diagnose::planning_refused(&refusal))?;

    let rendered = render_units(&draft)?;

    // The closure joins the rendered units and keeps the joined tree, so there
    // is nothing left to assemble on this road after the proof returns.
    let closure = ProjectionClosure::proved(
        planned.plan().identity(),
        planned.plan().membership(),
        rendered,
    )
    .map_err(|refusal| diagnose::closure_refused(&refusal))?;

    let explanation = explain::explained(&planned, &closure)
        .map_err(|refusal| diagnose::explanation_refused(&refusal))?;

    let (plan_value, cause_order) = planned.into_parts();
    Ok(ClosedExpansion::bound(
        draft.surface().clone(),
        plan_value,
        closure,
        explanation,
        cause_order,
    ))
}

/// How the callable text route refused.
///
/// Two postures, and they are genuinely different observations. A text that
/// cannot be cut into tokens never reached the grammar at all and has no span
/// table to point into; a text that cut fine and said the wrong thing has both.
/// Folding them together would hand a caller a diagnostic whose site indexes a
/// table that was never built.
#[must_use = "a refusal names which of the two ways the callable text route refused"]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextCompileRefusal {
    /// The text could not be cut into tokens.
    NotReadable(crate::token::TextReadRefusal),
    /// The text was read, and the compilation refused. The capture rides along
    /// so the diagnostic's token handle resolves against the same table the read
    /// issued.
    Refused(Box<(TextCapture, MacrocDiagnostic)>),
}

/// The callable route: read one declaration from TEXT and compile it.
///
/// This is what makes the callable-services reproduction route a real road rather
/// than a promise — no proc-macro anywhere in the path, and the byte offsets the
/// text read issued resolve every diagnostic's token handle.
///
/// # Errors
///
/// Returns [`TextCompileRefusal`] naming which of the two ways the route
/// refused.
pub fn compile_refusal_text(
    source: &str,
) -> Result<(TextCapture, ClosedExpansion), TextCompileRefusal> {
    let read = TextCapture::read(source).map_err(TextCompileRefusal::NotReadable)?;
    let context = RefusalCompileContext {
        spans: read.spans().clone(),
        machine: crate::diagnostics::MachineAnchoring::UnmintedAtThisSeam,
        owner_facts: RefusalOwnerFacts::declared(),
        nonclaims: Bounded::empty(),
    };
    match compile_refusal(read.input(), &context) {
        Ok(closed) => Ok((read, closed)),
        Err(diagnostic) => Err(TextCompileRefusal::Refused(Box::new((read, diagnostic)))),
    }
}

/// Render every planned role into a rendered unit.
///
/// The roster is fixed by the shape, so the rendering is built by matching on
/// the two answers rather than by folding a slice and repairing an empty fold.
/// [`RenderedProjection::complete`] settles the magnitude at compile time, which
/// is why neither arm carries a refusal road of its own.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the ruled service road returns; this helper hands \n              it straight through"
)]
fn render_units(
    draft: &RefusalDerivationDraft,
) -> Result<RenderedProjection<RenderedImplementation>, MacrocDiagnostic> {
    let family = rendered_unit(draft, RenderedImplementation::RenderedFamilyImpl)?;
    match draft.declared_membership() {
        DerivedMembership::FamilyOnly => Ok(RenderedProjection::complete(family, [])),
        DerivedMembership::FamilyAndCauseOrder => {
            let cause_order = rendered_unit(draft, RenderedImplementation::RenderedCauseOrderImpl)?;
            Ok(RenderedProjection::complete(family, [cause_order]))
        }
    }
}

/// Render one role into one materialized unit, projecting either refusal into a
/// diagnostic that names the exact magnitude and the role that overran it.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the ruled service road returns; this helper hands \n              it straight through"
)]
fn rendered_unit(
    draft: &RefusalDerivationDraft,
    role: RenderedImplementation,
) -> Result<RenderedUnit<RenderedImplementation>, MacrocDiagnostic> {
    let tree = match role {
        RenderedImplementation::RenderedFamilyImpl => {
            render::family_implementation(draft.surface())
        }
        RenderedImplementation::RenderedCauseOrderImpl => {
            render::cause_order_implementation(draft.surface())
        }
    }
    .map_err(|refusal| diagnose::render_refused(refusal, role))?;
    RenderedUnit::materialized(
        role,
        plan::semantic_key(draft, role),
        MemberDestination::AtDeclarationSite,
        plan::rust_declaration_profile(),
        plan::rust_declaration_profile_version(),
        plan::member_origin(draft, role),
        tree,
    )
    .map_err(|refusal| diagnose::rendering_refused(refusal, role))
}
