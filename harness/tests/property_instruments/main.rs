//! Neutral consumers for each generic property family retain both a lawful control and a typed damaged reading.

use arbitrary::{Arbitrary, Unstructured};
use core::cmp::Ordering;
use macroonz_harness::descriptor::{NameRefusal, PopulationRef};
use macroonz_harness::generate::{
    ByteSource, CaseWidth, CaseWidthRefusal, GenerationHalt, GenerationPlan, GenerationPlanRefusal,
    InputOrigin, PreconditionVerdict, RejectionAllowance, SizeProgression, admit_every_sequence,
};
use macroonz_harness::properties::{
    AMBIENT_PATHWAY_DISAGREEMENT, ANSWER_EXPECTED, Agreement, COMPOSED_CONSERVATION_DISAGREEMENT,
    COMPOSED_DETERMINISM_DISAGREEMENT, COMPOSED_IDEMPOTENCE_DISAGREEMENT,
    COMPOSED_RETURN_DISAGREEMENT, CONSERVATION_DISAGREEMENT, ComposedRoads, ContractRefusal,
    DETERMINISM_DISAGREEMENT, FAIL_CLOSED_ANSWERED, FUSED_VERSUS_SEPARATE_DISAGREEMENT, Holding,
    IDEMPOTENCE_DISAGREEMENT, LAWFUL_TWIN_REFUSED, MONOTONICITY_DISAGREEMENT, NO_SEQUENCE_DRIVEN,
    PERMUTATION_DISAGREEMENT, ParitySuite, PoisonResponse, REFUSAL_EXPECTED,
    ROUNDTRIP_DISAGREEMENT, RoadPairing, SharedSubstrate, SubstrateRef, SubstrateRefusal,
    SubstrateRoster, TemporalClaim, TemporalDemand, TemporalDriveStanding, TransitionContract,
    admits_lawful, ambient_pathway_invariance, composed, composed_conservation,
    composed_determinism, composed_idempotence, composed_return, conservation,
    determinism_run_twice, fail_closed, holds_over_drive, holds_over_history, idempotence,
    monotonicity, parity, permutation_insensitivity, roundtrip,
};
use macroonz_harness::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, GenerationProfile, TrialConclusion,
};
use std::fmt;
use std::sync::atomic::{AtomicU8, Ordering as AtomicOrdering};

const TEMPORAL_CAUSE: FindingCause = FindingCause::named("harness", "bounded-state");
const NEVER_CAUSE: FindingCause = FindingCause::named("harness", "never-above-three");
const EVENTUALLY_CAUSE: FindingCause = FindingCause::named("harness", "eventually-positive");
const LATCH_CAUSE: FindingCause = FindingCause::named("harness", "positive-latch");
const ORDER_CAUSE: FindingCause = FindingCause::named("harness", "state-order");

static METAMORPHIC_CALL: AtomicU8 = AtomicU8::new(0u8);
static COMPOSITION_CALL: AtomicU8 = AtomicU8::new(0u8);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Scalar(u8);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Word(u16);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Encoded([u8; 2]);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair([u8; 2]);

#[derive(Debug, Clone, PartialEq, Eq)]
struct State(u8);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Command(u8);

enum PropertyRoadFailure {
    Name(NameRefusal),
    Width(CaseWidthRefusal),
    Plan(GenerationPlanRefusal),
    Contract(ContractRefusal),
    Substrate(SubstrateRefusal),
}

impl fmt::Debug for PropertyRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Width(refusal) => formatter.debug_tuple("Width").field(refusal).finish(),
            Self::Plan(refusal) => formatter.debug_tuple("Plan").field(refusal).finish(),
            Self::Contract(refusal) => formatter.debug_tuple("Contract").field(refusal).finish(),
            Self::Substrate(refusal) => formatter.debug_tuple("Substrate").field(refusal).finish(),
        }
    }
}

impl From<NameRefusal> for PropertyRoadFailure {
    fn from(refusal: NameRefusal) -> Self {
        Self::Name(refusal)
    }
}

impl From<CaseWidthRefusal> for PropertyRoadFailure {
    fn from(refusal: CaseWidthRefusal) -> Self {
        Self::Width(refusal)
    }
}

impl From<GenerationPlanRefusal> for PropertyRoadFailure {
    fn from(refusal: GenerationPlanRefusal) -> Self {
        Self::Plan(refusal)
    }
}

impl From<ContractRefusal> for PropertyRoadFailure {
    fn from(refusal: ContractRefusal) -> Self {
        Self::Contract(refusal)
    }
}

impl From<SubstrateRefusal> for PropertyRoadFailure {
    fn from(refusal: SubstrateRefusal) -> Self {
        Self::Substrate(refusal)
    }
}

fn same_scalar(left: &Scalar, right: &Scalar) -> Agreement {
    if left == right {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

fn same_word(left: &Word, right: &Word) -> Agreement {
    if left == right {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

const SAME_U8: fn(&u8, &u8) -> Agreement = |left, right| {
    if left == right {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
};

fn natural_scalar(left: &Scalar, right: &Scalar) -> Ordering {
    left.0.cmp(&right.0)
}

fn encoded(value: &Word) -> Encoded {
    Encoded(value.0.to_le_bytes())
}

fn decoded(bytes: &Encoded) -> Word {
    Word(u16::from_le_bytes(bytes.0))
}

fn damaged_decode(bytes: &Encoded) -> Word {
    Word(u16::from_le_bytes(bytes.0).saturating_add(1u16))
}

fn normalize_even(value: &Scalar) -> Scalar {
    Scalar(value.0 & !1u8)
}

fn keep_changing(value: &Scalar) -> Scalar {
    Scalar(value.0.saturating_add(1u8))
}

fn swap_pair(value: &Pair) -> Pair {
    let [left, right] = value.0;
    Pair([right, left])
}

fn erase_second(value: &Pair) -> Pair {
    let [left, _right] = value.0;
    Pair([left, 0u8])
}

fn pair_total(value: &Pair) -> Word {
    let [left, right] = value.0;
    Word(u16::from(left).saturating_add(u16::from(right)))
}

fn double(value: &Scalar) -> Scalar {
    Scalar(value.0.saturating_mul(2u8))
}

fn descending(value: &Scalar) -> Scalar {
    Scalar(u8::MAX.saturating_sub(value.0))
}

fn first(value: &Pair) -> Scalar {
    let [first, _second] = value.0;
    Scalar(first)
}

fn triple(value: &Scalar) -> Scalar {
    Scalar(value.0.saturating_mul(3u8))
}

fn changing(_value: &Scalar) -> Scalar {
    Scalar(METAMORPHIC_CALL.fetch_add(1u8, AtomicOrdering::SeqCst))
}

fn changing_composition(_value: &Scalar) -> Scalar {
    Scalar(COMPOSITION_CALL.fetch_add(1u8, AtomicOrdering::SeqCst))
}

fn add_one(value: &Scalar) -> Scalar {
    Scalar(value.0.saturating_add(1u8))
}

fn subtract_one(value: &Scalar) -> Scalar {
    Scalar(value.0.saturating_sub(1u8))
}

fn identity(value: &Scalar) -> Scalar {
    value.clone()
}

fn scalar_value(value: &Scalar) -> u8 {
    value.0
}

fn refusal_signature(conclusion: &TrialConclusion) -> Option<(FailureClass, FindingCause)> {
    match conclusion {
        TrialConclusion::Passed => None,
        TrialConclusion::Refused(finding) => Some((finding.class(), finding.cause())),
    }
}

const fn property_refusal(cause: FindingCause) -> (FailureClass, FindingCause) {
    (FailureClass::PropertyDisagreement, cause)
}

const fn check_refusal(cause: FindingCause) -> (FailureClass, FindingCause) {
    (FailureClass::RefusedByCheck, cause)
}

fn standing_refusal(standing: &TemporalDriveStanding) -> Option<(FailureClass, FindingCause)> {
    match standing {
        TemporalDriveStanding::Concluded(conclusion) => refusal_signature(conclusion),
        TemporalDriveStanding::Incomplete => None,
    }
}

#[test]
fn algebraic_families_distinguish_lawful_and_damaged_subjects() {
    let word = Word(513u16);
    assert_eq!(
        roundtrip(encoded, decoded, same_word, &word),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&roundtrip(encoded, damaged_decode, same_word, &word)),
        Some(property_refusal(ROUNDTRIP_DISAGREEMENT))
    );

    let value = Scalar(7u8);
    assert_eq!(
        idempotence(normalize_even, same_scalar, &value),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&idempotence(keep_changing, same_scalar, &value)),
        Some(property_refusal(IDEMPOTENCE_DISAGREEMENT))
    );

    let pair = Pair([2u8, 3u8]);
    assert_eq!(
        conservation(swap_pair, pair_total, pair_total, same_word, &pair),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&conservation(
            erase_second,
            pair_total,
            pair_total,
            same_word,
            &pair,
        )),
        Some(property_refusal(CONSERVATION_DISAGREEMENT))
    );

    assert_eq!(
        monotonicity(
            double,
            natural_scalar,
            natural_scalar,
            &Scalar(2u8),
            &Scalar(3u8),
        ),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&monotonicity(
            descending,
            natural_scalar,
            natural_scalar,
            &Scalar(2u8),
            &Scalar(3u8),
        )),
        Some(property_refusal(MONOTONICITY_DISAGREEMENT))
    );
}

#[test]
fn metamorphic_families_distinguish_lawful_and_damaged_paths() {
    let pair = Pair([2u8, 5u8]);
    assert_eq!(
        permutation_insensitivity(pair_total, swap_pair, same_word, &pair),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&permutation_insensitivity(
            first,
            swap_pair,
            same_scalar,
            &pair,
        )),
        Some(property_refusal(PERMUTATION_DISAGREEMENT))
    );

    let value = Scalar(6u8);
    assert_eq!(
        ambient_pathway_invariance(double, double, same_scalar, &value),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&ambient_pathway_invariance(
            double,
            triple,
            same_scalar,
            &value
        )),
        Some(property_refusal(AMBIENT_PATHWAY_DISAGREEMENT))
    );

    assert_eq!(
        determinism_run_twice(double, same_scalar, &value),
        TrialConclusion::Passed
    );
    METAMORPHIC_CALL.store(0u8, AtomicOrdering::SeqCst);
    assert_eq!(
        refusal_signature(&determinism_run_twice(changing, same_scalar, &value)),
        Some(property_refusal(DETERMINISM_DISAGREEMENT))
    );
}

#[test]
fn composition_distinguishes_returning_and_damaged_wiring() {
    let lawful = ComposedRoads::wired(add_one, subtract_one, same_scalar);
    let damaged = ComposedRoads::wired(add_one, identity, same_scalar);
    let value = Scalar(7u8);
    assert_eq!(composed(&lawful, &value), value);
    assert_eq!(composed_return(&lawful, &value), TrialConclusion::Passed);
    assert_eq!(
        refusal_signature(&composed_return(&damaged, &value)),
        Some(property_refusal(COMPOSED_RETURN_DISAGREEMENT))
    );
    assert_eq!(
        composed_idempotence(&lawful, &value),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&composed_idempotence(&damaged, &value)),
        Some(property_refusal(COMPOSED_IDEMPOTENCE_DISAGREEMENT))
    );
    assert_eq!(
        composed_conservation(&lawful, scalar_value, scalar_value, SAME_U8, &value,),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&composed_conservation(
            &damaged,
            scalar_value,
            scalar_value,
            SAME_U8,
            &value,
        )),
        Some(property_refusal(COMPOSED_CONSERVATION_DISAGREEMENT))
    );

    COMPOSITION_CALL.store(0u8, AtomicOrdering::SeqCst);
    let nondeterministic = ComposedRoads::wired(changing_composition, identity, same_scalar);
    assert_eq!(
        refusal_signature(&composed_determinism(&nondeterministic, &value)),
        Some(property_refusal(COMPOSED_DETERMINISM_DISAGREEMENT))
    );
}

fn opening_state() -> State {
    State(0u8)
}

fn apply_command(state: &State, command: &Command) -> State {
    State(state.0.saturating_add(command.0))
}

fn at_most_three(state: &State) -> Holding {
    if state.0 <= 3u8 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

fn remains_zero(state: &State) -> Holding {
    if state.0 == 0u8 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

fn above_three(state: &State) -> Holding {
    if state.0 > 3u8 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

fn positive(state: &State) -> Holding {
    if state.0 > 0u8 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

fn replace_command(_state: &State, command: &Command) -> State {
    State(command.0)
}

fn state_order(left: &State, right: &State) -> Ordering {
    left.0.cmp(&right.0)
}

fn reject_every_sequence(_commands: &[Command]) -> PreconditionVerdict {
    PreconditionVerdict::Rejected
}

fn decode_command(source: &mut Unstructured<'_>) -> arbitrary::Result<Command> {
    Ok(Command(u8::arbitrary(source)?))
}

fn generation_plan(
    stem: &'static str,
    supplied: &[u8],
    cases: u32,
    bytes: u64,
) -> Result<GenerationPlan, PropertyRoadFailure> {
    Ok(GenerationPlan::declared(
        PopulationRef::named("harness", stem)?,
        GenerationProfile::declared(stem, 1u32),
        InputOrigin::Supplied(supplied.to_vec()),
        CaseBudget::declared(cases),
        ByteBudget::declared(bytes),
        RejectionAllowance::declared(2u32),
        SizeProgression::Constant {
            width: CaseWidth::declared(1usize)?,
        },
    )?)
}

fn temporal_contract(
    predicate: fn(&State) -> Holding,
) -> Result<TransitionContract<State, Command>, PropertyRoadFailure> {
    Ok(TransitionContract::declared(
        opening_state,
        apply_command,
        vec![TemporalClaim::declared(
            TEMPORAL_CAUSE,
            TemporalDemand::Always(predicate),
        )],
    )?)
}

#[test]
fn temporal_reading_distinguishes_complete_partial_counterexample_and_empty()
-> Result<(), PropertyRoadFailure> {
    let complete_plan = generation_plan("complete-temporal", &[1u8, 2u8], 2u32, 2u64)?;
    let complete_source = ByteSource::of_plan(&complete_plan);
    let complete_contract = temporal_contract(at_most_three)?;
    let complete = holds_over_drive(
        &complete_contract,
        &complete_plan,
        &complete_source,
        decode_command,
        admit_every_sequence::<Command>,
    );
    assert_eq!(complete.generated().halt(), GenerationHalt::CaseBudgetMet);
    assert_eq!(complete.evaluated(), complete.generated().sequences().len());
    assert_eq!(
        complete.standing(),
        &TemporalDriveStanding::Concluded(TrialConclusion::Passed)
    );

    let partial_plan = generation_plan("partial-temporal", &[1u8], 2u32, 2u64)?;
    let partial_source = ByteSource::of_plan(&partial_plan);
    let partial = holds_over_drive(
        &complete_contract,
        &partial_plan,
        &partial_source,
        decode_command,
        admit_every_sequence::<Command>,
    );
    assert_eq!(partial.generated().halt(), GenerationHalt::SourceExhausted);
    assert_eq!(partial.evaluated(), 1usize);
    assert_eq!(partial.standing(), &TemporalDriveStanding::Incomplete);

    let counterexample_contract = temporal_contract(remains_zero)?;
    let counterexample = holds_over_drive(
        &counterexample_contract,
        &partial_plan,
        &partial_source,
        decode_command,
        admit_every_sequence::<Command>,
    );
    assert_eq!(
        counterexample.generated().halt(),
        GenerationHalt::SourceExhausted
    );
    assert_eq!(counterexample.evaluated(), 1usize);
    assert_eq!(
        standing_refusal(counterexample.standing()),
        Some(property_refusal(TEMPORAL_CAUSE))
    );

    let empty_plan = generation_plan("empty-temporal", &[1u8], 1u32, 1u64)?;
    let empty_source = ByteSource::of_plan(&empty_plan);
    let empty = holds_over_drive(
        &complete_contract,
        &empty_plan,
        &empty_source,
        decode_command,
        reject_every_sequence,
    );
    assert_eq!(empty.generated().sequences().len(), 0usize);
    assert_eq!(empty.evaluated(), 0usize);
    assert_eq!(
        standing_refusal(empty.standing()),
        Some(check_refusal(NO_SEQUENCE_DRIVEN))
    );
    Ok(())
}

#[test]
fn temporal_history_reads_never_eventually_latch_and_order() -> Result<(), PropertyRoadFailure> {
    let never = TransitionContract::declared(
        opening_state,
        apply_command,
        vec![TemporalClaim::declared(
            NEVER_CAUSE,
            TemporalDemand::Never(above_three),
        )],
    )?;
    assert_eq!(
        holds_over_history(&never, &[Command(1u8)]),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&holds_over_history(&never, &[Command(4u8)])),
        Some(property_refusal(NEVER_CAUSE))
    );

    let eventually = TransitionContract::declared(
        opening_state,
        apply_command,
        vec![TemporalClaim::declared(
            EVENTUALLY_CAUSE,
            TemporalDemand::Eventually(positive),
        )],
    )?;
    assert_eq!(
        holds_over_history(&eventually, &[Command(1u8)]),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&holds_over_history(&eventually, &[])),
        Some(property_refusal(EVENTUALLY_CAUSE))
    );

    let latch = TransitionContract::declared(
        opening_state,
        replace_command,
        vec![TemporalClaim::declared(
            LATCH_CAUSE,
            TemporalDemand::OnceHoldingAlwaysHolding(positive),
        )],
    )?;
    assert_eq!(
        holds_over_history(&latch, &[Command(1u8), Command(2u8)]),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&holds_over_history(&latch, &[Command(1u8), Command(0u8)])),
        Some(property_refusal(LATCH_CAUSE))
    );

    let ordered = TransitionContract::declared(
        opening_state,
        replace_command,
        vec![TemporalClaim::declared(
            ORDER_CAUSE,
            TemporalDemand::NeverDecreases(state_order),
        )],
    )?;
    assert_eq!(
        holds_over_history(&ordered, &[Command(1u8), Command(2u8)]),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&holds_over_history(&ordered, &[Command(2u8), Command(1u8)])),
        Some(property_refusal(ORDER_CAUSE))
    );
    assert!(matches!(
        TransitionContract::<State, Command>::declared(opening_state, apply_command, Vec::new()),
        Err(ContractRefusal::NoClaimDeclared)
    ));
    Ok(())
}

#[test]
fn parity_reading_retains_suite_input_results_and_exact_refusal() -> Result<(), PropertyRoadFailure>
{
    let substrate = SubstrateRef::named("harness", "neutral-arithmetic")?;
    assert_eq!(
        SubstrateRoster::declared(&[]),
        Err(SubstrateRefusal::EmptyRoster)
    );
    assert_eq!(
        SubstrateRoster::declared(&[substrate, substrate]),
        Err(SubstrateRefusal::DuplicateSubstrate(substrate))
    );
    let standing = SharedSubstrate::Standing(SubstrateRoster::declared(&[substrate])?);
    let lawful = ParitySuite::fused_versus_separate(double, double, same_scalar, standing.clone());
    let input = Scalar(5u8);
    let lawful_reading = parity(&lawful, &input);
    assert!(core::ptr::eq(lawful_reading.suite(), &raw const lawful));
    assert!(core::ptr::eq(lawful_reading.input(), &raw const input));
    assert_eq!(lawful_reading.input(), &input);
    assert_eq!(lawful_reading.left(), &Scalar(10u8));
    assert_eq!(lawful_reading.right(), &Scalar(10u8));
    assert_eq!(lawful_reading.conclusion(), &TrialConclusion::Passed);
    assert_eq!(lawful_reading.suite().substrate(), &standing);

    let damaged = ParitySuite::fused_versus_separate(double, triple, same_scalar, standing);
    let damaged_reading = parity(&damaged, &input);
    assert_eq!(damaged_reading.left(), &Scalar(10u8));
    assert_eq!(damaged_reading.right(), &Scalar(15u8));
    assert_eq!(
        refusal_signature(damaged_reading.conclusion()),
        Some(property_refusal(FUSED_VERSUS_SEPARATE_DISAGREEMENT))
    );

    let pairing =
        macroonz_harness::descriptor::NamespacedName::named("harness", "declared-parity-pair")?;
    let declared = ParitySuite::over(
        RoadPairing::Declared(pairing),
        double,
        triple,
        same_scalar,
        SharedSubstrate::DeclaredIndependent,
    );
    assert_eq!(
        refusal_signature(parity(&declared, &input).conclusion()),
        Some(property_refusal(FindingCause::named(
            "harness",
            "declared-parity-pair",
        )))
    );
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Response {
    Refused,
    Answered,
}

fn refusal_response(_value: &Scalar) -> Response {
    Response::Refused
}

fn answer_response(_value: &Scalar) -> Response {
    Response::Answered
}

const RESPONSE_READING: fn(&Response) -> PoisonResponse = |response| match response {
    Response::Refused => PoisonResponse::Refused,
    Response::Answered => PoisonResponse::Answered,
};

#[test]
fn hostile_refusal_and_lawful_twin_must_both_hold() {
    let value = Scalar(7u8);
    assert_eq!(
        fail_closed(refusal_response, RESPONSE_READING, &value),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&fail_closed(answer_response, RESPONSE_READING, &value)),
        Some(check_refusal(FAIL_CLOSED_ANSWERED))
    );
    assert_eq!(
        admits_lawful(answer_response, RESPONSE_READING, &value),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&admits_lawful(refusal_response, RESPONSE_READING, &value)),
        Some(check_refusal(LAWFUL_TWIN_REFUSED))
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TinyRefusal {
    Expected,
    Other,
}

#[test]
fn typed_refusal_stamps_preserve_answer_refusal_and_exact_pattern() {
    let answered: Result<u8, TinyRefusal> = Ok(7u8);
    let refused: Result<u8, TinyRefusal> = Err(TinyRefusal::Expected);
    let other: Result<u8, TinyRefusal> = Err(TinyRefusal::Other);

    assert_eq!(
        macroonz_harness::ensure_ok!(answered, ANSWER_EXPECTED),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&macroonz_harness::ensure_ok!(refused, ANSWER_EXPECTED)),
        Some(check_refusal(ANSWER_EXPECTED))
    );
    assert_eq!(
        macroonz_harness::ensure_refused!(refused, REFUSAL_EXPECTED),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&macroonz_harness::ensure_refused!(
            answered,
            REFUSAL_EXPECTED
        )),
        Some(check_refusal(REFUSAL_EXPECTED))
    );
    assert_eq!(
        macroonz_harness::ensure_refused_with!(refused, TinyRefusal::Expected, REFUSAL_EXPECTED),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&macroonz_harness::ensure_refused_with!(
            other,
            TinyRefusal::Expected,
            REFUSAL_EXPECTED
        )),
        Some(check_refusal(REFUSAL_EXPECTED))
    );
}
