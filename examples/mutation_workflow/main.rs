//! Execute a supplied Cargo subject and retain its historical mutation manifest.

mod configuration;
mod retain;
mod types;
#[path = "../support/native_input/mod.rs"]
mod input;

use macroonz::harness::muterprater::MutationVerdict;
use macroonz::native_mutation::{self, MutationRun};
use std::io::Write;
use std::time::Duration;

fn main() -> Result<(), String> {
    let settings = configuration::read()?;
    let run = native_mutation::run(settings.request, |_| None, |_, _| None)
        .map_err(|error| error.finish_cleanup(Duration::from_secs(5)).to_string())?;
    let run = match run {
        MutationRun::Pending(pending) => pending.finish(Duration::from_secs(5)),
        finished @ MutationRun::Finished(_) => finished,
    };
    let MutationRun::Finished(output) = run else {
        return Err("mutation cleanup remains unfinished".to_owned());
    };
    let manifest = output.manifest().map_err(ToString::to_string)?;
    let reports = manifest.reading().run().reports();
    let caught = reports
        .iter()
        .filter(|report| report.verdict() == MutationVerdict::Killed)
        .count();
    let inconclusive = reports
        .iter()
        .filter(|report| report.verdict() == MutationVerdict::Inconclusive)
        .count();
    let historical = output
        .retain(retain::limits())
        .map_err(|error| error.to_string())?;
    let loaded = retain::round_trip(&settings.storage, &settings.batch, &historical)?;
    let comparison = native_mutation::compare_historical(&loaded, &settings.directory, 1_048_576)
        .map_err(|error| error.to_string())?;
    writeln!(std::io::stdout(), "retained {} reports: {caught} caught, {inconclusive} inconclusive; {} historical source claims match current files", reports.len(), comparison.current_sources().len()).map_err(|error| error.to_string())
}
