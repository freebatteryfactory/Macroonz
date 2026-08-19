#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! `types.rs` declares everything the annex can say — a golden vector, an
//! independently composed preimage, what an artifact was read to declare, what
//! a compiled artifact handed back, and the verdicts over all four. Its child
//! `type_guard.rs` holds the two roads that mint a value nobody may forge: the
//! public vector parser, and the transcript composition. `type_contract.rs`
//! holds the closed tables that name every finding and the one road from each
//! verdict into the record vocabulary.
//!
//! [`structural`] and [`compiled`] are the two readings, each with its own
//! comparison: the structural read decodes rendered text and states what the
//! artifact DECLARES; the compiled read-back compares what a compiler handed
//! back against what the caller declared it would. Neither subsumes the other
//! and neither is a weaker version of the other — a verdict is method-specific,
//! and reporting one as though it came from another is the collapse the annex
//! exists to refuse.

pub mod compiled;
pub mod structural;
mod type_contract;
mod types;

pub use types::{
    ArtifactStructure, ByteDifference, CompiledDisagreement, CompiledObservation, CompiledVerdict,
    ConstantReading, ContextRefusal, DeclaredArtifact, DeclaredBehaviour, DeclaredImplementation,
    DeclaredMember, DeclaredReadBack, DerivedIdentity, ImplPosture, ImplementationMember,
    ImplementationStructure, ORACLE_CAUSE_FAMILY, ObservedMember, ObservedValue, SpecifiedContext,
    StructuralDisagreement, StructuralVerdict, TranscriptDerivation, TranscriptDisagreement,
    TranscriptMember, TranscriptVerdict, VECTOR_PACK_MAGIC, VECTOR_PACK_VERSION,
    VectorDisagreement, VectorEntry, VectorPack, VectorPackRefusal, VectorSubject, VectorVerdict,
};
