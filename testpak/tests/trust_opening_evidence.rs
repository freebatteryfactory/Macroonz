//! The two typed facts a wrap reading opens trust with, built end to end and
//! then built wrong: the adapter qualification a reading's own profile earns,
//! the compiled-pressure witness read out of that reading, and the interpreted
//! lane's gate that consumes both.
//!
//! # Ownership
//!
//! The fail-closed boundary is stated in types and enforced in three
//! constructors, and a boundary of that shape is only evidence if the values it
//! admits can be BUILT and the ones it refuses can be built anyway and seen to
//! be wrong. Nothing in the workspace builds either half — no caller anywhere
//! reaches `read_output`, `AdapterQualification::of`, or
//! `CompiledPressureWitness::shown` — so this plane builds them: one console
//! text, read through the public roads into a reading, and every pairing the
//! three roads admit or refuse stated beside what it produces.
//!
//! Two directions, always. The positive control is load-bearing in its own
//! right: a boundary that refused everything would satisfy every hostile
//! assertion below and be worthless, so the first test carries the whole chain
//! opening — reading, qualification, witness, gate — and each reversal after it
//! is a repair restored HERE out of the same public values.
//!
//! # The fixture
//!
//! The console text below is authored against the line grammar `wrap.rs`
//! publishes on its own page and against nothing else. It is not a capture of a
//! backend's real output, and this plane could not honestly claim one: that
//! grammar is the BOOTSTRAP CONTRACT — an assumption about one tool's rendering
//! that a party checks against real output at the first toolchain contact — and
//! the standing which records such a check is exactly the value under judgement
//! here.
//!
//! Which published prose each fixture fact was written from:
//!
//! - the three line shapes — a roster line, a baseline line, and a mutant line
//!   — and the five outcome words: the output-grammar section of `wrap.rs`'s
//!   own page;
//! - only a clean pass qualifying the baseline: the same page's baseline
//!   sentence;
//! - a kill under this lane carrying no observed activation, and a non-kill
//!   never earning survived: the same page's section on what the lane can and
//!   cannot establish, and the claim ceiling stated with `ClaimCeiling`;
//! - the one pairing that qualifies and the three that do not: the construction
//!   paragraph on `AdapterQualification`;
//! - the witness married to the qualification of the very reading it was read
//!   out of: the construction paragraph on `CompiledPressureWitness`;
//! - the gate's own comparison and the roster of what it can be owed: the
//!   authority paragraph on `MissingTrustEvidence`, and `interpret`'s own page
//!   on the trust order.
//!
//! # The two version postures
//!
//! One console text, two readings, and they differ in exactly one fact. Three
//! of a console profile's four members are the adapter's own — `console_profile`
//! states the backend, the output, and the grammar version — so the only member
//! a caller states is the party's word about which backend version wrote the
//! text. Every reversal below therefore turns on that word, and the profile
//! disagreements the two later roads refuse are reachable through it and through
//! nothing else this lane declares.
//!
//! # Nonclaims
//!
//! This says the boundary admits what its pages say it admits and refuses what
//! they say it refuses. It says nothing about whether any backend really renders
//! these shapes — that is the check a grammar standing records, and no test can
//! perform it for a party.

use threadpak_testpak::descriptor::{ClaimRef, MutationPointRef};
use threadpak_testpak::muterprater::interpret::availability;
use threadpak_testpak::muterprater::wrap::read_output;
use threadpak_testpak::muterprater::{
    ActivationSite, AdapterQualification, AnnouncedRoster, BackendVersion, BackendVersionPosture,
    ClaimCeiling, CompiledPressureWitness, EvaluationSurface, GrammarStanding,
    InterpreterAvailability, MissingTrustEvidence, MutationPoint, MutationVerdict,
    OperatorFamilyRef, ParityStanding, PressureWitnessRefusal, QualificationRefusal, ReadingSource,
    SourceCoordinate, WrapReading, WrapStanding, WrappedBackend,
};
use threadpak_testpak::report::TrialConclusion;

// ---------------------------------------------------------------------------
// The fixture, and the numbers this plane wrote down about it.
// ---------------------------------------------------------------------------

/// One console stream, authored under the published line grammar: a roster
/// line, a qualified baseline line, and three mutant lines — one caught, one
/// missed, one unviable — so a lawful kill exists to read a witness out of and
/// two non-kills stand beside it.
const BACKEND_CONSOLE: &str = "Found 3 mutants to test\n\
    ok Unmutated baseline in 3.1s\n\
    caught src/subject/lane.rs:41:9: replace is_qualified -> bool with true in 4.0s\n\
    missed src/subject/lane.rs:58:13: replace == with != in 4.2s\n\
    unviable src/subject/lane.rs:72:5: replace bounds -> Bounds with the default in 0.6s";

/// The version the party states wrote that text.
const WRITING_VERSION: &str = "25.0.0";

/// A second version, named by a party that checked the adapter's shapes against
/// some other run of the backend.
const OTHER_VERSION: &str = "24.0.0";

/// How many mutants the fixture's roster line announces, stated here rather
/// than counted off the parse.
const DECLARED_ROSTER: u32 = 3;

/// How many of its mutant lines state a mutant the backend's own command
/// rejected.
const DECLARED_KILLS: u32 = 1;

/// How many state a mutant that established nothing: the missed one, whose
/// firing nothing could observe, and the unviable one, which never built.
const DECLARED_INCONCLUSIVES: u32 = 2;

/// How many can state a survivor: none, and structurally so. A console stream
/// carries no channel that could observe a damage firing, so its ceiling admits
/// no survivor at all and a reading carrying one is refused rather than
/// believed.
const DECLARED_SURVIVORS: u32 = 0;

/// The owner this plane's hand-authored evaluation surface is declared under.
const PLANE_NAMESPACE: &str = "testpak.trust-opening";

/// The bytes the surface's one point reads as under the no-mutation mutant.
const ORIGINAL_OPERATION: &[u8] = b"a < b";

/// The damages that point admits, in declared order.
const ADMITTED_ALTERNATIVES: &[&[u8]] = &[b"a <= b", b"a > b"];

// ---------------------------------------------------------------------------
// The two caller-supplied seams, and the values this plane builds.
// ---------------------------------------------------------------------------

/// The origin-graph reading this plane supplies: it answers no coordinate.
///
/// Answering nothing is the published posture for a mapping that is
/// unavailable — the target reports `MappingPosture::OwnerUnmapped` and the
/// lane widens its witness selection — and it decides nothing on the trust
/// road, where no constructor reads a target's owner at all.
fn no_owner(_coordinate: &SourceCoordinate) -> Option<ClaimRef> {
    None
}

/// The operator-family reading this plane supplies: it attributes no damage.
///
/// The bank's families are declared by what they attack and a backend's damage
/// prose is not a family name, so an unanswered lookup produces
/// `FamilyAttribution::OutsideTheBank` rather than a family this plane picked.
fn no_family(_coordinate: &SourceCoordinate, _damage: &[u8]) -> Option<OperatorFamilyRef> {
    None
}

/// One backend version, as the party that ran the backend spells it.
fn version(spelling: &str) -> Result<BackendVersion, ()> {
    BackendVersion::stated(spelling).map_err(|_| ())
}

/// One reading of the fixture text, taken under the version posture a caller
/// states for the run that wrote it.
fn reading(posture: BackendVersionPosture) -> Result<WrapReading, ()> {
    read_output(BACKEND_CONSOLE, posture, no_owner, no_family).map_err(|_| ())
}

/// One reading of the fixture text under a stated backend version.
fn stated_reading(spelling: &str) -> Result<WrapReading, ()> {
    reading(BackendVersionPosture::Stated(version(spelling)?))
}

/// The qualification a reading under this stated version earns.
///
/// The standing is a check against the very version that reading's profile
/// names, which is the one pairing that qualifies anything.
fn earned_qualification(spelling: &str) -> Result<AdapterQualification, ()> {
    let wrote = version(spelling)?;
    let read = reading(BackendVersionPosture::Stated(wrote.clone()))?;
    AdapterQualification::of(&read, GrammarStanding::Checked(wrote)).map_err(|_| ())
}

/// One conforming evaluation surface, hand-authored under the same
/// mutation-point contract a producer emits against.
fn surface() -> Result<EvaluationSurface, ()> {
    let point = MutationPoint::declared(
        MutationPointRef::named(PLANE_NAMESPACE, "the-point").map_err(|_| ())?,
        ClaimRef::named(PLANE_NAMESPACE, "the-claim").map_err(|_| ())?,
        ORIGINAL_OPERATION,
        ADMITTED_ALTERNATIVES,
        ActivationSite::named(PLANE_NAMESPACE, "the-site").map_err(|_| ())?,
    )
    .map_err(|_| ())?;
    EvaluationSurface::conforming(vec![point]).map_err(|_| ())
}

// ---------------------------------------------------------------------------
// The positive control.
// ---------------------------------------------------------------------------

/// The whole trust chain opens over one qualified reading: the reading carries
/// the profile it was read under, that profile earns a qualification, the
/// qualification's own reading shows a witness, and the gate opens on the two
/// of them.
///
/// Load-bearing in its own right. Every refusal below is only evidence because
/// this same road, over this same text, hands back all three values with their
/// seats occupied — a boundary that refused everything would satisfy every
/// hostile assertion in this file and be worthless.
///
/// The census is held against numbers this plane wrote down rather than against
/// the parse, and the roster the backend announced is held against the same
/// number a second time: the two denominators answer different questions, and
/// this fixture was authored so they agree.
#[test]
fn the_trust_chain_opens_over_a_qualified_reading() -> Result<(), ()> {
    let wrote = version(WRITING_VERSION)?;
    let read = reading(BackendVersionPosture::Stated(wrote.clone()))?;
    assert_eq!(read.profile().backend(), WrappedBackend::CargoMutants);
    assert_eq!(read.profile().source(), ReadingSource::ConsoleStream);
    assert_eq!(read.profile().ceiling(), ClaimCeiling::WitnessRejection);
    assert_eq!(
        *read.profile().version(),
        BackendVersionPosture::Stated(wrote.clone())
    );

    let census = read.run().census();
    assert_eq!(census.killed(), DECLARED_KILLS);
    assert_eq!(census.survived(), DECLARED_SURVIVORS);
    assert_eq!(census.inconclusive(), DECLARED_INCONCLUSIVES);
    assert_eq!(census.pressed(), DECLARED_ROSTER);
    assert_eq!(read.announced(), AnnouncedRoster::Stated(DECLARED_ROSTER));

    let standing = GrammarStanding::Checked(wrote);
    let qualification = AdapterQualification::of(&read, standing.clone()).map_err(|_| ())?;
    assert_eq!(qualification.profile(), read.profile());
    assert_eq!(*qualification.standing(), standing);
    assert_eq!(qualification.ceiling(), ClaimCeiling::WitnessRejection);

    let witness = CompiledPressureWitness::shown(WrapStanding::Reported(&read), &qualification)
        .map_err(|_| ())?;
    assert_eq!(witness.qualification(), &qualification);
    assert_eq!(witness.kill().verdict(), MutationVerdict::Killed);

    let conforming = surface()?;
    let parity = ParityStanding::of(&TrialConclusion::Passed);
    let opened = InterpreterAvailability::Available {
        surface: &conforming,
    };
    let answer = availability(Some(&conforming), &qualification, Some(&witness), parity);
    assert_eq!(answer, opened);
    Ok(())
}

// ---------------------------------------------------------------------------
// The planted reversals: the qualification road.
// ---------------------------------------------------------------------------

/// The unchecked standing qualifies nothing, and it is read before the version
/// question is put at all.
///
/// The restored repair is the one the honest bootstrap posture invites: reading
/// "nobody has checked" as a weaker qualification rather than as none. The road
/// hands back no qualification whatever the reading, and it names the STANDING
/// — including over a reading whose profile states no version, where a road
/// that weighed the two facts in the other order would have named the version
/// instead and told a caller to state one.
#[test]
fn the_unchecked_standing_qualifies_nothing() -> Result<(), ()> {
    let stated = stated_reading(WRITING_VERSION)?;
    assert_eq!(
        AdapterQualification::of(&stated, GrammarStanding::Unchecked),
        Err(QualificationRefusal::GrammarUnchecked)
    );

    let unstated = reading(BackendVersionPosture::Unstated)?;
    assert_eq!(
        AdapterQualification::of(&unstated, GrammarStanding::Unchecked),
        Err(QualificationRefusal::GrammarUnchecked)
    );
    Ok(())
}

/// A reading whose profile states no backend version qualifies nothing, however
/// real the check behind the standing was.
///
/// The restored repair is treating the bootstrap version posture as a blank to
/// be filled from the standing: the party checked SOMETHING, so let the reading
/// stand under that. It names nothing the reading stands under, and the road
/// refuses rather than adopting the standing's version as the reading's.
#[test]
fn a_reading_stating_no_version_qualifies_nothing() -> Result<(), ()> {
    let checked = version(WRITING_VERSION)?;
    let unstated = reading(BackendVersionPosture::Unstated)?;
    assert_eq!(
        AdapterQualification::of(&unstated, GrammarStanding::Checked(checked)),
        Err(QualificationRefusal::BackendVersionUnstated)
    );
    Ok(())
}

/// A check made against another version qualifies nothing, and the refusal
/// carries both versions.
///
/// The restored repair is the near-miss: a party really did check these shapes
/// against real output, just not against the run that wrote this text. What was
/// checked is a different version's rendering, so the road refuses — and which
/// two versions disagreed is the whole of that finding, so the assertion is
/// against a refusal this plane built out of the two version VALUES rather than
/// against any sentence somebody rendered.
#[test]
fn a_check_against_another_version_carries_both_versions() -> Result<(), ()> {
    let wrote = version(WRITING_VERSION)?;
    let elsewhere = version(OTHER_VERSION)?;
    let read = reading(BackendVersionPosture::Stated(wrote.clone()))?;
    let disagreement = QualificationRefusal::CheckedAgainstAnotherVersion {
        stated: wrote,
        checked: elsewhere.clone(),
    };
    assert_eq!(
        AdapterQualification::of(&read, GrammarStanding::Checked(elsewhere)),
        Err(disagreement)
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// The planted reversals: the witness road, and the gate.
// ---------------------------------------------------------------------------

/// A witness refuses a qualification taken over another reading's profile, and
/// the very same qualification stands behind the reading it was taken over.
///
/// The restored repair is a qualification borrowed from a neighbour: both
/// readings are of one text, both are qualified, and they differ in exactly the
/// fact a console profile leaves to the caller. The borrowed qualification
/// vouches for the other reading's adapter, so it stands behind nothing here
/// and no witness comes back — while over its own reading it produces one,
/// which is what keeps this a comparison rather than a road that refuses
/// everything.
#[test]
fn a_witness_refuses_another_readings_qualification() -> Result<(), ()> {
    let here = stated_reading(WRITING_VERSION)?;
    let elsewhere = stated_reading(OTHER_VERSION)?;
    let borrowed = earned_qualification(OTHER_VERSION)?;
    assert_ne!(here.profile(), elsewhere.profile());
    assert_eq!(
        CompiledPressureWitness::shown(WrapStanding::Reported(&here), &borrowed),
        Err(PressureWitnessRefusal::QualificationUnderAnotherProfile)
    );
    assert!(
        CompiledPressureWitness::shown(WrapStanding::Reported(&elsewhere), &borrowed).is_ok()
    );
    Ok(())
}

/// The gate refuses a witness shown under another adapter qualification, and it
/// answers with its own comparison rather than with the absent-evidence arm.
///
/// The restored repair is the gate accepting any witness in hand. The witness
/// below is entirely lawful — its own reading qualified it and its own run
/// demonstrated the kill — and it is evidence about that other adapter's
/// reading. Trust is being opened under a different qualification, so the gate
/// names the mismatch; offered the qualification the witness was actually shown
/// under, the same gate over the same surface opens.
#[test]
fn the_gate_refuses_a_witness_shown_under_another_qualification() -> Result<(), ()> {
    let mine = earned_qualification(WRITING_VERSION)?;
    let borrowed = earned_qualification(OTHER_VERSION)?;
    let elsewhere = stated_reading(OTHER_VERSION)?;
    let witness = CompiledPressureWitness::shown(WrapStanding::Reported(&elsewhere), &borrowed)
        .map_err(|_| ())?;
    let conforming = surface()?;
    let parity = ParityStanding::of(&TrialConclusion::Passed);

    let mismatch = InterpreterAvailability::TrustNotOpened {
        missing: MissingTrustEvidence::WitnessUnderAnotherQualification,
    };
    let answer = availability(Some(&conforming), &mine, Some(&witness), parity);
    assert_eq!(answer, mismatch);

    let opened = InterpreterAvailability::Available {
        surface: &conforming,
    };
    let under_its_own = availability(Some(&conforming), &borrowed, Some(&witness), parity);
    assert_eq!(under_its_own, opened);
    Ok(())
}
