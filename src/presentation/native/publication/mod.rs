#![doc = include_str!("README.md")]

mod command;
mod destination;
mod formatting;
mod inventory;
mod staging;

pub use command::{bake_error, bake_output};
pub use destination::{publication_destination_check, publication_destination_error};
pub use formatting::{publication_format_error, publication_format_output, publication_format_run};
pub use staging::{publication_staging_error, publication_staging_run};
