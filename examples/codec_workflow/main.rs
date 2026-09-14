//! Execute declared codecs with independent bytes and caller-owned refusal controls.

mod checks;
mod types;

pub use types::wire;

use std::io::Write;

fn main() -> Result<(), String> {
    checks::round_trip();
    checks::malformed()?;
    checks::checked_assembly()?;
    writeln!(
        std::io::stdout(),
        "codec: exact bytes and round trip agree; malformed bytes and zero count refuse"
    )
    .map_err(|error| error.to_string())
}
