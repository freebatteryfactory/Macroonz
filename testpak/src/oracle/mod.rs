#![doc = include_str!("README.md")]
//!
//! # The files
//!
//! `types.rs` declares everything the annex can say — a golden vector, an
//! independently composed preimage, what an artifact was read to declare, what
//! a compiled artifact handed back, and the verdicts over all four. Its child
//! `type_guard.rs` holds the two roads that mint a value nobody may forge: the
//! public vector parser, and the transcript composition. `conclude.rs` owns the
//! finding causes and the one road from each verdict into the record
//! vocabulary.
//!
//! [`structural`] and [`compiled`] are two comparisons over readings supplied by
//! the challenge side: a `syn` host maps rendered text into typed declarations,
//! while a compiled host maps rustc values into a read-back. Neither subsumes
//! the other and neither is a weaker version of the other — a verdict is
//! method-specific, and reporting one as though it came from another is the
//! collapse the annex exists to refuse.

pub mod compiled;
pub mod structural;
mod conclude;
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
