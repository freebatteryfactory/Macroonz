//! Kind-set accounting observed from outside: stamped rows remain usable, handwritten records cannot pass incomplete, and only a complete witness reaches `Accounted`.

use macroonz_compiler::{
    Accounted, Disposition, DispositionRecord, DispositionSet, DispositionSetError, Expansion,
    KindSet, NoQuestions, OwnerFact, SoleRole,
};

/// The fact under which this lane does not request one generated kind.
const NOT_REQUESTED: OwnerFact = OwnerFact {
    home: "kind-accounting-lane",
    name: "not-requested",
};

/// The fact under which this lane finds one generated kind inapplicable.
const NOT_APPLICABLE: OwnerFact = OwnerFact {
    home: "kind-accounting-lane",
    name: "not-applicable",
};

macroonz_compiler::kinds! {
    set = LaneKinds;
    dispositions = LaneDispositions;

    /// The first kind this lane accounts for.
    FirstKind = "lane.first", first => (), SoleRole, NoQuestions;
    /// The second kind this lane accounts for.
    SecondKind = "lane.second", second => (), SoleRole, NoQuestions;
}

/// A handwritten record that omits the second declared kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TruncatedDispositions {
    first: Disposition,
}

impl DispositionRecord for TruncatedDispositions {
    fn into_dispositions(self) -> impl Iterator<Item = (&'static str, Disposition)> {
        core::iter::once(("handwritten.first", self.first))
    }
}

/// A handwritten record with the right row count but the wrong kind at the second seat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DoubledDispositions {
    first: Disposition,
    second: Disposition,
}

impl DispositionRecord for DoubledDispositions {
    fn into_dispositions(self) -> impl Iterator<Item = (&'static str, Disposition)> {
        [
            ("handwritten.first", self.first),
            ("handwritten.first", self.second),
        ]
        .into_iter()
    }
}

/// A handwritten two-kind set whose record is structurally capable of surrendering too few rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HandwrittenKinds;

impl KindSet for HandwrittenKinds {
    type Dispositions = TruncatedDispositions;

    const NAMES: &'static [&'static str] = &["handwritten.first", "handwritten.second"];
}

/// A handwritten two-kind set whose record repeats the first named seat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DoubledKinds;

impl KindSet for DoubledKinds {
    type Dispositions = DoubledDispositions;

    const NAMES: &'static [&'static str] = &["handwritten.first", "handwritten.second"];
}

/// The stamp keeps its marker, set, record, and declaration order on one public surface.
#[test]
fn stamped_kinds_build_one_complete_disposition_witness() -> Result<(), ()> {
    let record = LaneDispositions {
        first: Disposition::NotRequested {
            because: NOT_REQUESTED,
        },
        second: Disposition::NotApplicable {
            because: NOT_APPLICABLE,
        },
    };

    assert_eq!(
        <LaneKinds as KindSet>::NAMES,
        &["lane.first", "lane.second"]
    );
    assert_eq!(
        LaneKinds::ALL,
        &[LaneKinds::FirstKind, LaneKinds::SecondKind]
    );
    assert_eq!(LaneKinds::FirstKind.name(), "lane.first");
    assert_eq!(
        record.under(LaneKinds::FirstKind),
        &Disposition::NotRequested {
            because: NOT_REQUESTED,
        }
    );

    let complete = DispositionSet::<LaneKinds>::complete(record).map_err(|_refusal| ())?;
    assert_eq!(complete.len(), 2usize);
    assert!(!complete.is_empty());
    assert_eq!(
        complete.iter().collect::<Vec<_>>(),
        vec![
            (
                "lane.first",
                &Disposition::NotRequested {
                    because: NOT_REQUESTED,
                },
            ),
            (
                "lane.second",
                &Disposition::NotApplicable {
                    because: NOT_APPLICABLE,
                },
            ),
        ]
    );

    let seating_road: fn(
        Expansion<FirstKind>,
        DispositionSet<LaneKinds>,
    ) -> Accounted<FirstKind, LaneKinds> = Accounted::seated;
    core::hint::black_box(seating_road);
    Ok(())
}

/// A consumer-owned record remains only input until the declared-name denominator agrees with every surrendered name and the whole row count.
#[test]
fn a_handwritten_record_cannot_shrink_its_kind_set() {
    let refusal = DispositionSet::<HandwrittenKinds>::complete(TruncatedDispositions {
        first: Disposition::NotRequested {
            because: NOT_REQUESTED,
        },
    });
    assert_eq!(
        refusal,
        Err(DispositionSetError::CountMismatch {
            expected: 2usize,
            observed: 1usize,
        })
    );
}

/// Matching the count cannot substitute a repeated kind for the declared second seat.
#[test]
fn a_handwritten_record_cannot_repeat_one_kind_at_another_kinds_seat() {
    let refusal = DispositionSet::<DoubledKinds>::complete(DoubledDispositions {
        first: Disposition::NotRequested {
            because: NOT_REQUESTED,
        },
        second: Disposition::NotApplicable {
            because: NOT_APPLICABLE,
        },
    });
    assert_eq!(
        refusal,
        Err(DispositionSetError::KindMismatch {
            expected: "handwritten.second",
            observed: "handwritten.first",
        })
    );
}
