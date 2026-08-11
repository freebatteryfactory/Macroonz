//! The one compile-time proof surface for the qualification plane, sectioned by
//! seat.
//!
//! A law that cannot fail is not a law. Each law below states the reversal it is
//! owed; the plane's own reversals land beside the reversals it holds for the
//! machine.

mod plan {
    use crate::plan::RedTwinLedger;

    /// law: plan.discharged-never-exceeds-expected — a ledger cannot report
    /// more red twins discharged than its denominator ever named. The road past
    /// the denominator returns no ledger at all, so an overrun is not a state
    /// to detect afterwards.
    /// Owed reversal (red twin): a `discharge` that saturates or clamps instead
    /// of refusing must break this law.
    #[test]
    fn discharged_never_exceeds_expected() {
        let opened = RedTwinLedger::opened(2);
        assert_eq!(opened.expected(), 2);
        assert_eq!(opened.discharged(), 0);
        assert_eq!(opened.outstanding(), 2);

        // Two discharges fit the denominator of two, and the third does not.
        // The road past it yields no ledger at all rather than a clamped one.
        let once = opened.discharge();
        assert!(once.is_some_and(|led| led.discharged() == 1 && led.outstanding() == 1));

        let settled = once.and_then(RedTwinLedger::discharge);
        assert!(
            settled.is_some_and(|led| led.discharged() == led.expected() && led.outstanding() == 0)
        );

        let overrun = settled.and_then(RedTwinLedger::discharge);
        assert!(overrun.is_none());
    }

    /// law: plan.an-empty-denominator-discharges-nothing — a plan expecting no
    /// red twins is settled from the moment it opens, and there is no road that
    /// invents a discharge under it.
    /// Owed reversal: a ledger admitting a discharge under a zero denominator
    /// must break this law.
    #[test]
    fn an_empty_denominator_discharges_nothing() {
        let empty = RedTwinLedger::opened(0);
        assert_eq!(empty.outstanding(), 0);
        assert!(empty.discharge().is_none());
    }
}

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
