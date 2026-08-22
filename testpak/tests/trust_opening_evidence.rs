//! The public mutation receiver from owner policy through compiled pressure, exact no-mutation parity, active execution, and ordinary report evidence.

use std::sync::atomic::{AtomicU32, Ordering};
use threadpak_testpak::clock::{HarnessClock, MeasurementReading};
use threadpak_testpak::descriptor::{
    Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite,
    MutationPointRef, Origin, PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute,
    Tag, TrialTableRefusal,
};
use threadpak_testpak::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use threadpak_testpak::muterprater::interpret::{
    availability, execute_active, observe_no_mutation, qualify_no_mutation,
};
use threadpak_testpak::muterprater::wrap::read_output;
use threadpak_testpak::muterprater::{
    ActiveSelection, AdapterQualification, AlternativeDeclaration, AlternativeId, BackendVersion,
    BackendVersionPosture, CompiledPressureWitness, EvaluationBinding, EvaluationControl,
    EvaluationFamilyRef, EvaluationObservation, EvaluationPair, EvaluationPairRefusal,
    EvaluationSurface, GrammarStanding, IntendedRejection, InterpretedExecutionRefusal,
    InterpreterAvailability, MissingTrustEvidence, MutationIdentity, MutationOutcome,
    MutationPermission, MutationPoint, MutationPolicy, MutationVerdict, MutationWitness,
    MutationWitnessRefusal, NoMutationObservationRefusal, OperatorFamilyRef,
    ParityQualificationRefusal, PermissionRefusal, PointCatalogPosture, PointRefusal,
    PolicyRefusal, PressureWitnessRefusal, ProductionBinding, QualificationRefusal,
    RewriteAdmission, RewriteWithheld, SourceCoordinate, SurfaceRefusal, WrapReading, WrapRefusal,
    WrapStanding,
};
use threadpak_testpak::properties::{Agreement, agreement};
use threadpak_testpak::report::{
    ByteBudget, CaseBudget, FindingCause, InvocationProfile, RunAttempt, TargetBinding,
    TargetTriple, TimeBudget, ToolchainIdentity, TrialConclusion, TrialSite, encode_bytes,
    encode_length,
};
use threadpak_testpak::runner::{Invocation, TrialBinding};

const OWNER: &str = "testpak.mutation.receiver";
const BACKEND_CONSOLE: &str = "Found 1 mutant to test\n\
    ok Unmutated baseline in 3.1s\n\
    caught src/subject/lane.rs:41:9: replace is_qualified -> bool with true in 4.0s";
const BACKEND_NO_KILL: &str = "Found 1 mutant to test\n\
    ok Unmutated baseline in 3.1s\n\
    missed src/subject/lane.rs:41:9: replace is_qualified -> bool with true in 4.0s";
const BACKEND_VERSION: &str = "25.0.0";
const MEANING_DISAGREEMENT: FindingCause = FindingCause::named(OWNER, "meaning-disagreement");
const REVISION_TAG: DomainTag = DomainTag::declared(
    "mutation-receiver-revision",
    IdentityProfileVersion::declared(1),
);
const POLICY_READING_TAG: DomainTag =
    DomainTag::declared("mutation-policy", IdentityProfileVersion::declared(1));
const ALTERNATIVE_READING_TAG: DomainTag =
    DomainTag::declared("mutation-alternative", IdentityProfileVersion::declared(1));
const SURFACE_READING_TAG: DomainTag =
    DomainTag::declared("evaluation-surface", IdentityProfileVersion::declared(1));
static CLAIM_MISMATCH_EVALUATION_CALLS: AtomicU32 = AtomicU32::new(0);

#[derive(Debug, PartialEq, Eq)]
enum MutationRoadFailure {
    Name,
    Permission(PermissionRefusal),
    Policy(PolicyRefusal),
    Point(PointRefusal),
    Surface(SurfaceRefusal),
    Pair(EvaluationPairRefusal),
    Table(TrialTableRefusal),
    Wrap(WrapRefusal),
    Qualification(QualificationRefusal),
    Pressure(PressureWitnessRefusal),
    Witness(MutationWitnessRefusal),
    Observation(NoMutationObservationRefusal),
    Interpreted(InterpretedFailureStage),
    MissingFamily,
    MissingAlternative,
    MissingActiveSelection,
    MissingQualification(ParityQualificationRefusal),
    MissingTrust(MissingTrustEvidence),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InterpretedFailureStage {
    Selection,
    WitnessClaim,
    DudPlant,
    Report,
}

impl From<PermissionRefusal> for MutationRoadFailure {
    fn from(refusal: PermissionRefusal) -> Self {
        Self::Permission(refusal)
    }
}

impl From<PolicyRefusal> for MutationRoadFailure {
    fn from(refusal: PolicyRefusal) -> Self {
        Self::Policy(refusal)
    }
}

impl From<PointRefusal> for MutationRoadFailure {
    fn from(refusal: PointRefusal) -> Self {
        Self::Point(refusal)
    }
}

impl From<SurfaceRefusal> for MutationRoadFailure {
    fn from(refusal: SurfaceRefusal) -> Self {
        Self::Surface(refusal)
    }
}

impl From<EvaluationPairRefusal> for MutationRoadFailure {
    fn from(refusal: EvaluationPairRefusal) -> Self {
        Self::Pair(refusal)
    }
}

impl From<TrialTableRefusal> for MutationRoadFailure {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::Table(refusal)
    }
}

impl From<WrapRefusal> for MutationRoadFailure {
    fn from(refusal: WrapRefusal) -> Self {
        Self::Wrap(refusal)
    }
}

impl From<QualificationRefusal> for MutationRoadFailure {
    fn from(refusal: QualificationRefusal) -> Self {
        Self::Qualification(refusal)
    }
}

impl From<PressureWitnessRefusal> for MutationRoadFailure {
    fn from(refusal: PressureWitnessRefusal) -> Self {
        Self::Pressure(refusal)
    }
}

impl From<MutationWitnessRefusal> for MutationRoadFailure {
    fn from(refusal: MutationWitnessRefusal) -> Self {
        Self::Witness(refusal)
    }
}

impl From<NoMutationObservationRefusal> for MutationRoadFailure {
    fn from(refusal: NoMutationObservationRefusal) -> Self {
        Self::Observation(refusal)
    }
}

impl From<InterpretedExecutionRefusal> for MutationRoadFailure {
    fn from(refusal: InterpretedExecutionRefusal) -> Self {
        let stage = match refusal {
            InterpretedExecutionRefusal::Selection(_) => InterpretedFailureStage::Selection,
            InterpretedExecutionRefusal::WitnessForAnotherClaim { .. } => {
                InterpretedFailureStage::WitnessClaim
            }
            InterpretedExecutionRefusal::DudPlant(_) => InterpretedFailureStage::DudPlant,
            InterpretedExecutionRefusal::Report(_) => InterpretedFailureStage::Report,
        };
        Self::Interpreted(stage)
    }
}

fn family(stem: &'static str) -> Result<EvaluationFamilyRef, MutationRoadFailure> {
    EvaluationFamilyRef::named(OWNER, stem).map_err(|_| MutationRoadFailure::Name)
}

fn push_name(into: &mut Vec<u8>, name: threadpak_testpak::descriptor::NamespacedName) {
    encode_bytes(name.namespace().written().as_bytes(), into);
    encode_bytes(name.stem().written().as_bytes(), into);
}

fn claim() -> Result<ClaimRef, MutationRoadFailure> {
    ClaimRef::named(OWNER, "comparison-behaviour").map_err(|_| MutationRoadFailure::Name)
}

fn operator() -> Result<OperatorFamilyRef, MutationRoadFailure> {
    OperatorFamilyRef::of_slug("comparison-boundaries").ok_or(MutationRoadFailure::MissingFamily)
}

fn policy(family: EvaluationFamilyRef) -> Result<MutationPolicy, MutationRoadFailure> {
    Ok(MutationPolicy::declared(
        family,
        vec![MutationPermission::declared(claim()?, vec![operator()?])?],
    )?)
}

fn point(
    policy: &MutationPolicy,
    stem: &'static str,
    alternatives: Vec<&'static [u8]>,
) -> Result<MutationPoint, MutationRoadFailure> {
    let admitted_family = operator()?;
    let declarations = alternatives
        .into_iter()
        .map(|operation| AlternativeDeclaration::stated(admitted_family, operation.to_vec()))
        .collect();
    Ok(MutationPoint::declared(
        policy,
        MutationPointRef::named(OWNER, stem).map_err(|_| MutationRoadFailure::Name)?,
        claim()?,
        b"a < b".to_vec(),
        declarations,
        threadpak_testpak::muterprater::ActivationSite::named(OWNER, stem)
            .map_err(|_| MutationRoadFailure::Name)?,
    )?)
}

fn surface_with(
    family: EvaluationFamilyRef,
    alternatives: Vec<&'static [u8]>,
) -> Result<EvaluationSurface, MutationRoadFailure> {
    let policy = policy(family)?;
    let point = point(&policy, "comparison-edge", alternatives)?;
    Ok(EvaluationSurface::conforming(&policy, vec![point])?)
}

fn production(input: &[u32; 3]) -> [u32; 3] {
    let [value, padding, sentinel] = *input;
    [value.saturating_add(1), padding, sentinel]
}

fn evaluation(input: &[u32; 3], control: EvaluationControl) -> EvaluationObservation<[u32; 3]> {
    let [value, padding, sentinel] = *input;
    match control {
        EvaluationControl::NoMutation => EvaluationObservation::observed(production(input), 0),
        EvaluationControl::Active(_) => {
            EvaluationObservation::observed([value.saturating_add(2), padding, sentinel], 1)
        }
    }
}

fn parity_broken(input: &[u32; 3], control: EvaluationControl) -> EvaluationObservation<[u32; 3]> {
    let [value, padding, sentinel] = *input;
    match control {
        EvaluationControl::NoMutation => {
            EvaluationObservation::observed([value.saturating_add(2), padding, sentinel], 0)
        }
        EvaluationControl::Active(_) => {
            EvaluationObservation::observed([value.saturating_add(2), padding, sentinel], 1)
        }
    }
}

fn activation_missing(
    input: &[u32; 3],
    control: EvaluationControl,
) -> EvaluationObservation<[u32; 3]> {
    let [value, padding, sentinel] = *input;
    match control {
        EvaluationControl::NoMutation => EvaluationObservation::observed(production(input), 0),
        EvaluationControl::Active(_) => {
            EvaluationObservation::observed([value.saturating_add(2), padding, sentinel], 0)
        }
    }
}

fn no_mutation_activates(
    input: &[u32; 3],
    control: EvaluationControl,
) -> EvaluationObservation<[u32; 3]> {
    match control {
        EvaluationControl::NoMutation | EvaluationControl::Active(_) => {
            EvaluationObservation::observed(production(input), 1)
        }
    }
}

fn activation_survives(
    input: &[u32; 3],
    control: EvaluationControl,
) -> EvaluationObservation<[u32; 3]> {
    match control {
        EvaluationControl::NoMutation => EvaluationObservation::observed(production(input), 0),
        EvaluationControl::Active(_) => EvaluationObservation::observed(production(input), 1),
    }
}

fn evaluation_counted(
    input: &[u32; 3],
    control: EvaluationControl,
) -> EvaluationObservation<[u32; 3]> {
    CLAIM_MISMATCH_EVALUATION_CALLS.fetch_add(1, Ordering::SeqCst);
    evaluation(input, control)
}

fn same(left: &[u32; 3], right: &[u32; 3]) -> Agreement {
    if left == right {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

fn check(meaning: &[u32; 3]) -> TrialConclusion {
    agreement(same, meaning, &[2u32, 0, 0], MEANING_DISAGREEMENT)
}

fn check_passes(_meaning: &[u32; 3]) -> TrialConclusion {
    TrialConclusion::Passed
}

fn check_evaluation_meaning(meaning: &[u32; 3]) -> TrialConclusion {
    agreement(same, meaning, &[3u32, 0, 0], MEANING_DISAGREEMENT)
}

fn check_refuses(meaning: &[u32; 3]) -> TrialConclusion {
    agreement(
        same,
        meaning,
        &[u32::MAX, u32::MAX, u32::MAX],
        MEANING_DISAGREEMENT,
    )
}

fn unused_trial_call(_invocation: &Invocation) -> TrialConclusion {
    TrialConclusion::Passed
}

fn trial_binding_for(claim_stem: &'static str) -> Result<TrialBinding, TrialTableRefusal> {
    let subject = SubjectRoute::named(OWNER, "comparison-subject")?;
    let check_ref = CheckRef::named(OWNER, "comparison-check")?;
    let row = Row::declared(
        ClaimRef::named(OWNER, claim_stem)?,
        ExecutionSuite::named(OWNER, "mutation-receiver")?,
        Classification::authored(
            vec![Role::named(OWNER, "mutation")?],
            vec![Tag::named(OWNER, "outside-consumer")?],
        )?,
        subject,
        check_ref,
        PopulationRef::named(OWNER, "one-input")?,
        Origin::HandWritten,
    )?;
    let revision = RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"trial"));
    Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check_ref, revision, revision, unused_trial_call),
        Provenance::Unproduced,
    )
    .map_err(TrialTableRefusal::from)
}

fn trial_binding() -> Result<TrialBinding, TrialTableRefusal> {
    trial_binding_for("comparison-behaviour")
}

fn check_ref() -> Result<CheckRef, MutationRoadFailure> {
    CheckRef::named(OWNER, "comparison-check").map_err(|_| MutationRoadFailure::Name)
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1),
            ByteBudget::declared(64),
            TimeBudget::declared(1_000_000_000),
        ),
        TargetBinding::bound(
            TargetTriple::declared("x86_64-pc-windows-msvc"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "mutation-receiver"),
        HarnessClock::unavailable(),
    )
}

fn pair(
    family: EvaluationFamilyRef,
    surface: &EvaluationSurface,
    evaluated: fn(&[u32; 3], EvaluationControl) -> EvaluationObservation<[u32; 3]>,
) -> Result<EvaluationPair<[u32; 3], [u32; 3]>, MutationRoadFailure> {
    pair_with_evaluation_revision(family, surface, evaluated, b"evaluation")
}

fn pair_with_evaluation_revision(
    family: EvaluationFamilyRef,
    surface: &EvaluationSurface,
    evaluated: fn(&[u32; 3], EvaluationControl) -> EvaluationObservation<[u32; 3]>,
    evaluation_revision_bytes: &[u8],
) -> Result<EvaluationPair<[u32; 3], [u32; 3]>, MutationRoadFailure> {
    let production_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"production"));
    let evaluation_revision = RevisionBinding::declared(ContentAddress::derived(
        REVISION_TAG,
        evaluation_revision_bytes,
    ));
    Ok(EvaluationPair::paired(
        ProductionBinding::declared(family, production_revision, production),
        EvaluationBinding::declared(surface, evaluation_revision, evaluated),
        same,
    )?)
}

fn no_owner(_coordinate: &SourceCoordinate) -> Option<ClaimRef> {
    None
}

fn no_family(_coordinate: &SourceCoordinate, _damage: &[u8]) -> Option<OperatorFamilyRef> {
    None
}

fn compiled_reading(family: EvaluationFamilyRef) -> Result<WrapReading, MutationRoadFailure> {
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    Ok(read_output(
        family,
        BACKEND_CONSOLE,
        BackendVersionPosture::Stated(version),
        no_owner,
        no_family,
    )?)
}

fn compiled_witness(
    pair: threadpak_testpak::muterprater::EvaluationPairStanding,
) -> Result<CompiledPressureWitness, MutationRoadFailure> {
    let reading = compiled_reading(pair.family())?;
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let qualification = AdapterQualification::of(&reading, GrammarStanding::Checked(version))?;
    Ok(CompiledPressureWitness::shown(
        pair,
        WrapStanding::Reported(&reading),
        &qualification,
    )?)
}

fn active_selection(surface: &EvaluationSurface) -> Result<ActiveSelection, MutationRoadFailure> {
    let point = surface
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let alternative = point
        .admitted_alternatives()
        .first()
        .map(threadpak_testpak::muterprater::AdmittedAlternative::identity)
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    surface
        .select(point.identity(), alternative)
        .map_err(|_| MutationRoadFailure::MissingActiveSelection)
}

/// Owner policy, point admission, and stable identities are independent of caller roster order.
#[test]
fn policy_admission_and_identity_are_structural() -> Result<(), MutationRoadFailure> {
    let evaluation_family = family("comparison-family")?;
    let first = surface_with(evaluation_family, vec![b"a <= b", b"a > b"])?;
    let reordered = surface_with(evaluation_family, vec![b"a > b", b"a <= b"])?;
    assert_eq!(first.identity(), reordered.identity());
    let first_point = first
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let reordered_point = reordered
        .points()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let first_ids = first_point
        .admitted_alternatives()
        .iter()
        .map(threadpak_testpak::muterprater::AdmittedAlternative::identity)
        .collect::<Vec<AlternativeId>>();
    let reordered_ids = reordered_point
        .admitted_alternatives()
        .iter()
        .map(threadpak_testpak::muterprater::AdmittedAlternative::identity)
        .collect::<Vec<AlternativeId>>();
    assert_eq!(first_ids, reordered_ids);

    let policy = policy(evaluation_family)?;
    let point_free = EvaluationSurface::conforming(&policy, Vec::new())?;
    assert_eq!(
        point_free.catalog_posture(),
        PointCatalogPosture::NoAdmittedPoints
    );
    assert!(point_free.selections().is_empty());
    assert_eq!(
        point(&policy, "empty-point", Vec::new()),
        Err(MutationRoadFailure::Point(
            PointRefusal::NoAdmittedAlternative
        ))
    );

    let boolean_family = OperatorFamilyRef::of_slug("boolean-operators")
        .ok_or(MutationRoadFailure::MissingFamily)?;
    let two_family_policy = MutationPolicy::declared(
        family("two-operator-family")?,
        vec![MutationPermission::declared(
            claim()?,
            vec![operator()?, boolean_family],
        )?],
    )?;
    let same_bytes_under_two_operators = MutationPoint::declared(
        &two_family_policy,
        MutationPointRef::named(OWNER, "operator-identity")
            .map_err(|_| MutationRoadFailure::Name)?,
        claim()?,
        b"a < b".to_vec(),
        vec![
            AlternativeDeclaration::stated(operator()?, b"a > b".to_vec()),
            AlternativeDeclaration::stated(boolean_family, b"a > b".to_vec()),
        ],
        threadpak_testpak::muterprater::ActivationSite::named(OWNER, "operator-identity")
            .map_err(|_| MutationRoadFailure::Name)?,
    )?;
    let first_alternative = same_bytes_under_two_operators
        .admitted_alternatives()
        .first()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    let last_alternative = same_bytes_under_two_operators
        .admitted_alternatives()
        .last()
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    assert_ne!(first_alternative.identity(), last_alternative.identity());
    Ok(())
}

/// Policy membership, operator permission, pair family, and surface-issued selection refuse every crossed join explicitly.
#[test]
fn mutation_constructor_and_selection_boundaries_refuse_crossed_joins()
-> Result<(), MutationRoadFailure> {
    let first_family = family("constructor-family")?;
    let first_policy = policy(first_family)?;
    let first_point = point(&first_policy, "first-point", vec![b"a <= b"])?;

    let boolean_family = OperatorFamilyRef::of_slug("boolean-operators")
        .ok_or(MutationRoadFailure::MissingFamily)?;
    let second_policy = MutationPolicy::declared(
        first_family,
        vec![MutationPermission::declared(
            claim()?,
            vec![operator()?, boolean_family],
        )?],
    )?;
    assert!(matches!(
        EvaluationSurface::conforming(&second_policy, vec![first_point.clone()]),
        Err(SurfaceRefusal::PointUnderAnotherPolicy {
            point,
            expected,
            found,
        }) if point == first_point.identity()
            && expected == second_policy.identity()
            && found == first_policy.identity()
    ));

    assert!(matches!(
        MutationPoint::declared(
            &first_policy,
            MutationPointRef::named(OWNER, "foreign-family-point")
                .map_err(|_| MutationRoadFailure::Name)?,
            claim()?,
            b"a < b".to_vec(),
            vec![AlternativeDeclaration::stated(boolean_family, b"true".to_vec())],
            threadpak_testpak::muterprater::ActivationSite::named(
                OWNER,
                "foreign-family-point",
            )
            .map_err(|_| MutationRoadFailure::Name)?,
        ),
        Err(PointRefusal::FamilyNotPermitted { at: 0, family }) if family == boolean_family
    ));

    let first_point_ref = first_point.identity();
    let first_surface = EvaluationSurface::conforming(&first_policy, vec![first_point])?;
    let second_point = point(&first_policy, "second-point", vec![b"a >= b"])?;
    let second_point_ref = second_point.identity();
    let second_alternative = second_point
        .admitted_alternatives()
        .first()
        .map(threadpak_testpak::muterprater::AdmittedAlternative::identity)
        .ok_or(MutationRoadFailure::MissingAlternative)?;
    assert_eq!(
        first_surface.select(second_point_ref, second_alternative),
        Err(threadpak_testpak::muterprater::SelectionRefusal::NoSuchPoint(second_point_ref,))
    );
    assert_eq!(
        first_surface.select(first_point_ref, second_alternative),
        Err(
            threadpak_testpak::muterprater::SelectionRefusal::NoSuchAlternative {
                point: first_point_ref,
                alternative: second_alternative,
            },
        )
    );

    let other_family = family("another-constructor-family")?;
    let other_surface = surface_with(other_family, vec![b"a <= b"])?;
    let production_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"production"));
    let evaluation_revision =
        RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, b"evaluation"));
    assert!(matches!(
        EvaluationPair::paired(
            ProductionBinding::declared(first_family, production_revision, production),
            EvaluationBinding::declared(&other_surface, evaluation_revision, evaluation),
            same,
        ),
        Err(EvaluationPairRefusal::FamilyMismatch {
            production,
            evaluation,
        }) if production == first_family && evaluation == other_family
    ));
    Ok(())
}

/// A point-free surface may earn parity trust but cannot claim an executable rewrite-audit road.
#[test]
fn point_free_trust_does_not_admit_mutation_execution() -> Result<(), MutationRoadFailure> {
    let family = family("point-free-family")?;
    let policy = policy(family)?;
    let surface = EvaluationSurface::conforming(&policy, Vec::new())?;
    let pair = pair(family, &surface, evaluation)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(&pair, witness, &input, &invocation())?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let compiled = compiled_witness(pair.standing())?;
    let availability = availability(Some(&surface), Some(&compiled), Some(qualification));
    assert!(matches!(
        &availability,
        InterpreterAvailability::Available(_)
    ));
    assert_eq!(
        threadpak_testpak::muterprater::rewrite::admission(&availability),
        RewriteAdmission::Withheld(RewriteWithheld::NoAdmittedPoint)
    );
    Ok(())
}

/// The three new content identities match independently framed owner facts rather than a digest copied from their encoder.
#[test]
fn mutation_identity_preimages_are_independently_read() -> Result<(), MutationRoadFailure> {
    let family = family("identity-family")?;
    let policy = policy(family)?;
    let point = point(&policy, "identity-point", vec![b"a <= b", b"a > b"])?;

    let mut policy_preimage = Vec::new();
    push_name(&mut policy_preimage, family.name());
    encode_length(policy.permissions().len(), &mut policy_preimage);
    for permission in policy.permissions() {
        push_name(&mut policy_preimage, permission.owner_claim().name());
        encode_length(permission.admitted_families().len(), &mut policy_preimage);
        for admitted in permission.admitted_families() {
            encode_bytes(admitted.slug().as_bytes(), &mut policy_preimage);
        }
    }
    assert_eq!(
        policy.identity().address(),
        ContentAddress::derived(POLICY_READING_TAG, &policy_preimage)
    );

    for alternative in point.admitted_alternatives() {
        let mut alternative_preimage = Vec::new();
        push_name(&mut alternative_preimage, point.identity().name());
        encode_bytes(
            alternative.family().slug().as_bytes(),
            &mut alternative_preimage,
        );
        encode_bytes(alternative.operation(), &mut alternative_preimage);
        assert_eq!(
            alternative.identity().address(),
            ContentAddress::derived(ALTERNATIVE_READING_TAG, &alternative_preimage)
        );
    }

    let surface = EvaluationSurface::conforming(&policy, vec![point])?;
    let mut surface_preimage = Vec::new();
    push_name(&mut surface_preimage, family.name());
    encode_bytes(
        policy.identity().address().as_bytes(),
        &mut surface_preimage,
    );
    encode_length(surface.points().len(), &mut surface_preimage);
    for surface_point in surface.points() {
        push_name(&mut surface_preimage, surface_point.identity().name());
        push_name(&mut surface_preimage, surface_point.owner_claim().name());
        encode_bytes(surface_point.original_operation(), &mut surface_preimage);
        push_name(
            &mut surface_preimage,
            surface_point.activation_site().name(),
        );
        encode_length(
            surface_point.admitted_alternatives().len(),
            &mut surface_preimage,
        );
        for alternative in surface_point.admitted_alternatives() {
            encode_bytes(
                alternative.identity().address().as_bytes(),
                &mut surface_preimage,
            );
            encode_bytes(
                alternative.family().slug().as_bytes(),
                &mut surface_preimage,
            );
            encode_bytes(alternative.operation(), &mut surface_preimage);
        }
    }
    assert_eq!(
        surface.identity().address(),
        ContentAddress::derived(SURFACE_READING_TAG, &surface_preimage)
    );
    Ok(())
}

/// A qualified supplied compiled reading and exact no-mutation parity open one active execution that is admitted through both report spines.
#[test]
fn compiled_and_interpreted_evidence_join_without_flattening() -> Result<(), MutationRoadFailure> {
    let family = family("comparison-family")?;
    let surface = surface_with(family, vec![b"a <= b", b"a > b"])?;
    let pair = pair(family, &surface, evaluation)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let reading = observe_no_mutation(&pair, witness, &input, &invocation())?;
    assert_eq!(*reading.production(), [2u32, 0, 0]);
    assert_eq!(*reading.evaluation(), [2u32, 0, 0]);
    assert_eq!(reading.evaluation_firings(), 0u32);
    assert_eq!(
        reading.production_report().trial(),
        reading.evaluation_report().trial()
    );
    assert_eq!(
        reading.production_report().measurement(),
        MeasurementReading::Unavailable
    );
    assert_eq!(
        reading.evaluation_report().measurement(),
        MeasurementReading::Unavailable
    );
    let standing = qualify_no_mutation(reading);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let compiled = compiled_witness(pair.standing())?;
    let trust = match availability(Some(&surface), Some(&compiled), Some(qualification)) {
        InterpreterAvailability::Available(trust) => trust,
        InterpreterAvailability::NoConformingSurface => {
            return Err(MutationRoadFailure::MissingTrust(
                MissingTrustEvidence::NoMutationParity,
            ));
        }
        InterpreterAvailability::TrustNotOpened { missing } => {
            return Err(MutationRoadFailure::MissingTrust(missing));
        }
    };
    let selection = active_selection(&surface)?;
    let evidence = execute_active(&trust, selection, &invocation())?;
    assert_eq!(evidence.selection(), selection);
    assert_eq!(*evidence.meaning(), [3u32, 0, 0]);
    assert_eq!(
        evidence.report().trial(),
        qualification.reading().production_report().trial()
    );
    assert_eq!(evidence.mutation().verdict(), MutationVerdict::Killed);
    assert!(matches!(
        evidence.mutation().activation().evidence(),
        Some(activation) if activation.witness() == evidence.report().trial()
    ));
    assert!(matches!(
        (evidence.report().attempt(), evidence.mutation().outcome()),
        (
            RunAttempt::Executed(TrialConclusion::Refused(report_finding)),
            MutationOutcome::Killed(IntendedRejection::Demonstrated(rejection)),
        ) if rejection.trial() == evidence.report().trial()
            && rejection.finding() == report_finding
    ));
    assert_eq!(
        evidence
            .mutation()
            .activation()
            .evidence()
            .map(threadpak_testpak::muterprater::ActivationEvidence::selection),
        Some(selection)
    );
    assert!(matches!(
        evidence.mutation().target().identity(),
        MutationIdentity::Interpreted { point: _, alternative } if alternative == selection.alternative()
    ));
    assert!(matches!(
        evidence.mutation().outcome(),
        MutationOutcome::Killed(_)
    ));
    Ok(())
}

/// The same admitted report authority also preserves a surviving active execution instead of hard-coding every firing as a kill.
#[test]
fn active_classification_is_derived_from_the_admitted_report() -> Result<(), MutationRoadFailure> {
    let family = family("surviving-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, activation_survives)?;
    let input = [1u32, 0, 0];
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let standing = qualify_no_mutation(observe_no_mutation(&pair, witness, &input, &invocation())?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let compiled = compiled_witness(pair.standing())?;
    let trust = match availability(Some(&surface), Some(&compiled), Some(qualification)) {
        InterpreterAvailability::Available(trust) => trust,
        InterpreterAvailability::NoConformingSurface => {
            return Err(MutationRoadFailure::MissingTrust(
                MissingTrustEvidence::NoMutationParity,
            ));
        }
        InterpreterAvailability::TrustNotOpened { missing } => {
            return Err(MutationRoadFailure::MissingTrust(missing));
        }
    };
    let evidence = execute_active(&trust, active_selection(&surface)?, &invocation())?;
    assert!(matches!(
        (evidence.report().attempt(), evidence.mutation().outcome()),
        (
            RunAttempt::Executed(TrialConclusion::Passed),
            MutationOutcome::Survived,
        )
    ));
    Ok(())
}

/// A passing trial conclusion cannot launder a no-mutation disagreement into parity qualification.
#[test]
fn no_mutation_agreement_must_be_earned() -> Result<(), MutationRoadFailure> {
    let family = family("parity-hostile-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, parity_broken)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check_passes)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(&pair, witness, &input, &invocation())?);
    let rejection = standing
        .rejection()
        .ok_or(MutationRoadFailure::MissingQualification(
            ParityQualificationRefusal::MeaningsDisagreed,
        ))?;
    assert_eq!(
        rejection.cause(),
        ParityQualificationRefusal::MeaningsDisagreed
    );
    assert!(matches!(
        rejection.reading().conclusion(),
        TrialConclusion::Refused(_)
    ));
    Ok(())
}

/// Production and evaluation reports retain their roles, and production refusal has declared priority when both roles refuse.
#[test]
fn no_mutation_report_roles_and_refusal_priority_are_observed() -> Result<(), MutationRoadFailure> {
    let family = family("report-role-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, parity_broken)?;
    let input = [1u32, 0, 0];

    let evaluation_rejected = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check)?,
        &input,
        &invocation(),
    )?);
    assert!(matches!(
        evaluation_rejected.rejection(),
        Some(rejection)
            if rejection.cause() == ParityQualificationRefusal::EvaluationDidNotQualify
    ));

    let production_rejected = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check_evaluation_meaning)?,
        &input,
        &invocation(),
    )?);
    assert!(matches!(
        production_rejected.rejection(),
        Some(rejection)
            if rejection.cause() == ParityQualificationRefusal::ProductionDidNotQualify
    ));

    let both_rejected = qualify_no_mutation(observe_no_mutation(
        &pair,
        MutationWitness::bound(trial_binding()?, check_ref()?, check_refuses)?,
        &input,
        &invocation(),
    )?);
    assert!(matches!(
        both_rejected.rejection(),
        Some(rejection)
            if rejection.cause() == ParityQualificationRefusal::ProductionDidNotQualify
    ));
    Ok(())
}

/// No-mutation semantic agreement cannot qualify when the evaluation copy reports any activation.
#[test]
fn no_mutation_requires_zero_firings() -> Result<(), MutationRoadFailure> {
    let family = family("no-mutation-firing-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, no_mutation_activates)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(&pair, witness, &input, &invocation())?);
    let rejection = standing
        .rejection()
        .ok_or(MutationRoadFailure::MissingQualification(
            ParityQualificationRefusal::NoMutationActivated { firings: 1 },
        ))?;
    assert_eq!(
        rejection.cause(),
        ParityQualificationRefusal::NoMutationActivated { firings: 1 }
    );
    assert_eq!(rejection.reading().evaluation_firings(), 1);
    Ok(())
}

/// A selected alternative that reports zero firings yields the exact dud and no admitted evidence.
#[test]
fn an_unfired_selection_is_not_mutation_evidence() -> Result<(), MutationRoadFailure> {
    let family = family("dud-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, activation_missing)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(&pair, witness, &input, &invocation())?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let compiled = compiled_witness(pair.standing())?;
    let trust = match availability(Some(&surface), Some(&compiled), Some(qualification)) {
        InterpreterAvailability::Available(trust) => trust,
        InterpreterAvailability::NoConformingSurface => {
            return Err(MutationRoadFailure::MissingTrust(
                MissingTrustEvidence::NoMutationParity,
            ));
        }
        InterpreterAvailability::TrustNotOpened { missing } => {
            return Err(MutationRoadFailure::MissingTrust(missing));
        }
    };
    let selection = active_selection(&surface)?;
    assert!(matches!(
        execute_active(&trust, selection, &invocation()),
        Err(InterpretedExecutionRefusal::DudPlant(dud)) if dud.selection() == selection
    ));
    Ok(())
}

/// An active selection cannot cross its issuing surface or owner claim, and either invalid join reaches no caller code.
#[test]
fn active_execution_keeps_surface_claim_and_witness_together() -> Result<(), MutationRoadFailure> {
    let family = family("claim-bound-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, evaluation_counted)?;
    let foreign_witness =
        MutationWitness::bound(trial_binding_for("another-behaviour")?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &pair,
        foreign_witness,
        &input,
        &invocation(),
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let compiled = compiled_witness(pair.standing())?;
    let trust = match availability(Some(&surface), Some(&compiled), Some(qualification)) {
        InterpreterAvailability::Available(trust) => trust,
        InterpreterAvailability::NoConformingSurface => {
            return Err(MutationRoadFailure::MissingTrust(
                MissingTrustEvidence::NoMutationParity,
            ));
        }
        InterpreterAvailability::TrustNotOpened { missing } => {
            return Err(MutationRoadFailure::MissingTrust(missing));
        }
    };
    let selection = active_selection(&surface)?;
    let expected_claim = claim()?;
    let foreign_claim =
        ClaimRef::named(OWNER, "another-behaviour").map_err(|_| MutationRoadFailure::Name)?;
    CLAIM_MISMATCH_EVALUATION_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        execute_active(&trust, selection, &invocation()),
        Err(InterpretedExecutionRefusal::WitnessForAnotherClaim { expected, found })
            if expected == expected_claim && found == foreign_claim
    ));
    assert_eq!(CLAIM_MISMATCH_EVALUATION_CALLS.load(Ordering::SeqCst), 0);

    let foreign_surface = surface_with(family, vec![b"a >= b"])?;
    let foreign_selection = active_selection(&foreign_surface)?;
    let expected_surface = surface.identity();
    let found_surface = foreign_surface.identity();
    CLAIM_MISMATCH_EVALUATION_CALLS.store(0, Ordering::SeqCst);
    assert!(matches!(
        execute_active(&trust, foreign_selection, &invocation()),
        Err(InterpretedExecutionRefusal::Selection(
            threadpak_testpak::muterprater::SelectionRefusal::SelectionFromAnotherSurface {
                expected,
                found,
            },
        )) if expected == expected_surface && found == found_surface
    ));
    assert_eq!(CLAIM_MISMATCH_EVALUATION_CALLS.load(Ordering::SeqCst), 0);
    Ok(())
}

/// A meaning-check callable cannot be placed under a different check identity than the exact trial row declares.
#[test]
fn a_mutation_witness_keeps_its_check_identity_and_callable_together()
-> Result<(), MutationRoadFailure> {
    let expected = check_ref()?;
    let found = CheckRef::named(OWNER, "another-check").map_err(|_| MutationRoadFailure::Name)?;
    assert!(matches!(
        MutationWitness::bound(trial_binding()?, found, check),
        Err(MutationWitnessRefusal::CheckMismatch {
            expected: refusal_expected,
            found: refusal_found,
        }) if refusal_expected == expected && refusal_found == found
    ));
    Ok(())
}

/// Compiled pressure from another family cannot open this evaluation surface.
#[test]
fn compiled_pressure_is_family_scoped() -> Result<(), MutationRoadFailure> {
    let local_family = family("local-family")?;
    let foreign = family("foreign-family")?;
    let surface = surface_with(local_family, vec![b"a <= b"])?;
    let evaluation_pair = pair(local_family, &surface, evaluation)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &evaluation_pair,
        witness,
        &input,
        &invocation(),
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let foreign_surface = surface_with(foreign, vec![b"a <= b"])?;
    let foreign_pair = pair(foreign, &foreign_surface, evaluation)?;
    let compiled = compiled_witness(foreign_pair.standing())?;
    assert!(matches!(
        availability(Some(&surface), Some(&compiled), Some(qualification)),
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledPressureForAnotherPair,
        }
    ));

    Ok(())
}

/// Compiled pressure scoped to another surface in the same family cannot open this exact evaluation pair.
#[test]
fn compiled_pressure_is_exact_pair_scoped() -> Result<(), MutationRoadFailure> {
    let family = family("same-family-pair-scope")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let evaluation_pair = pair(family, &surface, evaluation)?;
    let witness = MutationWitness::bound(trial_binding()?, check_ref()?, check)?;
    let input = [1u32, 0, 0];
    let standing = qualify_no_mutation(observe_no_mutation(
        &evaluation_pair,
        witness,
        &input,
        &invocation(),
    )?);
    let qualification =
        standing
            .qualification()
            .ok_or(MutationRoadFailure::MissingQualification(
                ParityQualificationRefusal::MeaningsDisagreed,
            ))?;
    let another_surface = surface_with(family, vec![b"a >= b"])?;
    let another_pair = pair(family, &another_surface, evaluation)?;
    assert_ne!(another_pair.standing(), evaluation_pair.standing());
    let surface_compiled = compiled_witness(another_pair.standing())?;
    assert!(matches!(
        availability(Some(&surface), Some(&surface_compiled), Some(qualification),),
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledPressureForAnotherPair,
        }
    ));

    let revision_pair = pair_with_evaluation_revision(
        family,
        &surface,
        evaluation,
        b"another-evaluation-revision",
    )?;
    assert_ne!(revision_pair.standing(), evaluation_pair.standing());
    let revision_compiled = compiled_witness(revision_pair.standing())?;
    assert!(matches!(
        availability(
            Some(&surface),
            Some(&revision_compiled),
            Some(qualification),
        ),
        InterpreterAvailability::TrustNotOpened {
            missing: MissingTrustEvidence::CompiledPressureForAnotherPair,
        }
    ));
    Ok(())
}

/// Adapter qualification remains bound to the exact backend profile whose reading earned it.
#[test]
fn a_compiled_witness_refuses_another_profile() -> Result<(), MutationRoadFailure> {
    let family = family("compiled-family")?;
    let surface = surface_with(family, vec![b"a <= b"])?;
    let pair = pair(family, &surface, evaluation)?;
    let here = compiled_reading(family)?;
    let other_version = BackendVersion::stated("24.0.0").map_err(|_| MutationRoadFailure::Name)?;
    let elsewhere = read_output(
        family,
        BACKEND_CONSOLE,
        BackendVersionPosture::Stated(other_version.clone()),
        no_owner,
        no_family,
    )?;
    let borrowed = AdapterQualification::of(&elsewhere, GrammarStanding::Checked(other_version))?;
    assert_eq!(
        CompiledPressureWitness::shown(pair.standing(), WrapStanding::Reported(&here), &borrowed,),
        Err(PressureWitnessRefusal::QualificationUnderAnotherProfile)
    );
    Ok(())
}

/// Adapter qualification preserves its complete refusal order over unchecked, unstated, and differently versioned profiles.
#[test]
fn adapter_qualification_requires_one_checked_profile_version() -> Result<(), MutationRoadFailure> {
    let family = family("qualification-family")?;
    let stated = compiled_reading(family)?;
    assert_eq!(
        AdapterQualification::of(&stated, GrammarStanding::Unchecked),
        Err(QualificationRefusal::GrammarUnchecked)
    );

    let checked = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let unstated = read_output(
        family,
        BACKEND_CONSOLE,
        BackendVersionPosture::Unstated,
        no_owner,
        no_family,
    )?;
    assert_eq!(
        AdapterQualification::of(&unstated, GrammarStanding::Checked(checked.clone())),
        Err(QualificationRefusal::BackendVersionUnstated)
    );

    let another = BackendVersion::stated("24.0.0").map_err(|_| MutationRoadFailure::Name)?;
    assert_eq!(
        AdapterQualification::of(&stated, GrammarStanding::Checked(another.clone())),
        Err(QualificationRefusal::CheckedAgainstAnotherVersion {
            stated: checked,
            checked: another,
        })
    );
    Ok(())
}

/// A compiled-pressure witness requires both a reported reading and a lawful kill from that reading.
#[test]
fn compiled_pressure_requires_a_reported_kill() -> Result<(), MutationRoadFailure> {
    let evaluation_family = family("compiled-pressure-staging")?;
    let surface = surface_with(evaluation_family, vec![b"a <= b"])?;
    let pair = pair(evaluation_family, &surface, evaluation)?;
    let version = BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let killed = compiled_reading(evaluation_family)?;
    let killed_qualification =
        AdapterQualification::of(&killed, GrammarStanding::Checked(version.clone()))?;
    assert_eq!(
        CompiledPressureWitness::shown(
            pair.standing(),
            WrapStanding::NotReported,
            &killed_qualification,
        ),
        Err(PressureWitnessRefusal::WrapNotReported)
    );

    let missed = read_output(
        evaluation_family,
        BACKEND_NO_KILL,
        BackendVersionPosture::Stated(version.clone()),
        no_owner,
        no_family,
    )?;
    let missed_qualification =
        AdapterQualification::of(&missed, GrammarStanding::Checked(version))?;
    assert_eq!(
        CompiledPressureWitness::shown(
            pair.standing(),
            WrapStanding::Reported(&missed),
            &missed_qualification,
        ),
        Err(PressureWitnessRefusal::NoKillDemonstrated)
    );

    let another_family = family("compiled-pressure-other-family")?;
    let another_reading = compiled_reading(another_family)?;
    let another_version =
        BackendVersion::stated(BACKEND_VERSION).map_err(|_| MutationRoadFailure::Name)?;
    let another_qualification =
        AdapterQualification::of(&another_reading, GrammarStanding::Checked(another_version))?;
    assert_eq!(
        CompiledPressureWitness::shown(
            pair.standing(),
            WrapStanding::Reported(&another_reading),
            &another_qualification,
        ),
        Err(PressureWitnessRefusal::ReadingForAnotherFamily {
            expected: evaluation_family,
            found: another_family,
        })
    );
    Ok(())
}
