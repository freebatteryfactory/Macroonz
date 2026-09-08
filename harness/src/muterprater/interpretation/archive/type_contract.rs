//! Error conversion preserves the semantic owner of each refusal.

use super::InterpretedArchiveRefusal;
use super::ParityArchiveRefusal;
use crate::descriptor::archive::BindingArchiveRefusal;
use crate::muterprater::backend_archive::SuitePressureArchiveRefusal;
use crate::muterprater::discovery_archive::SurfaceArchiveRefusal;
use crate::muterprater::specimen_archive::ProjectionArchiveRefusal;
use crate::muterprater::verdict_archive::MutationArchiveRefusal;
use crate::report::archive::ArchiveRefusal;

impl From<ArchiveRefusal> for ParityArchiveRefusal {
    fn from(cause: ArchiveRefusal) -> Self {
        Self::Record(cause)
    }
}

impl From<BindingArchiveRefusal> for ParityArchiveRefusal {
    fn from(cause: BindingArchiveRefusal) -> Self {
        Self::Binding(cause)
    }
}

impl From<ArchiveRefusal> for InterpretedArchiveRefusal {
    fn from(cause: ArchiveRefusal) -> Self {
        Self::Record(cause)
    }
}

impl From<SurfaceArchiveRefusal> for InterpretedArchiveRefusal {
    fn from(cause: SurfaceArchiveRefusal) -> Self {
        Self::Surface(cause)
    }
}

impl From<SuitePressureArchiveRefusal> for InterpretedArchiveRefusal {
    fn from(cause: SuitePressureArchiveRefusal) -> Self {
        Self::Suite(cause)
    }
}

impl From<ProjectionArchiveRefusal> for InterpretedArchiveRefusal {
    fn from(cause: ProjectionArchiveRefusal) -> Self {
        Self::Projection(cause)
    }
}

impl From<MutationArchiveRefusal> for InterpretedArchiveRefusal {
    fn from(cause: MutationArchiveRefusal) -> Self {
        Self::Mutation(cause)
    }
}
