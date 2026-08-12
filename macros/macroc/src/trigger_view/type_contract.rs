//! The trigger-view home's declarative surface: the shape its refusal family
//! declares.
//!
//! A constant shape and a constant selection order, stated rather than computed.
//! The family is an issue collection because several components may be undecided
//! while another is doubled, so no primary issue is ever elected and the
//! selection order is empty by declaration rather than by omission.

use super::TriggerViewComposition;
use threadpak::refusal::{FamilyShape, RefusalFamily};

impl RefusalFamily for TriggerViewComposition {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}
