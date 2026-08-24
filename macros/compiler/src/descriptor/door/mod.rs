#![doc = include_str!("README.md")]

mod bench;
mod mutations;
mod shadow;
mod trials;
mod types;
mod walk;

pub use bench::bench;
pub use mutations::mutations;
pub use shadow::shadow;
pub use trials::trials;
pub use types::{BENCH_FORM_FACT, SOLE_READING_FACT, TRIALS_FORM_FACT};
