#![doc = include_str!("README.md")]

#[cfg(any(unix, windows))]
mod files;
#[cfg(any(unix, windows))]
mod inspect;
#[cfg(any(unix, windows))]
mod install;
#[cfg(any(unix, windows))]
mod journal;
#[cfg(any(unix, windows))]
mod replace;
mod record;
mod type_contract;
mod types;

pub use types::{
    DestinationCheck, DestinationError, DestinationIssue, DestinationLimits, DestinationProblem,
    DestinationState, PublicationDestination, PublicationInstallation,
};
