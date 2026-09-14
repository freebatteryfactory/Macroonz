#![doc = include_str!("README.md")]

mod compose;
#[cfg(any(target_os = "windows", target_os = "linux", target_os = "macos"))]
mod read;

pub use compose::source;
