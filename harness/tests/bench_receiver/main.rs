//! The public handwritten benchmark receiver, observed by claim family through one Cargo integration target.

mod archive;
mod declaration;
mod fixture;
mod host_order;
mod qualification;
mod report_order;
mod support;
mod wall_observation;

#[path = "../support/archive_process.rs"]
mod archive_process;
