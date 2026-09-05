//! The backend home's trait participation: how one outcome word widens into the two record axes, how a reading source affords its ceiling, and the field-free spelling of the wrapped backend.

use super::types::{ClaimCeiling, ReadingSource, WrapOutcomeWord, WrappedBackend};
use crate::muterprater::{ExecutionAxis, MaterializationAxis};

impl From<WrapOutcomeWord> for MaterializationAxis {
    /// Whether the damage the backend's word describes became executable.
    ///
    /// Caught, missed, and timed-out all describe a mutant that built, because the backend could not have run a command over one that did not.
    /// Unviable is the backend saying the damage never became a thing, and a tool failure is the backend saying it does not know.
    fn from(word: WrapOutcomeWord) -> Self {
        match word {
            WrapOutcomeWord::Caught | WrapOutcomeWord::Missed | WrapOutcomeWord::TimedOut => {
                Self::Built
            }
            WrapOutcomeWord::Unviable => Self::Unviable,
            WrapOutcomeWord::ToolFailed => Self::ToolFailed,
        }
    }
}

impl From<WrapOutcomeWord> for ExecutionAxis {
    /// What became of the witness execution the backend's word describes.
    ///
    /// A mutant that never materialized never ran, and a backend that failed around one learned nothing about it: both read as an infrastructure failure.
    fn from(word: WrapOutcomeWord) -> Self {
        match word {
            WrapOutcomeWord::Caught | WrapOutcomeWord::Missed => Self::Completed,
            WrapOutcomeWord::TimedOut => Self::TimedOut,
            WrapOutcomeWord::Unviable | WrapOutcomeWord::ToolFailed => Self::InfrastructureFailed,
        }
    }
}

impl From<ReadingSource> for ClaimCeiling {
    /// The most a reading taken from one output can establish.
    ///
    /// A console stream states which of the backend's own mutants its command rejected and carries no channel that could observe a firing, so the reading it affords tops out at witness rejection.
    fn from(source: ReadingSource) -> Self {
        match source {
            ReadingSource::ConsoleStream => Self::WitnessRejection,
        }
    }
}

impl WrappedBackend {
    /// The backend's own name.
    ///
    /// A projection: a reader of a profile names the tool through it, and no decision anywhere consults it.
    #[must_use]
    pub const fn spelling(self) -> &'static str {
        match self {
            Self::CargoMutants => "cargo-mutants",
        }
    }
}
