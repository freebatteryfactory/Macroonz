//! Disposable generated-code runtime consumer with independent preflight and real-clock observations.
//! Fixed-structure call-count curves and fixed-call-count growing-structure curves remain distinct axes.
#![forbid(unsafe_code)]

mod measurement;
#[path = "runtime-shapes.rs"]
mod runtime_shapes;

use macroonz_harness::bench::{
    WorkConclusion, WorkCurve, WorkGapStanding, WorkJudgment, WorkJudgmentInput,
    WorkObservationRef, WorkRecorder, WorkRecordingRefusal,
};
use macroonz_harness::report::{
    FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding,
};
use macroonz_harness::runner::Invocation;
use measurement::{DOUBLE, SINGLE, Workload};
use std::hint::black_box;

const SIZES: [u64; 3] = [256, 1024, 4096];
const OBSERVATIONS: [&str; 3] = [
    "completed-operations",
    "unexpected-results",
    "consumed-checksum",
];
const CAUSE: FindingCause = FindingCause::named("runtime-pilot", "independent-behavior-or-work");

fn no_effect() {}
const fn allowed() -> bool {
    true
}
const fn denied() -> bool {
    false
}

macroonz::recipe! {
    pub mod dispatch {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State { Closed, Open }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event { Toggle, Remain }
        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, Toggle) => Open with(crate::no_effect);
                (Open, Toggle) => Closed with(crate::no_effect);
                (Open, Remain) => Open with(crate::no_effect);
            };
            absence(refused);
            projections { dispatch(apply); };
        }
    }
}

macroonz::recipe! {
    pub mod relation {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Stage { Draft, Published }
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Capability { Read, Write }
        bake! {
            vocabularies { Stage; Capability; };
            relations {
                policy(Stage, Capability) {
                    (Draft, Read) with(crate::allowed);
                    (Draft, Write) with(crate::allowed);
                    (Published, Write) with(crate::denied);
                };
            };
            postures { policy { repetition(refused); }; };
            projections {
                relation_tables {
                    policy {
                        pub fn decision(stage: &Stage, capability: &Capability) -> Option<fn() -> bool>;
                    };
                };
            };
        }
    }
}

macroonz::recipe! {
    pub mod codec {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Ledger { pub count: u16 }
        impl Ledger {
            pub const fn assembled(count: u16) -> Self { Self { count } }
        }
        bake! {
            codecs {
                ledger(Ledger) {
                    direction(round_trip);
                    refusal(LedgerDecodeError);
                    assembly(assembled, total);
                    members { count: u16 => count(required); };
                };
            };
            projections { codec; };
        }
    }
}

#[derive(Clone, Copy)]
enum Family {
    Dispatch,
    Relation,
    Codec,
    GrowingDispatch,
    GrowingRelation,
    GrowingCodec,
}

impl Family {
    const fn owner(self) -> &'static str {
        match self {
            Self::Dispatch => "runtime-dispatch",
            Self::Relation => "runtime-relation",
            Self::Codec => "runtime-codec",
            Self::GrowingDispatch => "runtime-growing-dispatch",
            Self::GrowingRelation => "runtime-growing-relation",
            Self::GrowingCodec => "runtime-growing-codec",
        }
    }

    const fn sizes(self) -> &'static [u64] {
        match self {
            Self::Dispatch | Self::Relation | Self::Codec => &SIZES,
            Self::GrowingDispatch | Self::GrowingRelation => &[2, 8, 16],
            Self::GrowingCodec => &[1, 8, 32],
        }
    }

    const fn calls(self, size: u64) -> u64 {
        match self {
            Self::Dispatch | Self::Relation | Self::Codec => size,
            Self::GrowingDispatch | Self::GrowingRelation | Self::GrowingCodec => 4096,
        }
    }

    fn workload(self) -> Workload {
        let (once, twice, four_times): (
            macroonz_harness::bench::BenchCall,
            macroonz_harness::bench::BenchCall,
            macroonz_harness::bench::BenchCall,
        ) = match self {
            Self::Dispatch => (
                |size, recorder| execute(size, 1, Family::Dispatch, recorder),
                |size, recorder| execute(size, 2, Family::Dispatch, recorder),
                |size, recorder| execute(size, 4, Family::Dispatch, recorder),
            ),
            Self::Relation => (
                |size, recorder| execute(size, 1, Family::Relation, recorder),
                |size, recorder| execute(size, 2, Family::Relation, recorder),
                |size, recorder| execute(size, 4, Family::Relation, recorder),
            ),
            Self::Codec => (
                |size, recorder| execute(size, 1, Family::Codec, recorder),
                |size, recorder| execute(size, 2, Family::Codec, recorder),
                |size, recorder| execute(size, 4, Family::Codec, recorder),
            ),
            Self::GrowingDispatch => (
                |size, recorder| execute(size, 1, Family::GrowingDispatch, recorder),
                |size, recorder| execute(size, 2, Family::GrowingDispatch, recorder),
                |size, recorder| execute(size, 4, Family::GrowingDispatch, recorder),
            ),
            Self::GrowingRelation => (
                |size, recorder| execute(size, 1, Family::GrowingRelation, recorder),
                |size, recorder| execute(size, 2, Family::GrowingRelation, recorder),
                |size, recorder| execute(size, 4, Family::GrowingRelation, recorder),
            ),
            Self::GrowingCodec => (
                |size, recorder| execute(size, 1, Family::GrowingCodec, recorder),
                |size, recorder| execute(size, 2, Family::GrowingCodec, recorder),
                |size, recorder| execute(size, 4, Family::GrowingCodec, recorder),
            ),
        };
        let judge: fn(&WorkJudgmentInput<'_>) -> WorkJudgment = match self {
            Self::Dispatch => |input: &WorkJudgmentInput<'_>| judgment(input, Family::Dispatch),
            Self::Relation => |input: &WorkJudgmentInput<'_>| judgment(input, Family::Relation),
            Self::Codec => |input: &WorkJudgmentInput<'_>| judgment(input, Family::Codec),
            Self::GrowingDispatch => {
                |input: &WorkJudgmentInput<'_>| judgment(input, Family::GrowingDispatch)
            }
            Self::GrowingRelation => {
                |input: &WorkJudgmentInput<'_>| judgment(input, Family::GrowingRelation)
            }
            Self::GrowingCodec => {
                |input: &WorkJudgmentInput<'_>| judgment(input, Family::GrowingCodec)
            }
        };
        Workload {
            owner: self.owner(),
            interval: "input-selection+generated-execution+consume+batch-recording;codec-includes-Vec-allocation-and-drop",
            sizes: self.sizes(),
            observations: &OBSERVATIONS,
            sources: &[
                include_bytes!("runtime-pilot.rs"),
                include_bytes!("runtime-shapes.rs"),
            ],
            preflight,
            judge,
            once,
            twice,
            four_times,
        }
    }
}

fn dispatch_at(index: u64) -> Option<dispatch::State> {
    use dispatch::{Event, State};
    let state = if index % 2 == 0 {
        State::Closed
    } else {
        State::Open
    };
    let event = if index % 4 < 2 {
        Event::Toggle
    } else {
        Event::Remain
    };
    black_box(dispatch::baked::apply(black_box(state), black_box(event))).ok()
}

fn relation_at(index: u64) -> Option<bool> {
    use relation::{Capability, Stage};
    let stage = if index % 2 == 0 {
        Stage::Draft
    } else {
        Stage::Published
    };
    let capability = if index % 4 < 2 {
        Capability::Read
    } else {
        Capability::Write
    };
    black_box(relation::baked::policy::decision(
        black_box(&stage),
        black_box(&capability),
    ))
    .map(|call| black_box(call)())
}

fn preflight(_: &Invocation) -> TrialConclusion {
    let checked = (|| -> Result<(), String> {
        runtime_shapes::check()?;
        use dispatch::State;
        let expected_dispatch = [
            Some(State::Open),
            Some(State::Closed),
            None,
            Some(State::Open),
        ];
        let expected_relation = [Some(true), None, Some(true), Some(false)];
        for (index, (expected_state, expected_decision)) in expected_dispatch
            .into_iter()
            .zip(expected_relation)
            .enumerate()
        {
            let index = u64::try_from(index).map_err(measurement::debug)?;
            if dispatch_at(index) != expected_state || relation_at(index) != expected_decision {
                return Err(
                    "generated lookup disagrees with the independent finite table".to_owned(),
                );
            }
        }
        // Both deliberately wrong expectations contradict the actual finite relation.
        if dispatch_at(0) == Some(State::Closed) || relation_at(3) == Some(true) {
            return Err("planted wrong independent expectation was accepted".to_owned());
        }
        for count in [0_u16, 1, 255, 256, 513, u16::MAX] {
            let value = codec::Ledger { count };
            let mut bytes = Vec::new();
            value.encode_canonical(&mut bytes);
            let expected = u64::from(count).to_be_bytes();
            if bytes != expected || codec::Ledger::decode_canonical(&expected) != Ok(value) {
                return Err(
                    "generated codec disagrees with the independent big-endian vector".to_owned(),
                );
            }
        }
        for invalid in [
            vec![],
            vec![0; 7],
            vec![0; 9],
            u64::MAX.to_be_bytes().to_vec(),
        ] {
            if codec::Ledger::decode_canonical(&invalid).is_ok() {
                return Err(
                    "truncated, trailing, or out-of-width codec input was accepted".to_owned(),
                );
            }
        }
        Ok(())
    })();
    match checked {
        Ok(()) => TrialConclusion::Passed,
        Err(error) => {
            eprintln!("runtime preflight: {error}");
            TrialConclusion::Refused(TrialFinding::established(
                FailureClass::RefusedByCheck,
                CAUSE,
                FindingLocation::at(file!(), line!()),
                None,
            ))
        }
    }
}

fn operation(family: Family, size: u64, index: u64) -> Result<u64, ()> {
    match family {
        Family::Dispatch => Ok(match dispatch_at(index) {
            Some(dispatch::State::Closed) => 1,
            Some(dispatch::State::Open) => 2,
            None => 3,
        }),
        Family::Relation => Ok(match relation_at(index) {
            Some(false) => 1,
            Some(true) => 2,
            None => 3,
        }),
        Family::Codec => {
            let count = u16::try_from(index).map_err(|_| ())?;
            let value = codec::Ledger {
                count: black_box(count),
            };
            let mut bytes = Vec::new();
            value.encode_canonical(&mut bytes);
            let decoded = codec::Ledger::decode_canonical(black_box(&bytes)).map_err(|_| ())?;
            black_box(decoded == value)
                .then_some(u64::from(black_box(decoded.count)) + 1)
                .ok_or(())
        }
        Family::GrowingDispatch | Family::GrowingRelation | Family::GrowingCodec => {
            runtime_shapes::operation(family, size, index)
        }
    }
}

fn execute(
    size: u64,
    repetitions: u64,
    family: Family,
    recorder: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let [completed, unexpected, checksum] =
        OBSERVATIONS.map(|name| WorkObservationRef::named(family.owner(), name));
    let completed = completed.map_err(WorkRecordingRefusal::ObservationName)?;
    let unexpected = unexpected.map_err(WorkRecordingRefusal::ObservationName)?;
    let checksum = checksum.map_err(WorkRecordingRefusal::ObservationName)?;
    for _ in 0..repetitions {
        let mut done = 0_u64;
        let mut errors = 0_u64;
        let mut consumed = 0_u64;
        for index in 0..black_box(family.calls(size)) {
            match operation(family, black_box(size), black_box(index)) {
                Ok(value) => {
                    done = done
                        .checked_add(1)
                        .ok_or(WorkRecordingRefusal::AmountOverflow {
                            observation: completed,
                            input_size: size,
                        })?;
                    consumed = consumed.checked_add(black_box(value)).ok_or(
                        WorkRecordingRefusal::AmountOverflow {
                            observation: checksum,
                            input_size: size,
                        },
                    )?;
                }
                Err(()) => {
                    errors = errors
                        .checked_add(1)
                        .ok_or(WorkRecordingRefusal::AmountOverflow {
                            observation: unexpected,
                            input_size: size,
                        })?
                }
            }
        }
        recorder.record(completed, done)?;
        recorder.record(unexpected, errors)?;
        recorder.record(checksum, consumed)?;
    }
    Ok(())
}

fn holds(curve: &WorkCurve, samples: u32, repetitions: u64, family: Family) -> bool {
    curve.points().len() == family.sizes().len()
        && curve
            .points()
            .iter()
            .zip(family.sizes())
            .all(|(point, size)| {
                let [completed, unexpected, checksum] = point.counts() else {
                    return false;
                };
                let names =
                    OBSERVATIONS.map(|name| WorkObservationRef::named(family.owner(), name));
                point.input_size() == *size
                    && names
                        .iter()
                        .zip([completed, unexpected, checksum])
                        .all(|(name, count)| {
                            name.as_ref().is_ok_and(|name| *name == count.observation())
                        })
                    && Some(completed.count())
                        == u64::from(samples)
                            .checked_mul(repetitions)
                            .and_then(|calls| calls.checked_mul(family.calls(*size)))
                    && unexpected.count() == 0
                    && checksum.count() > 0
            })
}

fn judgment(input: &WorkJudgmentInput<'_>, family: Family) -> WorkJudgment {
    let repetitions = input.formula().and_then(|formula| match formula.bytes() {
        SINGLE => Some(1_u64),
        DOUBLE => Some(2_u64),
        _ => None,
    });
    let measured = repetitions
        .is_some_and(|count| holds(input.measured(), input.budgets().samples(), count, family));
    let worse = repetitions
        .and_then(|count| count.checked_mul(2))
        .is_some_and(|count| {
            holds(
                input.planted_worse(),
                input.budgets().samples(),
                count,
                family,
            )
        });
    let gap = measured
        && worse
        && input
            .measured()
            .points()
            .iter()
            .zip(input.planted_worse().points())
            .all(|(left, right)| {
                left.counts()
                    .iter()
                    .zip(right.counts())
                    .all(|(left, right)| left.count().checked_mul(2) == Some(right.count()))
            });
    WorkJudgment::stated(
        if measured {
            WorkConclusion::Satisfied
        } else {
            WorkConclusion::Refused(CAUSE)
        },
        if worse {
            WorkConclusion::Refused(CAUSE)
        } else {
            WorkConclusion::Satisfied
        },
        if gap {
            WorkGapStanding::Distinguished
        } else {
            WorkGapStanding::NotDistinguished(CAUSE)
        },
    )
}

fn main() -> Result<(), String> {
    for family in [
        Family::Dispatch,
        Family::Relation,
        Family::Codec,
        Family::GrowingDispatch,
        Family::GrowingRelation,
        Family::GrowingCodec,
    ] {
        measurement::measure(&family.workload())?;
    }
    Ok(())
}
