//! Historical surface material cannot mint executable surfaces, points or selections.

use macroonz_harness::muterprater::discovery_archive::{
    ArchivedAlternative, ArchivedEvaluationSurface, ArchivedMutationPoint,
};
use macroonz_harness::muterprater::{ActiveSelection, AdmittedAlternative, EvaluationSurface, MutationPoint};

fn surface(historical: ArchivedEvaluationSurface) -> EvaluationSurface {
    historical
}

fn point(historical: ArchivedMutationPoint) -> MutationPoint {
    historical
}

fn alternative(historical: ArchivedAlternative) -> AdmittedAlternative {
    historical
}

fn selection(historical: ArchivedEvaluationSurface) -> ActiveSelection {
    historical
}

fn main() {}
