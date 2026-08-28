#![doc = include_str!("README.md")]

mod compare;
mod conclude;
mod parse;
mod types;

pub use compare::{compared, read};
pub use parse::declarations_in;
pub use types::{
    ArtifactStructure, ConstantReading, DeclaredArtifact, DeclaredImplementation, DeclaredMember,
    DeclaredMemberRoster, DeclaredMemberRosterRefusal, ImplPosture, ImplementationMember,
    ImplementationStructure, StructuralDisagreement, StructuralPath, StructuralPathRefusal,
    StructuralPathRoot, StructuralPathSegment, StructuralVerdict,
};
