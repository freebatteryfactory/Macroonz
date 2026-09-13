#![doc = include_str!("README.md")]

mod inventory;
mod formatter;
mod prepared;
mod staging;
mod destination;

pub use destination::{
    DestinationCheck, DestinationError, DestinationIssue, DestinationLimits, DestinationProblem,
    DestinationState, PublicationDestination, PublicationInstallation,
};

pub use staging::{
    AuthoredFile, CompiledPublication, PendingStaging, RefusedStaging, StagedPublication,
    StagingError, StagingObservationError, StagingPlan, StagingRun,
};

pub use prepared::{PreparationError, PreparedFile, PreparedPublication};

pub use formatter::{
    FormatError, FormatObservationError, FormatOutput, FormatRun, Formatter, PendingFormat,
};

pub use inventory::{
    CanonicalPublicationBytes, InventoryError, LandingBinding, Publication, PublicationBinding,
    PublicationFile, PublicationLimits, PublicationPath, PublishedBytes, published_digest,
};
mod command;

pub use command::{
    BakeCause, BakeCommand, BakeError, BakeFormatter, BakeGeneration, BakeOutput, BakePreparation,
    BakeToolBudget, bake,
};
