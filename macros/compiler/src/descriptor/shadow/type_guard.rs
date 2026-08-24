//! Constructors and readers for the shadow vocabulary.

use super::{ShadowCaptureError, ShadowRow, Shadows};
use crate::descriptor::{CaptureCause, Grammar, HelperRefusal};
use crate::token::SpanHandle;

impl ShadowRow {
    /// One covered name and its two spellings, minted only into the stated roster.
    #[must_use]
    pub(crate) const fn covered(
        name: &'static str,
        std_path: &'static [&'static str],
        loom_path: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            std_path,
            loom_path,
        }
    }

    /// The spelling a declaration chooses this row by.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    /// The path the ordinary build resolves the name to.
    #[must_use]
    pub const fn std_path(&self) -> &'static [&'static str] {
        self.std_path
    }

    /// The path the shadowed build resolves the name to.
    #[must_use]
    pub const fn loom_path(&self) -> &'static [&'static str] {
        self.loom_path
    }
}

impl Shadows {
    /// The chosen rows, in authored order, minted only by the capture reading.
    #[must_use]
    pub(crate) const fn declared(chosen: Vec<ShadowRow>) -> Self {
        Self { chosen }
    }

    /// The chosen rows, in authored order.
    #[must_use]
    pub fn chosen(&self) -> &[ShadowRow] {
        &self.chosen
    }
}

impl ShadowCaptureError {
    /// One refusal the shadow grammar's own reading established.
    pub const fn grammar_refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> Self {
        Self(HelperRefusal::grammar_refused(grammar, cause, at))
    }

    /// The refusal itself.
    pub const fn refusal(&self) -> &HelperRefusal {
        &self.0
    }
}
