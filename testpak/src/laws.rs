//! The one compile-time proof surface for the qualification plane, sectioned by
//! seat.
//!
//! A law that cannot fail is not a law. Each law below states the reversal it is
//! owed; the plane's own reversals land beside the reversals it holds for the
//! machine.

mod judge {
    use crate::judge::RenderVerdict;

    /// The closed verdict roster, proven closed by an exhaustive match.
    const fn verdict_index(verdict: RenderVerdict) -> usize {
        match verdict {
            RenderVerdict::Conforms => 0,
            RenderVerdict::Deviates => 1,
            RenderVerdict::Unreadable => 2,
        }
    }

    /// law: judge.verdicts-are-three-and-none-is-silence — the roster is closed
    /// at three distinct answers, and the third is a stated failure class rather
    /// than the absence of an answer. `Unreadable` is not `Conforms` and is not
    /// `Deviates`; a caller that folded it into either would be asserting over a
    /// reading that never happened.
    /// Owed reversal (red twin): an `Option<RenderVerdict>` road, or any
    /// conversion that maps `Unreadable` onto another verdict, must break this
    /// law.
    #[test]
    fn verdicts_are_three_and_none_is_silence() {
        let roster = [
            RenderVerdict::Conforms,
            RenderVerdict::Deviates,
            RenderVerdict::Unreadable,
        ];
        assert!(
            roster
                .iter()
                .copied()
                .map(verdict_index)
                .enumerate()
                .all(|(position, index)| index == position)
        );
        assert_ne!(RenderVerdict::Unreadable, RenderVerdict::Conforms);
        assert_ne!(RenderVerdict::Unreadable, RenderVerdict::Deviates);
    }
}
