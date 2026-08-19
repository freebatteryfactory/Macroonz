//! What this door says about EVERY projection kind over one captured
//! declaration.
//!
//! # Why a door owes the whole roster
//!
//! Two kinds are produced here — the implementation projection the declaration
//! IS, and the carrier that delivers what it deferred — and the joined value
//! carries both terminals whole. The other six kinds of the sealed roster
//! produce nothing at this door, and "nothing" is the one answer a disposition
//! has no variant for. So each of them is answered here, once, by a road that
//! states the ground in its own words.
//!
//! # The one ground, and the six roads that stand on it
//!
//! Every kind this door does not offer is blocked by the same fact and wants a
//! different thing from it. The fact is that an expansion holds no machine mint:
//! [`OwnerIdentityRef`] has one production road and it takes a commitment the
//! machine already minted, and a derive is handed a captured declaration and an
//! expansion context. What each kind wants — a schema and a byte role, a work
//! currency, a host contract, a port and a wire contract, an audience, an
//! authored pattern and its arguments — is written at that kind's own road
//! below, because "the profile does not offer it" is the standing and the seat a
//! kind could not fill is the reason a reader is looking for.
//!
//! # What this file did NOT grow, and why
//!
//! The obligation seat of the descriptor content is the anchoring precedent:
//! where a caller holds the machine's obligation identity it names one, and
//! where nothing has been linked it names the captured declaration the
//! descriptor was derived from instead — the two postures never read alike, and
//! neither is a missing obligation. That posture is honest because the seat
//! names the SUBJECT the projection is about, and the captured declaration IS
//! that subject.
//!
//! Several kinds carry one seat of exactly that shape: the schema a codec is
//! projected from, the unit a benchmark measures, the subject documentation
//! documents, the port a remote surface projects. Not one of them is grown here,
//! and the reason is the same in every case: each of those records carries a
//! second seat that is NOT a subject — a byte role, a work currency, an
//! audience, a wire contract — and no captured declaration stands for one of
//! those under any posture. A record that still cannot be filled is a record no
//! plan can be made from, so an anchoring grown at its subject seat would be
//! machinery nothing pulls, and the day the mints exist it would be a posture
//! decided by a door rather than by the home that owns the seat.
//!
//! The verdict is recorded at each kind's road rather than in a ledger, so the
//! ground travels with the answer.
//!
//! [`OwnerIdentityRef`]: crate::plane::OwnerIdentityRef

use super::plan::{rust_declaration_profile, rust_declaration_profile_version};
use super::types::RefusalFamilyExpansion;
use crate::closure::ClosedExpansion;
use crate::planning::{
    KindDispositions, ProjectionDisposition, ProjectionKind, ProjectionPlan,
    TestDescriptorProjection,
};

/// The posture this door's declared compiler profile stands in for every kind it
/// does not offer.
///
/// # The ground
///
/// The derive reads a declaration under its DECLARED COMPILER PROFILE, and that
/// profile reads TOKENS: an item, its attributes, and the words inside them.
/// Every kind below asks its plan for a fact a token reading does not produce —
/// one of the machine's own mints — and an expansion holds none of them.
///
/// It is not a refusal and it is not an absence. A refusal would stop a lawful
/// derivation over a lawful declaration; an absence would read as though
/// somebody had forgotten to decide. This is a decision, recorded — the same
/// decision, under the same profile at the same version, that the documentation
/// road already records for the one election it stops at
/// ([`documented`](super::documented)).
///
/// # Bounds
///
/// It names the PROFILE and its version, which is the whole of what this arm of
/// the disposition carries, and the seat a kind could not fill is stated at that
/// kind's own road. The repair is a profile that offers the kind — the machine's
/// linked declaration path, where the mints exist — and never a stand-in minted
/// here.
///
/// One road for the whole standing rather than one construction per kind: a
/// profile bump moves every kind that stands under it, and six literals would
/// move six times or drift.
#[must_use]
pub fn profile_does_not_offer() -> ProjectionDisposition {
    ProjectionDisposition::UnavailableUnderProfile {
        profile: rust_declaration_profile(),
        version: rust_declaration_profile_version(),
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
#[must_use]
pub fn codec_disposition() -> ProjectionDisposition {
    profile_does_not_offer()
}

/// What happened to the host-wrapper projection.
///
/// # The ground
///
/// Twice, independently. A wrapper binds one named HOST CONTRACT, which is the
/// machine's mint; and the kind declares
/// [`TargetRequirement::BoundHostContract`] while an expansion's context binds
/// no host contract at all ([`TargetBinding::TargetFree`]) — so a plan of this
/// kind refuses at this seam whatever content it were handed. Neither reason
/// stands on the other, and closing one would leave the other.
///
/// The wrapper home states the same standing in its own vocabulary, as a typed
/// reading rather than a crippled wrapper that answers anyway: see
/// [`WrapperAvailability`], whose no-contract arm carries what would open the
/// road.
///
/// [`TargetRequirement::BoundHostContract`]: crate::planning::TargetRequirement::BoundHostContract
/// [`TargetBinding::TargetFree`]: crate::planning::TargetBinding::TargetFree
/// [`WrapperAvailability`]: crate::host_wrapper::WrapperAvailability
#[must_use]
pub fn host_wrapper_disposition() -> ProjectionDisposition {
    profile_does_not_offer()
}

/// What happened to the remote-surface projection.
///
/// # The ground
///
/// A remote surface projects one declared PORT over one WIRE CONTRACT, and both
/// are the machine's mints. The port is the subject a captured declaration could
/// have stood in for; the wire contract is the protocol the surface speaks,
/// which is a fact of the machine era and not a thing a declaration's tokens
/// stand for. The kind declares [`TargetRequirement::BoundHostContract`]
/// besides, which an expansion's target-free context does not satisfy.
///
/// [`TargetRequirement::BoundHostContract`]: crate::planning::TargetRequirement::BoundHostContract
#[must_use]
pub fn remote_surface_disposition() -> ProjectionDisposition {
    profile_does_not_offer()
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
#[must_use]
pub fn benchmark_disposition() -> ProjectionDisposition {
    profile_does_not_offer()
}

/// What happened to the documentation projection.
///
/// # The ground, and how far the captured rows reach
///
/// The MATERIAL is readable and is read. [`documented`](super::documented) wires
/// the family seat's prose into the documentation home's own documented item,
/// through that home's own door, carried unchanged — a caller that wants the
/// material walks that road and receives it.
///
/// What cannot be made is the PLAN. A documentation plan names a SUBJECT, an
/// AUDIENCE, and the FACETS covered. The subject is the seat a captured
/// declaration could have stood in for — the declaration IS what the prose
/// documents. The AUDIENCE is not: which reader a piece of prose is pitched at
/// is a declared domain value the machine mints, and a declaration's tokens
/// stand for no audience under any posture. And the facets are the same election
/// the documentation road already stops at, under this same profile at this same
/// version, because which facet a sentence COVERS is a reading of meaning and
/// this profile reads tokens.
///
/// # Bounds
///
/// The door does not WALK the documentation reading to state this. A reading
/// refuses where an author's family line is not the one plain sentence that
/// home's law admits, and a door that refused a declaration over its prose would
/// stop a derivation that has always compiled — the reading stays the road a
/// caller walks, and this seat states the standing.
#[must_use]
pub fn documentation_disposition() -> ProjectionDisposition {
    profile_does_not_offer()
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
#[must_use]
pub fn pattern_stamp_disposition() -> ProjectionDisposition {
    profile_does_not_offer()
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
#[must_use]
pub fn accounted(
    implementation: &RefusalFamilyExpansion,
    carrier: &ClosedExpansion<TestDescriptorProjection>,
) -> KindDispositions {
    KindDispositions {
        codec: codec_disposition(),
        host_wrapper: host_wrapper_disposition(),
        remote_surface: remote_surface_disposition(),
        test_descriptor: generated(carrier.plan()),
        benchmark_descriptor: benchmark_disposition(),
        documentation: documentation_disposition(),
        derive_impl: generated(implementation.plan()),
        pattern_stamp: pattern_stamp_disposition(),
    }
}
