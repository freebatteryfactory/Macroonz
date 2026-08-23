//! Declarative refusal-family implementations owned by this crate.

use crate::{
    BoundedConstruction, CauseId, CauseOrderDeclaration, DeclaredCause, DeclaredCauseOrder,
    FamilyAdmission, FamilyShape, LocalCauseKey, NonEmptyBoundedConstruction, RefusalFamily,
    RefusalFamilyId,
};

impl RefusalFamily for BoundedConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
}

impl CauseOrderDeclaration for BoundedConstruction {
    const DECLARED_ORDER: DeclaredCauseOrder =
        DeclaredCauseOrder::declared(&[DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("root.bounded-construction"),
                LocalCauseKey::declared("over-limit"),
            ),
            "OverLimit",
        )]);
}

impl RefusalFamily for NonEmptyBoundedConstruction {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
}

impl CauseOrderDeclaration for NonEmptyBoundedConstruction {
    const DECLARED_ORDER: DeclaredCauseOrder =
        DeclaredCauseOrder::declared(&[DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("root.non-empty-bounded-construction"),
                LocalCauseKey::declared("over-limit"),
            ),
            "OverLimit",
        )]);
}

impl RefusalFamily for FamilyAdmission {
    const SHAPE: FamilyShape = FamilyShape::SingleCause;
}

impl CauseOrderDeclaration for FamilyAdmission {
    const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
        DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("refusal.family-admission"),
                LocalCauseKey::declared("not-shape-coherent"),
            ),
            "NotShapeCoherent",
        ),
        DeclaredCause::declared(
            CauseId::declared(
                RefusalFamilyId::declared("refusal.family-admission"),
                LocalCauseKey::declared("not-projected"),
            ),
            "NotProjected",
        ),
    ]);
}
