//! The template home's declarative surface: the shapes its two refusal families
//! declare.
//!
//! Two families, two shapes, and the difference is a statement about how many
//! checks can truthfully run. Binding one argument to one parameter runs exactly
//! one check, so that family is single-cause and its selection order names the
//! one cause. Constructing a template, a ceiling, or an application runs a
//! roster's worth of independent checks, so that family is an issue collection
//! and elects no primary issue at all.

use super::{TemplateBindingIssue, TemplateConstruction};
use threadpak::refusal::{FamilyShape, RefusalFamily};

impl RefusalFamily for TemplateBindingIssue {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["CategoryMismatch"];
}

impl RefusalFamily for TemplateConstruction {
    const SHAPE: FamilyShape = FamilyShape::IssueCollection;
    const SELECTION_ORDER: &'static [&'static str] = &[];
}
