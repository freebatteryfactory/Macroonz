//! An independently launched, bounded stdin subject for the public recipe compiler.

#![forbid(unsafe_code)]

#[path = "recipe_observation.rs"]
mod observation;

use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut bytes = Vec::new();
    let bound = u64::try_from(observation::INPUT_LIMIT).map_err(io::Error::other)?;
    io::stdin()
        .take(bound.saturating_add(1))
        .read_to_end(&mut bytes)?;
    if bytes.len() > observation::INPUT_LIMIT {
        return Err(io::Error::other(
            "recipe campaign input exceeds its byte ceiling",
        ));
    }
    let first = observation::observe(&bytes).map_err(io::Error::other)?;
    let second = observation::observe(&bytes).map_err(io::Error::other)?;
    if first != second {
        return Err(io::Error::other(
            "recipe outcome changed on identical input",
        ));
    }
    std::hint::black_box(first);
    Ok(())
}
