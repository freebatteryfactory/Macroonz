//! The host's invariant nucleus: every road that reaches the span custody's private seat.
//!
//! Declared inside `types.rs` as its own child, so the held spans are reachable here and nowhere else.
//! Handles are issued in reading order and never reused, which is the whole invariant: a handle answers about the token it was issued for, or the table says it does not reach.

use super::Spans;
use crate::token::{CaptureBound, SpanHandle};
use proc_macro::Span;

impl Spans {
    /// A table that has issued nothing.
    #[must_use]
    pub const fn empty() -> Self {
        Self { held: Vec::new() }
    }

    /// How many handles this table has issued; a handle at or past that index names no token in it.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::Tree`] where the table has already run past the width a handle is carried in.
    pub fn issued(&self) -> Result<u32, CaptureBound> {
        u32::try_from(self.held.len()).map_err(|_| CaptureBound::Tree)
    }

    /// Issue the next handle for one span.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureBound::Tree`] where the table has run past that width.
    /// An index that saturated instead would hand two tokens one handle, and a refusal about either would be reported at whichever of them the table reached first.
    pub(crate) fn issue(&mut self, span: Span) -> Result<SpanHandle, CaptureBound> {
        let index = self.issued()?;
        self.held.push(span);
        Ok(SpanHandle::at(index))
    }

    /// The compiler span one handle names, or the invocation where this table does not reach it.
    ///
    /// One lookup for every road that holds a handle, so a diagnostic the compiler composed and a capture this host refused point at their token the same way.
    /// Where the table does not reach, the invocation stands — never the declaration's first span, which is a real token the observation is not about and would read exactly like an answer.
    #[must_use]
    pub fn at(&self, handle: SpanHandle) -> Span {
        usize::try_from(handle.index())
            .ok()
            .and_then(|index| self.held.get(index).copied())
            .unwrap_or_else(Span::call_site)
    }
}
