//! Directional challenges for harness types that must never substitute for one another.
//!
//! A row does not establish a separation by itself, and the rows are never a census: a separation stated elsewhere is not weaker for having no row here.

use super::type_separation::types::SwapPair;

/// A trial's site offered where its semantic identity is required.
///
/// An identity is content-addressed and survives a file or module move, while a site is the locator — module path, file, line, display name — and moves whenever anyone tidies a directory.
/// A site standing in for an identity would renumber the whole denominator under a refactor that changed no meaning, and every claim computed over reports would silently be computed over a different world.
pub const TRIAL_IDENTITY_AGAINST_SITE: SwapPair = SwapPair {
    seat: "TrialId",
    substitute: "TrialSite",
    boundary: "semantic identity against location",
};

/// A bookkeeping revision offered where the execution key is required.
///
/// The keys split by job: a suite-tag or origin edit moves the row revision and owes no execution, while the execution key binds the trial identity, both revisions, the invocation profile, and the target and toolchain binding.
/// Substituting the bookkeeping key would let a run be skipped on a key that never carried the target binding, and the cost of refusing that substitution is reruns — cost, never truth.
pub const EXECUTION_KEY_AGAINST_ROW_REVISION: SwapPair = SwapPair {
    seat: "ExecutionKey",
    substitute: "RowRevisionId",
    boundary: "execution against bookkeeping",
};

/// The subject's revision binding offered where the check's is required.
///
/// An executable attachment carries one posture-bearing binding for each, and every per-posture sentence reads over their meet — the weaker of the two.
/// A derived subject binding read where the check's declared binding belongs lifts the meet to the stronger side, and the mixed attachment mints an exact-replay claim over a revision whose ceiling is the author's word.
pub const CHECK_REVISION_AGAINST_SUBJECT_REVISION: SwapPair = SwapPair {
    seat: "CheckRevisionId",
    substitute: "SubjectRevisionId",
    boundary: "the attachment's two posture-bearing revision bindings",
};

/// A stored proposal reference offered where the proposal's content identity is required.
///
/// The two have opposite lifetimes: a content identity is permanent provenance an admitted row cites, and a stored reference is a location, deliberately mortal, so a discharged review artifact may be deleted with nothing left dangling.
/// If a location could stand where the identity belongs, deleting that artifact would reach back and break the provenance of rows already admitted, and custody would depend on a directory surviving.
pub const PROPOSAL_IDENTITY_AGAINST_STORED_REFERENCE: SwapPair = SwapPair {
    seat: "ProposalId",
    substitute: "StoredProposalRef",
    boundary: "permanent provenance against mortal location",
};

/// A replay reference offered where the failure fingerprint is required.
///
/// A fingerprint is failure identity — a trial's identity joined with its typed cause and a normalized failure class — and it is what a minimized input must still carry, while a replay reference points at a capsule and is stable under shrinking by construction.
/// Substituting it would make fingerprint preservation compare pointers to themselves, so every shrink would pass, including the one that minimized into a different bug.
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
