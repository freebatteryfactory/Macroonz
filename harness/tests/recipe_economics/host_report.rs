//! Exact sample-population custody for separately executed compiler and runtime subjects.

#[path = "memory_report.rs"]
mod memory;

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};

const REPORT_LIMIT: u64 = 4 * 1024 * 1024;
const FAMILIES: [(&str, [u64; 3]); 4] = [
    ("compiler-density", [4, 16, 64]),
    ("compiler-codec-width", [1, 8, 32]),
    ("compiler-late-duplicate", [8, 32, 127]),
    ("compiler-near-limit", [62, 63, 64]),
];
const ROLES: [&str; 3] = ["single-a", "single-b", "double-execution"];
const INTERVAL: &str =
    "input-construction+capture+bake+canonical-output-or-exact-refusal+drop+recording";

#[derive(Clone, Copy)]
enum Subject {
    Compiler,
    Runtime,
}

impl Subject {
    const fn families(self) -> &'static [(&'static str, [u64; 3])] {
        match self {
            Self::Compiler => &FAMILIES,
            Self::Runtime => &[
                ("runtime-dispatch", [256, 1024, 4096]),
                ("runtime-relation", [256, 1024, 4096]),
                ("runtime-codec", [256, 1024, 4096]),
                ("runtime-growing-dispatch", [2, 8, 16]),
                ("runtime-growing-relation", [2, 8, 16]),
                ("runtime-growing-codec", [1, 8, 32]),
            ],
        }
    }

    const fn interval(self) -> &'static str {
        match self {
            Self::Compiler => INTERVAL,
            Self::Runtime => {
                "input-selection+generated-execution+consume+batch-recording;codec-includes-Vec-allocation-and-drop"
            }
        }
    }

    const fn preflight_material(self) -> usize {
        match self {
            Self::Compiler => 12,
            Self::Runtime => 0,
        }
    }
}

fn number(text: &str) -> Result<u64, String> {
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err("sample coordinate is not an unsigned decimal".to_owned());
    }
    text.parse::<u64>().map_err(|error| error.to_string())
}

fn family_size(subject: Subject, family: &str, size: u64) -> Result<(), String> {
    subject
        .families()
        .iter()
        .any(|(name, sizes)| *name == family && sizes.contains(&size))
        .then_some(())
        .ok_or_else(|| "undeclared economics family or input size".to_owned())
}

fn reconcile(text: &str, context: &[&str; 3], subject: Subject) -> Result<(), String> {
    if context
        .iter()
        .any(|part| part.is_empty() || part.contains([',', '\n', '\r']))
    {
        return Err("missing or malformed declared context".to_owned());
    }
    let [source, target, toolchain] = context;
    let interval = subject.interval();
    let mut headers = BTreeSet::new();
    let mut samples = BTreeSet::new();
    let mut preflight = BTreeMap::new();
    for line in text.lines() {
        let fields = line.split(',').collect::<Vec<_>>();
        match fields.as_slice() {
            ["pilot", family, ..] => {
                let expected = format!(
                    "pilot,{family},source={source},target={target},toolchain={toolchain},profile=release,interval={interval}"
                );
                if line != expected
                    || !subject.families().iter().any(|(name, _)| name == family)
                    || !headers.insert(*family)
                {
                    return Err("moved or repeated economics report context".to_owned());
                }
            }
            ["preflight", family, size, input, output] => {
                let size = number(size)?;
                family_size(subject, family, size)?;
                let input = input
                    .strip_prefix("input-bytes=")
                    .ok_or("missing input size")?;
                let output = output
                    .strip_prefix("consumed-output-bytes=")
                    .ok_or("missing consumed output size")?;
                let material = (number(input)?, number(output)?);
                if material.0 == 0 || material.1 == 0 {
                    return Err("empty preflight material".to_owned());
                }
                if preflight
                    .insert((*family, size), material)
                    .is_some_and(|previous| previous != material)
                {
                    return Err("preflight material changed between observations".to_owned());
                }
            }
            ["sample", family, round, role, size, ordinal, elapsed] => {
                let (round, size, ordinal) = (number(round)?, number(size)?, number(ordinal)?);
                family_size(subject, family, size)?;
                if round >= 4
                    || ordinal >= 5
                    || !ROLES.contains(role)
                    || number(elapsed)? == 0
                    || !samples.insert((*family, round, *role, size, ordinal))
                {
                    return Err("missing, repeated or undeclared timing coordinate".to_owned());
                }
            }
            _ => return Err("unrecognized economics observation record".to_owned()),
        }
    }
    if headers.len() != subject.families().len()
        || preflight.len() != subject.preflight_material()
        || Some(samples.len()) != subject.families().len().checked_mul(180)
    {
        return Err("incomplete economics observation population".to_owned());
    }
    Ok(())
}

fn read_bounded(path: &std::path::Path) -> Result<String, String> {
    read_input(std::fs::File::open(path).map_err(|error| error.to_string())?)
}

fn read_input(reader: impl Read) -> Result<String, String> {
    let mut bytes = Vec::new();
    reader
        .take(REPORT_LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if u64::try_from(bytes.len()).map_err(|error| error.to_string())? > REPORT_LIMIT {
        return Err("economics report exceeds its byte bound".to_owned());
    }
    String::from_utf8(bytes).map_err(|error| error.to_string())
}

#[test]
#[ignore = "requires an explicitly executed native compiler economics report"]
fn native_compiler_samples_match_the_declared_context_and_population() -> Result<(), String> {
    native_report("compiler-economics", Subject::Compiler)
}

#[test]
#[ignore = "requires an explicitly executed native generated-runtime economics report"]
fn native_runtime_samples_match_the_declared_context_and_population() -> Result<(), String> {
    native_report("runtime-economics", Subject::Runtime)
}

fn native_report(directory: &str, subject: Subject) -> Result<(), String> {
    let root = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join(directory);
    let declared = read_bounded(&root.join("context.txt"))?;
    let context = declared.lines().collect::<Vec<_>>();
    let [source, target, toolchain] = context.as_slice() else {
        return Err("native economics context must have three declared fields".to_owned());
    };
    let report = read_bounded(&root.join("timing.log"))?;
    reconcile(&report, &[*source, *target, *toolchain], subject)?;
    writeln!(std::io::stdout().lock(), "{report}").map_err(|error| error.to_string())?;
    Ok(())
}

fn complete_report(subject: Subject) -> String {
    let mut lines = Vec::new();
    let interval = subject.interval();
    for (family, sizes) in subject.families() {
        lines.push(format!(
            "pilot,{family},source=source,target=target,toolchain=toolchain,profile=release,interval={interval}"
        ));
        for size in sizes {
            if subject.preflight_material() > 0 {
                lines.push(format!(
                    "preflight,{family},{size},input-bytes=1,consumed-output-bytes=1"
                ));
            }
            append_samples(&mut lines, family, *size);
        }
    }
    lines.join("\n")
}

fn append_samples(lines: &mut Vec<String>, family: &str, size: u64) {
    for round in 0..4_u64 {
        for role in ROLES {
            for ordinal in 0..5_u64 {
                lines.push(format!("sample,{family},{round},{role},{size},{ordinal},1"));
            }
        }
    }
}

#[test]
fn native_report_input_keeps_its_byte_bound_and_refuses_invalid_encoding() -> Result<(), String> {
    for size in [REPORT_LIMIT - 1, REPORT_LIMIT] {
        let size = usize::try_from(size).map_err(|error| error.to_string())?;
        let input = vec![b'x'; size];
        assert_eq!(read_input(input.as_slice())?.len(), size);
    }
    let oversized = usize::try_from(REPORT_LIMIT + 1).map_err(|error| error.to_string())?;
    assert!(read_input(vec![b'x'; oversized].as_slice()).is_err());
    assert!(read_input([0xff_u8].as_slice()).is_err());
    Ok(())
}

#[test]
fn native_report_refuses_missing_repeated_foreign_and_unmeasured_material() -> Result<(), String> {
    let context = &["source", "target", "toolchain"];
    let report = complete_report(Subject::Compiler);
    reconcile(&report, context, Subject::Compiler)?;
    let sample = "sample,compiler-density,0,single-a,4,0,1";
    let header = "pilot,compiler-density,source=source";
    for damaged in [
        String::new(),
        without_record(&report, sample),
        without_record(&report, "pilot,compiler-density,"),
        without_record(&report, "preflight,compiler-density,4,"),
        format!("{report}\n{sample}"),
        format!("{report}\npreflight,compiler-density,4,input-bytes=2,consumed-output-bytes=1"),
        report.replace(sample, "sample,compiler-density,4,single-a,4,0,1"),
        report.replace(sample, "sample,compiler-density,0,single-a,5,0,1"),
        report.replace(sample, "sample,compiler-density,0,single-a,4,5,1"),
        report.replace(sample, "sample,compiler-density,0,unknown,4,0,1"),
        report.replace(sample, "sample,compiler-density,0,single-a,4,0,0"),
        report.replace(sample, "sample,compiler-density,0,single-a,4,0,NaN"),
        report.replace(header, "pilot,compiler-density,source=other"),
        report.replace("input-bytes=1", "input-bytes=0"),
        report.replace("consumed-output-bytes=1", "consumed-output-bytes=0"),
        format!("{report}\nunknown-record"),
    ] {
        assert!(reconcile(&damaged, context, Subject::Compiler).is_err());
    }
    assert!(reconcile(&report, &["", "target", "toolchain"], Subject::Compiler).is_err());
    Ok(())
}

#[test]
fn runtime_population_refuses_omitted_families_wrong_axes_and_foreign_intervals()
-> Result<(), String> {
    let context = &["source", "target", "toolchain"];
    let report = complete_report(Subject::Runtime);
    reconcile(&report, context, Subject::Runtime)?;
    assert!(reconcile(&report, context, Subject::Compiler).is_err());
    assert!(
        reconcile(
            &complete_report(Subject::Compiler),
            context,
            Subject::Runtime
        )
        .is_err()
    );
    let sample = "sample,runtime-growing-codec,0,single-a,32,0,1";
    for damaged in [
        String::new(),
        without_record(&report, "pilot,runtime-growing-codec,"),
        without_record(&report, sample),
        format!("{report}\n{sample}"),
        report.replace(sample, "sample,runtime-growing-codec,0,single-a,4096,0,1"),
        report.replace(sample, "sample,runtime-growing-codec,0,single-a,32,0,0"),
        report.replace(sample, "sample,runtime-growing-codec,0,single-a,32,0,NaN"),
        report.replace(sample, "sample,runtime-growing-codec,4,single-a,32,0,1"),
        report.replace(sample, "sample,runtime-growing-codec,0,single-a,32,5,1"),
        report.replace(sample, "sample,runtime-growing-codec,0,unknown,32,0,1"),
        report.replace(
            sample,
            "sample,runtime-growing-codec,0,single-a,32,0,18446744073709551616",
        ),
        report.replace("source=source", "source=other"),
        report.replace(Subject::Runtime.interval(), INTERVAL),
        format!("{report}\npreflight,runtime-codec,256,input-bytes=1,consumed-output-bytes=1"),
    ] {
        assert!(reconcile(&damaged, context, Subject::Runtime).is_err());
    }
    Ok(())
}

fn without_record(report: &str, prefix: &str) -> String {
    report
        .lines()
        .filter(|line| !line.starts_with(prefix))
        .collect::<Vec<_>>()
        .join("\n")
}
