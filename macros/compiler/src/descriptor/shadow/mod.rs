#![doc = include_str!("README.md")]

mod capture;
mod render;
mod type_contract;
mod types;

pub use capture::chosen;
pub use render::faces;
pub use types::{SHADOW_ROSTER, ShadowCaptureError, ShadowFace, ShadowRow, Shadows};
