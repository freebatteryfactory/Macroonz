//! Compose declared network deliveries with caller-owned command-order judgments.

mod concurrency;
mod network;
mod types;

pub use types::world;

use std::io::Write;

fn main() -> Result<(), String> {
    let deliveries = network::run()?;
    concurrency::run(&deliveries)?;
    writeln!(
        std::io::stdout(),
        "network: quiet and four fault forms agree with exact deliveries\n\
         concurrency: withdraw-first fails and replays; guarded histories hold\n\
         evidence: exhaustive and sampled standings stay distinct; empty pressure and zero work refuse"
    )
    .map_err(|error| error.to_string())
}
