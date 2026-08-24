#![doc = include_str!("README.md")]

mod capture;
mod render;
mod type_contract;
mod types;

pub use capture::declared;
pub use render::rendered;
pub use types::{
    DisciplineRow, FaultRow, LinkRow, NETWORK_HELPER_POSITION, NetworkCaptureError,
    NetworkDeclaration, NetworkModule, ScheduleRow,
};
