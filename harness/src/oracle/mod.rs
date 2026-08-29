#![doc = include_str!("README.md")]
//!
//! [`parse`] maps rendered Rust into a structural reading, and [`structural`] and [`compiled`] are the two comparisons.
//!
//! Neither comparison subsumes the other and neither is a weaker version of the other.
//! A verdict is method-specific, and reporting one as though it came from another is the collapse this home exists to refuse.

#[path = "compiled.rs"]
pub mod compiled;
pub mod parse;
#[path = "structural.rs"]
pub mod structural;

#[path = "compiled/mod.rs"]
mod compiled_owner;
#[path = "structural/mod.rs"]
mod structural_owner;
mod transcript;
mod types;
mod vector;

pub use compiled_owner::{
    CompilationDisagreement, CompilationVerdict, CompiledDisagreement, CompiledObservation,
    CompiledVerdict, DeclaredBehavior, DeclaredCompilation, DeclaredReadBack,
    DeclaredReadBackRoster, DeclaredReadBackRosterRefusal, DiagnosticAnchor, ObservedCompilation,
    ObservedMember, ObservedValue, PrimarySourceSpan, PrimarySourceSpanRefusal, RelativeSourcePath,
    RelativeSourcePathRefusal, RustcErrorCode, RustcErrorCodeRefusal, SourcePosition,
    SourcePositionRefusal,
};
pub use structural_owner::{
    ArtifactStructure, ConstantReading, DeclaredArtifact, DeclaredImplementation, DeclaredMember,
    DeclaredMemberRoster, DeclaredMemberRosterRefusal, ImplPosture, ImplementationMember,
    ImplementationStructure, StructuralDisagreement, StructuralPath, StructuralPathRefusal,
    StructuralPathRoot, StructuralPathSegment, StructuralVerdict,
};
pub use transcript::{
    ContextRefusal, DerivedIdentity, SpecifiedContext, TranscriptDerivation,
    TranscriptDisagreement, TranscriptMember, TranscriptVerdict,
};
pub use types::ORACLE_CAUSE_FAMILY;
pub use vector::{
    ByteDifference, VECTOR_PACK_MAGIC, VECTOR_PACK_VERSION, VectorDisagreement, VectorEntry,
    VectorPack, VectorPackRefusal, VectorSubject, VectorVerdict,
};
