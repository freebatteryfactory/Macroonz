//! Native child-process maximum RSS and independently declared allocation sensitivity.

use std::io::Write;
use std::path::Path;

use super::{number, read_bounded};

// Retained byte/checksum vectors bind the measured child to its consumed output,
// not an independent proof of the declaration's meaning or a memory threshold.
const OUTPUTS: [(&str, u64, u64, u64); 12] = [
    ("density", 4, 2424, 56533),
    ("density", 16, 6060, 113_843),
    ("density", 64, 20604, 342_980),
    ("codec-width", 1, 1243, 48414),
    ("codec-width", 8, 3497, 96525),
    ("codec-width", 32, 11269, 263_729),
    ("late-duplicate", 8, 144, 13281),
    ("late-duplicate", 32, 145, 13323),
    ("late-duplicate", 127, 145, 13330),
    ("near-limit", 62, 7900, 137_822),
    ("near-limit", 63, 8019, 139_721),
    ("near-limit", 64, 8138, 141_622),
];

fn reading(
    native: &str,
    stdout: &str,
    stderr: &str,
    status: &str,
    expected: &str,
) -> Result<u64, String> {
    if !stderr.is_empty() || status != "0\n" || stdout != expected {
        return Err("unsuccessful child or changed consumed output".to_owned());
    }
    let raw = native
        .strip_prefix("peak-kib=")
        .and_then(|text| text.strip_suffix(" status=0\n"))
        .ok_or("missing successful native maximum-RSS record")?;
    let peak = number(raw)?;
    if peak == 0 {
        return Err("native maximum RSS is unavailable or zero".to_owned());
    }
    Ok(peak)
}

fn observation(root: &Path, name: &str, expected: &str) -> Result<u64, String> {
    let native = read_bounded(&root.join(format!("{name}.time")))?;
    let stdout = read_bounded(&root.join(format!("{name}.stdout")))?;
    let stderr = read_bounded(&root.join(format!("{name}.stderr")))?;
    let status = read_bounded(&root.join(format!("{name}.status")))?;
    let peak = reading(&native, &stdout, &stderr, &status, expected)?;
    writeln!(
        std::io::stdout().lock(),
        "memory,{name},peak-kib={peak},scope=child-process-maximum-rss"
    )
    .map_err(|error| error.to_string())?;
    Ok(peak)
}

fn sensitive([empty, medium, large]: [u64; 3]) -> Result<(), String> {
    if empty > 0 && empty < medium && medium < large {
        Ok(())
    } else {
        Err("native memory instrument did not distinguish declared live allocations".to_owned())
    }
}

fn allocations(root: &Path, round: u64) -> Result<(), String> {
    let mut peaks = Vec::new();
    for bytes in [0_u64, 16_777_216, 67_108_864] {
        let checksum = bytes
            .checked_mul(165)
            .ok_or("allocation checksum overflow")?;
        let expected =
            format!("memory-control requested-bytes={bytes} consumed-checksum={checksum}\n");
        peaks.push(observation(
            root,
            &format!("allocation-{round}-{bytes}"),
            &expected,
        )?);
    }
    let [empty, medium, large] = peaks.as_slice() else {
        return Err("allocation sensitivity population changed".to_owned());
    };
    sensitive([*empty, *medium, *large])
}

#[test]
#[ignore = "requires explicitly executed native memory controls and compiler observations"]
fn native_memory_reports_require_completed_children_and_all_declared_observations()
-> Result<(), String> {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("compiler-economics");
    let declared = read_bounded(&root.join("context.txt"))?;
    let context = declared.lines().collect::<Vec<_>>();
    let [source, target, toolchain] = context.as_slice() else {
        return Err("native memory context must have three declared fields".to_owned());
    };
    super::reconcile(
        &read_bounded(&root.join("timing.log"))?,
        &[*source, *target, *toolchain],
        super::Subject::Compiler,
    )?;
    let root = root.join("memory");
    for round in 0..4_u64 {
        allocations(&root, round)?;
        for (family, size, bytes, checksum) in OUTPUTS {
            let expected = format!(
                "memory-compiler family=compiler-{family} input={size} retained-bytes={bytes} consumed-checksum={checksum}\n"
            );
            observation(
                &root,
                &format!("compiler-{round}-{family}-{size}"),
                &expected,
            )?;
        }
    }
    Ok(())
}

#[test]
fn native_memory_readings_refuse_invalid_numbers_failed_processes_and_readiness_only()
-> Result<(), String> {
    let expected = "ready and consumed\n";
    assert_eq!(
        reading("peak-kib=4096 status=0\n", expected, "", "0\n", expected)?,
        4096
    );
    for native in [
        "",
        "peak-kib=0 status=0\n",
        "peak-kib=-1 status=0\n",
        "peak-kib=NaN status=0\n",
        "peak-kib=18446744073709551616 status=0\n",
        "peak-kib=4096 status=1\n",
        "peak-kib=4096 status=0\nextra\n",
    ] {
        assert!(reading(native, expected, "", "0\n", expected).is_err());
    }
    for (stdout, stderr, status) in [
        ("", "", "0\n"),
        ("changed\n", "", "0\n"),
        (expected, "error", "0\n"),
        (expected, "", "1\n"),
        (expected, "", "124\n"),
        (expected, "", ""),
    ] {
        assert!(reading("peak-kib=4096 status=0\n", stdout, stderr, status, expected).is_err());
    }
    sensitive([1024, 17000, 66000])?;
    for peaks in [
        [0, 17000, 66000],
        [1024, 1024, 66000],
        [1024, 17000, 17000],
        [17000, 1024, 66000],
    ] {
        assert!(sensitive(peaks).is_err());
    }
    Ok(())
}
