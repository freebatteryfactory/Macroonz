//! The outside consumer fixture: the composition proof, shaped like a real
//! consumer.
//!
//! A compiler service never depends on its frontend surfaces, even for tests.
//! So the question "does a caller who holds the machine's types and wears the
//! shell's derive actually compile?" cannot be answered from inside either
//! participant — an answer from inside is the participant grading itself, and
//! it buys that answer with a dependency edge that reverses the topology.
//!
//! This crate answers it from outside. It depends on `threadpak` and on
//! `threadpak-macros`, exactly as an application would, and on neither of their
//! internals. It exports nothing: the whole crate is the fixture, and its tests
//! are the proof.
//!
//! It lives under `xtask/fixtures/` because it is tooling — first-class, never
//! on the production dependency path.

#[cfg(test)]
mod tests {
    use threadpak::declaration::FrontendRole;
    use threadpak::refusal::{
        CauseId, CauseOrderDeclaration, DeclaredCause, DeclaredCauseOrder, FamilyShape,
        RefusalFamily,
    };
    use threadpak_macros::{RefusalFamilyDerive, ThreadpakSkeleton};

    /// A consumer shaped like an application: it holds a type owned by the
    /// machine and wears a derive owned by the expansion shell. Nothing here
    /// reaches the services the shell is a surface over.
    #[derive(ThreadpakSkeleton)]
    struct Consumer {
        role: FrontendRole,
    }

    /// The composition proof: the machine's public type and the shell's derive
    /// meet on one struct in a crate that owns neither.
    #[test]
    fn a_consumer_composes_the_machine_and_the_shell() {
        let consumer = Consumer {
            role: FrontendRole::RustDeclaration,
        };
        assert!(matches!(consumer.role, FrontendRole::RustDeclaration));
        assert_ne!(consumer.role, FrontendRole::ApplicationLanguage);
    }

    /// The derived family. The caller states three things — the shape, the
    /// variants, and one stable identity per cause — and writes no
    /// selection-order string anywhere. The declared order is deliberately NOT
    /// the order the variants are written in, so a derive that read the body
    /// layout instead of the order clause would fail the parity below.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, RefusalFamilyDerive)]
    #[refusal(
        shape = single_cause,
        order(
            NotCanonical = "consumer.demo.not-canonical",
            NotAdmitted = "consumer.demo.not-admitted",
            Unbounded = "consumer.demo.unbounded",
        )
    )]
    enum DerivedDemoFamily {
        NotAdmitted,
        Unbounded,
        NotCanonical,
    }

    /// The hand-written twin: the same family, authored the way every refusal
    /// family in the machine is authored today. It is the bar the derive has to
    /// meet, and it is written here — outside the services — so that neither
    /// participant grades itself.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum HandWrittenDemoFamily {
        NotAdmitted,
        Unbounded,
        NotCanonical,
    }

    impl RefusalFamily for HandWrittenDemoFamily {
        const SHAPE: FamilyShape = FamilyShape::SingleCause;
        const SELECTION_ORDER: &'static [&'static str] =
            &["NotCanonical", "NotAdmitted", "Unbounded"];
    }

    impl CauseOrderDeclaration for HandWrittenDemoFamily {
        const DECLARED_ORDER: DeclaredCauseOrder = DeclaredCauseOrder::declared(&[
            DeclaredCause::declared(
                CauseId::declared("consumer.demo.not-canonical"),
                "NotCanonical",
            ),
            DeclaredCause::declared(
                CauseId::declared("consumer.demo.not-admitted"),
                "NotAdmitted",
            ),
            DeclaredCause::declared(CauseId::declared("consumer.demo.unbounded"), "Unbounded"),
        ]);
    }

    /// The parity proof: the derived implementation and the hand-written twin
    /// declare the same shape, the same textual selection order, and the same
    /// typed cause order — identity for identity and position for position.
    #[test]
    fn the_derived_family_matches_its_hand_written_twin() {
        assert_eq!(DerivedDemoFamily::SHAPE, HandWrittenDemoFamily::SHAPE);
        assert_eq!(
            DerivedDemoFamily::SELECTION_ORDER,
            HandWrittenDemoFamily::SELECTION_ORDER
        );
        assert_eq!(
            DerivedDemoFamily::DECLARED_ORDER.len(),
            HandWrittenDemoFamily::DECLARED_ORDER.len()
        );
        let identical = DerivedDemoFamily::DECLARED_ORDER
            .iter()
            .zip(HandWrittenDemoFamily::DECLARED_ORDER.iter())
            .all(|(derived, twin)| {
                derived.id() == twin.id() && derived.spelling() == twin.spelling()
            });
        assert!(identical);
    }

    /// The projection law holds of the derived implementation exactly as it
    /// holds of the twin: the textual selection order is the typed order's
    /// projection, and the derive emitted it from the typed rows rather than
    /// from anything the caller wrote as a string.
    #[test]
    fn the_derived_selection_order_projects_the_derived_typed_order() {
        assert!(DerivedDemoFamily::DECLARED_ORDER.projects_to(DerivedDemoFamily::SELECTION_ORDER));
        assert!(
            HandWrittenDemoFamily::DECLARED_ORDER
                .projects_to(HandWrittenDemoFamily::SELECTION_ORDER)
        );
        assert_eq!(
            DerivedDemoFamily::DECLARED_ORDER
                .ordinal_of(CauseId::declared("consumer.demo.unbounded"))
                .map(threadpak::refusal::CauseOrdinal::position),
            Some(2)
        );
    }

    /// The values themselves still behave like an ordinary Rust enum: the
    /// derive added declared facts and took nothing away.
    #[test]
    fn the_derived_family_is_still_an_ordinary_enum() {
        let causes = [
            DerivedDemoFamily::NotCanonical,
            DerivedDemoFamily::NotAdmitted,
            DerivedDemoFamily::Unbounded,
        ];
        let twins = [
            HandWrittenDemoFamily::NotCanonical,
            HandWrittenDemoFamily::NotAdmitted,
            HandWrittenDemoFamily::Unbounded,
        ];
        assert_eq!(causes.len(), twins.len());
        assert_ne!(causes.first(), causes.get(1));
        assert_ne!(twins.first(), twins.get(1));
        assert_eq!(causes.first(), Some(&DerivedDemoFamily::NotCanonical));
    }
}
