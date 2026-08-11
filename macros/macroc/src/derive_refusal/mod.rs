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
//! [`ReproductionRoute::CallableServices`] because that route is real, and
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
//! # What this home does not decide
//!
//! It decides no meaning. The three body shapes are band 00's; the canonical key
//! grammar is band 00's; the selection order's *content* is the author's; the
//! local keys are the author's; the `RefusalFamily` and `CauseOrderDeclaration`
//! contracts are band 00's. This home reads a declaration and writes down what
//! it already said.

pub mod capture;
pub mod explain;
pub mod plan;
pub mod render;
pub mod types;

pub use capture::{captured, captured_text};
pub use plan::DerivedPlan;
pub use render::RenderRefusal;
pub use types::{
    CapturedCause, CauseOrderStanding, ClosedExpansion, CrateBinding, DEFAULT_CRATE_BINDING,
    DerivedMembership, RefusalCompileContext, RefusalDerivationDraft, RefusalDeriveCapture,
    RefusalDeriveRefusal, RefusalDeriveSurface, RefusalOwnerFacts, SHAPE_WORD_INSEPARABLE_PAIR,
    SHAPE_WORD_ISSUE_COLLECTION, SHAPE_WORD_SINGLE_CAUSE,
};

use crate::closure::{ProjectionClosure, RenderedProjection, RenderedUnit};
use crate::diagnostics::{
    DiagnosticSite, MacrocDiagnostic, MacrocPhase, ObservedClassification, ReleasePosture,
    RepairAction, ReproductionRoute,
};
use crate::plane::{HumanTextLimit, OwnerFactRef, human_projection};
use crate::planning::{MemberDestination, RenderedImplementation};
use crate::token::{CapturedInput, SpanHandle, TextCapture};
use threadpak::evidence::CauseDisposition;
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
        .map_err(|_| step_refusal(MacrocPhase::Planning, ObservedClassification::BoundExceeded))?;

    let rendered = render_units(&draft)?;

    let closure =
        ProjectionClosure::proved(planned.plan().membership(), rendered).map_err(|_| {
            step_refusal(
                MacrocPhase::Rendering,
                ObservedClassification::IdentityDisagreement,
            )
        })?;

    let explanation = explain::explained(&planned, &closure)
        .map_err(|_| step_refusal(MacrocPhase::Inspection, ObservedClassification::SeatAbsent))?;

    let emitted = closure.rendered().joined_tree().map_err(|_| {
        step_refusal(
            MacrocPhase::Rendering,
            ObservedClassification::BoundExceeded,
        )
    })?;

    let (plan_value, cause_order) = planned.into_parts();
    Ok(ClosedExpansion::bound(
        draft.surface().clone(),
        plan_value,
        closure,
        explanation,
        cause_order,
        emitted,
    ))
}

/// How the callable text route refused.
///
/// Two postures, and they are genuinely different observations. A text that
/// cannot be cut into tokens never reached the grammar at all and has no span
/// table to point into; a text that cut fine and said the wrong thing has both.
/// Folding them together would hand a caller a diagnostic whose site indexes a
/// table that was never built.
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
/// This is what makes [`ReproductionRoute::CallableServices`] a real road rather
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
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the ruled service road returns; this helper hands \n              it straight through"
)]
fn render_units(
    draft: &RefusalDerivationDraft,
) -> Result<RenderedProjection<RenderedImplementation>, MacrocDiagnostic> {
    let mut units: Vec<RenderedUnit<RenderedImplementation>> = Vec::new();
    for role in draft.declared_membership().roles() {
        let tree = match role {
            RenderedImplementation::RenderedFamilyImpl => {
                render::family_implementation(draft.surface())
            }
            RenderedImplementation::RenderedCauseOrderImpl => {
                render::cause_order_implementation(draft.surface())
            }
        }
        .map_err(|_| {
            step_refusal(
                MacrocPhase::Rendering,
                ObservedClassification::BoundExceeded,
            )
        })?;
        let unit = RenderedUnit::materialized(
            *role,
            plan::semantic_key(draft, *role),
            MemberDestination::AtDeclarationSite,
            plan::rust_declaration_profile(),
            plan::rust_declaration_profile_version(),
            plan::member_origin(draft, *role),
            tree,
        )
        .map_err(|_| {
            step_refusal(
                MacrocPhase::Rendering,
                ObservedClassification::BoundExceeded,
            )
        })?;
        units.push(unit);
    }
    let mut rows = units.into_iter();
    let Some(first) = rows.next() else {
        // Unreachable: a declared membership is non-empty for both answers.
        return Err(step_refusal(
            MacrocPhase::Rendering,
            ObservedClassification::SeatAbsent,
        ));
    };
    RenderedProjection::materialized(first, rows.collect()).map_err(|_| {
        step_refusal(
            MacrocPhase::Rendering,
            ObservedClassification::BoundExceeded,
        )
    })
}

/// The diagnostic one non-capture step refuses with.
///
/// It points at the declaration's first token, which is the honest site: the
/// disagreement is about the declaration as a whole rather than about one token
/// inside it, and pretending otherwise would send a reader to an arbitrary spot.
fn step_refusal(phase: MacrocPhase, observed: ObservedClassification) -> MacrocDiagnostic {
    MacrocDiagnostic {
        summary: human_projection!(
            HumanTextLimit,
            "threadpak refusal-family derive: a declared magnitude was exceeded, or a rendering \n             did not close over the plan it claims to materialize"
        ),
        machine: crate::diagnostics::MachineAnchoring::UnmintedAtThisSeam,
        phase,
        site: DiagnosticSite {
            token: SpanHandle::at(0),
            coordinate: threadpak::declaration::SourceCoordinate {
                role: threadpak::declaration::CoordinateRole::SemanticOrigin,
                position: 0,
            },
        },
        expected: types::expected_contract(),
        observed,
        cause: CauseDisposition::UnresolvedCause,
        related: Bounded::empty(),
        repairs: Bounded::from_array([RepairAction {
            declared_by: OwnerFactRef::named("refusal", "family-shapes-are-three-and-closed"),
            description: human_projection!(
                HumanTextLimit,
                "a declared magnitude was exceeded, or a rendering did not close over its plan"
            ),
        }]),
        reproduction: ReproductionRoute::CallableServices {
            entry: types::callable_entry(),
        },
        release: ReleasePosture::NoReleasePromise,
    }
}
