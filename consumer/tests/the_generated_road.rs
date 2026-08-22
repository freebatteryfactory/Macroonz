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
    PopulationRef, Provenance, RevisionBinding, Role, Row, SubjectRoute, Tag, TrialKey,
    TrialTableRefusal,
};
use harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use harness::report::{OutcomeClass, TrialConclusion, TrialId};
use harness::runner::{Invocation, TrialBinding, TrialCall, TrialTable};
use std::collections::BTreeMap;
use std::fmt;
use threadpak_consumer::{CountRequest, Lot};

/// The executable attachment at the two types the engine instantiates.
type TrialAttachment = ExecutableAttachment<Invocation, TrialConclusion>;

enum GeneratedRoadFailure {
    Harness(TrialTableRefusal),
    MissingGeneratedTwin { trial: TrialKey },
}

impl fmt::Debug for GeneratedRoadFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Harness(refusal) => formatter.debug_tuple("Harness").field(refusal).finish(),
            Self::MissingGeneratedTwin { trial } => formatter
                .debug_struct("MissingGeneratedTwin")
                .field("trial", trial)
                .finish(),
        }
    }
}

impl From<TrialTableRefusal> for GeneratedRoadFailure {
    fn from(refusal: TrialTableRefusal) -> Self {
        Self::Harness(refusal)
    }
}

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
const CHECK_REVISION: &[u8] = b"threadpak-consumer/generated-checks/r2";

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
const DECLARED_TOOLCHAIN: &str = "1.98.0";

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

/// A pair of lawful lots whose counts fit under the merge ceiling.
const LAWFUL_MERGE_PAIR: MergeRequest = MergeRequest {
    left: CountRequest::stated("north-yard", 1u32),
    right: CountRequest::stated("north-yard", 1u32),
};

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
    reason = "must inhabit ResponseReading<MergeOutcome> = fn(&MergeOutcome) -> PoisonResponse; changing this parameter to MergeOutcome changes the declared seam, and THE_READING below is where rustc refuses it rather than where this sentence is believed"
)]
fn answered(outcome: &MergeOutcome) -> harness::properties::PoisonResponse {
    match *outcome {
        MergeOutcome::Merged => harness::properties::PoisonResponse::Answered,
        MergeOutcome::NotCounted | MergeOutcome::NotMerged => {
            harness::properties::PoisonResponse::Refused
        }
    }
}

/// The reading above, seated at the type the harness's law declares.
///
/// # Why this constant exists
///
/// It is the WITNESS for the exception on [`answered`]. A `#[expect]` whose
/// justification lives only in its own reason string is a sentence a reader has
/// to believe; this constant makes the compiler the proof. `ResponseReading` is a
/// function-POINTER alias, so a by-value parameter does not inhabit it — change
/// `answered` to take `MergeOutcome` and rustc refuses HERE, with the seam named,
/// rather than the lint quietly becoming right and the reason quietly becoming
/// false.
///
/// That is the standing shape for every exception whose ground is a
/// rustc-enforced contract: name the contract in the reason, and put a value
/// somewhere load-bearing that only compiles while the reason is true.
const THE_READING: harness::properties::ResponseReading<MergeOutcome> = answered;

// ---------------------------------------------------------------------------
// The checks the generated rows point at.
//
// They live HERE, in the target that runs them, which is the whole reason the
// attachment is supplied at the invocation: a check function written in a test
// target is not reachable from the crate the declaration sits in, and no
// rendered path could name it.
// ---------------------------------------------------------------------------

/// Two counts naming different lots come back refusal-shaped rather than answer-shaped.
fn mismatched_lots_refuse(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::fail_closed(merged, THE_READING, &MISMATCHED_LOTS)
}

/// Two counts of one lot that add up past the ceiling come back refusal-shaped rather than answer-shaped.
fn merged_count_past_limit_refuses(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::fail_closed(merged, THE_READING, &OVER_LIMIT_PAIR)
}

/// Two lawful lots that fit under the ceiling come back answer-shaped rather than refusal-shaped.
fn lawful_lots_merge(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::admits_lawful(merged, THE_READING, &LAWFUL_MERGE_PAIR)
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

    clock: harness::clock::HarnessClock::unavailable(),

    attachments: {
        lawful_lots_merge {
            subject_revision: crate::declared_revision(crate::SUBJECT_REVISION),
            check_revision: crate::declared_revision(crate::CHECK_REVISION),
            call: crate::lawful_lots_merge,
        },
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
fn generated_world() -> Result<TrialTable, TrialTableRefusal> {
    generated_merge_refusal_trials::table()
}

fn generated_twin(
    world: &TrialTable,
    trial: TrialKey,
) -> Result<&TrialBinding, GeneratedRoadFailure> {
    world
        .bindings()
        .iter()
        .find(|binding| binding.trial_key() == trial)
        .ok_or(GeneratedRoadFailure::MissingGeneratedTwin { trial })
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
fn a_generated_row_and_a_hand_row_state_one_trial() -> Result<(), GeneratedRoadFailure> {
    let world = generated_world()?;
    let (hand, _) = hand_twin(
        "mismatched-lots-refuse",
        "fail-closed-mismatched",
        "mismatched-lots",
        mismatched_lots_refuse,
    )?;
    let generated = generated_twin(&world, hand.trial_key())?;
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

/// A table's emitter and one binding's emitter are independent facts.
///
/// A producer may assemble a table containing a hand-written row it did not emit. The table truthfully names the producer of the assembly while the binding truthfully remains unproduced; neither seat speaks for the other.
#[test]
fn a_produced_table_may_hold_an_unproduced_hand_binding() -> Result<(), TrialTableRefusal> {
    let generated = generated_world()?;
    let (row, attachment) = hand_twin(
        "mismatched-lots-refuse",
        "mismatched-lots-refuse",
        "mismatched-lots",
        mismatched_lots_refuse,
    )?;
    let binding = harness::descriptor::Binding::bound(row, attachment, Provenance::Unproduced)?;
    let world = harness::descriptor::AuthoredTable::authored(
        harness::descriptor::AuthoredTableName::named(CONSUMER, "produced-mixed-emitter-world")?,
        generated.provenance(),
        vec![binding],
    )
    .map_err(TrialTableRefusal::TableNotAuthored)?;

    assert!(matches!(world.provenance(), Provenance::Produced { .. }));
    assert!(matches!(
        world
            .bindings()
            .first()
            .map(harness::descriptor::Binding::provenance),
        Some(Provenance::Unproduced)
    ));
    Ok(())
}

/// One selection over the generated world and its hand-written twin selects the
/// same semantic trials and reaches the same normalized outcomes.
///
/// # What this seat adds over the key comparison
///
/// That two roads state one TRIAL is a fact about a derivation. That one
/// SELECTION reaches the same semantic trials through both is a fact about a RUN,
/// and it is the one a consumer actually depends on:
/// a generated row whose suite, whose semantic coordinates, or whose binding
/// differed could leave the intended trial unselected or record another trial or outcome.
///
/// Both worlds run through the same engine, under the same invocation, against
/// the same one-suite selection. The generated world's census and the hand
/// world's complete censuses agree on the semantic trials and their normalized
/// outcomes. Origins, classifications, provenances, and row revisions remain
/// truthful properties of their own roads and are deliberately not compared.
#[test]
fn one_selection_reaches_same_trials_down_both_roads() -> Result<(), TrialTableRefusal> {
    let invocation = Invocation::declared(
        generated_merge_refusal_trials::INVOCATION,
        generated_merge_refusal_trials::target(),
        harness::report::TrialSite::located(
            core::module_path!(),
            core::file!(),
            core::line!(),
            "one_selection_reaches_same_trials_down_both_roads",
        ),
        generated_merge_refusal_trials::CLOCK,
    );
    let selection =
        harness::runner::SelectionPlan::of(harness::runner::Selection::ByExecutionSuite(
            std::collections::BTreeSet::from([ExecutionSuite::named(CONSUMER, "construction")?]),
        ));

    let generated = generated_world()?;
    let lawful_twin = hand_twin(
        "lawful-lots-merge",
        "admits-lawful-merge",
        "lawful-merge-pair",
        lawful_lots_merge,
    )?;
    let mismatched_twin = hand_twin(
        "mismatched-lots-refuse",
        "fail-closed-mismatched",
        "mismatched-lots",
        mismatched_lots_refuse,
    )?;
    let over_limit_twin = hand_twin(
        "merged-count-past-limit-refuses",
        "fail-closed-over-limit",
        "over-limit-pair",
        merged_count_past_limit_refuses,
    )?;
    let hand = harness::descriptor::AuthoredTable::authored(
        harness::descriptor::AuthoredTableName::named(CONSUMER, "the-hand-twin-world")?,
        Provenance::Unproduced,
        vec![
            harness::descriptor::Binding::bound(
                lawful_twin.0,
                lawful_twin.1,
                Provenance::Unproduced,
            )?,
            harness::descriptor::Binding::bound(
                mismatched_twin.0,
                mismatched_twin.1,
                Provenance::Unproduced,
            )?,
            harness::descriptor::Binding::bound(
                over_limit_twin.0,
                over_limit_twin.1,
                Provenance::Unproduced,
            )?,
        ],
    )
    .map_err(TrialTableRefusal::TableNotAuthored)?;

    let down_the_generated_road =
        harness::runner::run_all(&generated.view(), &selection, &invocation);
    let down_the_hand_road = harness::runner::run_all(&hand.view(), &selection, &invocation);

    let generated_outcomes: BTreeMap<TrialId, OutcomeClass> = down_the_generated_road
        .census()
        .iter()
        .map(|accounting| (accounting.trial(), accounting.disposition().outcome()))
        .collect();
    let hand_outcomes: BTreeMap<TrialId, OutcomeClass> = down_the_hand_road
        .census()
        .iter()
        .map(|accounting| (accounting.trial(), accounting.disposition().outcome()))
        .collect();

    assert_eq!(
        down_the_generated_road.selection(),
        harness::report::SelectionOutcome::Satisfied
    );
    assert_eq!(
        down_the_generated_road.selection(),
        down_the_hand_road.selection()
    );
    assert_eq!(
        down_the_generated_road.denominator(),
        down_the_hand_road.denominator()
    );
    assert_eq!(generated_outcomes, hand_outcomes);

    // The aggregate verdict remains the intentionally coarse reading over each
    // run after the per-trial parity comparison above.
    assert!(harness::runner::seat_verdict(&down_the_generated_road).is_ok());
    assert!(harness::runner::seat_verdict(&down_the_hand_road).is_ok());
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
fn the_generated_attachment_reaches_this_targets_own_check() -> Result<(), GeneratedRoadFailure> {
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
    let binding = generated_twin(&world, hand.trial_key())?;
    let through_the_delivery = harness::runner::run_one(binding, &invocation);
    let directly = mismatched_lots_refuse(&invocation);
    assert_eq!(
        *through_the_delivery.attempt(),
        harness::report::RunAttempt::Executed(directly)
    );
    Ok(())
}
