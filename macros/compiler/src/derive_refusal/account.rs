//! Carries this door's disposition for every projection kind in the closed compiler roster.
//!
//! The implementation and carrier seats name the outputs this door generated, while codec, benchmark descriptor, and pattern stamp carry typed unavailable-under-profile answers.
//! Each unavailable answer cites its own owner fact, so distinct missing inputs never collapse into one generic absence.

use super::plan::{rust_declaration_profile, rust_declaration_profile_version};
use super::types::{RefusalDeriveFact, RefusalFamilyExpansion};
use crate::closure::ClosedExpansion;
use crate::plane::OwnerFactRef;
use crate::planning::{
    KindDispositions, ProjectionDisposition, ProjectionKind, ProjectionPlan,
    TestDescriptorProjection,
};

/// Returns the typed unavailable-under-profile disposition under this door's declared profile and the caller-supplied owner fact.
pub fn profile_does_not_offer(because: OwnerFactRef) -> ProjectionDisposition {
    ProjectionDisposition::UnavailableUnderProfile {
        profile: rust_declaration_profile(),
        version: rust_declaration_profile_version(),
        because,
    }
}

/// What happened to the codec projection.
///
/// # The ground
///
/// A codec is projected FROM one schema and reads or writes one named BYTE ROLE,
/// and both are the machine's mints. The SCHEMA seat is the one a captured
/// declaration could have stood in for, on the obligation's own terms — a codec
/// planned at expansion time would be projected from the declaration it was
/// derived from. The BYTE ROLE is not a subject at all: it is the role an
/// artifact's canonical bytes are read under, a fact of the machine era, and no
/// declaration's tokens stand for one. So the record cannot be filled whichever
/// posture the schema stands under, and the schema seat is left exactly as the
/// planning home declares it.
///
/// # The citation
///
/// [`RefusalDeriveFact::AByteRoleIsNotReadOutOfACapture`], whose stable name
/// states exactly that: the byte role is the seat no posture over a captured
/// declaration reaches. It names the byte role alone because the byte
/// role alone is what stands — the schema seat is fillable in principle, and a
/// citation that named it too would report a blockage this ground does not
/// establish.
pub fn codec_disposition() -> ProjectionDisposition {
    profile_does_not_offer(RefusalDeriveFact::AByteRoleIsNotReadOutOfACapture.citation())
}

/// What happened to the benchmark-descriptor projection.
///
/// # The ground
///
/// A benchmark descriptor measures one declared UNIT and states its envelope in
/// one named WORK CURRENCY, and both are the machine's mints. The measured seat
/// is the one a captured declaration could have stood in for, on the
/// obligation's own terms. The currency is not a subject: it is the vocabulary a
/// measurement is stated in, and no declaration stands for one.
///
/// # Nonclaims
///
/// It is not [`bench_disposition`](super::bench_disposition), and the two answer
/// different questions. That one says what happened to the bench material this
/// CARRIER would have delivered — the carrier's published grammar writes a
/// trials seat and a deferred seat, and neither is the bench seat — and it would
/// stand unchanged if these mints existed. This one says what happened to the
/// benchmark-descriptor PROJECTION, and it would stand unchanged if that seat
/// were written.
///
/// The two are apart at the CITATION as well as in prose: that one cites the
/// carrier-seat fact, this one cites the work currency, so a reader holding
/// either answer knows which question it answers.
///
/// # The citation
///
/// [`RefusalDeriveFact::AWorkCurrencyIsNotReadOutOfACapture`], whose stable name
/// states the currency alone — the measured seat is a subject a captured
/// declaration could stand in for, so naming it beside the currency would report
/// a blockage this ground does not establish.
pub fn benchmark_disposition() -> ProjectionDisposition {
    profile_does_not_offer(RefusalDeriveFact::AWorkCurrencyIsNotReadOutOfACapture.citation())
}

/// What happened to the pattern-stamp projection.
///
/// # The ground
///
/// A stamp is planned from anchors a CALLER supplies — the authored pattern,
/// this instantiation of it, the typed arguments, and the byte role its artifact
/// is read under — and every one of them is the machine's mint. Not one is a
/// subject a captured declaration stands for: a pattern is authored elsewhere
/// and a stamp names which one it instantiates.
///
/// Its member lands as a publication ARTIFACT besides, which is bytes at an
/// address written under a receipt and committed by a human, and not a delivery
/// any expansion emits.
///
/// # The citation
///
/// [`RefusalDeriveFact::APatternApplicationAndPublicationAreNotHeldByAnExpansion`],
/// whose stable name states both: the authored application a caller supplies,
/// and the publication posture the member's delivery stands under. Two blockers,
/// independently true, and the name carries the conjunction so neither reads as
/// the whole of it.
pub fn pattern_stamp_disposition() -> ProjectionDisposition {
    profile_does_not_offer(
        RefusalDeriveFact::APatternApplicationAndPublicationAreNotHeldByAnExpansion.citation(),
    )
}

/// The GENERATED disposition for one plan, naming the output a disposition
/// names.
///
/// # Bounds
///
/// A disposition names ONE output, because that is the shape a disposition has,
/// and the one it names is the plan's first declared member. The complete set a
/// plan materializes is the plan's own membership, which is where a reader
/// asking what was materialized reads; this answers the narrower question — what
/// happened to the projection at all.
///
/// The output is READ off the plan rather than composed here, so the seat and
/// the terminal beside it cannot disagree about what was planned.
fn generated<K: ProjectionKind>(plan: &ProjectionPlan<K>) -> ProjectionDisposition {
    ProjectionDisposition::Generated {
        output: Box::new(plan.membership().first().output.clone()),
    }
}

/// What this door says happened to every kind of the sealed roster, over one
/// captured declaration.
///
/// Total, and every seat of the record is written out: the two kinds this door
/// generates read their answers off the terminals that produced them, and the
/// six it does not each state their own ground. A kind admitted to the roster
/// stops the compiler here until somebody says what this door does about it,
/// which is the whole reason the record has a seat per kind rather than a list.
pub fn accounted(
    implementation: &RefusalFamilyExpansion,
    carrier: &ClosedExpansion<TestDescriptorProjection>,
) -> KindDispositions {
    KindDispositions {
        codec: codec_disposition(),
        test_descriptor: generated(carrier.plan()),
        benchmark_descriptor: benchmark_disposition(),
        refusal_family_implementation: generated(implementation.plan()),
        pattern_stamp: pattern_stamp_disposition(),
    }
}
