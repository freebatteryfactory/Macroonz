//! Independently compiled child reporting only the two declared environment-control markers.

use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut output = std::io::stdout().lock();
    for key in ["MACROONZ_PROCESS_DECLARED", "MACROONZ_PROCESS_INHERITED"] {
        if let Some(value) = std::env::var_os(key) {
            writeln!(output, "{key}={}", value.to_string_lossy())?;
        }
    }
    std::fs::write("declared-directory", b"observed")?;
    Ok(())
}
