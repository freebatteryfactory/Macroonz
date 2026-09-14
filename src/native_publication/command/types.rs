use crate::compiler::Kind;
use crate::native_compiler::CompilerRequest;
use crate::native_process::ProcessTool;
use crate::native_publication::{
    AuthoredFile, CompiledPublication, DestinationCheck, DestinationError, FormatError,
    FormatOutput, FormatRun, InventoryError, PreparationError, PreparedPublication, Publication,
    PublicationDestination, PublicationLimits, StagingError, StagingRun,
};
use std::path::PathBuf;
use std::time::Duration;

#[path = "type_guard.rs"]
mod guard;

/// The complete selected native-process allowance for a publication command.
#[derive(Debug, Clone, Copy)]
pub struct BakeToolBudget {
    /// Maximum selected version-query, formatter and compiler invocations.
    pub runs: usize,
    /// Maximum sum of their declared execution and cleanup budgets.
    pub time: Duration,
    /// Maximum sum of their declared stdout and stderr retention bounds.
    pub capture_bytes: usize,
}

/// An explicit rustfmt tool selection and its existing configuration file.
#[derive(Debug, Clone)]
pub struct BakeFormatter {
    /// The declared executable, directory, environment and process bounds.
    pub tool: ProcessTool,
    /// The absolute configuration file selected for qualification and execution.
    pub configuration: PathBuf,
}

/// The explicit disposable workspace, optional formatter and complete preparation limits.
#[derive(Debug, Clone)]
pub struct BakePreparation {
    /// An existing disposable workspace coordinated through native-storage custody.
    pub workspace: PathBuf,
    /// Explicit rustfmt selection, or preservation of the compiler's unformatted source.
    pub formatter: Option<BakeFormatter>,
    /// Complete generated-file and physical-byte bounds before and after formatting.
    pub output: PublicationLimits,
    /// Aggregate declared native-process allowances.
    pub tools: BakeToolBudget,
}

/// The complete authored compilation inputs and destination selected for generation.
#[derive(Debug)]
pub struct BakeGeneration {
    /// Preparation and complete native-process bounds, including compilation.
    pub preparation: BakePreparation,
    /// The exact caller-owned fixture, manifest, lockfile and other staged inputs.
    pub authored: Vec<AuthoredFile>,
    /// The admitted compiler template relocated into the actual private source tree.
    pub compiler: CompilerRequest,
    /// Complete authored/generated staging bounds.
    pub source_limits: PublicationLimits,
    /// The explicitly opened output root and its installation/check limits.
    pub destination: PublicationDestination,
}

/// One explicitly selected publication action and the inputs that action needs.
#[derive(Debug)]
pub enum BakeCommand {
    /// Produce and inspect the compiler's complete canonical publication inventory without native effects.
    Prepare,
    /// Prepare optional formatted physical bytes without checking or installing a destination.
    Inspect(Box<BakePreparation>),
    /// Compare complete expected physical output without writing to the selected destination.
    Check {
        /// Explicit preparation inputs and limits.
        preparation: Box<BakePreparation>,
        /// The destination to inspect without creating a lock or record there.
        destination: PublicationDestination,
    },
    /// Compile privately, preflight and install the complete generated set.
    Generate(Box<BakeGeneration>),
    /// Complete retained installation intent without claiming a fresh compilation.
    Recover(PublicationDestination),
}

/// The original publication material and actual result of one completed command.
#[derive(Debug)]
pub enum BakeOutput<K: Kind> {
    /// The registered generation operation's complete sealed inventory.
    Declared(Publication<K>),
    /// Complete physical output and any actual formatter observations.
    Prepared(PreparedPublication<K>),
    /// Complete expected physical output and its read-only destination comparison.
    Checked {
        /// Expected output from the selected current generation operation.
        prepared: PreparedPublication<K>,
        /// The complete current/owned/expected comparison, which may report discrepancies.
        comparison: DestinationCheck,
    },
    /// Actual compiled physical output after successful installation and a current destination check.
    Generated(Box<CompiledPublication<K>>),
    /// Recovery returned successfully; any recovered claims retain their historical standing.
    Recovered,
}

/// A publication command failure retaining its typed cause and any unfinished workspace custody.
#[derive(Debug)]
pub struct BakeError<K: Kind, E> {
    pub(super) cause: Box<BakeCause<K, E>>,
    pub(super) lease: Option<std::fs::File>,
}

/// The owner-specific failure or stopped native observation from one publication command.
#[derive(Debug)]
pub enum BakeCause<K: Kind, E> {
    /// The caller-owned registered generation operation refused its declared input.
    Declaration(E),
    /// The selected command cannot fit its complete declared configuration or budgets.
    Configuration(String),
    /// The complete generated inventory exceeded its selected path or resource allowance.
    Inventory(InventoryError),
    /// Native workspace capability or exclusive custody failed.
    Storage(crate::native_storage::StorageError),
    /// Access to the disposable input file failed.
    Filesystem(std::io::Error),
    /// Formatter setup or startup failed, retaining a query observation when reached.
    Formatter(FormatError),
    /// Formatting stopped before a complete prepared set could be admitted.
    Formatting {
        /// Actual complete earlier formatter observations.
        completed: Vec<FormatOutput>,
        /// The failing or still-pending formatter execution.
        run: FormatRun,
    },
    /// Complete formatted-output admission refused.
    Preparation(PreparationError),
    /// Staging or compiler startup refused.
    Staging(StagingError),
    /// Staged compilation refused or retains unfinished cleanup.
    Compilation(StagingRun<K>),
    /// Destination inspection, installation or recovery refused.
    Destination {
        /// The destination owner's actual refusal.
        error: DestinationError,
        /// The actual compiled set, when generation reached that boundary.
        compiled: Option<Box<CompiledPublication<K>>>,
    },
}

#[derive(Debug)]
pub(super) struct Workspace {
    #[cfg(any(unix, windows))]
    pub(super) root: crate::native_storage::StorageRoot,
    pub(super) lease: std::fs::File,
}
