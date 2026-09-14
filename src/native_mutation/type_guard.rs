use super::{
    MutationCustodyError, MutationError, MutationObservationError, MutationOutput, MutationRequest,
    MutationRun, MutationSources, MutationToolchain, PendingMutation,
};
use crate::harness::muterprater::{AdapterQualification, CompiledSuiteArtifactManifest};
use crate::harness::oracle::RelativeSourcePath;
use crate::native_process::{
    ProcessLimits, ProcessOutput, ProcessRequest, ProcessRun, ProcessTool,
};
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

impl MutationToolchain {
    /// Admits explicit Cargo and rustc paths without acquiring host facts.
    ///
    /// # Errors
    /// Refuses relative, traversing, non-Unicode or NUL-bearing paths.
    pub fn declared(cargo: PathBuf, rustc: PathBuf) -> Result<Self, MutationError> {
        absolute(&cargo)?;
        absolute(&rustc)?;
        Ok(Self { cargo, rustc })
    }
}

impl MutationSources {
    /// Admits unique source files under one aggregate byte bound.
    ///
    /// # Errors
    /// Refuses an empty roster, zero or unaddressable bound, duplicate paths and backend glob metacharacters.
    pub fn declared(files: Vec<RelativeSourcePath>, bytes: usize) -> Result<Self, MutationError> {
        if files.is_empty() || bytes == 0 || isize::try_from(bytes).is_err() {
            return Err(configuration(
                "source files and an addressable positive bound are required",
            ));
        }
        let mut distinct = std::collections::BTreeSet::new();
        for file in &files {
            if file
                .spelling()
                .contains(['*', '?', '[', ']', '{', '}', '\0'])
                || file
                    .spelling()
                    .chars()
                    .any(|character| character.is_whitespace() || character.is_control())
                || !distinct.insert(file.spelling())
            {
                return Err(configuration(
                    "source files must be unique literal paths representable by the console grammar",
                ));
            }
        }
        Ok(Self { files, bytes })
    }

    /// The declared source paths in input order.
    pub fn files(&self) -> &[RelativeSourcePath] {
        &self.files
    }

    /// The aggregate source-read ceiling.
    #[must_use]
    pub const fn byte_bound(&self) -> usize {
        self.bytes
    }
}

impl MutationRequest {
    /// Binds the supported cargo-mutants command to explicit native inputs.
    ///
    /// # Errors
    /// Refuses invalid paths or targets, conflicting tool bindings and unrepresentable process arguments.
    pub fn cargo_mutants(
        backend: &ProcessTool,
        toolchain: MutationToolchain,
        sources: MutationSources,
        output: PathBuf,
        target_directory: &Path,
        target: &str,
    ) -> Result<Self, MutationError> {
        absolute(backend.executable())?;
        absolute(backend.directory())?;
        absolute(&output)?;
        absolute(target_directory)?;
        if target.is_empty() || target.contains('\0') {
            return Err(configuration("an explicit target triple is required"));
        }
        let mut environment = backend.environment().to_vec();
        binding(&mut environment, "CARGO", spelling(&toolchain.cargo)?)?;
        binding(&mut environment, "RUSTC", spelling(&toolchain.rustc)?)?;
        let mut arguments = [
            "mutants",
            "--no-config",
            "--baseline=run",
            "--caught",
            "--unviable",
            "--no-times",
            "--no-shuffle",
            "--colors=never",
            "--annotations=none",
            "--jobs=1",
            "--jobserver=false",
            "--copy-target=false",
            "--copy-vcs=false",
            "--cap-lints=false",
            "--test-tool=cargo",
            "--dir",
            spelling(backend.directory())?,
            "--output",
            spelling(&output)?,
            "--cargo-arg=--locked",
            "--cargo-arg=--offline",
            "--cargo-arg=--jobs=1",
        ]
        .map(str::to_owned)
        .to_vec();
        arguments.push(format!("--cargo-arg=--target={target}"));
        arguments.push(format!(
            "--cargo-arg=--target-dir={}",
            spelling(target_directory)?
        ));
        for file in &sources.files {
            arguments.push("--file".to_owned());
            arguments.push(file.spelling().to_owned());
            arguments.push("--re".to_owned());
            arguments.push(super::super::prepare::source_filter(file.spelling())?);
        }
        let compiler = ProcessRequest::informed(
            toolchain.rustc,
            backend.directory().to_path_buf(),
            vec!["--version".to_owned()],
            environment.clone(),
            backend.limits(),
            &[],
        )
        .map_err(MutationError::Process)?;
        let process = ProcessRequest::informed(
            backend.executable().to_path_buf(),
            backend.directory().to_path_buf(),
            arguments,
            environment,
            backend.limits(),
            &[],
        )
        .map_err(MutationError::Process)?;
        Ok(Self {
            process,
            compiler,
            sources,
            output,
            target: target.to_owned(),
        })
    }

    /// Selects independent limits for backend and compiler version queries before execution.
    ///
    /// # Errors
    /// Preserves native process admission refusals when rebuilding the query request.
    pub fn version_queries(mut self, limits: ProcessLimits) -> Result<Self, MutationError> {
        self.compiler = ProcessRequest::informed(
            self.compiler.executable().to_path_buf(),
            self.compiler.directory().to_path_buf(),
            self.compiler.arguments().to_vec(),
            self.compiler.environment().to_vec(),
            limits,
            &[],
        )
        .map_err(MutationError::Process)?;
        Ok(self)
    }

    /// The version-query execution, cleanup and capture limits.
    #[must_use]
    pub const fn version_limits(&self) -> ProcessLimits {
        self.compiler.limits()
    }

    /// The exact command and effective environment selected for backend execution.
    #[must_use]
    pub const fn process(&self) -> &ProcessRequest {
        &self.process
    }

    /// The declared source capture roster and bound.
    #[must_use]
    pub const fn sources(&self) -> &MutationSources {
        &self.sources
    }

    /// The exclusively created output directory for this execution.
    #[must_use]
    pub fn output_directory(&self) -> &Path {
        &self.output
    }
}

impl MutationOutput {
    /// Retains this execution through the existing archive with its original material.
    ///
    /// # Errors
    /// Refuses an incomplete observation or the archive owner's bounds and consistency checks.
    pub fn retain(
        &self,
        limits: crate::harness::muterprater::backend_archive::BackendArchiveLimits,
    ) -> Result<
        crate::harness::muterprater::backend_archive::ArchivedBackendManifest,
        MutationCustodyError,
    > {
        let observed = self
            .observation
            .as_ref()
            .map_err(|error| MutationCustodyError::Observation(error.clone()))?;
        let sources: Vec<_> = self
            .original_sources()
            .filter(|(file, _)| {
                observed
                    .manifest
                    .sources()
                    .iter()
                    .any(|source| source.file() == *file)
            })
            .collect();
        crate::harness::muterprater::backend_archive::retain_backend_with_material(
            &observed.manifest,
            &observed.console,
            &sources,
            limits,
        )
        .map_err(MutationCustodyError::Archive)
    }

    /// Rereads the manifest's complete source roster and delegates current custody to its owner.
    ///
    /// # Errors
    /// Refuses an incomplete observation, bounded source read failure or moved source bytes.
    pub fn current_sources(
        &self,
        root: &Path,
        byte_bound: usize,
    ) -> Result<crate::harness::muterprater::CompiledSuiteArtifactCustody, MutationCustodyError>
    {
        let manifest = self
            .manifest()
            .map_err(|error| MutationCustodyError::Observation(error.clone()))?;
        let current = super::super::custody::revisions(
            root,
            manifest
                .sources()
                .iter()
                .map(crate::harness::muterprater::MutationSourceRevision::file),
            byte_bound,
        )?;
        crate::harness::muterprater::CompiledSuiteArtifactCustody::current(
            manifest.clone(),
            current,
        )
        .map_err(MutationCustodyError::Current)
    }

    /// The immutable request that produced this result.
    #[must_use]
    pub const fn request(&self) -> &MutationRequest {
        &self.context.request
    }

    /// The actual backend process exit, stop and pipe observations.
    #[must_use]
    pub const fn process(&self) -> &ProcessOutput {
        &self.process
    }

    /// The backend version query retained before mutation execution.
    #[must_use]
    pub const fn backend_version_output(&self) -> &ProcessOutput {
        &self.context.backend_version
    }

    /// The selected compiler's version query retained before mutation execution.
    #[must_use]
    pub const fn compiler_version_output(&self) -> &ProcessOutput {
        &self.context.compiler_version
    }

    /// The original declared source bytes retained before backend execution.
    pub fn original_sources(&self) -> impl Iterator<Item = (&str, &[u8])> {
        self.context
            .sources
            .iter()
            .map(|source| (source.file.as_str(), source.bytes.as_slice()))
    }

    /// The existing parser's manifest bound to this actual execution.
    ///
    /// # Errors
    /// Returns the specific failure that prevented a complete observation.
    pub fn manifest(&self) -> Result<&CompiledSuiteArtifactManifest, &MutationObservationError> {
        self.observation.as_ref().map(|observed| &observed.manifest)
    }

    /// The existing grammar qualification for this execution's observed profile.
    ///
    /// # Errors
    /// Returns the observation-establishment failure without granting grammar standing.
    pub fn qualification(&self) -> Result<&AdapterQualification, &MutationObservationError> {
        self.observation
            .as_ref()
            .map(|observed| &observed.qualification)
    }
}

impl PendingMutation {
    /// The process owner awaiting cleanup.
    pub fn process(&self) -> &crate::native_process::PendingProcess {
        &self.process
    }

    /// Retries cleanup while retaining the original source and execution context.
    pub fn finish(self, budget: Duration) -> MutationRun {
        super::super::execute::finish(self.context, self.process.finish(budget))
    }
}

impl MutationError {
    /// Retries a failed version query's pending cleanup without resuming preparation.
    #[must_use]
    pub fn finish_cleanup(self, budget: Duration) -> Self {
        match self {
            Self::Query {
                phase,
                request,
                run,
                cause,
            } => {
                let run = match *run {
                    ProcessRun::Pending(pending) => pending.finish(budget),
                    finished @ ProcessRun::Finished(_) => finished,
                };
                Self::Query {
                    phase,
                    request,
                    run: Box::new(run),
                    cause,
                }
            }
            other @ (Self::Configuration(_)
            | Self::Filesystem(_)
            | Self::SourceBound { .. }
            | Self::Process(_)) => other,
        }
    }
}

fn binding(
    environment: &mut Vec<(String, String)>,
    name: &str,
    value: &str,
) -> Result<(), MutationError> {
    if let Some((_, existing)) = environment.iter().find(|(key, _)| {
        if cfg!(windows) {
            key.eq_ignore_ascii_case(name)
        } else {
            key == name
        }
    }) {
        if existing != value {
            return Err(configuration("conflicting explicit tool binding"));
        }
    } else {
        environment.push((name.to_owned(), value.to_owned()));
    }
    Ok(())
}

fn absolute(path: &Path) -> Result<(), MutationError> {
    let text = spelling(path)?;
    if !path.is_absolute()
        || text.contains('\0')
        || text
            .split(std::path::is_separator)
            .any(|part| matches!(part, "." | ".."))
        || path
            .components()
            .any(|part| matches!(part, Component::CurDir | Component::ParentDir))
    {
        return Err(configuration("absolute normal paths are required"));
    }
    Ok(())
}

fn spelling(path: &Path) -> Result<&str, MutationError> {
    path.to_str()
        .ok_or_else(|| configuration("mutation paths must be Unicode"))
}

fn configuration(message: &str) -> MutationError {
    MutationError::Configuration(message.to_owned())
}
