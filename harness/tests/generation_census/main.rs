//! The public generation driver classifies real cases into a non-uniform census and retains their exact ordinals and inputs.
//!
//! The disposition roster already owns census width and storage structurally.
//! This lane observes the separate behavioral claim: the driver assigns each reached case to the right seat exactly once.

use arbitrary::Unstructured;
use macroonz_harness::descriptor::{NameRefusal, PopulationRef};
use macroonz_harness::generate::driver::{admit_every_sequence, decode_arbitrary, drive};
use macroonz_harness::generate::types::{
    ByteDraw, ByteSource, ByteSourceAddress, CaseWidth, CaseWidthRefusal, GENERATION_CHUNK_TAG,
    GENERATION_SOURCE_TAG, GenerationDisposition, GenerationHalt, GenerationPlan,
    GenerationPlanRefusal, InputOrigin, PreconditionVerdict, RejectionAllowance, RootSeed,
    SOURCE_CHUNK_BYTES, SizeProgression, StreamCursor,
};
use macroonz_harness::identity::{ContentAddress, encode_bytes};
use macroonz_harness::report::{ByteBudget, CaseBudget, GenerationProfile};
use std::fmt;

const SOURCE: [u8; 3] = [2u8, 1u8, 4u8];

enum GenerationRoadFailure {
    Name(NameRefusal),
    Width(CaseWidthRefusal),
    Plan(GenerationPlanRefusal),
    Fixture,
}

impl fmt::Debug for GenerationRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Width(refusal) => formatter.debug_tuple("Width").field(refusal).finish(),
            Self::Plan(refusal) => formatter.debug_tuple("Plan").field(refusal).finish(),
            Self::Fixture => formatter.write_str("Fixture"),
        }
    }
}

impl From<NameRefusal> for GenerationRoadFailure {
    fn from(refusal: NameRefusal) -> Self {
        Self::Name(refusal)
    }
}

impl From<CaseWidthRefusal> for GenerationRoadFailure {
    fn from(refusal: CaseWidthRefusal) -> Self {
        Self::Width(refusal)
    }
}

impl From<GenerationPlanRefusal> for GenerationRoadFailure {
    fn from(refusal: GenerationPlanRefusal) -> Self {
        Self::Plan(refusal)
    }
}

fn admits_even(commands: &[u8]) -> PreconditionVerdict {
    match commands.first() {
        Some(command) if command % 2u8 == 0u8 => PreconditionVerdict::Admitted,
        Some(_) | None => PreconditionVerdict::Rejected,
    }
}

fn rejects_every_sequence(_commands: &[u8]) -> PreconditionVerdict {
    PreconditionVerdict::Rejected
}

fn refusing_decoder(_source: &mut Unstructured<'_>) -> arbitrary::Result<u8> {
    Err(arbitrary::Error::NotEnoughData)
}

fn non_consuming_decoder(source: &mut Unstructured<'_>) -> arbitrary::Result<u8> {
    if source.is_empty() {
        return Err(arbitrary::Error::NotEnoughData);
    }
    Ok(7u8)
}

fn supplied_plan(
    name: &'static str,
    source: Vec<u8>,
    cases: u32,
    bytes: u64,
    width: usize,
) -> Result<GenerationPlan, GenerationRoadFailure> {
    Ok(GenerationPlan::declared(
        PopulationRef::named("harness", name)?,
        GenerationProfile::declared(name, 1u32),
        InputOrigin::Supplied(source),
        CaseBudget::declared(cases),
        ByteBudget::declared(bytes),
        RejectionAllowance::declared(0u32),
        SizeProgression::Constant {
            width: CaseWidth::declared(width)?,
        },
    )?)
}

fn seeded_prefix_plan(cases: u32, bytes: u64) -> Result<GenerationPlan, GenerationRoadFailure> {
    Ok(GenerationPlan::declared(
        PopulationRef::named("harness", "monotonic-prefix")?,
        GenerationProfile::declared("monotonic-prefix", 1u32),
        InputOrigin::Seeded(RootSeed::declared(0x0102_0304_0506_0708u64)),
        CaseBudget::declared(cases),
        ByteBudget::declared(bytes),
        RejectionAllowance::NoRejections,
        SizeProgression::Constant {
            width: CaseWidth::declared(4usize)?,
        },
    )?)
}

#[test]
fn drive_classifies_each_reached_case_once() -> Result<(), GenerationRoadFailure> {
    let plan = GenerationPlan::declared(
        PopulationRef::named("harness", "non-uniform-generation")?,
        GenerationProfile::declared("non-uniform-generation", 1u32),
        InputOrigin::Supplied(SOURCE.to_vec()),
        CaseBudget::declared(3u32),
        ByteBudget::declared(3u64),
        RejectionAllowance::declared(2u32),
        SizeProgression::Constant {
            width: CaseWidth::declared(1usize)?,
        },
    )?;
    let source = ByteSource::of_plan(&plan);
    let generated = drive(&plan, &source, decode_arbitrary::<u8>, admits_even);
    let census = generated.census();

    assert_eq!(census.count_of(GenerationDisposition::Generated), 2u32);
    assert_eq!(
        census.count_of(GenerationDisposition::PreconditionRejected),
        1u32
    );
    assert_eq!(
        census.count_of(GenerationDisposition::BytesInsufficient),
        0u32
    );
    assert_eq!(
        census.count_of(GenerationDisposition::GeneratorRefused),
        0u32
    );
    assert_eq!(
        census.count_of(GenerationDisposition::GeneratorContractViolated),
        0u32
    );
    assert_eq!(
        census.count_of(GenerationDisposition::GenerationBudgetExhausted),
        0u32
    );
    assert_eq!(Some(census.attempted()), u32::try_from(SOURCE.len()).ok());
    assert_eq!(generated.halt(), GenerationHalt::CaseBudgetMet);

    let observed: Vec<(u32, &[u8], &[u8])> = generated
        .sequences()
        .iter()
        .map(|sequence| {
            (
                sequence.case().ordinal(),
                sequence.input(),
                sequence.commands(),
            )
        })
        .collect();
    assert_eq!(
        observed,
        vec![
            (0u32, &[2u8][..], &[2u8][..]),
            (2u32, &[4u8][..], &[4u8][..])
        ]
    );
    Ok(())
}

#[test]
fn positive_rejection_allowance_stops_after_exactly_that_many_rejections()
-> Result<(), GenerationRoadFailure> {
    let plan = GenerationPlan::declared(
        PopulationRef::named("harness", "bounded-rejections")?,
        GenerationProfile::declared("bounded-rejections", 1u32),
        InputOrigin::Supplied(vec![1u8, 3u8, 5u8]),
        CaseBudget::declared(3u32),
        ByteBudget::declared(3u64),
        RejectionAllowance::declared(1u32),
        SizeProgression::Constant {
            width: CaseWidth::declared(1usize)?,
        },
    )?;
    let generated = drive(
        &plan,
        &ByteSource::of_plan(&plan),
        decode_arbitrary::<u8>,
        rejects_every_sequence,
    );

    assert_eq!(
        generated
            .census()
            .count_of(GenerationDisposition::PreconditionRejected),
        1u32
    );
    assert_eq!(generated.census().attempted(), 1u32);
    assert_eq!(generated.halt(), GenerationHalt::RejectionAllowanceSpent);
    Ok(())
}

#[test]
fn zero_rejection_allowance_permits_success_then_retains_the_first_rejection()
-> Result<(), GenerationRoadFailure> {
    let plan = GenerationPlan::declared(
        PopulationRef::named("harness", "zero-rejections")?,
        GenerationProfile::declared("zero-rejections", 1u32),
        InputOrigin::Supplied(vec![2u8, 1u8]),
        CaseBudget::declared(2u32),
        ByteBudget::declared(2u64),
        RejectionAllowance::declared(0u32),
        SizeProgression::Constant {
            width: CaseWidth::declared(1usize)?,
        },
    )?;
    let generated = drive(
        &plan,
        &ByteSource::of_plan(&plan),
        decode_arbitrary::<u8>,
        admits_even,
    );

    assert_eq!(
        generated
            .census()
            .count_of(GenerationDisposition::Generated),
        1u32
    );
    assert_eq!(
        generated
            .census()
            .count_of(GenerationDisposition::PreconditionRejected),
        1u32
    );
    assert_eq!(generated.census().attempted(), 2u32);
    assert_eq!(generated.halt(), GenerationHalt::RejectionAllowanceSpent);
    Ok(())
}

#[test]
fn source_and_chunk_addresses_match_their_declared_byte_preimages()
-> Result<(), GenerationRoadFailure> {
    let seed = RootSeed::declared(0x0102_0304_0506_0708u64);
    let plan = GenerationPlan::declared(
        PopulationRef::named("harness", "identity-observer")?,
        GenerationProfile::declared("identity-observer", 3u32),
        InputOrigin::Seeded(seed),
        CaseBudget::declared(2u32),
        ByteBudget::declared(64u64),
        RejectionAllowance::declared(1u32),
        SizeProgression::Constant {
            width: CaseWidth::declared(8usize)?,
        },
    )?;
    let differently_windowed = GenerationPlan::declared(
        plan.population(),
        plan.profile(),
        InputOrigin::Seeded(seed),
        CaseBudget::declared(7u32),
        ByteBudget::declared(512u64),
        RejectionAllowance::declared(4u32),
        SizeProgression::Doubling {
            base: CaseWidth::declared(3usize)?,
        },
    )?;

    let mut source_preimage = Vec::new();
    plan.population().name().encode_into(&mut source_preimage);
    encode_bytes(plan.profile().name().as_bytes(), &mut source_preimage);
    source_preimage.extend_from_slice(&plan.profile().version().to_be_bytes());
    source_preimage.push(1u8);
    source_preimage.extend_from_slice(&seed.value().to_be_bytes());
    let address = ByteSourceAddress::of_plan(&plan);
    assert_eq!(
        address.address(),
        ContentAddress::derived(GENERATION_SOURCE_TAG, &source_preimage)
    );
    assert_eq!(address, ByteSourceAddress::of_plan(&differently_windowed));

    let counter = 3u64;
    let mut chunk_preimage = Vec::new();
    encode_bytes(address.address().as_bytes(), &mut chunk_preimage);
    chunk_preimage.extend_from_slice(&counter.to_be_bytes());
    let expected = ContentAddress::derived(GENERATION_CHUNK_TAG, &chunk_preimage);
    let Ok(cursor) = StreamCursor::at(counter, 0usize) else {
        return Err(GenerationRoadFailure::Fixture);
    };
    let ByteDraw::Drawn { bytes, next } =
        ByteSource::Derived(address).draw(cursor, SOURCE_CHUNK_BYTES)
    else {
        return Err(GenerationRoadFailure::Fixture);
    };
    let Ok(expected_next) = StreamCursor::at(counter.saturating_add(1u64), 0usize) else {
        return Err(GenerationRoadFailure::Fixture);
    };
    assert_eq!(bytes.as_slice(), expected.as_bytes());
    assert_eq!(next, expected_next);
    Ok(())
}

#[test]
fn extending_the_budget_preserves_the_exact_prefix_and_direct_seek()
-> Result<(), GenerationRoadFailure> {
    let short_plan = seeded_prefix_plan(2u32, 8u64)?;
    let extended_plan = seeded_prefix_plan(4u32, 16u64)?;
    assert_eq!(
        ByteSourceAddress::of_plan(&short_plan),
        ByteSourceAddress::of_plan(&extended_plan)
    );

    let short = drive(
        &short_plan,
        &ByteSource::of_plan(&short_plan),
        decode_arbitrary::<u8>,
        admit_every_sequence::<u8>,
    );
    let extended = drive(
        &extended_plan,
        &ByteSource::of_plan(&extended_plan),
        decode_arbitrary::<u8>,
        admit_every_sequence::<u8>,
    );
    assert_eq!(short.sequences().len(), 2usize);
    assert_eq!(extended.sequences().len(), 4usize);
    assert!(
        short
            .sequences()
            .iter()
            .zip(extended.sequences())
            .all(|(left, right)| left == right)
    );

    let source = ByteSource::of_plan(&extended_plan);
    let ByteDraw::Drawn { bytes: first, next } = source.draw(StreamCursor::opening(), 4usize)
    else {
        return Err(GenerationRoadFailure::Fixture);
    };
    let Ok(direct) = StreamCursor::at(0u64, 4usize) else {
        return Err(GenerationRoadFailure::Fixture);
    };
    assert_eq!(next, direct);
    let ByteDraw::Drawn { bytes: second, .. } = source.draw(direct, 4usize) else {
        return Err(GenerationRoadFailure::Fixture);
    };
    let ByteDraw::Drawn { bytes: joined, .. } = source.draw(StreamCursor::opening(), 8usize) else {
        return Err(GenerationRoadFailure::Fixture);
    };
    assert_eq!(first.into_iter().chain(second).collect::<Vec<_>>(), joined);
    Ok(())
}

#[test]
fn driver_exposes_source_budget_refusal_and_contract_stops() -> Result<(), GenerationRoadFailure> {
    let insufficient = supplied_plan("insufficient", vec![1u8], 1u32, 2u64, 2usize)?;
    let exhausted = supplied_plan("exhausted", vec![2u8, 3u8], 2u32, 1u64, 1usize)?;
    let refused = supplied_plan("refused", vec![4u8], 1u32, 1u64, 1usize)?;
    let violated = supplied_plan("violated", vec![5u8], 1u32, 1u64, 1usize)?;

    let source_stop = drive(
        &insufficient,
        &ByteSource::of_plan(&insufficient),
        decode_arbitrary::<u8>,
        admits_even,
    );
    assert_eq!(
        source_stop
            .census()
            .count_of(GenerationDisposition::BytesInsufficient),
        1u32
    );
    assert_eq!(source_stop.halt(), GenerationHalt::SourceExhausted);

    let budget_stop = drive(
        &exhausted,
        &ByteSource::of_plan(&exhausted),
        decode_arbitrary::<u8>,
        admits_even,
    );
    assert_eq!(
        budget_stop
            .census()
            .count_of(GenerationDisposition::GenerationBudgetExhausted),
        1u32
    );
    assert_eq!(budget_stop.halt(), GenerationHalt::ByteBudgetExhausted);

    let refusal_stop = drive(
        &refused,
        &ByteSource::of_plan(&refused),
        refusing_decoder,
        admits_even,
    );
    assert_eq!(
        refusal_stop
            .census()
            .count_of(GenerationDisposition::GeneratorRefused),
        1u32
    );
    assert_eq!(refusal_stop.halt(), GenerationHalt::RejectionAllowanceSpent);

    let contract_stop = drive(
        &violated,
        &ByteSource::of_plan(&violated),
        non_consuming_decoder,
        admits_even,
    );
    assert_eq!(
        contract_stop
            .census()
            .count_of(GenerationDisposition::GeneratorContractViolated),
        1u32
    );
    assert_eq!(
        contract_stop.halt(),
        GenerationHalt::GeneratorContractViolated
    );
    Ok(())
}

#[test]
fn plan_refusals_follow_the_declared_first_failure_order() -> Result<(), GenerationRoadFailure> {
    let population = PopulationRef::named("harness", "refusal-order")?;
    let profile = GenerationProfile::declared("refusal-order", 1u32);
    let progression = SizeProgression::Constant {
        width: CaseWidth::declared(1usize)?,
    };
    assert_eq!(
        GenerationPlan::declared(
            population,
            profile,
            InputOrigin::Supplied(Vec::new()),
            CaseBudget::declared(0u32),
            ByteBudget::declared(0u64),
            RejectionAllowance::declared(0u32),
            progression,
        ),
        Err(GenerationPlanRefusal::ZeroCaseBudget)
    );
    assert_eq!(
        GenerationPlan::declared(
            population,
            profile,
            InputOrigin::Supplied(Vec::new()),
            CaseBudget::declared(1u32),
            ByteBudget::declared(0u64),
            RejectionAllowance::declared(0u32),
            progression,
        ),
        Err(GenerationPlanRefusal::ZeroByteBudget)
    );
    assert_eq!(
        GenerationPlan::declared(
            population,
            profile,
            InputOrigin::Supplied(Vec::new()),
            CaseBudget::declared(1u32),
            ByteBudget::declared(1u64),
            RejectionAllowance::declared(0u32),
            progression,
        ),
        Err(GenerationPlanRefusal::EmptySuppliedBytes)
    );
    Ok(())
}
