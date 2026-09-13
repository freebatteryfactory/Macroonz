use super::{
    AuthoredFile, CacheUse, CompiledPublication, PendingStaging, RefusedStaging, StagedPublication,
    StagingError, StagingObservationError, StagingPlan, StagingRun,
};
use crate::compiler::Kind;
use crate::native_compiler::{CompilerOutput, CompilerRequest, PendingCompilation};
use crate::native_publication::{PreparedPublication, PublicationLimits, PublicationPath};
use crate::native_storage::StorageName;
#[cfg(any(unix, windows))]
use crate::native_storage::StorageRoot;
use std::path::Path;
use std::time::Duration;

impl<K: Kind> StagingPlan<K> {
    /// Creates or exactly compares a disposable tree named by its complete declared source bytes.
    ///
    /// # Errors
    /// Refuses changed cached material, links, invalid parents and native filesystem failures.
    pub fn cached(self, parent: &Path) -> Result<StagedPublication<K>, StagingError> {
        let name = super::super::cache::name(&self)?;
        self.materialize(parent, &name, CacheUse::Compare)
    }

    /// Admits all authored and prepared files before any filesystem operation.
    ///
    /// # Errors
    /// Refuses aliases, parent collisions and complete-set file or byte bounds.
    pub fn declared(
        prepared: PreparedPublication<K>,
        authored: Vec<AuthoredFile>,
        limits: PublicationLimits,
    ) -> Result<Self, StagingError> {
        let plan = Self { prepared, authored };
        drop(
            crate::native_publication::inventory::bounded_paths(
                plan.files().map(|(path, bytes)| (path, bytes.len())),
                limits,
            )
            .map_err(StagingError::Inventory)?,
        );
        Ok(plan)
    }

    /// Exclusively creates and populates one explicitly named private compilation tree.
    ///
    /// # Errors
    /// Refuses invalid parents, existing stage names and filesystem failures without replacing files.
    pub fn stage(
        self,
        parent: &Path,
        name: &StorageName,
    ) -> Result<StagedPublication<K>, StagingError> {
        self.materialize(parent, name, CacheUse::Refuse)
    }

    fn materialize(
        self,
        parent: &Path,
        name: &StorageName,
        reuse: CacheUse,
    ) -> Result<StagedPublication<K>, StagingError> {
        if !parent.is_absolute()
            || parent.to_str().is_none_or(|path| {
                path.split(std::path::is_separator)
                    .any(|part| matches!(part, "." | ".."))
            })
        {
            return Err(StagingError::Configuration(
                "an absolute normal staging parent is required".to_owned(),
            ));
        }
        #[cfg(any(unix, windows))]
        {
            let root = StorageRoot::open(parent).map_err(StagingError::Storage)?;
            let name = format!(".macroonz-stage-{}", name.spelling());
            let created = match root.directory().create_dir(&name) {
                Ok(()) => CacheUse::Refuse,
                Err(error)
                    if error.kind() == std::io::ErrorKind::AlreadyExists
                        && reuse == CacheUse::Compare =>
                {
                    CacheUse::Compare
                }
                Err(error) => return Err(StagingError::Filesystem(error)),
            };
            if !root
                .directory()
                .symlink_metadata(&name)
                .map_err(StagingError::Filesystem)?
                .is_dir()
            {
                return Err(StagingError::Source(
                    "staging cache is not a directory".to_owned(),
                ));
            }
            let directory = root
                .directory()
                .open_dir(&name)
                .map_err(StagingError::Filesystem)?;
            match created {
                CacheUse::Refuse => super::super::files::populate(&directory, &self)?,
                CacheUse::Compare => super::super::files::compare(&directory, &self)?,
            }
            Ok(StagedPublication {
                plan: self,
                path: parent.join(name),
                directory,
            })
        }
        #[cfg(not(any(unix, windows)))]
        {
            let _inputs = (self, parent, name, reuse);
            Err(StagingError::Unavailable)
        }
    }

    pub(in crate::native_publication::staging) fn files(
        &self,
    ) -> impl Iterator<Item = (PublicationPath, &[u8])> {
        self.prepared
            .files()
            .map(|file| (file.path().clone(), file.bytes()))
            .chain(
                self.authored
                    .iter()
                    .map(|file| (file.path.clone(), file.bytes.as_slice())),
            )
    }
}

impl<K: Kind> StagedPublication<K> {
    /// The explicitly named private source directory.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The original prepared publication whose files must occur in the compiled dependency roster.
    #[must_use]
    pub const fn prepared(&self) -> &PreparedPublication<K> {
        &self.plan.prepared
    }

    /// Compiles this private tree with the existing native compiler owner.
    ///
    /// # Errors
    /// Refuses a different source root, unstaged inputs, output collisions, changed source or startup failure.
    pub fn compile(self, request: &CompilerRequest) -> Result<StagingRun<K>, StagingError> {
        super::super::compile::run(self, request)
    }

    pub(in crate::native_publication::staging) fn compared(&self) -> Result<(), StagingError> {
        #[cfg(any(unix, windows))]
        {
            super::super::files::compare(&self.directory, &self.plan)?;
            if !std::fs::symlink_metadata(&self.path)
                .map_err(StagingError::Filesystem)?
                .is_dir()
            {
                return Err(StagingError::Source(
                    "staging root is not a directory".to_owned(),
                ));
            }
            let visible = StorageRoot::open(&self.path).map_err(StagingError::Storage)?;
            super::super::files::compare(visible.directory(), &self.plan)
        }
        #[cfg(not(any(unix, windows)))]
        {
            Err(StagingError::Unavailable)
        }
    }
}

impl<K: Kind> CompiledPublication<K> {
    /// The original prepared physical bytes qualified by this compilation.
    #[must_use]
    pub const fn prepared(&self) -> &PreparedPublication<K> {
        &self.staged.plan.prepared
    }

    /// The actual compiler observation, including source dependencies and artifact freshness.
    #[must_use]
    pub const fn compiler(&self) -> &CompilerOutput {
        &self.compiler
    }
}

impl<K: Kind> RefusedStaging<K> {
    /// The refusal that prevented staged publication qualification.
    #[must_use]
    pub const fn reason(&self) -> &StagingObservationError {
        &self.reason
    }

    /// The actual completed compiler observation.
    #[must_use]
    pub const fn compiler(&self) -> &CompilerOutput {
        &self.compiler
    }

    /// The private source tree retained for inspection.
    #[must_use]
    pub const fn staged(&self) -> &StagedPublication<K> {
        &self.staged
    }
}

impl<K: Kind> PendingStaging<K> {
    /// The still-owned compiler cleanup state.
    pub fn compiler(&self) -> &PendingCompilation {
        &self.compiler
    }

    /// Retries cleanup before comparing the source tree and compiler dependency roster.
    pub fn finish(self, budget: Duration) -> StagingRun<K> {
        super::super::compile::finish(self.staged, self.compiler.finish(budget))
    }
}
