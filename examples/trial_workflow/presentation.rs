//! The same admitted input and complete report in each public display format.

use macroonz::workflow::InputRun;
use std::io::Write;

pub(super) fn write(label: &str, run: &InputRun) -> Result<(), String> {
    let shown = macroonz::presentation::input_run(run);
    for (format, text) in [
        ("JSON", shown.json()),
        ("Markdown", shown.markdown()),
        ("HTML", shown.html()),
    ] {
        writeln!(std::io::stdout(), "--- {label} {format} ---\n{text}")
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}
