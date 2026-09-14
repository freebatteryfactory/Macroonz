use super::{
    FormatContext, FormatError, FormatObservationError, FormatOutput, FormatRun, Formatter,
    PendingFormat,
};
use crate::compiler::Role;
use crate::compiler::identity::Identity;
use crate::native_process::{ProcessLimits, ProcessOutput, ProcessRequest, ProcessTool};
use crate::native_publication::{
    CanonicalPublicationBytes, PublicationFile, PublicationPath, PublishedBytes, published_digest,
};
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;

impl Formatter {
    /// Observes the selected rustfmt version and captures its explicit configuration file.
    ///
    /// # Errors
    /// Refuses unavailable configuration, an unsupported version or incomplete query cleanup.
    pub fn qualified(tool: &ProcessTool, config: PathBuf) -> Result<Self, FormatError> {
        super::super::execute::qualify(tool, config)
    }

    /// Selects file-execution budgets independently of the completed version query.
    ///
    /// # Errors
    /// Returns native admission failure while preserving the selected command and environment.
    pub fn execution_limits(mut self, limits: ProcessLimits) -> Result<Self, FormatError> {
        self.process = ProcessRequest::informed(
            self.process.executable().to_path_buf(),
            self.process.directory().to_path_buf(),
            self.process.arguments().to_vec(),
            self.process.environment().to_vec(),
            limits,
            &[],
        )
        .map_err(FormatError::Process)?;
        Ok(self)
    }

    /// Formats one admitted file using an explicitly supplied disposable read/write input file.
    ///
    /// The scratch file is truncated, populated, rewound and consumed as child stdin.
    ///
    /// # Errors
    /// Refuses changed configuration, unsuitable scratch input or native process startup failure.
    pub fn format<R: Role>(
        &self,
        source: &PublicationFile<'_, R>,
        scratch: File,
    ) -> Result<FormatRun, FormatError> {
        let version = std::str::from_utf8(self.version.stdout().bytes())
            .map_err(|error| FormatError::Configuration(error.to_string()))?
            .to_owned();
        let context = FormatContext {
            path: source.path().clone(),
            canonical: source.canonical_digest(),
            source: source.source(),
            request: self.process.clone(),
            version,
            config: self.config.clone(),
            configuration: self.configuration.clone(),
        };
        super::super::execute::format(context, scratch)
    }

    /// The exact version query and its native observation.
    #[must_use]
    pub const fn version(&self) -> (&ProcessRequest, &ProcessOutput) {
        (&self.query, &self.version)
    }

    /// The exact command used for file formatting.
    #[must_use]
    pub const fn request(&self) -> &ProcessRequest {
        &self.process
    }
}

impl FormatOutput {
    /// The source file's declared destination.
    #[must_use]
    pub const fn path(&self) -> &PublicationPath {
        &self.context.path
    }

    /// The original unformatted source sent to the child.
    #[must_use]
    pub fn original(&self) -> &str {
        &self.context.source
    }

    /// The actual formatting command.
    #[must_use]
    pub const fn request(&self) -> &ProcessRequest {
        &self.context.request
    }

    /// The observed version text and exact explicit configuration bytes.
    #[must_use]
    pub fn profile(&self) -> (&str, &[u8]) {
        (&self.context.version, &self.context.configuration)
    }

    /// The actual native result, including failed or truncated output.
    #[must_use]
    pub const fn process(&self) -> &ProcessOutput {
        &self.process
    }

    /// The admitted formatted source from complete successful stdout.
    ///
    /// # Errors
    /// Returns the specific process, text or configuration refusal.
    pub fn source(&self) -> Result<&str, FormatObservationError> {
        self.observation.clone()?;
        std::str::from_utf8(self.process.stdout().bytes())
            .map_err(|_error| FormatObservationError::Text)
    }

    /// The original canonical token commitment, unaffected by formatting.
    #[must_use]
    pub const fn canonical_digest(&self) -> Identity<CanonicalPublicationBytes> {
        self.context.canonical
    }

    /// The commitment to exactly the admitted physical output bytes.
    ///
    /// # Errors
    /// Refuses when the formatter established no publishable text.
    pub fn published_digest(&self) -> Result<Identity<PublishedBytes>, FormatObservationError> {
        self.source()
            .map(|source| published_digest(source.as_bytes()))
    }
}

impl PendingFormat {
    /// The pending native cleanup owner.
    pub fn process(&self) -> &crate::native_process::PendingProcess {
        &self.process
    }

    /// Retries cleanup before admitting any formatter output.
    pub fn finish(self, budget: Duration) -> FormatRun {
        super::super::execute::finish(self.context, self.process.finish(budget))
    }
}
