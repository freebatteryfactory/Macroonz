#![doc = include_str!("README.md")]

mod encode;
mod encode_record;
mod encode_run;
mod size_record;
mod type_contract;
mod types;

pub(crate) use encode::{activation_size, target_size, write_activation, write_target};
pub use encode_record::retain_mutation;
pub use encode_run::retain_mutation_run;
pub use types::{
    ArchivedActivation, ArchivedActivationReading, ArchivedMutationIdentity, ArchivedMutationSite,
    ArchivedMutationTarget,
};
pub use types::{
    ArchivedMutation, ArchivedMutationOutcome, ArchivedRejection, MUTATION_ARCHIVE_TAG,
    MutationArchiveRefusal, read_mutation,
};
pub use types::{
    ArchivedMutationRun, MUTATION_RUN_ARCHIVE_TAG, MutationRunArchiveLimits, read_mutation_run,
};
pub(crate) use types::{read_activation, read_target};
