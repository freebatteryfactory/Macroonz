//! The composition home's declarative surface: the shape its refusal family
//! declares.
//!
//! A constant shape and a constant selection order, stated rather than computed.
//! The family is an issue collection because several providers may be doubled in
//! one declaration, so no primary issue is ever elected and the selection order
//! is empty by declaration rather than by omission.

use super::CompositionRootDeclaration;
use macroonz::{FamilyShape, RefusalFamily};

impl RefusalFamily for CompositionRootDeclaration {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
}
