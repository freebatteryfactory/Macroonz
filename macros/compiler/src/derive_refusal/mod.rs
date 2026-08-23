#![doc = include_str!("README.md")]

pub mod account;
pub mod capture;
pub mod carry;
pub mod diagnose;
pub mod explain;
pub mod plan;
pub mod render;
mod type_contract;
pub mod types;

pub use account::{
    accounted, benchmark_disposition, codec_disposition, pattern_stamp_disposition,
    profile_does_not_offer,
};
pub use capture::{captured, captured_text};
pub use carry::{
    assembly, bench_disposition, carrier_expansion, carrier_kind, carrier_node, carrier_origin,
    carrier_plan, carrier_semantic_key, evaluation_axis, rows_disposition, trials_axis,
};
pub use diagnose::composed;
pub use plan::DerivedPlan;
pub use render::{CAUSE_ORDER_CONTRACT, FAMILY_CONTRACT};
pub use types::{
    CapturedCause, CapturedCommitments, CapturedDocumentation, CapturedFamilyFacts,
    CarrierRoadRefusal, CauseOrderStanding, CrateBinding, DEFAULT_CRATE_BINDING, DIAGNOSTIC_PREFIX,
    DeclaredMutations, DeclaredTrials, DeriveCauseLimit, DerivedMembership, DocumentedDeclaration,
    ExplanationBindingRefusal, ExplanationSeat, LineBody, LineSite, MemberRenderCause,
    MemberRenderRefusal, MutationDeclarationPosture, RefusalClass, RefusalCompileContext,
    RefusalDerivationDraft, RefusalDeriveCapture, RefusalDeriveFact, RefusalDeriveRefusal,
    RefusalDeriveSurface, RefusalFamilyExpansion, RefusalLine, RefusalOwnerFacts, RefusalSite,
    RenderRefusal, RenderedMagnitude, SHAPE_WORD_INSEPARABLE_PAIR, SHAPE_WORD_ISSUE_COLLECTION,
    SHAPE_WORD_SINGLE_CAUSE, SurfaceCaptureRefusal, TextCompileRefusal, TrialDeclarationPosture,
};

use crate::closure::{ProjectionClosure, RenderedProjection, RenderedUnit};
use crate::diagnostics::MacrocDiagnostic;
use crate::generated_support::{AccountedExpansion, JoinedExpansion};
use crate::planning::RenderedImplementation;
use crate::token::{CapturedInput, TextCapture};
use macroonz::Bounded;

/// Captures, plans, renders, closes, and explains one refusal-family declaration through the one callable road.
///
/// # Errors
///
/// Returns a [`MacrocDiagnostic`] when capture, planning, rendering, closure, explanation, or final binding refuses.
/// Every refusal occurs before emission because emitted material is reachable only from the returned expansion.
#[expect(
    clippy::result_large_err,
    reason = "the diagnostic is seat-complete by law, and the settled service signature returns it by value: boxing it here would move a required seat behind a pointer to satisfy a size lint"
)]
pub fn compile_refusal(
    input: &CapturedInput,
    context: &RefusalCompileContext,
) -> Result<RefusalFamilyExpansion, MacrocDiagnostic> {
    // Two grammars answer at the capture, and each is projected by the road its
    // own home owns: a declaration the derive grammar could not read is this
    // home's diagnostic, and a trial declaration the carrier's grammar could not
    // read is projected at the exact clause it was established at.
    let surface = captured(input).map_err(|refusal| match refusal {
        SurfaceCaptureRefusal::Declaration(read) => read.diagnosed(&context.spans),
        SurfaceCaptureRefusal::Trials(trials) => {
            diagnose::trial_declaration_refused(trials, &context.spans)
        }
        SurfaceCaptureRefusal::Mutations(mutations) => {
            diagnose::mutation_declaration_refused(mutations, &context.spans)
        }
    })?;
    let draft = surface.planned();

    let planned = plan::planned(&draft, context.owner_facts, context.nonclaims.clone())
        .map_err(|refusal| diagnose::planning_refused(&refusal))?;

    let rendered = render_units(&draft).map_err(diagnose::member_render_refused)?;

    // The closure SPLITS the rendered units across the deliveries their members
    // declared, joins each emission in role-roster order, keeps them, and
    // commits to each one's digest — so there is nothing left to assemble on
    // this road after the proof returns, and the tokens each build receives are
    // inside what was proved rather than concatenated behind it.
    let closure = ProjectionClosure::proved(planned.plan(), rendered)
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
/// the carrier that delivers what it deferred, and state what happened to every
/// other kind of the sealed roster — the whole door, in one call.
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
/// site, a carrier a consumption target can invoke, and the account of what
/// became of everything else — and the difference between the two is what is
/// added, never a different first road.
///
/// # What comes back
///
/// [`AccountedExpansion`] over this family's own view. Inside it stands the
/// [`JoinedExpansion`] the joined road produces — BOTH terminals and the
/// assembly that joined them — whose two declaration-site cargos are exactly the
/// two terminals' declaration-site partitions, the implementation members and
/// the shell definition, read off the terminals themselves. An emitter writes
/// both; writing one would leave a carrier nobody defined or a declaration that
/// never expands.
///
/// Beside it stands one typed disposition for every kind of the sealed roster.
/// The two kinds this door generates read their answers off the terminals that
/// produced them; the six it does not each carry the ground stated at their own
/// road in [`account`]. Nothing is silently absent, and nothing is a seat
/// generated to look full.
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
/// The roster's remaining dispositions add no refusal road of their own: they
/// are decisions this door records, and a decision does not fail.
#[expect(
    clippy::result_large_err,
    reason = "the diagnostic is seat-complete by law, and the settled service signature returns it by value: boxing it here would move a required seat behind a pointer to satisfy a size lint"
)]
pub fn compile_declaration(
    input: &CapturedInput,
    context: &RefusalCompileContext,
) -> Result<AccountedExpansion<RefusalFamilyExpansion>, MacrocDiagnostic> {
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
    // Each step of the carrier road answers in the vocabulary of the home that
    // owns it, and this door is where those bodies become one line. The
    // projection is the same road for every arm, so no step's answer is
    // summarized on the way here.
    let assembly =
        assembly(&draft, implementation.expansion()).map_err(diagnose::carrier_road_refused)?;
    let plan = carrier_plan(&draft).map_err(|refusal| diagnose::planning_refused(&refusal))?;
    let carrier =
        carrier_expansion(&draft, plan, &assembly).map_err(diagnose::carrier_road_refused)?;

    // Read BEFORE the two terminals move into the joined value, and read off
    // the terminals themselves: what a generated kind produced is its plan's
    // answer, and a disposition composed here would be a second one.
    let dispositions = accounted(&implementation, &carrier);
    Ok(AccountedExpansion::accounted(
        JoinedExpansion::joined(implementation, carrier, assembly),
        dispositions,
    ))
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
        owner_facts: RefusalOwnerFacts::declared(),
        nonclaims: Bounded::empty(),
    };
    match compile_refusal(read.input(), &context) {
        Ok(closed) => Ok((read, closed)),
        Err(diagnostic) => Err(TextCompileRefusal::Refused(Box::new((read, diagnostic)))),
    }
}

/// Renders the exact role set selected by [`DerivedMembership`] into role-bound units.
/// The set contains the family implementation, the cause-order implementation where the shape owns one, and the generated mutation module where the helper declares it.
///
/// # Errors
///
/// Returns [`MemberRenderRefusal`] with the exact role and rendering cause.
fn render_units(
    draft: &RefusalDerivationDraft,
) -> Result<RenderedProjection<RenderedImplementation>, MemberRenderRefusal<RenderedImplementation>>
{
    let family = RenderedImplementation::RenderedFamilyImpl;
    let cause_order = RenderedImplementation::RenderedCauseOrderImpl;
    let family_implementation = rendered_unit(draft, family)?;
    match draft.declared_membership() {
        DerivedMembership::FamilyOnly => Ok(RenderedProjection::of_one(family_implementation)),
        DerivedMembership::FamilyAndCauseOrder => {
            let order_implementation = rendered_unit(draft, cause_order)?;
            Ok(RenderedProjection::complete(
                family_implementation,
                [order_implementation],
            ))
        }
        DerivedMembership::FamilyCauseOrderAndMutationEvaluation => {
            let order_implementation = rendered_unit(draft, cause_order)?;
            let mutation =
                rendered_unit(draft, RenderedImplementation::RenderedMutationEvaluation)?;
            Ok(RenderedProjection::complete(
                family_implementation,
                [order_implementation, mutation],
            ))
        }
    }
}

/// Render one role into one materialized unit and retain the role at any refusal.
///
/// The mutation role constructs one complete informed request before the mechanical renderer is called.
///
/// # Errors
///
/// Returns [`MemberRenderRefusal`] with the renderer or materialization refusal.
fn rendered_unit(
    draft: &RefusalDerivationDraft,
    role: RenderedImplementation,
) -> Result<RenderedUnit<RenderedImplementation>, MemberRenderRefusal<RenderedImplementation>> {
    let refused = |cause: MemberRenderCause| MemberRenderRefusal { role, cause };
    let implemented = match role {
        RenderedImplementation::RenderedFamilyImpl => {
            render::family_implementation(draft.surface())
        }
        RenderedImplementation::RenderedCauseOrderImpl => {
            render::cause_order_implementation(draft.surface())
        }
        RenderedImplementation::RenderedMutationEvaluation => {
            render::mutation_projection_request(draft.surface()).and_then(|request| {
                crate::mutation_descriptor::render::generated_module(&request)
                    .map_err(|_| RenderRefusal::MutationRenderingUnbounded)
            })
        }
    }
    .map_err(|cause| refused(MemberRenderCause::Rendered(cause)))?;
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
        implemented,
    )
    .map_err(|cause| refused(MemberRenderCause::Materialized(cause)))
}
