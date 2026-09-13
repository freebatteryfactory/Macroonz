//! Construction from owning projections and format selection.

use super::Presentation;
use crate::presentation::{render, value::object};
use serde_json::Value;

impl Presentation {
    pub(in crate::presentation) fn projected(
        kind: &str,
        owner: &str,
        standing: &str,
        record: Value,
    ) -> Self {
        Self {
            value: object([
                ("schema", "macroonz-presentation-v1".into()),
                ("kind", kind.into()),
                ("owner", owner.into()),
                ("standing", standing.into()),
                ("record", record),
            ]),
        }
    }

    /// The complete structured JSON document.
    #[must_use]
    pub fn json(&self) -> String {
        self.value.to_string()
    }

    /// The complete escaped Markdown path/value table.
    #[must_use]
    pub fn markdown(&self) -> String {
        render::markdown(&self.value)
    }

    /// The complete escaped HTML table fragment.
    #[must_use]
    pub fn html(&self) -> String {
        render::html(&self.value)
    }
}
