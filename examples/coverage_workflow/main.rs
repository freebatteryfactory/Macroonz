//! Instrument a supplied Rust subject and replay its coverage-earned corpus through bounded native processes.

mod configuration;
mod types;
#[path = "../support/native_input/mod.rs"]
mod input;

use macroonz::harness::corpus::{SeedInput, pack, warm_start};
use macroonz::harness::fuzz::{CoverageAdmission, CoverageTool, ReadyPreflight};
use macroonz::harness::generate::InputOrigin;
use macroonz::harness::oracle::{CompilationVerdict, DeclaredCompilation};
use macroonz::native_compiler::{self, CompilerRun};
use macroonz::native_coverage;
use macroonz::native_process::ProcessLimits;
use std::io::Write;
use std::time::Duration;

fn main() -> Result<(), String> {
    let settings = configuration::read()?;
    let CompilerRun::Finished(compiled) =
        native_compiler::compile(&settings.compiler).map_err(|error| error.to_string())?
    else {
        return Err("compiler cleanup is unfinished".to_owned());
    };
    if compiled.compared(&DeclaredCompilation::compiles()) != Ok(CompilationVerdict::Conforms) {
        return Err(format!(
            "compilation did not conform: {:?}",
            compiled.observed()
        ));
    }
    let limits = ProcessLimits::informed(
        Duration::from_secs(2),
        Duration::from_secs(5),
        65_536,
        65_536,
    )
    .map_err(|error| error.to_string())?;
    let coverage = native_coverage::preflight(settings.coverage, settings.tool, limits)
        .map_err(|failure| failure.to_string())?;
    report_tools(coverage.ready())?;
    let mut corpus = coverage.corpus();
    for candidate in settings.candidates {
        let observed = native_coverage::observe(&coverage, &mut corpus, &candidate)
            .map_err(|failure| failure.to_string())?;
        let admission = corpus
            .admit(observed)
            .map_err(|error| format!("coverage admission refused: {error:?}"))?;
        let disposition = match admission {
            CoverageAdmission::Interesting(_) => "interesting",
            CoverageAdmission::Known => "known",
        };
        writeln!(std::io::stdout(), "candidate {candidate:?}: {disposition}")
            .map_err(|error| error.to_string())?;
    }
    let seeds = corpus
        .interesting()
        .iter()
        .map(|seed| {
            SeedInput::declared(seed.as_bytes().to_vec())
                .map_err(|error| format!("seed: {error:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let retained = pack(coverage.ready().standing().campaign().population(), seeds)
        .map_err(|error| format!("pack: {error:?}"))?;
    for origin in warm_start(&retained) {
        let InputOrigin::Supplied(candidate) = origin else {
            return Err("retained seed did not supply bytes".to_owned());
        };
        let observed = native_coverage::observe(&coverage, &mut corpus, &candidate)
            .map_err(|failure| failure.to_string())?;
        if corpus.admit(observed) != Ok(CoverageAdmission::Known) {
            return Err("replayed seed changed its coverage frontier".to_owned());
        }
    }
    writeln!(
        std::io::stdout(),
        "replayed {} retained seeds through fresh target processes",
        retained.seeds().len()
    )
    .map_err(|error| error.to_string())
}

fn report_tools(ready: &ReadyPreflight) -> Result<(), String> {
    let mut output = std::io::stdout().lock();
    writeln!(
        output,
        "compiler {}: {}",
        ready.rustc().display(),
        ready.release()
    )
    .map_err(|error| error.to_string())?;
    writeln!(
        output,
        "host {}; target {}",
        ready.host(),
        ready.standing().target().target().spelling()
    )
    .map_err(|error| error.to_string())?;
    writeln!(
        output,
        "compiler LLVM {}; matching tools {}",
        ready.llvm_version(),
        ready.tool_version()
    )
    .map_err(|error| error.to_string())?;
    for tool in [CoverageTool::Profdata, CoverageTool::Cov] {
        writeln!(output, "{tool:?}: {}", ready.tool_path(tool).display())
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}
