//! The host's invariant nucleus: every road that reaches the span custody's private seat.
//!
//! Declared inside `types.rs` as its own child, so the held spans are reachable here and nowhere else.
//! Handles are issued in reading order and never reused, which is the whole invariant: a handle answers about the token it was issued for, or the table says it does not reach.

use super::Spans;
use crate::token::{CaptureBuilder, SpanHandle, SpanResolutionRefusal};
use proc_macro::Span;

impl Spans {
    /// A table that has issued nothing.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            builder: CaptureBuilder::declared(),
        }
    }

    /// The checked builder that owns this table's handles and positions.
    pub(crate) const fn builder(&mut self) -> &mut CaptureBuilder<Span> {
        &mut self.builder
    }

    /// The compiler span one handle names, or the invocation where this table does not reach it.
    ///
    /// One lookup for every road that holds a handle, so a diagnostic the compiler composed and a capture this host refused point at their token the same way.
    /// Where the table does not reach, the invocation stands — never the declaration's first span, which is a real token the observation is not about and would read exactly like an answer.
    #[must_use]
    pub fn at(&self, handle: SpanHandle) -> Span {
        self.resolve(handle).unwrap_or_else(|_| Span::call_site())
    }

    /// Resolve one preserved source handle for host emission without inventing a fallback span.
    pub(crate) fn resolve(&self, handle: SpanHandle) -> Result<Span, SpanResolutionRefusal> {
        usize::try_from(handle.index())
            .ok()
            .and_then(|index| self.builder.positions().get(index).copied())
            .ok_or(SpanResolutionRefusal {
                handle,
                reaches: self.builder.positions().len(),
            })
    }
}
