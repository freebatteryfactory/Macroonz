#![doc = include_str!("README.md")]

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
pub use refusal::{admits_lawful, fail_closed};
pub use temporal::{holds_over_drive, holds_over_history};
pub use types::{
    ALWAYS_BROKEN, AMBIENT_PATHWAY_DISAGREEMENT, ANSWER_EXPECTED, Agreement,
    COMPOSED_CONSERVATION_DISAGREEMENT, COMPOSED_DETERMINISM_DISAGREEMENT,
    COMPOSED_IDEMPOTENCE_DISAGREEMENT, COMPOSED_RETURN_DISAGREEMENT, CONSERVATION_DISAGREEMENT,
    Check, ComposedRoads, ContractRefusal, DETERMINISM_DISAGREEMENT, EVENTUALLY_UNREACHED,
    Equivalence, FAIL_CLOSED_ANSWERED, FUSED_VERSUS_SEPARATE_DISAGREEMENT, Holding,
    IDEMPOTENCE_DISAGREEMENT, LATCH_BROKEN, LAWFUL_TWIN_REFUSED, LIVE_VERSUS_REPLAYED_DISAGREEMENT,
    MONOTONICITY_DISAGREEMENT, Measure, NEVER_BROKEN, NO_SEQUENCE_DRIVEN, ORDER_DECREASED, Order,
    PERMUTATION_DISAGREEMENT, ParityReading, ParitySuite, PoisonResponse, REFUSAL_EXPECTED,
    ROUNDTRIP_DISAGREEMENT, ResponseReading, Road, RoadPairing, SharedSubstrate, StatePredicate,
    SubstrateRef, SubstrateRefusal, SubstrateRoster, TemporalClaim, TemporalDemand,
    TemporalDriveReading, TemporalDriveStanding, TransitionContract,
};
