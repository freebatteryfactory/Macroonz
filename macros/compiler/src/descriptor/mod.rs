#![doc = include_str!("README.md")]

pub mod bench;
pub mod concurrency;
pub mod door;
pub mod mutation;
pub mod network;
pub mod shadow;
pub mod trial;

mod binding;
mod emitting;
pub mod vocabulary;

mod composition;
mod type_contract;
mod types;

pub use types::{
    Binding, BoundPath, COMPOSITION_ISSUE_LIMIT, CaptureCause, CaptureIssue, Composition,
    CompositionError, CompositionIssue, DESCRIPTOR_MEANING_FACT, DeclarationError, DirectBinding,
    Emitter, FunctionName, Grammar, HelperRefusal, ModuleName, Name, PATH_SEGMENT_LIMIT,
    PROVIDER_LIMIT, Provider, RENDERED_SPELLING_FACT, Seat, SupportName, TypeName,
    rendered_identifier, rendered_name,
};
