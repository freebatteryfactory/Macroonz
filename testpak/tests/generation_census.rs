//! The public generation driver classifies real cases into a non-uniform census and retains their exact ordinals and inputs.
//!
//! The disposition roster already owns census width and storage structurally.
//! This lane observes the separate behavioral claim: the driver assigns each reached case to the right seat exactly once.

use std::fmt;
use threadpak_testpak::descriptor::{NameRefusal, PopulationRef};
use threadpak_testpak::generate::{
    ByteSource, CaseWidth, CaseWidthRefusal, GenerationDisposition, GenerationHalt, GenerationPlan,
    GenerationPlanRefusal, InputOrigin, PreconditionVerdict, RejectionBudget, SizeProgression,
    decode_arbitrary, drive,
};
use threadpak_testpak::report::{ByteBudget, CaseBudget, GenerationProfile};

const SOURCE: [u8; 3] = [2u8, 1u8, 4u8];

enum GenerationRoadFailure {
    Name(NameRefusal),
    Width(CaseWidthRefusal),
    Plan(GenerationPlanRefusal),
}

impl fmt::Debug for GenerationRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name(refusal) => formatter.debug_tuple("Name").field(refusal).finish(),
            Self::Width(refusal) => formatter.debug_tuple("Width").field(refusal).finish(),
            Self::Plan(refusal) => formatter.debug_tuple("Plan").field(refusal).finish(),
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

#[test]
fn drive_classifies_each_reached_case_once() -> Result<(), GenerationRoadFailure> {
    let plan = GenerationPlan::declared(
        PopulationRef::named("testpak", "non-uniform-generation")?,
        GenerationProfile::declared("non-uniform-generation", 1u32),
        InputOrigin::Supplied(SOURCE.to_vec()),
        CaseBudget::declared(3u32),
        ByteBudget::declared(3u64),
        RejectionBudget::declared(2u32),
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
