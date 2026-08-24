#![doc = include_str!("README.md")]
//!
//! [`parse`] maps rendered Rust into a structural reading, and [`structural`] and [`compiled`] are the two comparisons.
//!
//! Neither comparison subsumes the other and neither is a weaker version of the other.
//! A verdict is method-specific, and reporting one as though it came from another is the collapse this home exists to refuse.

pub mod compiled;
pub mod parse;
pub mod structural;

mod conclude;
mod types;

pub use types::{
    ArtifactStructure, ByteDifference, CompiledDisagreement, CompiledObservation, CompiledVerdict,
    ConstantReading, ContextRefusal, DeclaredArtifact, DeclaredBehavior, DeclaredImplementation,
    DeclaredMember, DeclaredReadBack, DerivedIdentity, ImplPosture, ImplementationMember,
    ImplementationStructure, ORACLE_CAUSE_FAMILY, ObservedMember, ObservedValue, SpecifiedContext,
    StructuralDisagreement, StructuralVerdict, TranscriptDerivation, TranscriptDisagreement,
    TranscriptMember, TranscriptVerdict, VECTOR_PACK_MAGIC, VECTOR_PACK_VERSION,
    VectorDisagreement, VectorEntry, VectorPack, VectorPackRefusal, VectorSubject, VectorVerdict,
};
