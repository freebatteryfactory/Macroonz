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
//! Two mechanisms are read here under that one binding, and they are proven by
//! opposite halves of the same fact. The refusal-family DERIVE writes paths for
//! the CONSUMER, so its proof is that the rendering names `tp`. The
//! `closed_register!` STAMP writes paths for ITSELF, through `$crate`, so its
//! proof is that the expansion resolves at all under a name the machine never
//! hears about — and that the constant its refusal sentence names is reachable
//! under the consumer's own binding, which is the repair route that sentence
//! sends a reader down.
//!
//! It is a separate crate rather than a module of the other fixture because
//! Cargo refuses one package depending on one path package under two names —
//! which is itself the honest reason a renamed consumer is a different consumer.

#[cfg(test)]
mod tests {
    use tp::refusal::{
        CauseId, CauseOrderDeclaration, DeclaredCause, DeclaredCauseOrder, FamilyShape,
        LocalCauseKey, RefusalFamily, RefusalFamilyId,
    };

    /// The family identity both implementations below declare, written once
    /// here because the hand-written twin states it as a value exactly as the
    /// derived implementation emits it.
    const FAMILY: RefusalFamilyId = RefusalFamilyId::declared("consumer.renamed");

    // `closed_register!` is exported at the machine's crate root, so this
    // consumer reaches it as `tp::closed_register!`. Every path the expansion
    // writes for ITSELF goes through `$crate`, which resolves to the machine
    // whatever this crate chose to call it — so a stamp that spelled its own
    // crate name instead would name a crate this consumer does not have, and
    // this module would not compile. That it compiles is the proof, exactly as
    // it is for the derive above.
    tp::closed_register! {
        /// The renamed consumer's own closed roster, stamped through the
        /// renamed binding.
        ///
        /// Synthetic and deliberately meaningless: it stands for no vocabulary,
        /// and its rows are numbered only because what is under judgement here
        /// is the stamp reaching a consumer that renamed the machine.
        enum RenamedRow {
            /// The first row.
            First = "first", "the first row";
            /// The second row.
            Second = "second", "the second row";
            /// The third row.
            Third = "third", "the third row";
        }
    }

    /// The derived family, reached through the renamed binding.
    ///
    /// The caller writes no cause identity out in full: it declares the family
    /// identity once and one local key per cause, and the derive mints each
    /// identity as band 00's pair — the family seat and the local seat, each
    /// through its own constructor on the renamed binding.
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
            DeclaredCause::declared(
                CauseId::declared(FAMILY, LocalCauseKey::declared("not-bound")),
                "NotBound",
            ),
            DeclaredCause::declared(
                CauseId::declared(FAMILY, LocalCauseKey::declared("not-covered")),
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

    /// The minted identity is the family seat and the local seat, and the
    /// ordinal is the position in the declared order — all of it read back off
    /// the derived implementation rather than off anything the caller wrote.
    #[test]
    fn the_minted_identity_is_the_family_and_the_key() {
        assert_eq!(
            RenamedDemoFamily::DECLARED_ORDER
                .ordinal_of(CauseId::declared(
                    FAMILY,
                    LocalCauseKey::declared("not-covered")
                ))
                .map(tp::refusal::CauseOrdinal::position),
            Some(1)
        );
        assert!(
            RenamedDemoFamily::DECLARED_ORDER
                .iter()
                .all(|row| row.id().family() == FAMILY)
        );
        assert!(RenamedDemoFamily::DECLARED_ORDER.projects_to(RenamedDemoFamily::SELECTION_ORDER));
        assert_ne!(RenamedDemoFamily::NotBound, RenamedDemoFamily::NotCovered);
        assert_ne!(
            HandWrittenRenamedFamily::NotBound,
            HandWrittenRenamedFamily::NotCovered
        );
    }

    /// The stamp expands through the renamed binding, and every reading it
    /// writes reads back: the declared order, each row's exact position, and
    /// both declared spellings.
    ///
    /// The positions are held against the roster's OWN layout rather than
    /// against numbers written here, the same relational shape the outside
    /// consumer seat in testpak uses. A stamp that paired rows with anything
    /// other than its declared supply fails this without a number having to be
    /// maintained on this side.
    #[test]
    fn the_stamp_expands_through_the_renamed_binding() {
        assert_eq!(
            RenamedRow::ALL,
            [RenamedRow::First, RenamedRow::Second, RenamedRow::Third]
        );
        assert!(
            RenamedRow::ALL
                .iter()
                .enumerate()
                .all(|(position, row)| usize::from(row.slot()) == position)
        );
        assert_eq!(RenamedRow::Third.stable_name(), "third");
        assert_eq!(RenamedRow::Third.described(), "the third row");
    }

    /// The ceiling the stamp's refusal names, resolved under the name this
    /// consumer chose.
    ///
    /// The refusal sentence names `CLOSED_REGISTER_ROW_CEILING` and no crate
    /// path, because a compiler message has no way to learn what this crate
    /// called its dependency. This is that sentence's instruction carried out
    /// by the reader it was written for: the constant resolves under the
    /// consumer's own binding, and the position the stamp paired the last row
    /// with lies inside the supply that constant measures.
    ///
    /// The ceiling's MAGNITUDE is not this file's claim. That belongs to
    /// `testpak/tests/stamp_row_ceiling.rs`, which spends the supply to its last
    /// position; and the value itself is recorded once, on the constant's own
    /// documentation, and nowhere else. What is established here is only that
    /// the name the refusal gives resolves for a consumer the machine never
    /// hears about.
    #[test]
    fn the_ceiling_resolves_under_the_consumers_own_binding() {
        assert!(usize::from(RenamedRow::Third.slot()) < tp::CLOSED_REGISTER_ROW_CEILING);
    }
}
