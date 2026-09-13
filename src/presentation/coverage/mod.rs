#![doc = include_str!("README.md")]

mod axes;
mod preflight;
mod profile;
mod record;

#[cfg(feature = "native-tooling")]
pub(in crate::presentation) use axes::command;
pub use preflight::coverage_preflight_refusal;
#[cfg(feature = "native-tooling")]
pub(in crate::presentation) use preflight::preflight;
#[cfg(feature = "native-tooling")]
pub(in crate::presentation) use profile::profile;
pub use profile::{coverage_admission_refusal, coverage_profile_refusal};
#[cfg(feature = "native-tooling")]
pub(in crate::presentation) use record::readiness;
pub use record::{coverage_admission, coverage_corpus, coverage_readiness, coverage_result};
