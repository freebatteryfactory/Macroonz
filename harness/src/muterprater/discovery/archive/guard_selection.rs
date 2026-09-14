//! Historical selection admission without a current surface mint.

use super::super::{
    ArchivedAlternative, ArchivedEvaluationSurface, ArchivedMutationPoint, ArchivedSelection,
};
use crate::descriptor::archive::ArchivedName;
use crate::identity::BodyReader;
use crate::report::archive::{AddressClaim, ArchiveLimits, ArchiveRefusal, claim, name};

pub(crate) fn read_selection(
    reader: &mut BodyReader<'_, ArchiveRefusal>,
    limits: ArchiveLimits,
) -> Result<ArchivedSelection, ArchiveRefusal> {
    Ok(ArchivedSelection {
        surface: claim(reader, limits)?,
        point: name(reader, limits)?,
        alternative: claim(reader, limits)?,
    })
}

impl ArchivedSelection {
    /// The historical surface address claim.
    #[must_use]
    pub const fn surface(&self) -> AddressClaim {
        self.surface
    }

    /// The historical point name.
    #[must_use]
    pub const fn point(&self) -> &ArchivedName {
        &self.point
    }

    /// The historical alternative address claim.
    #[must_use]
    pub const fn alternative(&self) -> AddressClaim {
        self.alternative
    }
}

impl ArchivedEvaluationSurface {
    /// Resolve historical selection claims only against this complete historical roster.
    pub(crate) fn selected_alternative(
        &self,
        selection: &ArchivedSelection,
    ) -> Option<(&ArchivedMutationPoint, &ArchivedAlternative)> {
        if selection.surface().as_bytes() != self.identity().as_bytes() {
            return None;
        }
        let point = self
            .points()
            .iter()
            .find(|point| point.name() == selection.point())?;
        let alternative = point.alternatives().iter().find(|alternative| {
            alternative.identity().as_bytes() == selection.alternative().as_bytes()
        })?;
        Some((point, alternative))
    }
}
