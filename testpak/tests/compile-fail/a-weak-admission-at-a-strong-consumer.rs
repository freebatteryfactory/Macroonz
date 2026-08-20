//! The reversal for band 00's coverage hierarchy: the weaker admission cannot
//! stand in for the stronger one.
//!
//! The family below declares its typed cause order and its textual projection,
//! and the two agree — so `admit_order` would have admitted it. The caller runs
//! `admit_shape` instead and reaches for the order-sensitive consumer with what
//! that road returned. Nothing about the DECLARATION is wrong here; what is
//! wrong is the claim, and the claim is the whole content of the witness. A
//! coverage carried as a runtime field would have let this compile and refused
//! at the branch, or not refused at all; carried as a type parameter, the
//! substitution has no expression.

use threadpak::refusal::{
    CauseId, CauseOrderDeclaration, DeclaredCause, DeclaredCauseOrder, FamilyShape, LocalCauseKey,
    RefusalFamily, RefusalFamilyId, admit_shape,
};

struct DemoFamily;

impl RefusalFamily for DemoFamily {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
    const SELECTION_ORDER: &'static [&'static str] = &["NotCanonical"];
}

impl CauseOrderDeclaration for DemoFamily {
    const DECLARED_ORDER: DeclaredCauseOrder =
        DeclaredCauseOrder::declared(&[DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("demo.weak-admission"),
                LocalCauseKey::declared("not-canonical"),
            ),
            "NotCanonical",
        )]);
}

fn main() {
    if let Ok(weak) = admit_shape::<DemoFamily>() {
        let _order = weak.cause_order();
    }
}
