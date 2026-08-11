//! The renamed-dependency consumer fixture: the machine under a name the
//! consumer chose.
//!
//! A consumer is allowed to rename its dependencies. This crate takes
//! `tp = { package = "threadpak" }` and nothing else, exactly as an application
//! that renamed the machine would — there is no `threadpak` name in scope here
//! at all.
//!
//! That absence is the whole fixture. A renderer that hardcoded `::threadpak`
//! would emit a path naming a crate this consumer does not have, and the
//! expansion would fail to compile for a reason that has nothing to do with the
//! declaration. **That this crate compiles is the proof the binding travelled**
//! from the `#[refusal(crate = tp, …)]` clause, through the capture, the plan,
//! and the rendering, to the emitted tokens.
//!
//! It is a separate crate rather than a module of the other fixture because
//! Cargo refuses one package depending on one path package under two names —
//! which is itself the honest reason a renamed consumer is a different consumer.

#[cfg(test)]
mod tests {
    use tp::refusal::{
        CauseId, CauseOrderDeclaration, DeclaredCause, DeclaredCauseOrder, FamilyShape,
        RefusalFamily,
    };

    /// The derived family, reached through the renamed binding.
    ///
    /// The caller writes no cause identity out in full: it declares the family
    /// identity once and one local key per cause, and the derive composes the
    /// identities under band 00's canonical key grammar.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, threadpak_macros::RefusalFamily)]
    #[refusal(
        crate = tp,
        family = "consumer.renamed",
        shape = single_cause,
        order(
            NotBound = "not-bound",
            NotCovered = "not-covered",
        )
    )]
    enum RenamedDemoFamily {
        NotCovered,
        NotBound,
    }

    /// The hand-written twin, authored against the same renamed binding.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum HandWrittenRenamedFamily {
        NotCovered,
        NotBound,
    }

    impl RefusalFamily for HandWrittenRenamedFamily {
        const SHAPE: FamilyShape = FamilyShape::SingleCause;
        const SELECTION_ORDER: &'static [&'static str] = &["NotBound", "NotCovered"];
    }

    impl CauseOrderDeclaration for HandWrittenRenamedFamily {
        const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
            DeclaredCause::declared(CauseId::declared("consumer.renamed.not-bound"), "NotBound"),
            DeclaredCause::declared(
                CauseId::declared("consumer.renamed.not-covered"),
                "NotCovered",
            ),
        ]);
    }

    /// The parity proof under the renamed binding: the derived implementation
    /// and the hand-written twin declare the same shape, the same textual
    /// selection order, and the same typed cause order — identity for identity
    /// and position for position.
    #[test]
    fn the_renamed_binding_derives_the_same_facts() {
        assert_eq!(RenamedDemoFamily::SHAPE, HandWrittenRenamedFamily::SHAPE);
        assert_eq!(
            RenamedDemoFamily::SELECTION_ORDER,
            HandWrittenRenamedFamily::SELECTION_ORDER
        );
        assert_eq!(
            RenamedDemoFamily::DECLARED_ORDER.len(),
            HandWrittenRenamedFamily::DECLARED_ORDER.len()
        );
        let identical = RenamedDemoFamily::DECLARED_ORDER
            .iter()
            .zip(HandWrittenRenamedFamily::DECLARED_ORDER.iter())
            .all(|(derived, twin)| {
                derived.id() == twin.id() && derived.spelling() == twin.spelling()
            });
        assert!(identical);
    }

    /// The composed identity is the family identity joined to the local key, and
    /// the ordinal is the position in the declared order — both read back off
    /// the derived implementation rather than off anything the caller wrote.
    #[test]
    fn the_composed_identity_is_the_family_and_the_key() {
        assert_eq!(
            RenamedDemoFamily::DECLARED_ORDER
                .ordinal_of(CauseId::declared("consumer.renamed.not-covered"))
                .map(tp::refusal::CauseOrdinal::position),
            Some(1)
        );
        assert!(RenamedDemoFamily::DECLARED_ORDER.projects_to(RenamedDemoFamily::SELECTION_ORDER));
        assert_ne!(RenamedDemoFamily::NotBound, RenamedDemoFamily::NotCovered);
        assert_ne!(
            HandWrittenRenamedFamily::NotBound,
            HandWrittenRenamedFamily::NotCovered
        );
    }
}
