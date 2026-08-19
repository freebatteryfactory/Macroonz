#![doc = include_str!("README.md")]

pub mod capture;
pub mod carry;
pub mod diagnose;
pub mod document;
pub mod explain;
pub mod plan;
pub mod render;
mod type_contract;
pub mod types;

pub use capture::{captured, captured_text};
pub use carry::{
    assembly, bench_disposition, carrier_expansion, carrier_kind, carrier_node, carrier_origin,
    carrier_plan, carrier_semantic_key, deferred_selectors, evaluation_axis, rows_disposition,
};
pub use diagnose::{LineBody, LineSite, RefusalClass, RefusalLine, RenderedMagnitude, composed};
pub use document::{CapturedDocumentationReading, documented};
pub use explain::{ExplanationBindingRefusal, ExplanationSeat};
pub use plan::DerivedPlan;
pub use render::{
    CAUSE_ORDER_CONTRACT, EVALUATION_SUBJECT, FAMILY_CONTRACT, REFUSAL_MODULE, RenderRefusal,
};
pub use types::{
    CapturedCause, CapturedDocumentation, CauseOrderStanding, CrateBinding, DEFAULT_CRATE_BINDING,
    DIAGNOSTIC_PREFIX, DerivedMembership, DocumentedDeclaration, RefusalCompileContext,
    RefusalDerivationDraft, RefusalDeriveCapture, RefusalDeriveFact, RefusalDeriveRefusal,
    RefusalDeriveSurface, RefusalFamilyExpansion, RefusalOwnerFacts, RefusalSite,
    SHAPE_WORD_INSEPARABLE_PAIR, SHAPE_WORD_ISSUE_COLLECTION, SHAPE_WORD_SINGLE_CAUSE,
};

use crate::closure::{ProjectionClosure, RenderedProjection, RenderedUnit};
use crate::derive_impl::{EvaluationBinding, MutationPointTable, evaluation_copy};
use crate::diagnostics::MacrocDiagnostic;
use crate::generated_support::JoinedExpansion;
use crate::planning::RenderedImplementation;
use crate::token::{CapturedInput, GeneratedTree, TextCapture};
use threadpak::types::Bounded;

/// Capture, plan, render, close, and explain one refusal-family declaration —
/// the whole road, in one call, and the only road there is.
///
/// # Errors
///
/// Returns a [`MacrocDiagnostic`] whenever any step refuses: a declaration the
/// grammar does not admit, a plan whose magnitudes are exceeded, a rendering
/// that outgrows its bound or that cannot be copied over the evaluation
/// subject, a closure the rendering does not satisfy, an explanation that does
/// not cover its kind's questions, or a binding whose three values do not belong
/// to one expansion.
/// **Every one of those refusals happens BEFORE a token exists to emit**,
/// because every emission is reachable only off the value this function returns
/// on success.
#[expect(
    clippy::result_large_err,
    reason = "the diagnostic is seat-complete by law, and the settled service signature returns it by value: boxing it here would move a required seat behind a pointer to satisfy a size lint"
)]
pub fn compile_refusal(
    input: &CapturedInput,
    context: &RefusalCompileContext,
) -> Result<RefusalFamilyExpansion, MacrocDiagnostic> {
    let surface = captured(input)
        .map_err(|refusal| refusal.diagnosed(&context.spans, context.machine.clone()))?;
    let draft = surface.planned();

    let planned = plan::planned(&draft, context.owner_facts, context.nonclaims.clone())
        .map_err(|refusal| diagnose::planning_refused(&refusal))?;

    let rendered = render_units(&draft)?;

    // The closure SPLITS the rendered units across the deliveries their members
    // declared, joins each emission in role-roster order, keeps them, and
    // commits to each one's digest — so there is nothing left to assemble on
    // this road after the proof returns, and the tokens each build receives are
    // inside what was proved rather than concatenated behind it.
    let closure = ProjectionClosure::proved(
        planned.plan().identity(),
        planned.plan().membership(),
        rendered,
    )
    .map_err(|refusal| diagnose::closure_refused(&refusal))?;

    let explanation = explain::explained(&planned, &closure)
        .map_err(|refusal| diagnose::explanation_refused(&refusal))?;

    let (plan_value, cause_order) = planned.into_parts();
    RefusalFamilyExpansion::bound(
        draft.surface().clone(),
        plan_value,
        closure,
        explanation,
        cause_order,
    )
    .map_err(|refusal| diagnose::expansion_refused(&refusal))
}

/// Capture, plan, render, close, and explain one refusal-family declaration AND
/// the carrier that delivers what it deferred — the whole joined road, in one
/// call.
///
/// # The same public steps, joined
///
/// This is not a second door and it is not a lobby. It walks
/// [`compile_refusal`] whole for the implementation projection, and then the
/// same eight public steps a second time for the CARRIER projection: an account
/// over the same captured surface, a context, a plan, a rendering, a proved
/// closure, an explanation, and the terminal that binds them. What sits between
/// the two is the assembly, which is not a step of either road — it is the
/// physical statement that the first road's proved cargo composes into the
/// second road's vehicle.
///
/// # Why the impl-only road stays
///
/// [`compile_refusal`] is the projection road and its callers stand: a caller
/// that wants the implementation projection and nothing else asks for exactly
/// that and is handed a terminal. This road is what a caller asks for when it
/// wants the declaration DELIVERED — the implementations at the declaration
/// site, and a carrier a consumption target can invoke — and the difference
/// between the two is a second terminal rather than a different first one.
///
/// # What comes back
///
/// [`JoinedExpansion`] over this family's own view: BOTH terminals and the
/// assembly that joined them. Its two declaration-site cargos are exactly the
/// two terminals' declaration-site partitions — the implementation members, and
/// the shell definition — read off the terminals themselves. An emitter writes
/// both; writing one would leave a carrier nobody defined or a declaration that
/// never expands.
///
/// # Errors
///
/// Returns a [`MacrocDiagnostic`] wherever any step of either road refuses, and
/// wherever the two roads' outputs do not compose into one carrier: cargo from a
/// terminal planned over another declaration, a gate pinned against an
/// expectation these services do not publish, one terminal's partition read
/// twice, cargo read from a partition the axis does not deliver from, or tokens
/// that are not the terminal's own.
/// **Every one of those refusals happens BEFORE a token exists to emit**, on the
/// same terms the impl-only road states.
#[expect(
    clippy::result_large_err,
    reason = "the diagnostic is seat-complete by law, and the settled service signature returns it by value: boxing it here would move a required seat behind a pointer to satisfy a size lint"
)]
pub fn compile_declaration(
    input: &CapturedInput,
    context: &RefusalCompileContext,
) -> Result<JoinedExpansion<RefusalFamilyExpansion>, MacrocDiagnostic> {
    let implementation = compile_refusal(input, context)?;
    // The draft is read off the terminal's own captured surface rather than
    // captured a second time: two captures of one declaration produce one
    // surface, and a second walk would be a second reading of the material the
    // first road already stands on.
    let draft = implementation.surface().clone().planned();

    // The assembly stands between the two roads: it reads the implementation
    // terminal's proved test-carrier cargo, states the trials and bench axes
    // absent with the dispositions that say why, and verifies the whole before
    // any carrier token exists.
    let assembly = carry::assembly(&draft, implementation.expansion())?;
    let plan = carry::carrier_plan(&draft)?;
    let carrier = carry::carrier_expansion(plan, &assembly)?;
    Ok(JoinedExpansion::joined(implementation, carrier, assembly))
}

/// How the callable text route refused.
///
/// Two postures, and they are genuinely different observations.
/// A text that cannot be cut into tokens never reached the grammar at all and
/// has no span table to point into; a text that cut fine and said the wrong
/// thing has both.
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
) -> Result<(TextCapture, RefusalFamilyExpansion), TextCompileRefusal> {
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

/// The active-point enum the family contract's evaluation copy declares.
///
/// A literal identifier and never a spelling composed from the declaration:
/// composing one would be this home deciding how an author's own type name
/// becomes a Rust identifier, which is a spelling law nobody gave it.
const FAMILY_ACTIVE_POINT: &str = "RefusalFamilyActivePoint";

/// The name the family contract's evaluation copy reads its selector through.
///
/// The copy READS this name and never declares it. What brings it into scope at
/// every activation site is the generated support shell's splice, which
/// declares the constant beside the local subject the copy stands over — inside
/// the shell's own module, in the same expansion, so the copy and the name it
/// reads arrive at the consumption target together or not at all.
/// That splice is the shell's rendering and not this home's; what this home owns
/// is the spelling, which the shell reads as the data it is.
const FAMILY_ACTIVE_POINT_SELECTOR: &str = "REFUSAL_FAMILY_ACTIVE_POINT";

/// The active-point enum the cause-order contract's evaluation copy declares.
const CAUSE_ORDER_ACTIVE_POINT: &str = "CauseOrderActivePoint";

/// The name the cause-order contract's evaluation copy reads its selector
/// through.
const CAUSE_ORDER_ACTIVE_POINT_SELECTOR: &str = "CAUSE_ORDER_ACTIVE_POINT";

/// The two spellings one contract's evaluation copy names: the active-point
/// enum it declares, and the selector it reads.
///
/// One pair per CONTRACT rather than one pair for the home. A declaration that
/// carries both contracts delivers two evaluation copies, and one enum spelling
/// across them would declare a single type twice wherever the shell lands them.
///
/// Read through either half of a pair the answer is the same, because the
/// spellings belong to the PAIR rather than to one half of it — so no caller
/// has to hold which half it is looking at, and the two evaluation seats are
/// written out beside the two production ones rather than collapsed under a
/// wildcard.
const fn evaluation_spellings(role: RenderedImplementation) -> (&'static str, &'static str) {
    match role {
        RenderedImplementation::RenderedFamilyImpl
        | RenderedImplementation::RenderedFamilyEvaluation => {
            (FAMILY_ACTIVE_POINT, FAMILY_ACTIVE_POINT_SELECTOR)
        }
        RenderedImplementation::RenderedCauseOrderImpl
        | RenderedImplementation::RenderedCauseOrderEvaluation => {
            (CAUSE_ORDER_ACTIVE_POINT, CAUSE_ORDER_ACTIVE_POINT_SELECTOR)
        }
    }
}

/// Transform one contract's body, rendered over the evaluation subject, into
/// its mutation-evaluation copy.
///
/// # No mutation point is admitted here, and that is a stated fact
///
/// Which operation is worth damaging, which alternatives stand against it, and
/// which claim owns the site are the harness's declarations, and nothing
/// reaches this home carrying any of them: [`compile_refusal`] is handed a
/// captured declaration and an expansion context, and neither names a point.
///
/// So the table composed below is the honest minimum this delivery admits.
/// [`MutationPointTable::over`] seats the mandatory no-mutation control
/// STRUCTURALLY, and the admitted set beside it is empty — a stated fact about
/// a declaration nobody admitted a damage against, rather than a set somebody
/// forgot to supply. Under the control every point renders its original
/// operation, so a copy with no admitted point carries exactly the production
/// surface's own operations under another subject's head, which is the parity
/// the control exists to prove and the whole of what the rendering guard
/// established before this seat was reached.
///
/// # Errors
///
/// Returns the token-magnitude diagnostic naming the role that overran it.
///
/// The three roads below refuse in the derive-implementation home's own
/// families, and at THIS seat all three reduce to that one observation. The
/// two spellings are the literal identifiers declared above, so a spelling that
/// is not one is not a value these calls can produce; the admitted set is
/// empty, so no name can be doubled, no point can claim the control's name, and
/// no count can outgrow its magnitude; and with no point to stand in, every
/// composition issue that names one is unestablishable. What remains is the
/// copy outgrowing the declared token magnitude, which is exactly what
/// [`RenderRefusal::Unbounded`] names. A seat that ever admitted a caller's
/// points would owe [`diagnose`] a projection of the composition family; this
/// one admits none.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the settled service road returns; this helper hands \n              it straight through"
)]
fn evaluation_tree(
    production: &GeneratedTree,
    role: RenderedImplementation,
) -> Result<GeneratedTree, MacrocDiagnostic> {
    let unbounded = || diagnose::render_refused(RenderRefusal::Unbounded, role);
    let (active_enum, selector) = evaluation_spellings(role);
    let binding = EvaluationBinding::declared(active_enum, selector).map_err(|_| unbounded())?;
    let table = MutationPointTable::over(Vec::new()).map_err(|_| unbounded())?;
    evaluation_copy(&binding, &table, production).map_err(|_| unbounded())
}

/// Render every planned role into a rendered unit.
///
/// The roster is fixed by the shape, so the rendering is built by matching on
/// the two answers — and each answer names TWO members per contract, because
/// one implementation meaning is delivered as two surfaces and the plan
/// declares both ([`plan::membership`]). A rendering that materialized the
/// production half alone would leave the closure rebuilding a membership half
/// the size of the one the plan states, and the proof would refuse — so the
/// copy is rendered here rather than being a surface that crosses the wall
/// outside the declared set.
/// [`RenderedProjection::complete`] settles the magnitude at compile time, which
/// is why neither arm carries a refusal road of its own.
///
/// The twin is READ from the roster rather than named here
/// ([`RenderedImplementation::twin`]), exactly as the plan reads it, so a
/// roster that paired its seats differently pairs these units differently too.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the settled service road returns; this helper hands \n              it straight through"
)]
fn render_units(
    draft: &RefusalDerivationDraft,
) -> Result<RenderedProjection<RenderedImplementation>, MacrocDiagnostic> {
    let family = RenderedImplementation::RenderedFamilyImpl;
    let cause_order = RenderedImplementation::RenderedCauseOrderImpl;
    let family_implementation = rendered_unit(draft, family)?;
    let family_evaluation = rendered_unit(draft, family.twin())?;
    match draft.declared_membership() {
        DerivedMembership::FamilyOnly => Ok(RenderedProjection::complete(
            family_implementation,
            [family_evaluation],
        )),
        DerivedMembership::FamilyAndCauseOrder => {
            let order_implementation = rendered_unit(draft, cause_order)?;
            let order_evaluation = rendered_unit(draft, cause_order.twin())?;
            Ok(RenderedProjection::complete(
                family_implementation,
                [family_evaluation, order_implementation, order_evaluation],
            ))
        }
    }
}

/// Render one role into one materialized unit, projecting either refusal into a
/// diagnostic that names what the rendering could not be done under and the
/// role it was refused at.
///
/// Total in the role it is handed, and the four seats are written out one by
/// one because they are four renderings rather than two: both halves of a pair
/// stand over one contract's BODY, and the two halves stand that body over two
/// SUBJECTS. The production member is the body implemented for the type the
/// declaration named; the evaluation member is the same body implemented for
/// the support shell's own local subject, and then transformed into the copy.
/// A fifth role stops the compiler here rather than falling through a wildcard
/// into whichever contract the last arm happened to name.
///
/// The body is rendered again for the copy rather than handed over from the
/// member beside it, and the two renderings could not share a tree in any case:
/// each unit is a function of the DECLARATION alone, no member's rendering
/// depends on another member's having been rendered first — which is the
/// ordering the roster exists to remove — and the copy's head names a subject
/// the production member's head does not.
#[expect(
    clippy::result_large_err,
    reason = "the same seat-complete diagnostic the settled service road returns; this helper hands \n              it straight through"
)]
fn rendered_unit(
    draft: &RefusalDerivationDraft,
    role: RenderedImplementation,
) -> Result<RenderedUnit<RenderedImplementation>, MacrocDiagnostic> {
    let implemented = match role {
        RenderedImplementation::RenderedFamilyImpl => {
            render::family_implementation(draft.surface())
        }
        RenderedImplementation::RenderedCauseOrderImpl => {
            render::cause_order_implementation(draft.surface())
        }
        RenderedImplementation::RenderedFamilyEvaluation => {
            render::family_evaluation_implementation(draft.surface())
        }
        RenderedImplementation::RenderedCauseOrderEvaluation => {
            render::cause_order_evaluation_implementation(draft.surface())
        }
    }
    .map_err(|refusal| diagnose::render_refused(refusal, role))?;
    let tree = if role.is_evaluation_copy() {
        evaluation_tree(&implemented, role)?
    } else {
        implemented
    };
    RenderedUnit::materialized(
        role,
        plan::semantic_key(draft, role),
        // The roster's own constant answer, not a literal repeated here: where a
        // member under a role lands is the role's fact, and the plan states this
        // member's destination by reading the same answer.
        role.destination(),
        plan::rust_declaration_profile(),
        plan::rust_declaration_profile_version(),
        plan::member_origin(draft, role),
        tree,
    )
    .map_err(|refusal| diagnose::rendering_refused(refusal, role))
}
