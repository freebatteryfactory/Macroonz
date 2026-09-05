//! Disposable real-execution curves for density, codec width, late refusal, and bounded vocabulary size.
//! Each timed call constructs and compiles its input again; a recorded count never substitutes for execution.
#![forbid(unsafe_code)]

mod measurement;
mod resident;

use macroonz_compiler::diagnostic::Observed;
use macroonz_compiler::recipe::{HarnessPosture, VOCABULARY_LIMIT, bake};
use macroonz_compiler::{CrateBinding, Door, Producer, TextCapture};
use macroonz_harness::bench::{
    BenchCall, WorkConclusion, WorkCurve, WorkGapStanding, WorkJudgment, WorkJudgmentInput,
    WorkObservationRef, WorkRecorder, WorkRecordingRefusal,
};
use macroonz_harness::report::{
    FailureClass, FindingCause, FindingLocation, TrialConclusion, TrialFinding,
};
use macroonz_harness::runner::Invocation;
use measurement::{DOUBLE, SINGLE, Workload};
use std::hint::black_box;

const OBSERVATIONS: [&str; 3] = [
    "completed-compilations",
    "unexpected-results",
    "consumed-output-bytes",
];
const CAUSE: FindingCause = FindingCause::named("compiler-axes", "declared-work-disagrees");
const DOOR: Door = Door::declared(
    "compiler-axes",
    "compiler-axes.recipe",
    "compiler-axes::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "compiler-axes",
        name: "recipe",
    },
);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Density,
    CodecWidth,
    LateDuplicate,
    NearLimit,
}

impl Family {
    const fn owner(self) -> &'static str {
        match self {
            Self::Density => "compiler-density",
            Self::CodecWidth => "compiler-codec-width",
            Self::LateDuplicate => "compiler-late-duplicate",
            Self::NearLimit => "compiler-near-limit",
        }
    }

    const fn sizes(self) -> &'static [u64] {
        match self {
            Self::Density => &[4, 16, 64],
            Self::CodecWidth => &[1, 8, 32],
            Self::LateDuplicate => &[8, 32, 127],
            Self::NearLimit => &[62, 63, 64],
        }
    }

    fn workload(self) -> Workload {
        let (once, twice, four_times): (BenchCall, BenchCall, BenchCall) = match self {
            Self::Density => (
                |size, into| execute(Self::Density, size, 1, into),
                |size, into| execute(Self::Density, size, 2, into),
                |size, into| execute(Self::Density, size, 4, into),
            ),
            Self::CodecWidth => (
                |size, into| execute(Self::CodecWidth, size, 1, into),
                |size, into| execute(Self::CodecWidth, size, 2, into),
                |size, into| execute(Self::CodecWidth, size, 4, into),
            ),
            Self::LateDuplicate => (
                |size, into| execute(Self::LateDuplicate, size, 1, into),
                |size, into| execute(Self::LateDuplicate, size, 2, into),
                |size, into| execute(Self::LateDuplicate, size, 4, into),
            ),
            Self::NearLimit => (
                |size, into| execute(Self::NearLimit, size, 1, into),
                |size, into| execute(Self::NearLimit, size, 2, into),
                |size, into| execute(Self::NearLimit, size, 4, into),
            ),
        };
        let judge = match self {
            Self::Density => |input: &WorkJudgmentInput<'_>| judgment(input, Self::Density),
            Self::CodecWidth => |input: &WorkJudgmentInput<'_>| judgment(input, Self::CodecWidth),
            Self::LateDuplicate => {
                |input: &WorkJudgmentInput<'_>| judgment(input, Self::LateDuplicate)
            }
            Self::NearLimit => |input: &WorkJudgmentInput<'_>| judgment(input, Self::NearLimit),
        };
        Workload {
            owner: self.owner(),
            interval: "input-construction+capture+bake+canonical-output-or-exact-refusal+drop+recording",
            sizes: self.sizes(),
            observations: &OBSERVATIONS,
            sources: &[
                include_bytes!("compiler-axes.rs"),
                include_bytes!("resident.rs"),
            ],
            preflight,
            judge,
            once,
            twice,
            four_times,
        }
    }
}

fn variants(size: u64) -> String {
    (0..size)
        .map(|index| format!("V{index}"))
        .collect::<Vec<_>>()
        .join(",")
}

fn relation_source(rows: u64, duplicate: bool) -> String {
    let side = if duplicate { 16 } else { 8 };
    let mut source = format!(
        "pub mod measured {{ pub enum Node {{ {} }} bake! {{ vocabularies {{ Node; }}; relations {{ links(Node, Node) {{",
        variants(side)
    );
    for index in 0..rows {
        source.push_str(&format!("(V{}, V{});", index / side, index % side));
    }
    if duplicate {
        source.push_str("(V0, V0);");
    }
    source.push_str("}; }; postures { links { repetition(refused); }; }; projections { relation_tables { links; }; }; } }");
    source
}

fn source(family: Family, size: u64) -> String {
    match family {
        Family::Density => relation_source(size, false),
        Family::LateDuplicate => relation_source(size, true),
        Family::NearLimit => format!(
            "pub mod measured {{ pub enum Choice {{ {} }} bake! {{ vocabularies {{ Choice; }}; projections {{ companions; }}; }} }}",
            variants(size)
        ),
        Family::CodecWidth => {
            let mut source = String::from("pub mod measured { pub struct Ledger {");
            for index in 0..size {
                source.push_str(&format!("pub field{index}: u16,"));
            }
            source.push_str("} bake! { codecs { ledger(Ledger) { direction(encode); refusal(LedgerError); assembly(assembled, total); members {");
            for index in 0..size {
                source.push_str(&format!("field{index}: u16 => count(required);"));
            }
            source.push_str("}; }; }; projections { codec; }; } }");
            source
        }
    }
}

fn compile(family: Family, size: u64, inspect: bool) -> Result<Vec<u8>, String> {
    let material = source(family, size);
    let capture = TextCapture::read(&material).map_err(measurement::debug)?;
    let result = bake(capture.input(), HarnessPosture::Available, &DOOR);
    if family == Family::LateDuplicate {
        let refusal = result.err().ok_or("the late duplicate was accepted")?;
        let summary = refusal.summary();
        let position = summary
            .strip_prefix("compiler-axes: the declaration was not read: relation `links` states endpoint pair `V0` and `V0` more than once (at semantic-origin position ")
            .and_then(|tail| tail.strip_suffix(')'))
            .and_then(|position| position.parse::<usize>().ok());
        if position.is_none() || refusal.observed() != Observed::IdentityDisagreement {
            return Err(format!("wrong late-refusal cause: {summary}"));
        }
        return Ok(summary.as_bytes().to_vec());
    }
    let baked = result.map_err(measurement::debug)?;
    if inspect {
        let account = baked.projection().plan().content();
        let size = usize::try_from(size).map_err(measurement::debug)?;
        match family {
            Family::Density => {
                let relation = account.relation("links").ok_or("relation missing")?;
                let expected = (0..8)
                    .flat_map(|left| {
                        (0..8).map(move |right| (format!("V{left}"), format!("V{right}")))
                    })
                    .take(size)
                    .collect::<Vec<_>>();
                let actual = relation
                    .rows()
                    .map(|row| (row.left().to_owned(), row.right().to_owned()))
                    .collect::<Vec<_>>();
                if actual != expected {
                    return Err("informed endpoints differ from the finite grid".to_owned());
                }
            }
            Family::NearLimit => {
                let vocabulary = account.vocabulary("Choice").ok_or("vocabulary missing")?;
                let actual = vocabulary
                    .members()
                    .members()
                    .map(|member| member.spelling().to_owned())
                    .collect::<Vec<_>>();
                let expected = (0..size)
                    .map(|index| format!("V{index}"))
                    .collect::<Vec<_>>();
                if actual != expected {
                    return Err("informed vocabulary differs from its declared sequence".to_owned());
                }
            }
            Family::CodecWidth => {
                let codec = account.codec("ledger").ok_or("codec missing")?;
                if codec.content().shape.count() != size {
                    return Err("informed codec field denominator changed".to_owned());
                }
            }
            Family::LateDuplicate => {
                return Err("refused work reached the accepted account".to_owned());
            }
        }
    }
    let output = baked
        .emit()
        .tokens()
        .ok_or("no emitted Rust")?
        .canonical_bytes();
    if output.is_empty() {
        return Err("empty emitted Rust".to_owned());
    }
    Ok(output)
}

fn preflight(_: &Invocation) -> TrialConclusion {
    let checked = (|| -> Result<(), String> {
        if VOCABULARY_LIMIT != 64 {
            return Err("near-limit witness must be reviewed for the changed maximum".to_owned());
        }
        for family in [
            Family::Density,
            Family::CodecWidth,
            Family::LateDuplicate,
            Family::NearLimit,
        ] {
            for &size in family.sizes() {
                let first = compile(family, size, true)?;
                if first != compile(family, size, true)? {
                    return Err("repeatable output changed".to_owned());
                }
                println!(
                    "preflight,{},{size},input-bytes={},consumed-output-bytes={}",
                    family.owner(),
                    source(family, size).len(),
                    first.len()
                );
            }
        }
        let over = source(Family::NearLimit, 65);
        let captured = TextCapture::read(&over).map_err(measurement::debug)?;
        let refusal = bake(captured.input(), HarnessPosture::Available, &DOOR)
            .err()
            .ok_or("first-over vocabulary control was accepted")?;
        if refusal.observed() != Observed::BoundExceeded {
            return Err(format!(
                "first-over vocabulary had the wrong cause: {}",
                refusal.summary()
            ));
        }
        // The paired accepted relation prevents a generic refusal from standing in for duplicate detection.
        if compile(Family::Density, 8, true).is_err() {
            return Err("paired lawful relation refused".to_owned());
        }
        Ok(())
    })();
    match checked {
        Ok(()) => TrialConclusion::Passed,
        Err(error) => {
            eprintln!("compiler axes preflight: {error}");
            TrialConclusion::Refused(TrialFinding::established(
                FailureClass::RefusedByCheck,
                CAUSE,
                FindingLocation::at(file!(), line!()),
                None,
            ))
        }
    }
}

fn execute(
    family: Family,
    size: u64,
    repetitions: u64,
    into: &mut WorkRecorder,
) -> Result<(), WorkRecordingRefusal> {
    let [completed, unexpected, bytes] =
        OBSERVATIONS.map(|name| WorkObservationRef::named(family.owner(), name));
    let completed = completed.map_err(WorkRecordingRefusal::ObservationName)?;
    let unexpected = unexpected.map_err(WorkRecordingRefusal::ObservationName)?;
    let bytes = bytes.map_err(WorkRecordingRefusal::ObservationName)?;
    for _ in 0..repetitions {
        match compile(family, black_box(size), false) {
            Ok(output) => {
                let length = u64::try_from(output.len()).map_err(|_| {
                    WorkRecordingRefusal::AmountOverflow {
                        observation: bytes,
                        input_size: size,
                    }
                })?;
                drop(black_box(output));
                into.record(completed, 1)?;
                into.record(bytes, length)?;
            }
            Err(_) => into.record(unexpected, 1)?,
        }
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
                let [completed, unexpected, bytes] = point.counts() else {
                    return false;
                };
                let names =
                    OBSERVATIONS.map(|name| WorkObservationRef::named(family.owner(), name));
                point.input_size() == *size
                    && names
                        .iter()
                        .zip([completed, unexpected, bytes])
                        .all(|(name, count)| {
                            name.as_ref().is_ok_and(|name| *name == count.observation())
                        })
                    && Some(completed.count()) == u64::from(samples).checked_mul(repetitions)
                    && unexpected.count() == 0
                    && bytes.count() > 0
            })
}

fn judgment(input: &WorkJudgmentInput<'_>, family: Family) -> WorkJudgment {
    let repeats = input.formula().and_then(|formula| match formula.bytes() {
        SINGLE => Some(1_u64),
        DOUBLE => Some(2_u64),
        _ => None,
    });
    let measured = repeats
        .is_some_and(|count| holds(input.measured(), input.budgets().samples(), count, family));
    let worse = repeats
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
    let mut arguments = std::env::args().skip(1);
    if let Some(mode) = arguments.next() {
        if mode != "memory" {
            return Err("the only explicit axes mode is memory".to_owned());
        }
        let family = match arguments.next().as_deref() {
            Some("density") => Family::Density,
            Some("codec-width") => Family::CodecWidth,
            Some("late-duplicate") => Family::LateDuplicate,
            Some("near-limit") => Family::NearLimit,
            _ => return Err("one declared compiler family is required".to_owned()),
        };
        let size = arguments
            .next()
            .ok_or("one declared input size is required")?
            .parse::<u64>()
            .map_err(measurement::debug)?;
        if arguments.next().is_some() || !family.sizes().contains(&size) {
            return Err(
                "memory input must be one declared family/size pair with no extra argument"
                    .to_owned(),
            );
        }
        let material = compile(family, black_box(size), true)?;
        let checksum = resident::checksum(&material)?;
        println!(
            "memory-compiler source={} target={} toolchain={} family={} input={size} retained-bytes={} consumed-checksum={checksum}",
            env!("PILOT_SOURCE"),
            env!("PILOT_TARGET"),
            env!("PILOT_TOOLCHAIN"),
            family.owner(),
            material.len()
        );
        resident::wait_for_release(&material, checksum)?;
        drop(black_box(material));
        return Ok(());
    }
    for family in [
        Family::Density,
        Family::CodecWidth,
        Family::LateDuplicate,
        Family::NearLimit,
    ] {
        measurement::measure(&family.workload())?;
    }
    Ok(())
}
