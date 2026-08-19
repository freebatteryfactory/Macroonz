#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! `types.rs` declares this home's public vocabulary: the check shape, the
//! owner-supplied comparison seams, the demand verdict, the parity suite and the
//! substrate it names, the transition contract and its temporal claims, the
//! composed-roads suite, and the typed causes this home cites. Every road that
//! reaches one of its private fields is its own child, `type_guard.rs`.
//!
//! The laws are role-named pure-function modules, one family each. `conclude` is
//! the nucleus every one of them reaches its verdict through, and the one place
//! a disagreement becomes a finding. `algebra` carries the declared-algebra laws
//! and `metamorphic` the relations between two runs; `parity` carries the suite
//! over two roads to one meaning, `composition` the suite over a wiring,
//! `temporal` the suite over histories, and `refusal` the checks that judge
//! whether a subject refused what it owed a refusal. [`ensure`] is the macro
//! battery over the nucleus; its macros land at the crate root, which is Rust's
//! rule rather than this home's choice.

mod algebra;
mod composition;
mod conclude;
pub mod ensure;
mod metamorphic;
mod parity;
mod refusal;
mod temporal;
mod types;

pub use algebra::{conservation, idempotence, monotonicity, roundtrip};
pub use composition::{
    composed, composed_conservation, composed_determinism, composed_idempotence, composed_return,
};
pub use conclude::{admitted, agreement, concluded, raised_here, ranking, refused};
pub use metamorphic::{
    ambient_pathway_invariance, determinism_run_twice, permutation_insensitivity,
};
pub use parity::parity;
pub use refusal::{admits_lawful, fail_closed, panic_freedom};
pub use temporal::{holds_over_drive, holds_over_history};
pub use types::{
    ALWAYS_BROKEN, AMBIENT_PATHWAY_DISAGREEMENT, ANSWER_EXPECTED, Agreement,
    COMPOSED_CONSERVATION_DISAGREEMENT, COMPOSED_DETERMINISM_DISAGREEMENT,
    COMPOSED_IDEMPOTENCE_DISAGREEMENT, COMPOSED_RETURN_DISAGREEMENT, CONSERVATION_DISAGREEMENT,
    Check, ComposedRoads, ContractRefusal, DETERMINISM_DISAGREEMENT, EVENTUALLY_UNREACHED,
    Equivalence, FAIL_CLOSED_ANSWERED, FUSED_VERSUS_SEPARATE_DISAGREEMENT, Holding,
    IDEMPOTENCE_DISAGREEMENT, LATCH_BROKEN, LAWFUL_TWIN_REFUSED,
    LIVE_VERSUS_REPLAYED_DISAGREEMENT, MONOTONICITY_DISAGREEMENT, Measure, NEVER_BROKEN,
    NO_SEQUENCE_DRIVEN, ORDER_DECREASED, Order, PERMUTATION_DISAGREEMENT, ParitySuite,
    PoisonResponse, REFUSAL_EXPECTED, ROUNDTRIP_DISAGREEMENT, ResponseReading, Road, RoadPairing,
    SharedSubstrate, StatePredicate, SubstrateRef, SubstrateRefusal, SubstrateRoster, TemporalClaim,
    TemporalDemand, TransitionContract,
};
