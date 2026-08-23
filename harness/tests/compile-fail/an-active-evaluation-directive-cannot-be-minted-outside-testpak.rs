//! An active evaluation directive is minted only after the harness resolves a surface-issued selection.
//!
//! Generated code may inspect the borrowed point and alternative it receives, but it cannot assemble those values into mutation authority of its own.

use threadpak_testpak::muterprater::{
    ActiveSelection, AdmittedAlternative, EvaluationDirective, MutationPoint, ResolvedMutation,
};

fn fabricate<'surface>(
    selection: ActiveSelection,
    point: &'surface MutationPoint,
    alternative: &'surface AdmittedAlternative,
) -> EvaluationDirective<'surface> {
    let resolved = ResolvedMutation {
        selection,
        point,
        alternative,
    };
    EvaluationDirective {
        resolved: Some(resolved),
    }
}

fn main() {}
