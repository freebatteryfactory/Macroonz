//! The consumer's own trials over the family it DERIVED: the declaration-site
//! surface the derive delivers into an ordinary build, read back as compiled
//! values.
//!
//! # What this file is evidence of
//!
//! One public, fully documented refusal-family declaration went through the
//! derive in this crate's library, and what came back at the declaration site is
//! compiled here and read as values. Every path into either ThreadPak crate is
//! spelled `harness::` or `tp::`, and the derived implementations spell the
//! machine `tp::` as well — the declaration states the binding, and the
//! rendering follows it — so an implementation that resolved only under a
//! canonical spelling would fail to resolve at this seat rather than in somebody
//! else's tree later.
//!
//! # What is read, and how
//!
//! Values, and never text. The declared body shape, the typed cause order with
//! every cause's family identity, local key, Rust spelling, and position, and
//! the machine's own admission road over the whole declaration — each one a
//! compiled constant, compared against what this crate's declaration states,
//! under an equivalence this consumer declares. Nothing here inspects a
//! rendering, scans a spelling, or reads a projection back.
//!
//! # The parity this crate's pair earns
//!
//! [`declared_facts`] is generic in the contracts, so the family realized by
//! hand and the family derived from a declaration stand in one seat: both
//! satisfy `tp::refusal::RefusalFamily`, both satisfy
//! `tp::refusal::CauseOrderDeclaration`, and both are read by one road taken
//! twice rather than by two roads that happen to agree. The two rows that take
//! it also settle what the two families' shared word buys: each declares a cause
//! under the local key `over-limit`, and the two causes are different identities
//! because their family seats differ.
//!
//! # What this file does not establish
//!
//! The declaration site is the whole of what it reads. The same declaration also
//! delivers trial rows and evaluation support into a CONSUMPTION target, and
//! that delivery is `tests/the_generated_road.rs`'s to invoke and to observe —
//! no seat below reaches for it, so what this file establishes stays exactly
//! what an ordinary build compiles.
//!
//! Nothing about a true outsider, either. This package is a workspace member, so
//! it shares this workspace's resolution and its lint wall; the packaged check
//! that stands an outsider up is the blessing day's.

use harness::descriptor::{
    CheckRef, ClaimRef, Classification, ExecutableAttachment, ExecutionSuite, Origin,
    PopulationRef, RevisionBinding, Role, Row, SubjectRoute, Tag, TrialTableRefusal,
};
use harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use harness::properties::Agreement;
use harness::report::{FindingCause, TrialConclusion};
use harness::runner::{Invocation, TrialCall};
use threadpak_consumer::{LotRefusal, MergeRefusal};

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
/// Its own, and not the hand road's: two files committing to revisions of two
/// different subjects under one domain derive addresses nobody can tell apart.
const REVISION_TAG: DomainTag = DomainTag::declared(
    "consumer-derived-revision",
    IdentityProfileVersion::declared(1),
);

/// The subject revision this consumer commits to by hand.
const SUBJECT_REVISION: &[u8] = b"threadpak-consumer/refusal-family-declarations/r1";

/// The check revision this consumer commits to by hand, for every check in this
/// file at once.
const CHECK_REVISION: &[u8] = b"threadpak-consumer/derived-checks/r1";

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

// ---------------------------------------------------------------------------
// What a family declares about itself, as one value read through the contracts.
// ---------------------------------------------------------------------------

/// The declared facts one refusal family carries: the body shape it takes, and
/// the typed canonical order it declares over its causes.
///
/// One value rather than two loose readings, because the two are one
/// declaration: a shape read from one family beside an order read from another
/// is a pair nothing in the types keeps honest, and the rows below compare
/// whole declarations for exactly that reason.
#[derive(Debug, Clone, Copy)]
struct DeclaredFacts {
    /// The body shape the family declares.
    shape: tp::refusal::FamilyShape,
    /// The canonical order the family declares over its causes, by stable
    /// identity.
    order: tp::refusal::DeclaredCauseOrder,
}

/// One family's declared facts, read through the contracts rather than off a
/// type.
///
/// Generic on purpose: the bound is the parity statement. A family realized by
/// hand and a family derived from a declaration both stand in this seat, so
/// what the rows below take is one road taken twice.
fn declared_facts<Family: tp::refusal::CauseOrderDeclaration>() -> DeclaredFacts {
    DeclaredFacts {
        shape: Family::SHAPE,
        order: Family::DECLARED_ORDER,
    }
}

// ---------------------------------------------------------------------------
// What this crate's two declarations state, written out as typed values.
// ---------------------------------------------------------------------------

/// The identity the derived family's declaration states.
const MERGE_FAMILY: tp::refusal::RefusalFamilyId =
    tp::refusal::RefusalFamilyId::declared("consumer.lot-merge");

/// The identity the hand-realized family's declaration states.
const LOT_FAMILY: tp::refusal::RefusalFamilyId =
    tp::refusal::RefusalFamilyId::declared("consumer.lot");

/// The cause rows the DERIVED family's declaration states, written out here
/// from exactly the facts that declaration carries: the family identity, each
/// cause's local key, each cause's Rust spelling, and the order they stand in.
///
/// Written by this file rather than read off the derived implementation,
/// because a reading taken from the value under test compares it to itself. A
/// derivation that minted an identity under another family, joined a prefix
/// into one string, dropped a cause, or ordered the rows differently answers
/// with a value that differs from these.
const THE_DERIVED_ROWS: &[tp::refusal::DeclaredCause] = &[
    tp::refusal::DeclaredCause::declared(
        tp::refusal::CauseId::declared(
            MERGE_FAMILY,
            tp::refusal::LocalCauseKey::declared("not-the-same-lot"),
        ),
        "NotTheSameLot",
    ),
    tp::refusal::DeclaredCause::declared(
        tp::refusal::CauseId::declared(
            MERGE_FAMILY,
            tp::refusal::LocalCauseKey::declared("over-limit"),
        ),
        "OverLimit",
    ),
];

/// The cause rows the HAND-REALIZED family's declaration states, on the same
/// terms.
///
/// Its second row carries the local key the derived family's second row
/// carries, and the two are different causes: the key is a word, and the family
/// seat beside it is the ownership.
const THE_HAND_ROWS: &[tp::refusal::DeclaredCause] = &[
    tp::refusal::DeclaredCause::declared(
        tp::refusal::CauseId::declared(
            LOT_FAMILY,
            tp::refusal::LocalCauseKey::declared("not-labelled"),
        ),
        "NotLabelled",
    ),
    tp::refusal::DeclaredCause::declared(
        tp::refusal::CauseId::declared(
            LOT_FAMILY,
            tp::refusal::LocalCauseKey::declared("over-limit"),
        ),
        "OverLimit",
    ),
];

/// What the DERIVED family's declaration states, whole.
const THE_DERIVED_DECLARATION: DeclaredFacts = DeclaredFacts {
    shape: tp::refusal::FamilyShape::SingleCause,
    order: tp::refusal::DeclaredCauseOrder::declared(THE_DERIVED_ROWS),
};

/// What the HAND-REALIZED family's declaration states, whole.
const THE_HAND_REALIZATION: DeclaredFacts = DeclaredFacts {
    shape: tp::refusal::FamilyShape::SingleCause,
    order: tp::refusal::DeclaredCauseOrder::declared(THE_HAND_ROWS),
};

/// The cause this consumer cites when the derived family's declared facts are
/// not the facts its declaration states.
const DERIVED_NOT_READ_BACK: FindingCause =
    FindingCause::named(CONSUMER, "derived-declaration-not-read-back");

/// The cause this consumer cites when the hand-realized family's declared facts
/// are not the facts its declaration states.
const HAND_NOT_READ_BACK: FindingCause =
    FindingCause::named(CONSUMER, "hand-declaration-not-read-back");

/// The cause this consumer cites when the derived family's declaration does not
/// close its joins.
const FAMILY_NOT_ADMITTED: FindingCause = FindingCause::named(CONSUMER, "family-not-admitted");

// ---------------------------------------------------------------------------
// The owner-supplied seam: this consumer's own declaration of sameness.
// ---------------------------------------------------------------------------

/// This consumer's own declaration of when two families declare one thing.
///
/// Both seats count, and every seat inside the order counts: the same causes,
/// by stable identity and by Rust spelling, in the same positions, and no more
/// of them. Nothing here is presentational, so nothing here is exempt.
fn the_same_facts(left: &DeclaredFacts, right: &DeclaredFacts) -> Agreement {
    if left.shape == right.shape && left.order == right.order {
        Agreement::Agrees
    } else {
        Agreement::Differs
    }
}

// ---------------------------------------------------------------------------
// The checks: thin functions binding a declared surface to a harness law.
// ---------------------------------------------------------------------------

/// The derived family's declared facts, read back off the implementations the
/// derive delivered and compared against what its declaration states.
fn the_derived_declaration_is_read_back(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::agreement(
        the_same_facts,
        &declared_facts::<MergeRefusal>(),
        &THE_DERIVED_DECLARATION,
        DERIVED_NOT_READ_BACK,
    )
}

/// The machine's own admission road over the derived declaration: the declared
/// shape and the declared selection order cohere, and the typed cause order
/// projects onto the textual one.
///
/// The textual selection order is reached here and nowhere else in this file.
/// It needs no roster written out beside it, because this road IS the join
/// between it and the typed order — a second roster would be one fact stated
/// twice, and the two copies would be free to drift.
fn the_derived_family_closes_its_joins(_invocation: &Invocation) -> TrialConclusion {
    let witness = tp::refusal::admit_order::<MergeRefusal>();
    harness::properties::admitted(&witness, FAMILY_NOT_ADMITTED)
}

/// The parity row: the hand-realized family's declared facts, read back through
/// the SAME road, and compared against what its own declaration states.
///
/// What it establishes is about the roads. Both families answer one pair of
/// contracts, so one generic reading reaches both — that half the compiler
/// settled — and each answers with its own facts, which is the half this row
/// reads. A shared local key across the two orders carries no shared ownership,
/// and the family seat inside each cause identity is where that shows.
fn the_hand_declaration_is_read_back(_invocation: &Invocation) -> TrialConclusion {
    harness::properties::agreement(
        the_same_facts,
        &declared_facts::<LotRefusal>(),
        &THE_HAND_REALIZATION,
        HAND_NOT_READ_BACK,
    )
}

// ---------------------------------------------------------------------------
// The authoring road: one row and one attachment, per declared trial.
// ---------------------------------------------------------------------------

/// One revision identity this consumer commits to by hand.
///
/// Declared rather than derived: the identity moves when this file says it
/// moved, and a declaration moving is exactly the event this file's author is
/// the one to notice.
fn declared_revision(material: &[u8]) -> RevisionBinding {
    RevisionBinding::declared(ContentAddress::derived(REVISION_TAG, material))
}

/// One hand-written row and the attachment that executes it.
///
/// The origin is the hand's, and that is the honest word for it: what a
/// derivation produced here is the SUBJECT these rows are about, and a person
/// wrote every row below. A row claiming a producer's origin would be claiming
/// a producer emitted the descriptor, which nothing did.
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
/// a name that would not parse, a repeated label, or a row whose canonical
/// bytes could not be written.
fn row_parts(
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
            vec![Tag::named(CONSUMER, "declared-surface")?],
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
    /// The complete world this consumer authored over its two declarations.
    mod declared_world named("consumer", "the-derived-world") {
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

        suite derivation named("consumer", "derivation") {
            the_derived_declaration_is_read_back: {
                let (row, attachment) = crate::row_parts(
                    "the-derived-declaration-is-read-back",
                    "derivation",
                    "lot-merge-family",
                    "declared-facts-read-back",
                    "the-derived-declaration",
                    crate::the_derived_declaration_is_read_back,
                )?;
                harness::descriptor::Binding::bound(
                    row,
                    attachment,
                    harness::descriptor::Provenance::Unproduced,
                )
            },
            the_derived_family_closes_its_joins: {
                let (row, attachment) = crate::row_parts(
                    "the-derived-family-closes-its-joins",
                    "derivation",
                    "lot-merge-family",
                    "family-admission",
                    "the-derived-declaration",
                    crate::the_derived_family_closes_its_joins,
                )?;
                harness::descriptor::Binding::bound(
                    row,
                    attachment,
                    harness::descriptor::Provenance::Unproduced,
                )
            },
        }

        suite parity named("consumer", "parity") {
            the_hand_declaration_is_read_back: {
                let (row, attachment) = crate::row_parts(
                    "the-hand-declaration-is-read-back",
                    "parity",
                    "lot-refusal-family",
                    "declared-facts-read-back",
                    "the-hand-realization",
                    crate::the_hand_declaration_is_read_back,
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
