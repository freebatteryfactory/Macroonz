//! The host's declarations: the span custody a capture builds, how a capture refuses, and what a value answers to be emitted.
//!
//! Declarations only, with every road that reaches a private field in `type_guard.rs`, this file's own child.

use crate::closure::PartitionCargo;
use crate::token::{
    CaptureBound, CaptureBuilder, LiteralReadCause, SpanHandle, SpanResolutionRefusal, TokenPath,
};
use proc_macro::Span;

#[path = "type_guard.rs"]
mod guard;

/// The compiler spans one capture issued handles for.
///
/// A handle means "the token at this index of the table its producer built", and this is that table when the producer is a proc macro.
/// It is what lets a refusal land on the offending token rather than on the declaration's first one, because it is what holds the compiler's own spans.
#[derive(Debug)]
pub struct Spans {
    builder: CaptureBuilder<Span>,
}

/// How reading one declared input into a captured surface refuses.
///
/// Two rows, because the two are facts about different things — and the difference is where each is reported.
#[must_use = "a capture refusal names what stopped the read and what it is a fact about"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaptureError {
    /// The read ran past one of the declared magnitudes.
    Unbounded {
        /// Which magnitude.
        bound: CaptureBound,
    },
    /// One literal's spelling could not be read into the value it names, with declaration identity and producer placement retained separately.
    Unread {
        /// Why it could not be read.
        cause: LiteralReadCause,
        /// Which token of the declared input could not be read.
        path: TokenPath,
        /// The producer-local handle already bound to the token's compiler span.
        at: SpanHandle,
    },
}

/// Why one admitted generated literal could not cross the compiler-token host.
///
/// This is a host contradiction rather than a declaration diagnostic: the ordinary compiler admitted the literal before the proc API was asked to materialize it.
#[must_use = "an emission refusal names the admitted literal the compiler host could not materialize"]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmissionError {
    /// The proc-macro literal API rejected one already admitted numeric spelling.
    NumberRejected {
        /// The exact admitted numeric spelling.
        spelling: String,
    },
    /// The proc-macro C-string literal API rejected already admitted C-string material.
    NulTerminatedTextRejected,
    /// A generated tree's private source roster no longer matches its recursive token denominator.
    SourceSpanRosterContradiction,
    /// One preserved source handle does not resolve in the capture table supplied for emission.
    SourceSpanUnresolved(SpanResolutionRefusal),
}

/// What one value answers to have its declaration-site cargo emitted.
///
/// A cargo per delivery it carries, in the order a compiler receives them: a value delivering two says so rather than handing over one stream that claims to be both.
pub trait Emittable {
    /// The declaration-site cargos this value delivers.
    fn cargos(&self) -> impl Iterator<Item = &PartitionCargo>;
}
