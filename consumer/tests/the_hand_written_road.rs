//! The consumer's own trials: the whole authoring road, hand-written, with both
//! ThreadPak crates reached under the names this package chose for them.
//!
//! # What this file is evidence of
//!
//! Every path below is spelled `harness::` or `tp::`. Neither published package
//! name appears anywhere in it, and the stamp's expansion adds none: what
//! `harness::trial_table!` writes into this crate reaches its defining crate
//! through `$crate`, which is the mechanism this file exercises rather than
//! describes. A generated reference that resolved only under a canonical
//! spelling would fail to resolve here, at this seat, rather than in somebody
//! else's tree later.
//!
//! # The road, as a hand walks it
//!
//! A [`harness::descriptor::Row`] is pure descriptor data carrying the
//! hand-written origin; a [`harness::descriptor::ExecutableAttachment`] carries
//! the callable and the two revision bindings this consumer commits to; the
//! public binding constructor marries them in the row slot itself, where its own
//! refusal travels the stamped road's one family. The checks are the harness's
//! property combinators over this crate's public surface — nothing here judges
//! anything the harness does not already know how to judge.
//!
//! # What it does not establish
//!
//! Nothing about a true outsider. This package is a workspace member, so it
//! shares this workspace's resolution and its lint wall; the packaged check that
//! stands an outsider up is the blessing day's.

use harness::descriptor::{
    CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite, Origin,
    PopulationRef, RevisionBinding, Role, Row, SubjectRoute, Tag, TrialTableRefusal,
};
use harness::identity::{ContentAddress, DomainTag};
use harness::properties::{Agreement, PoisonResponse};
use harness::report::{FindingCause, TrialConclusion};
use harness::runner::{Invocation, TrialCall};
use threadpak_consumer::{CountRequest, Lot, LotRefusal};

/// The executable attachment at the two types the engine instantiates.
type TrialAttachment = ExecutableAttachment<Invocation, TrialConclusion>;

// ---------------------------------------------------------------------------
// What this consumer declares about itself.
// ---------------------------------------------------------------------------

/// The owner every reference this consumer spells is declared under.
const CONSUMER: &str = "consumer";

/// The derivation domain this consumer declares for its own revision
/// identities.
///
/// Declared by the home that owns the kind, which here is this test target: the
/// harness's identity substrate knows nothing about what is being named.
const REVISION_TAG: DomainTag = DomainTag::declared("consumer-hand-revision");

/// The subject revision this consumer commits to by hand.
const SUBJECT_REVISION: &[u8] = b"threadpak-consumer/lot-counted/r1";

/// The check revision this consumer commits to by hand, for every check in this
/// file at once.
const CHECK_REVISION: &[u8] = b"threadpak-consumer/checks/r1";

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

/// A request this crate admits.
const A_LAWFUL_REQUEST: CountRequest = CountRequest::stated("north-yard", 12u32);

/// A request stating no lot at all, which the constructor owes a refusal.
const AN_UNLABELLED_REQUEST: CountRequest = CountRequest::stated("", 12u32);

/// A request one item past the ceiling, which the constructor owes a refusal.
const A_REQUEST_PAST_THE_CEILING: CountRequest =
    CountRequest::stated("north-yard", Lot::CEILING.saturating_add(1u32));

/// The cause this consumer cites when its own refusal family's declaration does
/// not close its joins.
const FAMILY_NOT_ADMITTED: FindingCause = FindingCause::named(CONSUMER, "family-not-admitted");

// ---------------------------------------------------------------------------
// The owner-supplied seams: one road, one reading, one equivalence.
// ---------------------------------------------------------------------------

/// The subject under test, as the road the harness's laws take.
fn counted(request: &CountRequest) -> Result<Lot, LotRefusal> {
    Lot::counted(request.label(), request.items())
}

/// This consumer's own reading of what its constructor answered.
///
/// The reading is the owner's because a refusal is spelled in the subject's own
/// vocabulary, and the harness may not read one.
fn answered(outcome: &Result<Lot, LotRefusal>) -> PoisonResponse {
    match *outcome {
        Ok(_) => PoisonResponse::Answered,
        Err(_) => PoisonResponse::Refused,
    }
}

/// This consumer's own declaration of when two outcomes are the same outcome.
///
/// Both seats of a counted lot count, and a refusal is the same outcome only as
/// the same cause.
fn the_same_outcome(left: &Result<Lot, LotRefusal>, right: &Result<Lot, LotRefusal>) -> Agreement {
    let same = match (left, right) {
        (Ok(one), Ok(other)) => one.label() == other.label() && one.items() == other.items(),
        (Err(one), Err(other)) => one == other,
        (Ok(_), Err(_)) | (Err(_), Ok(_)) => false,
    };
    if same {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

// ---------------------------------------------------------------------------
// The checks: thin functions binding this crate's subject to a harness law.
// ---------------------------------------------------------------------------

/// The lawful twin: a request this crate admits comes back as an answer.
fn admits_a_lawful_request(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::admits_lawful(counted, answered, &A_LAWFUL_REQUEST)
}

/// The fail-closed law over the first declared cause: an unlabelled request
/// comes back as a refusal rather than as a lot named by nothing.
fn refuses_an_unlabelled_request(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::fail_closed(counted, answered, &AN_UNLABELLED_REQUEST)
}

/// The fail-closed law over the second declared cause: a request past the
/// ceiling comes back as a refusal rather than as a capped lot.
fn refuses_a_request_past_the_ceiling(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::fail_closed(counted, answered, &A_REQUEST_PAST_THE_CEILING)
}

/// The determinism law: one request, counted twice, is one outcome.
fn counts_the_same_twice(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::determinism_run_twice(counted, the_same_outcome, &A_LAWFUL_REQUEST)
}

/// The machine's own admission road over this consumer's hand-declared refusal
/// family: the declared shape and the declared selection order cohere, and the
/// typed cause order projects onto the textual one.
fn closes_the_family_joins(_invocation: &Invocation) -> TrialConclusion {
    let witness = tp::refusal::admit_order::<LotRefusal>();
    harness::properties::admitted(&witness, FAMILY_NOT_ADMITTED)
}

// ---------------------------------------------------------------------------
// The authoring road: one row and one attachment, per declared trial.
// ---------------------------------------------------------------------------

/// One revision identity this consumer commits to by hand.
///
/// Declared rather than derived: the ceiling is the author's word, and the
/// identity moves when this file says it moved.
fn declared_revision(material: &[u8]) -> RevisionBinding {
    RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, material))
}

/// One hand-written row and the attachment that executes it.
///
/// Every reference is parsed here, so a name that states no owner is refused
/// rather than carried, and the row and the attachment are handed the SAME
/// parsed subject and check — which is the pairing the binding constructor
/// verifies where the two are married.
///
/// # Errors
///
/// Refuses whatever the harness's own constructors refuse, each carried into
/// the stamped road's one family by the discharge that family declares for it:
/// a name that would not parse, a repeated label, or a row whose origin and
/// admission do not cohere.
fn hand_parts(
    claim_stem: &'static str,
    suite_stem: &'static str,
    subject_stem: &'static str,
    check_stem: &'static str,
    population_stem: &'static str,
    call: TrialCall,
) -> Result<(Row, TrialAttachment), TrialTableRefusal> {
    let subject = SubjectRoute::named(CONSUMER, subject_stem)?;
    let check = CheckRef::named(CONSUMER, check_stem)?;
    let row = Row::declared(
        ClaimRef::named(CONSUMER, claim_stem)?,
        ExecutionSuite::named(CONSUMER, suite_stem)?,
        Classification::authored(
            vec![Role::named(CONSUMER, "smoke")?],
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

// ---------------------------------------------------------------------------
// The stamp: one declaration, one complete world, one seat per suite.
// ---------------------------------------------------------------------------

harness::trial_table! {
    /// The complete world this consumer authored by hand.
    mod authored_world named("consumer", "the-hand-written-world") {
        provenance: unproduced,
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

        suite construction named("consumer", "construction") {
            a_lawful_request_is_counted: {
                let (row, attachment) = crate::hand_parts(
                    "a-lawful-request-is-counted",
                    "construction",
                    "lot-counted",
                    "admits-lawful",
                    "the-lawful-request",
                    crate::admits_a_lawful_request,
                )?;
                harness::descriptor::Binding::bound(
                    row,
                    attachment,
                    harness::descriptor::Provenance::Unproduced,
                )
            },
            an_unlabelled_request_is_refused: {
                let (row, attachment) = crate::hand_parts(
                    "an-unlabelled-request-is-refused",
                    "construction",
                    "lot-counted",
                    "fail-closed-unlabelled",
                    "the-unlabelled-request",
                    crate::refuses_an_unlabelled_request,
                )?;
                harness::descriptor::Binding::bound(
                    row,
                    attachment,
                    harness::descriptor::Provenance::Unproduced,
                )
            },
            a_request_past_the_ceiling_is_refused: {
                let (row, attachment) = crate::hand_parts(
                    "a-request-past-the-ceiling-is-refused",
                    "construction",
                    "lot-counted",
                    "fail-closed-over-limit",
                    "the-request-past-the-ceiling",
                    crate::refuses_a_request_past_the_ceiling,
                )?;
                harness::descriptor::Binding::bound(
                    row,
                    attachment,
                    harness::descriptor::Provenance::Unproduced,
                )
            },
        }

        suite ambient_freedom named("consumer", "ambient-freedom") {
            counting_is_deterministic: {
                let (row, attachment) = crate::hand_parts(
                    "counting-is-deterministic",
                    "ambient-freedom",
                    "lot-counted",
                    "determinism-run-twice",
                    "the-lawful-request",
                    crate::counts_the_same_twice,
                )?;
                harness::descriptor::Binding::bound(
                    row,
                    attachment,
                    harness::descriptor::Provenance::Unproduced,
                )
            },
        }

        suite declaration named("consumer", "declaration") {
            the_refusal_family_closes_its_joins: {
                let (row, attachment) = crate::hand_parts(
                    "the-refusal-family-closes-its-joins",
                    "declaration",
                    "lot-refusal-family",
                    "family-admission",
                    "the-declared-family",
                    crate::closes_the_family_joins,
                )?;
                harness::descriptor::Binding::bound(
                    row,
                    attachment,
                    harness::descriptor::Provenance::Unproduced,
                )
            },
        }
    }
}
