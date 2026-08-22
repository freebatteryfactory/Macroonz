//! Neutral consumers for each generic property family retain both a lawful control and a typed damaged reading.

use arbitrary::{Arbitrary, Unstructured};
use core::cmp::Ordering;
use std::fmt;
use threadpak_testpak::descriptor::{NameRefusal, PopulationRef};
use threadpak_testpak::generate::{
    ByteSource, CaseWidth, CaseWidthRefusal, GenerationHalt, GenerationPlan, GenerationPlanRefusal,
    InputOrigin, PreconditionVerdict, RejectionBudget, SizeProgression, admit_every_sequence,
};
use threadpak_testpak::properties::{
    AMBIENT_PATHWAY_DISAGREEMENT, ANSWER_EXPECTED, Agreement, COMPOSED_RETURN_DISAGREEMENT,
    CONSERVATION_DISAGREEMENT, ContractRefusal, FUSED_VERSUS_SEPARATE_DISAGREEMENT, Holding,
    IDEMPOTENCE_DISAGREEMENT, MONOTONICITY_DISAGREEMENT, PERMUTATION_DISAGREEMENT, ParitySuite,
    REFUSAL_EXPECTED, ROUNDTRIP_DISAGREEMENT, SharedSubstrate, SubstrateRef, SubstrateRefusal,
    SubstrateRoster, TemporalClaim, TemporalDemand, TemporalDriveStanding, TransitionContract,
    ambient_pathway_invariance, composed_return, conservation, idempotence, monotonicity, parity,
    permutation_insensitivity, roundtrip,
};
use threadpak_testpak::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, GenerationProfile, TrialConclusion,
};

const TEMPORAL_CAUSE: FindingCause = FindingCause::named("testpak", "bounded-state");

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

fn add_one(value: &Scalar) -> Scalar {
    Scalar(value.0.saturating_add(1u8))
}

fn subtract_one(value: &Scalar) -> Scalar {
    Scalar(value.0.saturating_sub(1u8))
}

fn identity(value: &Scalar) -> Scalar {
    value.clone()
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
}

#[test]
fn composition_distinguishes_returning_and_damaged_wiring() {
    let lawful =
        threadpak_testpak::properties::ComposedRoads::wired(add_one, subtract_one, same_scalar);
    let damaged =
        threadpak_testpak::properties::ComposedRoads::wired(add_one, identity, same_scalar);
    assert_eq!(
        composed_return(&lawful, &Scalar(7u8)),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&composed_return(&damaged, &Scalar(7u8))),
        Some(property_refusal(COMPOSED_RETURN_DISAGREEMENT))
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
        PopulationRef::named("testpak", stem)?,
        GenerationProfile::declared(stem, 1u32),
        InputOrigin::Supplied(supplied.to_vec()),
        CaseBudget::declared(cases),
        ByteBudget::declared(bytes),
        RejectionBudget::declared(2u32),
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
    let complete = threadpak_testpak::properties::holds_over_drive(
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
    let partial = threadpak_testpak::properties::holds_over_drive(
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
    let counterexample = threadpak_testpak::properties::holds_over_drive(
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
    let empty = threadpak_testpak::properties::holds_over_drive(
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
        Some(check_refusal(
            threadpak_testpak::properties::NO_SEQUENCE_DRIVEN
        ))
    );
    Ok(())
}

#[test]
fn parity_reading_retains_suite_input_results_and_exact_refusal() -> Result<(), PropertyRoadFailure>
{
    let substrate = SubstrateRef::named("testpak", "neutral-arithmetic")?;
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
    Ok(())
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
        threadpak_testpak::ensure_ok!(answered, ANSWER_EXPECTED),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&threadpak_testpak::ensure_ok!(refused, ANSWER_EXPECTED)),
        Some(check_refusal(ANSWER_EXPECTED))
    );
    assert_eq!(
        threadpak_testpak::ensure_refused!(refused, REFUSAL_EXPECTED),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&threadpak_testpak::ensure_refused!(
            answered,
            REFUSAL_EXPECTED
        )),
        Some(check_refusal(REFUSAL_EXPECTED))
    );
    assert_eq!(
        threadpak_testpak::ensure_refused_with!(refused, TinyRefusal::Expected, REFUSAL_EXPECTED),
        TrialConclusion::Passed
    );
    assert_eq!(
        refusal_signature(&threadpak_testpak::ensure_refused_with!(
            other,
            TinyRefusal::Expected,
            REFUSAL_EXPECTED
        )),
        Some(check_refusal(REFUSAL_EXPECTED))
    );
}
