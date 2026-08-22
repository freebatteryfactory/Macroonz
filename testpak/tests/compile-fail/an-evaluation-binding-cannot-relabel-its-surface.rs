//! An evaluation binding derives its family and surface identity from the exact surface it receives.
//!
//! A caller cannot seat parallel family or surface labels beside the callable and thereby relabel which evaluation copy it executes.

use threadpak_testpak::descriptor::RevisionBinding;
use threadpak_testpak::muterprater::{
    EvaluationBinding, EvaluationControl, EvaluationFamilyRef, EvaluationObservation,
    EvaluationSurface,
};

fn evaluated(_input: &u8, _control: EvaluationControl) -> EvaluationObservation<u8> {
    EvaluationObservation::observed(0, 0)
}

fn relabel(
    surface: &EvaluationSurface,
    family: EvaluationFamilyRef,
    revision: RevisionBinding,
) {
    let _ = EvaluationBinding::declared(family, revision, surface.identity(), evaluated);
}

fn main() {}
