//! Prepare, inspect, check, generate or recover a caller-owned publication through the root library.

mod configuration;
mod generate;
mod type_contract;
mod types;
#[path = "../support/native_input/mod.rs"]
mod input;
#[path = "../support/publication_configuration/mod.rs"]
mod publication_configuration;

use macroonz::native_publication::{BakeOutput, bake};
use macroonz::presentation::{Presentation, bake_error, bake_output};
use std::io::Write;

fn main() -> Result<(), String> {
    let (command, value) = configuration::read()?;
    let output = match bake(command, || generate::publication(value)) {
        Ok(output) => output,
        Err(error) => {
            let error = error.finish_cleanup(std::time::Duration::from_secs(5));
            write(&bake_error(&error))?;
            return Err(error.to_string());
        }
    };
    write(&bake_output(&output))?;
    if let BakeOutput::Checked { comparison, .. } = &output
        && !comparison.is_current()
    {
        return Err("publication check found discrepancies".to_owned());
    }
    Ok(())
}

fn write(report: &Presentation) -> Result<(), String> {
    writeln!(std::io::stdout(), "{}", report.json()).map_err(|error| error.to_string())
}
