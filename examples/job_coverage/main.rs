//! Observe the Job record reader through bounded native coverage and independent input expectations.

mod campaign;
mod checks;
mod configuration;
mod prepare;
mod types;
#[path = "../support/native_input/mod.rs"]
mod input;

use std::io::Write;

fn main() -> Result<(), String> {
    let coverage = prepare::ready(configuration::read()?)?;
    let corpus = checks::execute(&coverage)?;
    let mut output = std::io::stdout().lock();
    writeln!(
        output,
        "{}",
        macroonz::presentation::coverage_readiness(coverage.ready()).json()
    )
    .map_err(|error| error.to_string())?;
    writeln!(
        output,
        "{}",
        macroonz::presentation::coverage_corpus(&corpus).json()
    )
    .map_err(|error| error.to_string())
}
