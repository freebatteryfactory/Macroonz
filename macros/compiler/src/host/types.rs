//! The host's declarations: the span custody a capture builds, how a capture refuses, and what a value answers to be emitted.
//!
//! Declarations only, with every road that reaches a private field in `type_guard.rs`, this file's own child.

use crate::closure::PartitionCargo;
use crate::token::{CaptureBound, CaptureBuilder, LiteralReadCause, SpanHandle};
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CaptureError {
    /// The read ran past one of the declared magnitudes.
    Unbounded {
        /// Which magnitude.
        bound: CaptureBound,
    },
    /// One literal's spelling could not be read into the value it names.
    Unread {
        /// Why it could not be read.
        cause: LiteralReadCause,
        /// The token it is about, whose handle was issued before its payload was read.
        at: SpanHandle,
    },
}

/// What one value answers to have its declaration-site cargo emitted.
///
/// A cargo per delivery it carries, in the order a compiler receives them: a value delivering two says so rather than handing over one stream that claims to be both.
pub trait Emittable {
    /// The declaration-site cargos this value delivers.
    fn cargos(&self) -> impl Iterator<Item = &PartitionCargo>;
}
