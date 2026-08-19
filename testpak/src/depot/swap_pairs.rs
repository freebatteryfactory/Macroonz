//! The anti-substitution swap-pair population: pairs of role-distinct types the
//! generator turns into compile-refusal cases.
//!
//! # What a row buys
//!
//! Every pair below is a separation this harness already states in prose at the
//! instrument that owns it. Prose is a claim held by whoever reads it, and a
//! separation held that way survives exactly as long as nobody is in a hurry.
//! A row moves that separation onto the compiler: the generator emits a case
//! that offers the substitute where the seat's type is required, and the
//! evidence is a refusal `rustc` produced without being asked what the two
//! values mean.
//!
//! The rows are material and never a census: this bank states the pairs it can
//! ground in the harness's own vocabulary, and a separation somebody states
//! elsewhere is not weaker for having no row here — it is a row this bank has
//! not earned yet.

use super::types::SwapPair;

/// A trial's site offered where its semantic identity is required.
///
/// The two are addressed differently on purpose: semantic identity is
/// content-addressed and survives file and module moves, while a site is the
/// locator — module path, file, line, display name — and moves whenever anyone
/// tidies a directory. Reports join both, which is exactly why they must not
/// unify: a site standing in for an identity would renumber the whole
/// denominator under a refactor that changed no meaning, and every claim
/// computed over reports would silently be computed over a different world.
pub const TRIAL_IDENTITY_AGAINST_SITE: SwapPair = SwapPair {
    seat: "TrialId",
    substitute: "TrialSite",
    boundary: "semantic identity against location",
};

/// A bookkeeping revision offered where the execution key is required.
///
/// The keys split by job, and the split is the whole point: a suite-tag or
/// origin edit moves the row revision, aggregation recomputes, and no execution
/// is owed — while the execution key binds the trial identity, the subject and
/// check revisions, the invocation profile, and the target and toolchain
/// binding, because a cache hit across targets is a claim nothing verified.
/// Substituting the bookkeeping key here would let a run be skipped on a key
/// that never carried the target binding, and the cost of refusing that
/// substitution is reruns — cost, never truth.
pub const EXECUTION_KEY_AGAINST_ROW_REVISION: SwapPair = SwapPair {
    seat: "ExecutionKey",
    substitute: "RowRevisionId",
    boundary: "execution against bookkeeping",
};

/// The subject's revision binding offered where the check's is required.
///
/// An executable attachment carries one posture-bearing revision binding for
/// each, and every per-posture sentence reads over their meet — the weaker of
/// the two. That is why they cannot be one type: a derived subject binding read
/// where the check's declared binding belongs lifts the meet to the stronger
/// side, and the mixed attachment mints an exact-replay claim over a revision
/// whose ceiling is the author's word. The meet law is the report instrument's;
/// this row is what keeps the compiler from having to be told about it.
pub const CHECK_REVISION_AGAINST_SUBJECT_REVISION: SwapPair = SwapPair {
    seat: "CheckRevisionId",
    substitute: "SubjectRevisionId",
    boundary: "the attachment's two posture-bearing revision bindings",
};

/// A stored proposal reference offered where the proposal's content identity is
/// required.
///
/// The two have opposite lifetimes. A proposal's content identity is permanent
/// provenance, and an admitted row cites it; the stored reference a sink
/// returns is a location, deliberately mortal, so the review artifact may be
/// deleted after any ruling with nothing left dangling. If a location could
/// stand where the identity belongs, deleting a discharged review artifact
/// would reach back and break the provenance of rows already admitted — and
/// custody would depend on a directory surviving.
pub const PROPOSAL_IDENTITY_AGAINST_STORED_REFERENCE: SwapPair = SwapPair {
    seat: "ProposalId",
    substitute: "StoredProposalRef",
    boundary: "permanent provenance against mortal location",
};

/// A replay reference offered where the failure fingerprint is required.
///
/// A fingerprint is failure identity — the trial's semantic identity joined
/// with its typed cause and a normalized failure class — and it is what
/// deduplicates finds across runs and what a minimized input must still carry.
/// A replay reference is a pointer at the capsule an admission act authored, and
/// it is stable under shrinking by construction. Substituting it would make
/// fingerprint preservation compare pointers to themselves: every shrink would
/// pass, including the one that minimized into a different bug, which is the
/// single outcome the preservation rule exists to refuse.
pub const FINGERPRINT_AGAINST_REPLAY_REFERENCE: SwapPair = SwapPair {
    seat: "Fingerprint",
    substitute: "ReplayRef",
    boundary: "failure identity against reproduction pointer",
};

/// The swap pairs this bank grounds, in the order it states them.
pub const SWAP_PAIRS: [SwapPair; 5] = [
    TRIAL_IDENTITY_AGAINST_SITE,
    EXECUTION_KEY_AGAINST_ROW_REVISION,
    CHECK_REVISION_AGAINST_SUBJECT_REVISION,
    PROPOSAL_IDENTITY_AGAINST_STORED_REFERENCE,
    FINGERPRINT_AGAINST_REPLAY_REFERENCE,
];
