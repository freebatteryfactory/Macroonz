//! Constructors and readers for the shadow vocabulary.

use super::{ShadowCaptureError, ShadowRow, Shadows};
use crate::descriptor::{CaptureCause, CaptureIssue, DirectBinding, Grammar, HelperRefusal};
use crate::token::SpanHandle;

impl ShadowRow {
    /// One covered name and its two spellings, minted only into the stated roster.
    #[must_use]
    pub(crate) const fn covered(
        name: &'static str,
        std_path: &'static [&'static str],
        shadow_path: &'static [&'static str],
    ) -> Self {
        Self {
            name,
            std_path,
            shadow_path,
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

    /// The path after the declared Loom binding that the shadowed build resolves the name to.
    #[must_use]
    pub const fn shadow_path(&self) -> &'static [&'static str] {
        self.shadow_path
    }
}

impl Shadows {
    /// The chosen rows, in authored order, minted only by the capture reading.
    #[must_use]
    pub(crate) const fn declared(loom: DirectBinding, chosen: Vec<ShadowRow>) -> Self {
        Self { loom, chosen }
    }

    /// The physical path to the Loom-compatible shadow vocabulary.
    #[must_use]
    pub const fn loom(&self) -> &DirectBinding {
        &self.loom
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

    /// One refusal from the direct-binding reading, retained without flattening its owner.
    pub const fn binding_refused(grammar: Grammar, issue: CaptureIssue, at: SpanHandle) -> Self {
        Self(HelperRefusal::capture_refused(grammar, issue, at))
    }

    /// The refusal itself.
    pub const fn refusal(&self) -> &HelperRefusal {
        &self.0
    }
}
