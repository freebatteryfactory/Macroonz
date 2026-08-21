//! The consumer's own trials over the family it DERIVED, delivered by the
//! producer: the trial rows this crate's declaration states, carried across the
//! wall inside a generated support shell and invoked here by the name that
//! declaration chose.
//!
//! # What this file is evidence of
//!
//! The first complete generated row road. One declaration in this crate's
//! library states its own trial rows; the derive reads them, plans a carrier over
//! them, renders one exported shell and one caller-named alias, and pins the
//! delivery against the harness's published generated-support schema identity.
//! This target invokes that alias, supplies the facts only it holds, and runs the
//! rows through the same engine the hand-written road runs through.
//!
//! Every path below into either ThreadPak crate is spelled `harness::` or `tp::`,
//! and the expansion adds none: what the shell writes reaches the harness through
//! the binding this file passes at the invocation, and what the deferred cargo
//! writes reaches the machine through the binding the DECLARATION stated. A
//! generated reference that resolved only under a canonical spelling would fail
//! to resolve at this seat rather than in somebody else's tree later.
//!
//! # What this target supplies, and why it is this target that supplies it
//!
//! The declaration states descriptor MEANING and stops. What is missing from it
//! is everything a producer cannot honestly hold:
//!
//! - the two REVISION commitments each row's subject and check are bound under,
//!   which are this consumer's own word about its own code;
//! - the CALLABLE that reaches each row's conclusion, which is written in THIS
//!   crate and has no crate binding a rendered path could be rooted at;
//! - the declared BUDGETS, because a producer that wrote them would be declaring
//!   how long somebody else's machine may spend;
//! - the TARGET and toolchain, because nothing in the harness derives either and
//!   a guess would enter a cache key;
//! - the CLOCK, because what a nanosecond reading is worth is a fact about this
//!   host.
//!
//! The carrier's matcher names every one of them, so a delivery short one
//! attachment does not match the carrier at all rather than expanding into a row
//! nothing runs.
//!
//! # What it does not establish
//!
//! Nothing about a true outsider. This package is a workspace member, so it
//! shares this workspace's resolution and its lint wall; the packaged check that
//! stands an outsider up is the blessing day's.

use harness::descriptor::{
    CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite, Origin,
    PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute, Tag, TrialTableRefusal,
};
use harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use harness::report::{FindingCause, TrialConclusion};
use harness::runner::{Invocation, TrialCall};
use threadpak_consumer::{CountRequest, Lot};

/// The executable attachment at the two types the engine instantiates.
type TrialAttachment = ExecutableAttachment<Invocation, TrialConclusion>;

// ---------------------------------------------------------------------------
// What this consumer declares about itself.
// ---------------------------------------------------------------------------

/// The owner every reference this consumer spells is declared under.
const CONSUMER: &str = "consumer";

/// The derivation domain this consumer declares for the revision identities it
/// commits the GENERATED road's rows to.
///
/// Its own, and not either hand road's: three files committing to revisions of
/// three different subjects under one domain derive addresses nobody can tell
/// apart.
const REVISION_TAG: DomainTag = DomainTag::declared(
    "consumer-generated-revision",
    IdentityProfileVersion::declared(1),
);

/// The subject revision this consumer commits to by hand.
const SUBJECT_REVISION: &[u8] = b"threadpak-consumer/lot-merged/r1";

/// The check revision this consumer commits to by hand, for every check in this
/// file at once.
const CHECK_REVISION: &[u8] = b"threadpak-consumer/generated-checks/r1";

/// The compilation target this seat's runs are DECLARED to stand on.
///
/// Declared rather than read: nothing in the harness derives a triple, and a
/// triple assembled out of predicates would be a plausible spelling entering a
/// cache key. A run on another target therefore runs under a declaration that
/// does not describe it, which costs a cache key nothing verified — never a
/// verdict.
const DECLARED_TARGET_TRIPLE: &str = "x86_64-pc-windows-msvc";

/// The toolchain this seat's runs are DECLARED to stand on: the channel the
/// workspace pins, stated here because nothing in the harness can read one.
const DECLARED_TOOLCHAIN: &str = "1.97.1";

/// A pair naming two different lots, which merging owes a refusal.
const MISMATCHED_LOTS: MergeRequest = MergeRequest {
    left: CountRequest::stated("north-yard", 1u32),
    right: CountRequest::stated("south-yard", 1u32),
};

/// A pair of one lot whose counts add up one past the ceiling, which merging
/// owes a refusal.
const OVER_LIMIT_PAIR: MergeRequest = MergeRequest {
    left: CountRequest::stated("north-yard", Lot::CEILING),
    right: CountRequest::stated("north-yard", 1u32),
};

/// The cause this consumer cites when the generated road and the hand road do
/// not state one trial.
const NOT_ONE_TRIAL: FindingCause =
    FindingCause::named(CONSUMER, "generated-row-is-not-the-hand-row");

// ---------------------------------------------------------------------------
// The owner-supplied seams.
// ---------------------------------------------------------------------------

/// What one caller states when it asks for two lots to be merged.
///
/// Unparsed on purpose, exactly as one count request is: whether either half
/// names a lawful lot is the constructor's question rather than this value's, and
/// stating the pair as REQUESTS is what lets the subject road below be total.
#[derive(Debug, Clone, Copy)]
struct MergeRequest {
    /// The count on the left of the merge.
    left: CountRequest,
    /// The count on the right of it.
    right: CountRequest,
}

/// What merging one stated pair did.
///
/// Three arms rather than two, because the road carries two refusals and they are
/// different facts about the subject: a request this crate could not COUNT never
/// reached a merge, and folding it into the merge's own refusal would report a
/// counting defect as a merging one.
///
/// # Bounds
///
/// No arm carries a payload, and the absence is the honest shape rather than a
/// dropped fact. What a check needs of this value is which ROAD answered, and the
/// arm is that; the merged lot and the two typed refusals are read by nothing
/// here, and a seat nobody reads is a seat that reads as evidence and is not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MergeOutcome {
    /// The two counts merged into the one count they add up to.
    Merged,
    /// One half of the pair is not a lawful lot.
    NotCounted,
    /// Both halves are lawful lots and merging them refused.
    NotMerged,
}

/// Merging one stated pair, as the road the harness's laws take.
///
/// TOTAL, which is what keeps a check from having to unwrap anything: every
/// request reaches an outcome, and which of the three it reaches is the subject's
/// answer rather than this file's.
fn merged(request: &MergeRequest) -> MergeOutcome {
    let left = Lot::counted(request.left.label(), request.left.items());
    let right = Lot::counted(request.right.label(), request.right.items());
    match (left, right) {
        (Ok(left), Ok(right)) => match left.merged(right) {
            Ok(_) => MergeOutcome::Merged,
            Err(_) => MergeOutcome::NotMerged,
        },
        (Err(_), _) | (Ok(_), Err(_)) => MergeOutcome::NotCounted,
    }
}

/// This consumer's own reading of what merging answered.
///
/// The reading is the owner's because a refusal is spelled in the subject's own
/// vocabulary, and the harness may not read one. Both refusing arms read as a
/// refusal, and they stay two arms on the value so a reader of a finding knows
/// which road refused.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "the shape is the harness's seam rather than this file's choice: a poison reading is `fn(&Outcome) -> PoisonResponse`, and a by-value reading is not a value that seat accepts"
)]
fn answered(outcome: &MergeOutcome) -> harness::properties::PoisonResponse {
    match *outcome {
        MergeOutcome::Merged => harness::properties::PoisonResponse::Answered,
        MergeOutcome::NotCounted | MergeOutcome::NotMerged => {
            harness::properties::PoisonResponse::Refused
        }
    }
}

// ---------------------------------------------------------------------------
// The checks the generated rows point at.
//
// They live HERE, in the target that runs them, which is the whole reason the
// attachment is supplied at the invocation: a check function written in a test
// target is not reachable from the crate the declaration sits in, and no
// rendered path could name it.
// ---------------------------------------------------------------------------

/// The fail-closed law over the first declared cause: two counts naming
/// different lots come back as a refusal rather than as a merged lot.
fn mismatched_lots_refuse(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::fail_closed(merged, answered, &MISMATCHED_LOTS)
}

/// The fail-closed law over the second declared cause: two counts of one lot
/// that add up past the ceiling come back as a refusal rather than as a capped
/// lot.
fn merged_count_past_limit_refuses(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::fail_closed(merged, answered, &OVER_LIMIT_PAIR)
}

// ---------------------------------------------------------------------------
// The revision commitments this target makes about its own code.
// ---------------------------------------------------------------------------

/// One revision identity this consumer commits to by hand.
///
/// Declared rather than derived: the ceiling is the author's word, and the
/// identity moves when this file says it moved.
fn declared_revision(material: &[u8]) -> RevisionBinding {
    RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, material))
}

// ---------------------------------------------------------------------------
// The generated delivery.
//
// One invocation of the alias the declaration chose. Everything inside it is a
// fact this target holds and the producer does not.
// ---------------------------------------------------------------------------

threadpak_consumer::merge_refusal_trials! {
    harness: harness,

    invocation: harness::report::InvocationProfile::declared(
        harness::report::CaseBudget::declared(1u32),
        harness::report::ByteBudget::declared(64u64),
        harness::report::TimeBudget::declared(1_000_000_000u64),
    ),

    target: harness::report::TargetBinding::bound(
        harness::report::TargetTriple::declared(crate::DECLARED_TARGET_TRIPLE),
        harness::report::ToolchainIdentity::declared(crate::DECLARED_TOOLCHAIN),
    ),

    clock: harness::runner::HostClock::unmeasured(),

    attachments: {
        mismatched_lots_refuse {
            subject_revision: crate::declared_revision(crate::SUBJECT_REVISION),
            check_revision: crate::declared_revision(crate::CHECK_REVISION),
            call: crate::mismatched_lots_refuse,
        },
        merged_count_past_limit_refuses {
            subject_revision: crate::declared_revision(crate::SUBJECT_REVISION),
            check_revision: crate::declared_revision(crate::CHECK_REVISION),
            call: crate::merged_count_past_limit_refuses,
        },
    },
}

// ---------------------------------------------------------------------------
// The parity: one trial, two roads.
// ---------------------------------------------------------------------------

/// The hand-written twin of one generated row: the same four coordinates, stated
/// by a hand rather than by a producer.
///
/// The origin is the HAND's and that is the honest word for it — a person wrote
/// this row. What the pair earns is stated at the seat below: two roads that
/// state one TRIAL and disagree about who wrote it.
///
/// # Errors
///
/// Refuses whatever the harness's own constructors refuse, each carried into the
/// stamped road's one family by the discharge that family declares for it.
fn hand_twin(
    claim_stem: &'static str,
    check_stem: &'static str,
    population_stem: &'static str,
    call: TrialCall,
) -> Result<(Row, TrialAttachment), TrialTableRefusal> {
    let subject = SubjectRoute::named(CONSUMER, "lot-merged")?;
    let check = CheckRef::named(CONSUMER, check_stem)?;
    let row = Row::declared(
        ClaimRef::named(CONSUMER, claim_stem)?,
        ExecutionSuite::named(CONSUMER, "construction")?,
        Classification::authored(
            vec![Role::named(CONSUMER, "hand-twin")?],
            vec![Tag::named(CONSUMER, "hand-written")?],
        )?,
        subject,
        check,
        PopulationRef::named(CONSUMER, population_stem)?,
        Origin::HandWritten,
    )?;
    let attachment = ExecutableAttachment::attached(
        subject,
        check,
        declared_revision(SUBJECT_REVISION),
        declared_revision(CHECK_REVISION),
        call,
    );
    Ok((row, attachment))
}

/// The generated table's bindings, read off the module the stamp wrote.
///
/// # Errors
///
/// Refuses exactly as the stamped table's own construction refuses: the road
/// this reads is the one the seats run, so a table a seat could not build is a
/// table this reading cannot either.
fn generated_world() -> Result<harness::runner::TrialTable, TrialTableRefusal> {
    generated_merge_refusal_trials::table()
}

/// Two roads over one trial state the same TRIAL and different ORIGINS.
///
/// # What the pair establishes
///
/// A trial key is derived over the four coordinates a row states about itself —
/// the claim, the subject, the check, and the population — and over nothing else.
/// So the generated row and the hand row below are ONE trial by the harness's own
/// derivation, while their classifications, their origins, and their provenances
/// are three facts they are entitled to disagree about.
///
/// It is the strongest statement this pairing can make, and the fact that the row
/// REVISION identities are allowed to differ is part of it: a revision identity
/// is derived over the whole row, origin and classification included, so
/// requiring those to agree would be requiring the producer to have written what
/// a hand wrote.
#[test]
fn a_generated_row_and_a_hand_row_state_one_trial() -> Result<(), TrialTableRefusal> {
    let world = generated_world()?;
    let (hand, _) = hand_twin(
        "mismatched-lots-refuse",
        "fail-closed-mismatched",
        "mismatched-lots",
        mismatched_lots_refuse,
    )?;
    let generated = world
        .bindings()
        .iter()
        .find(|binding| binding.trial_key() == hand.trial_key());
    let Some(generated) = generated else {
        return Err(TrialTableRefusal::NameNotParsed(
            harness::descriptor::NameRefusal::EmptyStem,
        ));
    };
    assert_eq!(generated.row().coordinates(), hand.coordinates());
    assert_ne!(generated.row().origin(), hand.origin());
    assert!(matches!(generated.row().origin(), Origin::Generated(_)));
    assert!(matches!(hand.origin(), Origin::HandWritten));
    assert!(matches!(
        generated.provenance(),
        Provenance::Produced { .. }
    ));
    Ok(())
}

/// The generated table states exactly the rows the declaration declared, under
/// the producer's own provenance.
///
/// The count is read off the world rather than asserted against a number written
/// twice: what a declaration states is the declaration's fact, and a lane that
/// spelled the number again would be a second authority over it. What this seat
/// establishes is that the two roads AGREE on it — every binding the world holds
/// carries a produced provenance and a generated origin, which is the whole of
/// what "a producer emitted this table" means.
#[test]
fn every_generated_binding_carries_the_producers_own_act() -> Result<(), TrialTableRefusal> {
    let world = generated_world()?;
    assert!(matches!(world.provenance(), Provenance::Produced { .. }));
    for binding in world.bindings() {
        assert!(matches!(binding.row().origin(), Origin::Generated(_)));
        assert!(matches!(binding.provenance(), Provenance::Produced { .. }));
    }
    assert_eq!(
        world.name(),
        harness::descriptor::AuthoredTableName::named(CONSUMER, "merge-refusal-trials")?
    );
    Ok(())
}

/// One check reached through the generated attachment and through a direct call
/// concludes the same thing.
///
/// The two roads to one conclusion are the point: the generated row carries a
/// function POINTER this file supplied, and a pointer that reached a different
/// function would conclude about a different subject. The invocation is the
/// stamped module's own, so the facts the trial runs under are the ones the
/// delivery carries rather than ones this seat composed.
#[test]
fn the_generated_attachment_reaches_this_targets_own_check() -> Result<(), TrialTableRefusal> {
    let world = generated_world()?;
    let invocation = Invocation::declared(
        generated_merge_refusal_trials::INVOCATION,
        generated_merge_refusal_trials::target(),
        harness::report::TrialSite::located(
            core::module_path!(),
            core::file!(),
            core::line!(),
            "the_generated_attachment_reaches_this_targets_own_check",
        ),
        generated_merge_refusal_trials::CLOCK,
    );
    let (hand, _) = hand_twin(
        "mismatched-lots-refuse",
        "fail-closed-mismatched",
        "mismatched-lots",
        mismatched_lots_refuse,
    )?;
    let Some(binding) = world
        .bindings()
        .iter()
        .find(|binding| binding.trial_key() == hand.trial_key())
    else {
        return Err(TrialTableRefusal::NameNotParsed(
            harness::descriptor::NameRefusal::EmptyStem,
        ));
    };
    let through_the_delivery = harness::runner::run_one(binding, &invocation);
    let directly = mismatched_lots_refuse(&invocation);
    assert_eq!(
        *through_the_delivery.attempt(),
        harness::report::RunAttempt::Executed(directly)
    );
    let _ = NOT_ONE_TRIAL;
    Ok(())
}
