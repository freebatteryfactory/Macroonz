#![doc = include_str!("README.md")]

mod render;
mod stamp;
mod types;

pub use render::{key, path, road};
pub use types::{HarnessName, HarnessWord};
