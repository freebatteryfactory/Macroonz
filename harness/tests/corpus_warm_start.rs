//! An outside consumer writes and reads a content-addressed seed pack, warms the public generation road, and records the property conclusion through the ordinary runner.

use threadpak_testpak::clock::HarnessClock;
use threadpak_testpak::corpus::{
    SEED_PACK_FORMAT_VERSION, SEED_PACK_TAG, SeedInput, SeedInputRefusal, SeedPackRefusal, pack,
    read, warm_start,
};
use threadpak_testpak::descriptor::{
    AuthoredTableName, Binding, CheckRef, ClaimRef, Classification, ExecutableAttachment,
    ExecutionSuite, NameRefusal, Origin, PopulationRef, Provenance, RevisionBinding, Role, Row,
    SubjectRoute, Tag, TrialTableRefusal,
};
use threadpak_testpak::generate::{
    ByteSource, CaseWidth, CaseWidthRefusal, GenerationPlan, GenerationPlanRefusal, InputOrigin,
    RejectionAllowance, SizeProgression, admit_every_sequence, decode_arbitrary,
};
use threadpak_testpak::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use threadpak_testpak::properties::{
    ContractRefusal, Holding, TemporalClaim, TemporalDemand, TemporalDriveStanding,
    TransitionContract, holds_over_drive,
};
use threadpak_testpak::report::{
    ByteBudget, CaseBudget, FailureClass, FindingCause, Fingerprint, GenerationProfile,
    InvocationProfile, RunAttempt, SelectionOutcome, TargetBinding, TargetTriple, TimeBudget,
    ToolchainIdentity, TrialConclusion, TrialSite, encode_bytes, encode_length,
};
use threadpak_testpak::runner::{Invocation, Selection, SelectionPlan, TrialTable, run_all};

const CONSUMER: &str = "testpak.corpus.consumer";
const PROPERTY_CAUSE: FindingCause = FindingCause::named(CONSUMER, "value-at-most-two");
const FIXTURE_CAUSE: FindingCause = FindingCause::named(CONSUMER, "fixture-refused");
const REVISION_TAG: DomainTag = DomainTag::declared(
    "corpus-consumer-revision",
    IdentityProfileVersion::declared(1),
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ValueState(u8);

enum CorpusRoadFailure {
    Name(NameRefusal),
    Seed(SeedInputRefusal),
    Pack(SeedPackRefusal),
    Width(CaseWidthRefusal),
    Plan(GenerationPlanRefusal),
    Contract(ContractRefusal),
    Table(TrialTableRefusal),
    MissingEvidence,
    EvidenceMismatch,
}

impl core::fmt::Debug for CorpusRoadFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Seed(refusal) => formatter.debug_tuple("Seed").field(refusal).finish(),
            Self::Pack(refusal) => formatter.debug_tuple("Pack").field(refusal).finish(),
            Self::Width(refusal) => formatter.debug_tuple("Width").field(refusal).finish(),
            Self::Plan(refusal) => formatter.debug_tuple("Plan").field(refusal).finish(),
            Self::Contract(refusal) => formatter.debug_tuple("Contract").field(refusal).finish(),
            Self::Table(refusal) => formatter.debug_tuple("Table").field(refusal).finish(),
            Self::MissingEvidence => formatter.write_str("MissingEvidence"),
            Self::EvidenceMismatch => formatter.write_str("EvidenceMismatch"),
        }
    }
}

impl From<NameRefusal> for CorpusRoadFailure {
    fn from(refusal: NameRefusal) -> Self {
        Self::Name(refusal)
    }
}

impl From<SeedInputRefusal> for CorpusRoadFailure {
    fn from(refusal: SeedInputRefusal) -> Self {
        Self::Seed(refusal)
    }
}

impl From<SeedPackRefusal> for CorpusRoadFailure {
    fn from(refusal: SeedPackRefusal) -> Self {
        Self::Pack(refusal)
    }
}

impl From<CaseWidthRefusal> for CorpusRoadFailure {
    fn from(refusal: CaseWidthRefusal) -> Self {
        Self::Width(refusal)
    }
}

impl From<GenerationPlanRefusal> for CorpusRoadFailure {
    fn from(refusal: GenerationPlanRefusal) -> Self {
        Self::Plan(refusal)
    }
}

impl From<ContractRefusal> for CorpusRoadFailure {
    fn from(refusal: ContractRefusal) -> Self {
        Self::Contract(refusal)
    }
}

impl From<TrialTableRefusal> for CorpusRoadFailure {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::Table(refusal)
    }
}

fn population(stem: &'static str) -> Result<PopulationRef, NameRefusal> {
    PopulationRef::named(CONSUMER, stem)
}

fn seeds() -> Result<Vec<SeedInput>, SeedInputRefusal> {
    Ok(vec![
        SeedInput::declared(vec![1u8])?,
        SeedInput::declared(vec![4u8])?,
    ])
}

fn opening_state() -> ValueState {
    ValueState(0u8)
}

#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "TransitionContract's callable shape borrows both state and input"
)]
fn apply_value(_state: &ValueState, value: &u8) -> ValueState {
    ValueState(*value)
}

#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "TemporalDemand's predicate shape borrows the observed state"
)]
fn at_most_two(state: &ValueState) -> Holding {
    if state.0 <= 2u8 {
        Holding::Holds
    } else {
        Holding::Fails
    }
}

fn contract() -> Result<TransitionContract<ValueState, u8>, ContractRefusal> {
    TransitionContract::declared(
        opening_state,
        apply_value,
        vec![TemporalClaim::declared(
            PROPERTY_CAUSE,
            TemporalDemand::Always(at_most_two),
        )],
    )
}

fn plan(
    population: PopulationRef,
    origin: InputOrigin,
) -> Result<GenerationPlan, CorpusRoadFailure> {
    Ok(GenerationPlan::declared(
        population,
        GenerationProfile::declared("corpus-warm-start", 1u32),
        origin,
        CaseBudget::declared(1u32),
        ByteBudget::declared(1u64),
        RejectionAllowance::declared(1u32),
        SizeProgression::Constant {
            width: CaseWidth::declared(1usize)?,
        },
    )?)
}

fn pack_conclusion() -> Result<TrialConclusion, CorpusRoadFailure> {
    let expected_population = population("warm-start-values")?;
    let written = pack(expected_population, seeds()?)?;
    let admitted = read(expected_population, written.encoded())?;
    let contract = contract()?;
    let mut conclusions = Vec::new();
    for (origin, seed) in warm_start(&admitted).zip(admitted.seeds()) {
        let InputOrigin::Supplied(input) = &origin else {
            return Err(CorpusRoadFailure::EvidenceMismatch);
        };
        if input.as_slice() != seed.bytes() {
            return Err(CorpusRoadFailure::EvidenceMismatch);
        }
        let plan = plan(expected_population, origin)?;
        let source = ByteSource::of_plan(&plan);
        let reading = holds_over_drive(
            &contract,
            &plan,
            &source,
            decode_arbitrary::<u8>,
            admit_every_sequence::<u8>,
        );
        let Some(sequence) = reading.generated().sequences().first() else {
            return Err(CorpusRoadFailure::MissingEvidence);
        };
        if sequence.input() != seed.bytes() {
            return Err(CorpusRoadFailure::EvidenceMismatch);
        }
        let TemporalDriveStanding::Concluded(conclusion) = reading.standing() else {
            return Err(CorpusRoadFailure::MissingEvidence);
        };
        conclusions.push(conclusion.clone());
    }
    let mut conclusions = conclusions.into_iter();
    let Some(lawful) = conclusions.next() else {
        return Err(CorpusRoadFailure::MissingEvidence);
    };
    let Some(counterexample) = conclusions.next() else {
        return Err(CorpusRoadFailure::MissingEvidence);
    };
    if conclusions.next().is_some() || lawful != TrialConclusion::Passed {
        return Err(CorpusRoadFailure::EvidenceMismatch);
    }
    match &counterexample {
        TrialConclusion::Refused(finding)
            if finding.class() == FailureClass::PropertyDisagreement
                && finding.cause() == PROPERTY_CAUSE => {}
        TrialConclusion::Passed | TrialConclusion::Refused(_) => {
            return Err(CorpusRoadFailure::EvidenceMismatch);
        }
    }
    Ok(counterexample)
}

fn fixture_refusal() -> TrialConclusion {
    threadpak_testpak::properties::concluded(
        Holding::Fails,
        FailureClass::RefusedByCheck,
        FIXTURE_CAUSE,
    )
}

fn corpus_trial(_invocation: &Invocation) -> TrialConclusion {
    pack_conclusion().unwrap_or_else(|_refusal| fixture_refusal())
}

fn world() -> Result<TrialTable, TrialTableRefusal> {
    let subject = SubjectRoute::named(CONSUMER, "one-byte-state")?;
    let check = CheckRef::named(CONSUMER, "warm-start-property")?;
    let row = Row::declared(
        ClaimRef::named(CONSUMER, "corpus-reaches-generation")?,
        ExecutionSuite::named(CONSUMER, "corpus-warm-start")?,
        Classification::authored(
            vec![Role::named(CONSUMER, "generation")?],
            vec![Tag::named(CONSUMER, "corpus")?],
        )?,
        subject,
        check,
        population("warm-start-values")?,
        Origin::HandWritten,
    )?;
    let revision = RevisionBinding::declared(ContentAddress::derived(
        REVISION_TAG,
        b"corpus-warm-start/v1",
    ));
    let binding = Binding::bound(
        row,
        ExecutableAttachment::attached(subject, check, revision, revision, corpus_trial),
        Provenance::Unproduced,
    )?;
    TrialTable::authored(
        AuthoredTableName::named(CONSUMER, "corpus-world")?,
        Provenance::Unproduced,
        vec![binding],
    )
    .map_err(TrialTableRefusal::TableNotAuthored)
}

fn invocation() -> Invocation {
    Invocation::declared(
        InvocationProfile::declared(
            CaseBudget::declared(1u32),
            ByteBudget::declared(1u64),
            TimeBudget::declared(1u64),
        ),
        TargetBinding::bound(
            TargetTriple::declared("neutral-test-target"),
            ToolchainIdentity::declared("1.98.0"),
        ),
        TrialSite::located(module_path!(), file!(), line!(), "corpus-warm-start"),
        HarnessClock::unavailable(),
    )
}

fn foreign_envelope(population: PopulationRef, seeds: &[&[u8]], trailing: &[u8]) -> Vec<u8> {
    let mut body = Vec::new();
    body.extend_from_slice(&SEED_PACK_FORMAT_VERSION.to_be_bytes());
    let name = population.name();
    encode_bytes(name.namespace().written().as_bytes(), &mut body);
    encode_bytes(name.stem().written().as_bytes(), &mut body);
    encode_length(seeds.len(), &mut body);
    for seed in seeds {
        encode_bytes(seed, &mut body);
    }
    body.extend_from_slice(trailing);
    let address = ContentAddress::derived(SEED_PACK_TAG, &body);
    let mut envelope = Vec::new();
    envelope.extend_from_slice(address.as_bytes());
    envelope.extend_from_slice(&body);
    envelope
}

#[test]
fn pack_read_warm_start_and_report_retain_their_authority_boundaries()
-> Result<(), CorpusRoadFailure> {
    let expected_population = population("warm-start-values")?;
    let written = pack(expected_population, seeds()?)?;
    let admitted = read(expected_population, written.encoded())?;
    assert_eq!(admitted.population(), expected_population);
    assert_eq!(admitted.address(), written.address());
    assert_eq!(admitted.encoded(), written.encoded());
    assert_eq!(
        admitted
            .seeds()
            .iter()
            .map(SeedInput::bytes)
            .collect::<Vec<_>>(),
        written
            .seeds()
            .iter()
            .map(SeedInput::bytes)
            .collect::<Vec<_>>()
    );
    assert_eq!(warm_start(&admitted).len(), admitted.seeds().len());

    let world = world()?;
    let report = run_all(
        &world.view(),
        &SelectionPlan::of(Selection::All),
        &invocation(),
    );
    assert_eq!(report.selection(), SelectionOutcome::Satisfied);
    assert_eq!(report.denominator(), world.view().bindings().count());
    let accounting = report
        .census()
        .first()
        .ok_or(CorpusRoadFailure::MissingEvidence)?;
    let trial = accounting
        .disposition()
        .report()
        .ok_or(CorpusRoadFailure::MissingEvidence)?;
    let RunAttempt::Executed(TrialConclusion::Refused(finding)) = trial.attempt() else {
        return Err(CorpusRoadFailure::EvidenceMismatch);
    };
    assert_eq!(finding.class(), FailureClass::PropertyDisagreement);
    assert_eq!(finding.cause(), PROPERTY_CAUSE);
    let fingerprint = Fingerprint::of(accounting.trial(), finding);
    assert_eq!(fingerprint.trial(), accounting.trial());
    assert_eq!(fingerprint.class(), finding.class());
    assert_eq!(fingerprint.cause(), finding.cause());
    assert_eq!(
        fingerprint.address(),
        Fingerprint::over(
            accounting.trial(),
            PROPERTY_CAUSE,
            FailureClass::PropertyDisagreement
        )
        .address()
    );
    Ok(())
}

#[test]
fn foreign_envelopes_refuse_corruption_duplicates_and_malformed_members()
-> Result<(), CorpusRoadFailure> {
    let expected_population = population("warm-start-values")?;
    let written = pack(expected_population, seeds()?)?;
    let mut corrupted = written.encoded().to_vec();
    let address_width = written.address().address().as_bytes().len();
    let Some(format_byte) = corrupted
        .get_mut(address_width..)
        .and_then(<[u8]>::first_mut)
    else {
        return Err(CorpusRoadFailure::MissingEvidence);
    };
    *format_byte ^= 1u8;
    assert!(matches!(
        read(expected_population, &corrupted),
        Err(SeedPackRefusal::AddressMismatch { .. })
    ));

    let duplicate = foreign_envelope(expected_population, &[&[7u8], &[7u8]], &[]);
    assert_eq!(
        read(expected_population, &duplicate),
        Err(SeedPackRefusal::DuplicateSeed {
            first: 0usize,
            duplicate: 1usize,
        })
    );
    assert_eq!(
        pack(
            expected_population,
            vec![
                SeedInput::declared(vec![7u8])?,
                SeedInput::declared(vec![7u8])?,
            ],
        ),
        Err(SeedPackRefusal::DuplicateSeed {
            first: 0usize,
            duplicate: 1usize,
        })
    );
    assert_eq!(
        read(expected_population, &[]),
        Err(SeedPackRefusal::Truncated)
    );

    let other = pack(population("other-values")?, seeds()?)?;
    assert_eq!(
        read(expected_population, other.encoded()),
        Err(SeedPackRefusal::PopulationMismatch)
    );
    let trailing = foreign_envelope(expected_population, &[&[1u8]], &[9u8]);
    assert_eq!(
        read(expected_population, &trailing),
        Err(SeedPackRefusal::TrailingBytes { count: 1usize })
    );
    let empty = foreign_envelope(expected_population, &[&[]], &[]);
    assert_eq!(
        read(expected_population, &empty),
        Err(SeedPackRefusal::EmptySeed { at: 0usize })
    );
    assert_eq!(
        SeedInput::declared(Vec::new()),
        Err(SeedInputRefusal::Empty)
    );
    Ok(())
}
