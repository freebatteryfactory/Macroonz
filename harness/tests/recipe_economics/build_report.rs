//! Adopter build observations keep phase, executable, source and consumer custody together.

use std::collections::BTreeMap;
use std::io::Write;

use super::{number, read_bounded};

const PHASES: [&str; 4] = ["cold", "warm", "edited", "edited-warm"];
const INTERVAL: &str =
    "interval,cargo-start-to-observed-exit;excludes-drain-edit-cleanup-json-consumer";

struct Sample<'a> {
    artifacts: u64,
    frequency: u64,
    source: &'a str,
    executable: &'a str,
    lock: &'a str,
}

fn digest(value: &str) -> Result<(), String> {
    if value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Ok(())
    } else {
        Err("build material has no complete SHA-256 identity".to_owned())
    }
}

fn sample<'a>(fields: &[&'a str]) -> Result<((u64, &'a str), Sample<'a>), String> {
    let [
        "build",
        round,
        phase,
        ticks,
        frequency,
        artifacts,
        fresh,
        rebuilt,
        adopter_fresh,
        source,
        executable,
        lock,
        "build=0",
        "consumer=0",
        consumer,
    ] = fields
    else {
        return Err("incomplete or unsuccessful build observation".to_owned());
    };
    let round = number(round)?;
    let (artifacts, fresh, rebuilt) = (number(artifacts)?, number(fresh)?, number(rebuilt)?);
    let frequency = number(frequency)?;
    if round >= 3 || !PHASES.contains(phase) || number(ticks)? == 0 || frequency == 0 {
        return Err("undeclared phase or unavailable build clock".to_owned());
    }
    if artifacts == 0 || fresh.checked_add(rebuilt) != Some(artifacts) {
        return Err("build artifact population does not reconcile".to_owned());
    }
    let expected_fresh = match *phase {
        "cold" if fresh == 0 && rebuilt == artifacts => "false",
        "warm" | "edited-warm" if fresh == artifacts && rebuilt == 0 => "true",
        "edited" if rebuilt == 1 => "false",
        _ => return Err("artifact freshness contradicts the declared build phase".to_owned()),
    };
    if *adopter_fresh != expected_fresh {
        return Err("adopter freshness contradicts the declared build phase".to_owned());
    }
    let edited = matches!(*phase, "edited" | "edited-warm");
    let expected = if (round == 1) == edited {
        "open"
    } else {
        "closed"
    };
    if *consumer != format!("adopter-domain=2 expected-target={expected} effects=1") {
        return Err(
            "independent executable consumer did not match its declared expectation".to_owned(),
        );
    }
    for value in [source, executable, lock] {
        digest(value)?;
    }
    Ok((
        (round, phase),
        Sample {
            artifacts,
            frequency,
            source,
            executable,
            lock,
        },
    ))
}

fn reconcile(report: &str, context: &[&str; 3]) -> Result<(), String> {
    if context
        .iter()
        .any(|part| part.is_empty() || part.contains([',', '\r', '\n']))
    {
        return Err("missing or malformed build context".to_owned());
    }
    let [source, target, toolchain] = context;
    let mut lines = report.lines();
    if lines.next() != Some(format!("adopter-build,{source},{target},{toolchain}").as_str())
        || lines.next() != Some(INTERVAL)
    {
        return Err("build source context or timed interval changed".to_owned());
    }
    let mut samples = BTreeMap::new();
    for line in lines {
        let fields = line.split(',').collect::<Vec<_>>();
        let (coordinate, observation) = sample(&fields)?;
        if samples.insert(coordinate, observation).is_some() {
            return Err("repeated build observation".to_owned());
        }
    }
    if samples.len() != 12 {
        return Err("incomplete build observation population".to_owned());
    }
    let baseline = samples.get(&(0, "cold")).ok_or("missing cold build")?;
    let alternate = samples.get(&(0, "edited")).ok_or("missing edited build")?;
    if baseline.source == alternate.source || baseline.executable == alternate.executable {
        return Err("declared edit did not change source and executable".to_owned());
    }
    for ((round, phase), current) in &samples {
        if current.artifacts != baseline.artifacts
            || current.frequency != baseline.frequency
            || current.lock != baseline.lock
        {
            return Err("build population, clock scale or locked graph changed".to_owned());
        }
        let edited = matches!(*phase, "edited" | "edited-warm");
        let expected_source = if (*round == 1) == edited {
            baseline.source
        } else {
            alternate.source
        };
        if current.source != expected_source {
            return Err("build source does not match the alternating edit".to_owned());
        }
        if *phase == "edited" {
            let cold = samples.get(&(*round, "cold")).ok_or("missing cold build")?;
            if current.executable == cold.executable {
                return Err("edited build reused its preceding executable".to_owned());
            }
        }
        let preceding = match *phase {
            "warm" => Some("cold"),
            "edited-warm" => Some("edited"),
            _ => None,
        };
        if let Some(preceding) = preceding {
            let previous = samples
                .get(&(*round, preceding))
                .ok_or("missing preceding build")?;
            if previous.executable != current.executable {
                return Err("unchanged build changed the executable".to_owned());
            }
        }
    }
    Ok(())
}

#[test]
#[ignore = "requires explicitly observed native cold, warm and edited adopter builds"]
fn native_build_samples_preserve_phases_source_freshness_and_independent_consumers()
-> Result<(), String> {
    let root = std::path::Path::new(env!("CARGO_TARGET_TMPDIR")).join("adopter-economics");
    let declared = read_bounded(&root.join("context.txt"))?;
    let fields = declared.lines().collect::<Vec<_>>();
    let [source, target, toolchain] = fields.as_slice() else {
        return Err("native build context must have three declared fields".to_owned());
    };
    let report = read_bounded(&root.join("builds.log"))?;
    reconcile(&report, &[*source, *target, *toolchain])?;
    writeln!(std::io::stdout().lock(), "{report}").map_err(|error| error.to_string())
}

fn complete_report() -> String {
    let mut lines = vec![
        "adopter-build,source,target,toolchain".to_owned(),
        INTERVAL.to_owned(),
    ];
    for round in 0..3_u64 {
        for phase in PHASES {
            let edited = matches!(phase, "edited" | "edited-warm");
            let (source, executable, expected) = if (round == 1) == edited {
                ("a".repeat(64), "c".repeat(64), "open")
            } else {
                ("b".repeat(64), "d".repeat(64), "closed")
            };
            let freshness = match phase {
                "cold" => "0,48,false",
                "edited" => "47,1,false",
                _ => "48,0,true",
            };
            let lock = "e".repeat(64);
            lines.push(format!("build,{round},{phase},1,10000000,48,{freshness},{source},{executable},{lock},build=0,consumer=0,adopter-domain=2 expected-target={expected} effects=1"));
        }
    }
    lines.join("\n")
}

#[test]
fn build_reports_refuse_missing_or_changed_phase_clock_graph_and_behavior() -> Result<(), String> {
    let context = &["source", "target", "toolchain"];
    let report = complete_report();
    reconcile(&report, context)?;
    for (from, to) in [
        ("build,0,cold", "build,3,cold"),
        ("build,0,cold", "build,0,unknown"),
        (",1,10000000,48,", ",0,10000000,48,"),
        (",1,10000000,48,", ",1,0,48,"),
        (",1,10000000,48,", ",NaN,10000000,48,"),
        (",1,10000000,48,", ",18446744073709551616,10000000,48,"),
        (",48,0,48,false,", ",48,0,47,false,"),
        (",48,0,48,false,", ",0,0,0,false,"),
        (",48,0,48,false,", ",48,18446744073709551615,1,false,"),
        (",48,0,48,false,", ",49,0,49,false,"),
        (",48,48,0,true,", ",48,47,1,true,"),
        (",48,47,1,false,", ",48,48,0,false,"),
        (",48,47,1,false,", ",48,46,2,false,"),
        (",48,0,48,false,", ",48,0,48,true,"),
        ("build=0", "build=1"),
        ("consumer=0", "consumer=101"),
        ("effects=1", "effects=0"),
        ("expected-target=open", "expected-target=closed"),
        ("adopter-build,source,", "adopter-build,other,"),
        ("excludes-drain", "includes-drain"),
    ] {
        assert!(
            reconcile(&report.replacen(from, to, 1), context).is_err(),
            "{from} -> {to}"
        );
    }
    for damaged in [
        String::new(),
        super::without_record(&report, "build,0,warm,"),
        format!("{report}\nunknown"),
        report.replace(&"b".repeat(64), &"a".repeat(64)),
        report.replace(&"d".repeat(64), &"c".repeat(64)),
        report.replacen(&"e".repeat(64), &"f".repeat(64), 1),
        report.replacen(&"a".repeat(64), "short", 1),
        report.replacen(&"a".repeat(64), &"z".repeat(64), 1),
        report.replacen(&"a".repeat(64), &"b".repeat(64), 1),
        report.replacen("10000000", "9999999", 1),
        report.replacen(&"c".repeat(64), &"f".repeat(64), 1),
    ] {
        assert!(reconcile(&damaged, context).is_err());
    }
    let duplicate = report
        .lines()
        .find(|line| line.starts_with("build,0,cold,"))
        .ok_or("missing fixture row")?;
    assert!(reconcile(&format!("{report}\n{duplicate}"), context).is_err());
    assert!(reconcile(&report, &["", "target", "toolchain"]).is_err());
    let stale_second_round = report
        .lines()
        .map(|line| {
            if line.starts_with("build,1,edited") {
                line.replace(&"c".repeat(64), &"d".repeat(64))
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(reconcile(&stale_second_round, context).is_err());
    Ok(())
}
