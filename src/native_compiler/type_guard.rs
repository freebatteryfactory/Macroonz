use super::{
    CargoFixture, CargoTarget, CompilerError, CompilerObservationError, CompilerOutput,
    CompilerRequest, CompilerRun, PendingCompilation, Protocol, SelectedCargoMessage,
};
use crate::harness::oracle::{
    CompilationVerdict, DeclaredCompilation, ObservedCompilation, RelativeSourcePath,
};
use crate::native_process::{ProcessOutput, ProcessRequest, ProcessRun, ProcessTool};
use std::path::{Path, PathBuf};
use std::time::Duration;

impl CargoFixture {
    /// Admits one explicit Cargo package and target without inspecting the filesystem.
    ///
    /// # Errors
    /// Refuses nonabsolute paths, empty selectors or unrepresentable arguments.
    pub fn informed(
        manifest: PathBuf,
        target_directory: PathBuf,
        package: String,
        target: CargoTarget,
        triple: String,
    ) -> Result<Self, CompilerError> {
        absolute(&manifest)?;
        absolute(&target_directory)?;
        selector(&package)?;
        selector(&triple)?;
        if let CargoTarget::Binary(name) = &target {
            selector(name)?;
        }
        Ok(Self {
            manifest,
            target_directory,
            package,
            target,
            triple,
        })
    }

    pub(in crate::native_compiler) fn target_directory(&self) -> &Path {
        &self.target_directory
    }
    pub(in crate::native_compiler) const fn target(&self) -> &CargoTarget {
        &self.target
    }
}

impl CompilerRequest {
    /// Relocates the declared source root while preserving tool policy, output selection and diagnostic locus.
    ///
    /// # Errors
    /// Refuses invalid roots, a Cargo manifest outside the original root or inconsistent owned arguments.
    pub fn relocated(mut self, directory: PathBuf) -> Result<Self, CompilerError> {
        absolute(&directory)?;
        let mut arguments = self.process.arguments().to_vec();
        match &mut self.protocol {
            Protocol::Rustc { .. } => {
                let original = self.process.directory().join(self.locus.spelling());
                let input = arguments.last_mut().ok_or_else(|| {
                    CompilerError::Configuration("rustc source argument absent".to_owned())
                })?;
                if input != spelling(&original)? {
                    return Err(CompilerError::Configuration(
                        "rustc source argument differs from its declared locus".to_owned(),
                    ));
                }
                spelling(&directory.join(self.locus.spelling()))?.clone_into(input);
            }
            Protocol::Cargo(fixture) => {
                let relative = fixture
                    .manifest
                    .strip_prefix(self.process.directory())
                    .map_err(|_error| {
                        CompilerError::Configuration(
                            "Cargo manifest is outside the original source root".to_owned(),
                        )
                    })?;
                if relative.as_os_str().is_empty() {
                    return Err(CompilerError::Configuration(
                        "Cargo manifest must name a file below its source root".to_owned(),
                    ));
                }
                let next = directory.join(relative);
                let at = arguments
                    .iter()
                    .position(|argument| argument == "--manifest-path")
                    .and_then(|at| at.checked_add(1))
                    .ok_or_else(|| {
                        CompilerError::Configuration("Cargo manifest argument absent".to_owned())
                    })?;
                let argument = arguments.get_mut(at).ok_or_else(|| {
                    CompilerError::Configuration("Cargo manifest value absent".to_owned())
                })?;
                if argument != spelling(&fixture.manifest)? {
                    return Err(CompilerError::Configuration(
                        "Cargo manifest argument differs from its declared fixture".to_owned(),
                    ));
                }
                spelling(&next)?.clone_into(argument);
                fixture.manifest = next;
            }
        }
        self.process = ProcessRequest::informed(
            self.process.executable().to_path_buf(),
            directory,
            arguments,
            self.process.environment().to_vec(),
            self.process.limits(),
            &[],
        )
        .map_err(CompilerError::Process)?;
        Ok(self)
    }

    /// The selected Cargo manifest, absent for a direct rustc invocation.
    #[must_use]
    pub fn manifest(&self) -> Option<&Path> {
        match &self.protocol {
            Protocol::Cargo(fixture) => Some(&fixture.manifest),
            Protocol::Rustc { .. } => None,
        }
    }

    /// The selected dependency-output file, when capture was requested.
    #[must_use]
    pub fn dependency_file(&self) -> Option<&Path> {
        self.dependencies
            .as_ref()
            .map(|selection| selection.path.as_path())
    }

    /// The Cargo target directory or direct rustc link-output parent.
    #[must_use]
    pub fn output_root(&self) -> Option<&Path> {
        match &self.protocol {
            Protocol::Cargo(fixture) => Some(&fixture.target_directory),
            Protocol::Rustc { artifact } => artifact.parent(),
        }
    }

    pub(crate) fn artifact_file(&self) -> Option<&Path> {
        match &self.protocol {
            Protocol::Rustc { artifact } => Some(artifact),
            Protocol::Cargo(_) => None,
        }
    }
    /// Selects Rust source-coverage instrumentation for this compiler invocation.
    ///
    /// Cargo receives an explicit encoded flag selection for the target and its Rust dependencies.
    ///
    /// # Errors
    /// Refuses conflicting caller Rust flag entries or an unrepresentable process request.
    pub fn instrumented(mut self) -> Result<Self, CompilerError> {
        let mut arguments = self.process.arguments().to_vec();
        let mut environment = self.process.environment().to_vec();
        match self.protocol {
            Protocol::Rustc { .. } => {
                let flag = "-Cinstrument-coverage";
                if !arguments.iter().any(|argument| argument == flag) {
                    arguments.insert(0, flag.to_owned());
                }
            }
            Protocol::Cargo(_) => {
                environment = cargo_coverage_environment(&self.process, "-Cinstrument-coverage")?;
            }
        }
        self.process = ProcessRequest::informed(
            self.process.executable().to_path_buf(),
            self.process.directory().to_path_buf(),
            arguments,
            environment,
            self.process.limits(),
            &[],
        )
        .map_err(CompilerError::Process)?;
        Ok(self)
    }

    /// Selects Rust source coverage for the chosen target without adding instrumentation to its dependencies.
    ///
    /// Cargo receives the coverage flag on its final rustc invocation and an empty encoded flag selection for dependencies.
    ///
    /// # Errors
    /// Refuses conflicting caller Rust flags, repeated Cargo instrumentation or an unrepresentable process request.
    pub fn instrumented_target(mut self) -> Result<Self, CompilerError> {
        if matches!(self.protocol, Protocol::Rustc { .. }) {
            return self.instrumented();
        }
        let environment = cargo_coverage_environment(&self.process, "")?;
        let mut arguments = self.process.arguments().to_vec();
        let command = arguments
            .first_mut()
            .ok_or_else(|| CompilerError::Configuration("Cargo command absent".to_owned()))?;
        "rustc".clone_into(command);
        arguments.extend(["--".to_owned(), "-Cinstrument-coverage".to_owned()]);
        self.process = ProcessRequest::informed(
            self.process.executable().to_path_buf(),
            self.process.directory().to_path_buf(),
            arguments,
            environment,
            self.process.limits(),
            &[],
        )
        .map_err(CompilerError::Process)?;
        Ok(self)
    }

    /// Builds a Rust 2024 binary fixture with structured diagnostics and artifact notifications.
    ///
    /// # Errors
    /// Refuses an invalid target, output path or process argument.
    pub fn rustc(
        tool: &ProcessTool,
        source: RelativeSourcePath,
        artifact: PathBuf,
        target: &str,
    ) -> Result<Self, CompilerError> {
        absolute(&artifact)?;
        selector(target)?;
        let input_path = tool.directory().join(source.spelling());
        let arguments = [
            "--edition=2024",
            "--crate-name=macroonz_fixture",
            "--crate-type=bin",
            "--error-format=json",
            "--json=artifacts",
            "--target",
            target,
            "-o",
            spelling(&artifact)?,
            spelling(&input_path)?,
        ]
        .map(str::to_owned)
        .to_vec();
        let process = tool.invocation(arguments).map_err(CompilerError::Process)?;
        absolute(process.directory())?;
        Ok(Self {
            process,
            protocol: Protocol::Rustc { artifact },
            locus: source,
            dependencies: None,
        })
    }

    /// Builds one locked, offline Cargo fixture with JSON messages and one compilation job.
    ///
    /// The selected working directory is the declared basis of rustc's relative source paths.
    ///
    /// # Errors
    /// Refuses an invalid working directory or process argument.
    pub fn cargo(
        tool: &ProcessTool,
        fixture: CargoFixture,
        locus: RelativeSourcePath,
    ) -> Result<Self, CompilerError> {
        let mut arguments = [
            "build",
            "--locked",
            "--offline",
            "--message-format=json",
            "--jobs=1",
            "--manifest-path",
            spelling(&fixture.manifest)?,
            "--target-dir",
            spelling(&fixture.target_directory)?,
            "--package",
            &fixture.package,
            "--target",
            &fixture.triple,
        ]
        .map(str::to_owned)
        .to_vec();
        match &fixture.target {
            CargoTarget::Library => arguments.push("--lib".to_owned()),
            CargoTarget::Binary(name) => {
                arguments.push("--bin".to_owned());
                arguments.push(name.clone());
            }
        }
        let process = tool.invocation(arguments).map_err(CompilerError::Process)?;
        absolute(process.directory())?;
        Ok(Self {
            process,
            protocol: Protocol::Cargo(fixture),
            locus,
            dependencies: None,
        })
    }

    /// Selects bounded source-dependency capture from an explicit compiler output path.
    ///
    /// Rustc additionally emits that dependency file alongside its selected link artifact.
    ///
    /// # Errors
    /// Refuses repeated selection, ambiguous paths and collisions with the primary input or link output.
    pub fn with_dependencies(
        mut self,
        path: PathBuf,
        limits: super::super::DependencyLimits,
    ) -> Result<Self, CompilerError> {
        absolute(&path)?;
        if self.dependencies.is_some()
            || path == self.process.directory().join(self.locus.spelling())
        {
            return Err(CompilerError::Configuration(
                "dependency output collides with its source or repeats selection".to_owned(),
            ));
        }
        if let Protocol::Rustc { artifact } = &self.protocol {
            if path == *artifact {
                return Err(CompilerError::Configuration(
                    "dependency and link outputs collide".to_owned(),
                ));
            }
            let mut arguments = self.process.arguments().to_vec();
            arguments.insert(0, format!("--emit=link,dep-info={}", spelling(&path)?));
            self.process = ProcessRequest::informed(
                self.process.executable().to_path_buf(),
                self.process.directory().to_path_buf(),
                arguments,
                self.process.environment().to_vec(),
                self.process.limits(),
                &[],
            )
            .map_err(CompilerError::Process)?;
        }
        self.dependencies = Some(super::DependencyCapture { path, limits });
        Ok(self)
    }

    /// The exact invocation selected by the compiler host.
    #[must_use]
    pub const fn process(&self) -> &ProcessRequest {
        &self.process
    }
    /// The logical source selected independently of an expected diagnostic code.
    pub const fn locus(&self) -> &RelativeSourcePath {
        &self.locus
    }
}

impl CompilerOutput {
    /// The optional execution-time source-dependency observation.
    pub fn dependencies(
        &self,
    ) -> Option<Result<&super::super::DependencyInfo, &super::super::DependencyError>> {
        self.dependencies.as_ref().map(Result::as_ref)
    }
    /// The exact invocation that produced this output.
    #[must_use]
    pub const fn request(&self) -> &CompilerRequest {
        &self.request
    }
    /// The complete process observation, including exit and pipe standing.
    #[must_use]
    pub const fn process(&self) -> &ProcessOutput {
        &self.process
    }
    /// The compiler's established acceptance or exact refusal.
    ///
    /// # Errors
    /// Returns the specific reason this process did not establish a compiler observation.
    pub fn observed(&self) -> Result<&ObservedCompilation, &CompilerObservationError> {
        self.observation
            .as_ref()
            .map(|observation| &observation.compilation)
    }
    /// Compares the established observation through the existing exact compilation owner.
    ///
    /// # Errors
    /// Returns an observation-establishment failure without creating a semantic verdict.
    pub fn compared(
        &self,
        declared: &DeclaredCompilation,
    ) -> Result<CompilationVerdict, &CompilerObservationError> {
        self.observed().map(|observed| {
            crate::harness::oracle::compiled::compared_compilation(observed, declared)
        })
    }
    /// The executable reported for a successful binary build, when one exists.
    #[must_use]
    pub fn executable(&self) -> Option<&Path> {
        self.observation
            .as_ref()
            .ok()?
            .artifact
            .as_ref()?
            .executable
            .as_deref()
    }
    /// Cargo's selected artifact freshness, absent for rustc or a refused build.
    #[must_use]
    pub fn cargo_fresh(&self) -> Option<bool> {
        self.observation
            .as_ref()
            .ok()?
            .artifact
            .as_ref()?
            .cargo_fresh
    }
    /// Executes the successfully compiled binary under an explicitly supplied read-back policy.
    ///
    /// # Errors
    /// Refuses a missing compiled executable, a different requested executable or process startup failure.
    pub fn read_back(
        &self,
        request: &ProcessRequest,
        stdin: Option<std::fs::File>,
    ) -> Result<ProcessRun, CompilerError> {
        if self.executable() != Some(request.executable()) {
            return Err(CompilerError::Configuration(
                "read-back must select this successful build's executable".to_owned(),
            ));
        }
        crate::native_process::run(request, stdin).map_err(CompilerError::Process)
    }
}

impl PendingCompilation {
    /// The native cleanup owner whose stop and latest cleanup error remain visible.
    pub fn process(&self) -> &crate::native_process::PendingProcess {
        &self.process
    }
    /// Retries cleanup before attempting to interpret compiler output.
    pub fn finish(self, budget: Duration) -> CompilerRun {
        super::super::execute::finish(self.request, self.process.finish(budget))
    }
}

impl<'message> SelectedCargoMessage<'message> {
    pub(in crate::native_compiler) fn for_fixture(
        value: &'message serde_json::Value,
        fixture: &CargoFixture,
    ) -> Result<Option<Self>, CompilerObservationError> {
        use super::super::source::{array, field, text};
        if Path::new(text(value, "manifest_path")?) != fixture.manifest {
            return Ok(None);
        }
        let target = field(value, "target")?;
        let kinds = array(target, "kind")?;
        let selected = match &fixture.target {
            CargoTarget::Binary(name) => {
                text(target, "name")? == name
                    && matches!(kinds, [serde_json::Value::String(kind)] if kind == "bin")
            }
            CargoTarget::Library => kinds.iter().any(|kind| {
                matches!(
                    kind.as_str(),
                    Some("lib" | "rlib" | "dylib" | "staticlib" | "cdylib" | "proc-macro")
                )
            }),
        };
        Ok(selected.then_some(Self { value }))
    }

    pub(in crate::native_compiler) const fn value(&self) -> &'message serde_json::Value {
        self.value
    }
}

fn selector(value: &str) -> Result<(), CompilerError> {
    if value.is_empty() || value.contains('\0') {
        Err(CompilerError::Configuration(
            "empty or NUL-containing selector".to_owned(),
        ))
    } else {
        Ok(())
    }
}

fn cargo_coverage_environment(
    process: &ProcessRequest,
    flags: &str,
) -> Result<Vec<(String, String)>, CompilerError> {
    let mut environment = process.environment().to_vec();
    if environment.iter().any(|(key, _)| {
        key.eq_ignore_ascii_case("RUSTFLAGS") || key.eq_ignore_ascii_case("CARGO_ENCODED_RUSTFLAGS")
    }) {
        return Err(CompilerError::Configuration(
            "coverage instrumentation owns the explicit Rust flag selection".to_owned(),
        ));
    }
    environment.push(("CARGO_ENCODED_RUSTFLAGS".to_owned(), flags.to_owned()));
    Ok(environment)
}

fn spelling(path: &Path) -> Result<&str, CompilerError> {
    path.to_str()
        .ok_or_else(|| CompilerError::Configuration("compiler paths must be Unicode".to_owned()))
}

fn absolute(path: &Path) -> Result<(), CompilerError> {
    if !path.is_absolute()
        || spelling(path)?.contains('\0')
        || spelling(path)?
            .split(std::path::is_separator)
            .any(|segment| matches!(segment, "." | ".."))
        || path.components().any(|part| {
            matches!(
                part,
                std::path::Component::CurDir | std::path::Component::ParentDir
            )
        })
    {
        Err(CompilerError::Configuration(
            "absolute normal compiler paths are required".to_owned(),
        ))
    } else {
        Ok(())
    }
}
