//! Evidence and Macroonz handoff material for the F0 pilot.

use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use macroonz_f0_target::CaptureOutcome;

use crate::classify::{self, ExecutionClass};

/// One retained handoff record for Macroonz corpus/replay owners.
#[derive(Debug, Clone)]
pub(crate) struct HandoffCase {
    pub(crate) name: &'static str,
    pub(crate) bytes: Vec<u8>,
    pub(crate) outcome: CaptureOutcome,
    pub(crate) class: ExecutionClass,
}

/// Write handoff bytes and an index that points at Macroonz owners without reimplementing them.
pub(crate) fn write_handoff(dir: &Path, cases: &[HandoffCase]) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    let index_path = dir.join("INDEX.tsv");
    let mut index = fs::File::create(index_path)?;
    writeln!(index, "name\tclass\toutcome\tbytes-path\tmacroonz-receiving-owners")?;
    for case in cases {
        let file_name = format!("{}.bin", case.name);
        let path = dir.join(&file_name);
        fs::write(&path, &case.bytes)?;
        writeln!(
            index,
            "{}\t{}\t{}\t{}\tharness/src/corpus + harness report/replay roads",
            case.name,
            case.class.as_str(),
            classify::outcome_label(case.outcome),
            file_name
        )?;
    }
    Ok(())
}

/// Cross-host disposition: credible upstream road; native Macroonz receipts deferred to Wave F.
pub(crate) fn write_cross_host_disposition(writer: &mut impl Write) -> io::Result<()> {
    writeln!(
        writer,
        "disposition\tlinux\tcredible-unexecuted\tLibAFL+Frida upstream supports native Linux; Macroonz Linux receipts deferred to Wave F; WSL rustc 1.98 installed as toolchain prep only"
    )?;
    writeln!(
        writer,
        "disposition\tmacos\tcredible-unexecuted\tLibAFL+Frida upstream supports native macOS; Macroonz macOS receipts deferred to Wave F"
    )?;
    writeln!(
        writer,
        "disposition\twindows\tobserved\tnative Windows Frida EventSink + LibAFL loop executed on this host"
    )?;
    Ok(())
}

/// Record dependency and CRT ceilings without turning them into a selection.
pub(crate) fn write_cost_ceiling(writer: &mut impl Write) -> io::Result<()> {
    writeln!(writer, "cost\tlibafl\t0.16.1")?;
    writeln!(writer, "cost\tlibafl_bolts\t0.16.1")?;
    writeln!(writer, "cost\tfrida-gum\t0.17.2")?;
    writeln!(writer, "cost\tfrida-devkit\t17.9.5-windows-x86_64")?;
    writeln!(
        writer,
        "ceiling\tcrt\tLNK4098 residual after crt-static and NODEFAULTLIB:msvcrt; Frida LIBCMT vs Rust dylib/UCRT; /IGNORE:4098 experimental only"
    )?;
    writeln!(
        writer,
        "ceiling\tlib-search\tLIB must append MSVC and Windows SDK um/ucrt x64 roots; replacing LIB drops kernel32.lib"
    )?;
    writeln!(
        writer,
        "ceiling\trust-std-dll\tdriver and target import std-*.dll from the Rust 1.98 sysroot; that directory must be on PATH at run time"
    )?;
    Ok(())
}
