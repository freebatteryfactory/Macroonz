use super::{
    CanonicalPublicationBytes, Destination, InventoryError, LandingBinding, Publication,
    PublicationBinding, PublicationFile, PublicationLimits, PublicationPath, StampFiles,
};
use crate::compiler::identity::Identity;
use crate::compiler::stamp::PublishedStamp;
use crate::compiler::{Expansion, GeneratedTree, Kind, RenderedUnit, Role};
use crate::harness::oracle::RelativeSourcePath;

impl PublicationPath {
    /// Admits one destination under the inventory's portable path contract.
    ///
    /// # Errors
    /// Refuses non-normal, reserved or unrepresentable publication paths.
    pub fn informed(spelling: &str) -> Result<Self, InventoryError> {
        let relative =
            RelativeSourcePath::informed(spelling).map_err(|_error| InventoryError::Path)?;
        if spelling.len() > 4096usize {
            return Err(InventoryError::Path);
        }
        for segment in spelling.split('/') {
            let folded = segment.to_ascii_lowercase();
            let stem = folded.split('.').next().ok_or(InventoryError::Path)?;
            if segment.len() > 240usize
                || segment.ends_with('.')
                || folded.starts_with(".macroonz-")
                || !segment
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.'))
                || matches!(stem, "con" | "prn" | "aux" | "nul")
                || matches!(
                    stem.as_bytes(),
                    [b'c', b'o', b'm', b'1'..=b'9'] | [b'l', b'p', b't', b'1'..=b'9']
                )
            {
                return Err(InventoryError::Path);
            }
        }
        Ok(Self(relative))
    }

    /// The admitted spelling, without normalization.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.0.spelling()
    }
}

impl<K: Kind> Publication<K> {
    /// Binds every proved publication unit to exactly one declared destination.
    ///
    /// # Errors
    /// Refuses incomplete or repeated addresses, path collisions and aggregate bounds.
    pub fn declared(
        expansion: Expansion<K>,
        bindings: &[PublicationBinding],
        limits: PublicationLimits,
    ) -> Result<Self, InventoryError> {
        if bindings.len() != expansion.published().count() {
            return Err(InventoryError::Binding);
        }
        for (at, binding) in bindings.iter().enumerate() {
            if bindings
                .iter()
                .take(at)
                .any(|earlier| earlier.address == binding.address)
            {
                return Err(InventoryError::Binding);
            }
        }
        let destinations = expansion
            .published()
            .map(|unit| {
                let binding = bindings
                    .iter()
                    .find(|binding| Some(binding.address) == unit.address())
                    .ok_or(InventoryError::Binding)?;
                Ok(Destination {
                    path: binding.path.clone(),
                    stamp: None,
                })
            })
            .collect::<Result<Vec<_>, InventoryError>>()?;
        let publication = Self {
            expansion,
            destinations,
            limits,
        };
        publication.admitted()?;
        Ok(publication)
    }

    /// Adds the complete landing set of a stamp that agrees with its sealed definition.
    ///
    /// # Errors
    /// Refuses another unit, changed definition, repeated stamp, unmatched landings or total-set bounds.
    pub fn with_stamp(
        mut self,
        stamp: PublishedStamp,
        bindings: &[LandingBinding],
    ) -> Result<Self, InventoryError> {
        let record = stamp.record();
        let (at, unit) = self
            .expansion
            .published()
            .enumerate()
            .find(|(_, unit)| unit.semantic_key() == record.unit())
            .ok_or(InventoryError::Stamp)?;
        if unit.bytes() != stamp.definition().canonical_bytes()
            || unit.digest() != unit.digest_under(record.staged())
        {
            return Err(InventoryError::Stamp);
        }
        let destination = self
            .destinations
            .get_mut(at)
            .ok_or(InventoryError::Binding)?;
        if destination.stamp.is_some() {
            return Err(InventoryError::Stamp);
        }
        let paths = landing_paths(&stamp, bindings)?;
        destination.stamp = Some(StampFiles { stamp, paths });
        self.admitted()?;
        Ok(self)
    }

    /// The original sealed compiler account.
    pub const fn expansion(&self) -> &Expansion<K> {
        &self.expansion
    }

    /// The shared file and byte allowances.
    #[must_use]
    pub const fn limits(&self) -> PublicationLimits {
        self.limits
    }

    /// Borrows definitions in proved unit order, each followed by its declared stamp landings.
    pub fn files(&self) -> impl Iterator<Item = PublicationFile<'_, K::Role>> {
        self.expansion
            .published()
            .zip(&self.destinations)
            .flat_map(|(unit, destination)| {
                let definition = PublicationFile {
                    path: &destination.path,
                    tokens: unit.tree(),
                    unit,
                    site: None,
                };
                let landings =
                    destination.stamp.iter().flat_map(move |bound| {
                        bound.stamp.landings().iter().zip(&bound.paths).map(
                            move |(landing, path)| PublicationFile {
                                path,
                                tokens: landing.invocation(),
                                unit,
                                site: Some(landing.site()),
                            },
                        )
                    });
                std::iter::once(definition).chain(landings)
            })
    }

    fn admitted(&self) -> Result<(), InventoryError> {
        super::super::paths::bounded_paths(
            self.files()
                .map(|file| (file.path().clone(), file.source().len())),
            self.limits,
        )
        .map(|_names| ())
    }
}

impl<R: Role> PublicationFile<'_, R> {
    /// The declared relative destination.
    #[must_use]
    pub const fn path(&self) -> &PublicationPath {
        self.path
    }

    /// The original borrowed token material.
    #[must_use]
    pub const fn tokens(&self) -> &GeneratedTree {
        self.tokens
    }

    /// The actual sealed unit, also retained for a stamp's landing files.
    #[must_use]
    pub const fn unit(&self) -> &RenderedUnit<R> {
        self.unit
    }

    /// The stamp site name, absent for the unit's definition.
    #[must_use]
    pub const fn site(&self) -> Option<&str> {
        self.site
    }

    /// Projects the tokens as unformatted source and appends LF.
    #[must_use]
    pub fn source(&self) -> String {
        let mut source = self.tokens.inspected();
        source.push('\n');
        source
    }

    /// Commits to the original canonical token bytes independently of physical formatting.
    #[must_use]
    pub fn canonical_digest(&self) -> Identity<CanonicalPublicationBytes> {
        super::super::digest::canonical(&self.tokens.canonical_bytes())
    }
}

fn landing_paths(
    stamp: &PublishedStamp,
    bindings: &[LandingBinding],
) -> Result<Vec<PublicationPath>, InventoryError> {
    if bindings.len() != stamp.count() {
        return Err(InventoryError::Landing);
    }
    for (at, binding) in bindings.iter().enumerate() {
        if bindings
            .iter()
            .take(at)
            .any(|earlier| earlier.site == binding.site)
        {
            return Err(InventoryError::Landing);
        }
    }
    stamp
        .landings()
        .iter()
        .map(|landing| {
            bindings
                .iter()
                .find(|binding| binding.site == landing.site())
                .map(|binding| binding.path.clone())
                .ok_or(InventoryError::Landing)
        })
        .collect()
}
