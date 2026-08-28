//! Complete axis-set verification.
use super::super::cargo::{AxisCargo, CargoAxis, ProvedCargo, SupportAxes};
use super::AssemblyIssue;
use crate::identity::{self, Identity};
pub(super) fn carried_axes(axes: &SupportAxes) -> Vec<(CargoAxis, &ProvedCargo)> {
    [
        (CargoAxis::Deferred, held(&axes.deferred)),
        (CargoAxis::Bench, held(&axes.bench)),
    ]
    .into_iter()
    .filter_map(|(axis, cargo)| cargo.map(|proved| (axis, proved)))
    .collect()
}
pub(super) fn root_issues(
    root: Identity<identity::CapturedDeclaration>,
    axes: &SupportAxes,
    carried: &[(CargoAxis, &ProvedCargo)],
) -> Vec<AssemblyIssue> {
    let mut issues = Vec::new();
    if let AxisCargo::Carried(declared) = &axes.declared
        && declared.root() != root
    {
        issues.push(AssemblyIssue::RootsDisagree {
            axis: CargoAxis::Declared,
            stated: root,
            carried: declared.root(),
        });
    }
    issues.extend(
        carried
            .iter()
            .filter(|(_, proved)| proved.root() != root)
            .map(|(axis, proved)| AssemblyIssue::RootsDisagree {
                axis: *axis,
                stated: root,
                carried: proved.root(),
            }),
    );
    issues
}
pub(super) fn destination_issues(carried: &[(CargoAxis, &ProvedCargo)]) -> Vec<AssemblyIssue> {
    carried
        .iter()
        .filter(|(axis, proved)| axis.reads_from() != proved.destination())
        .map(
            |(axis, proved)| AssemblyIssue::CargoReachesASecondDestination {
                axis: *axis,
                destination: proved.destination(),
            },
        )
        .collect()
}
pub(super) fn consumption_issues(carried: &[(CargoAxis, &ProvedCargo)]) -> Vec<AssemblyIssue> {
    let mut issues = Vec::new();
    for (position, (_, proved)) in carried.iter().enumerate() {
        if carried.iter().take(position).any(|(_, other)| {
            other.source() == proved.source() && other.destination() == proved.destination()
        }) {
            issues.push(AssemblyIssue::CargoConsumedTwice {
                source: proved.source(),
                destination: proved.destination(),
            });
        }
    }
    issues
}
pub(super) fn form_issues(axes: &SupportAxes) -> Vec<AssemblyIssue> {
    let mut issues = Vec::new();
    if matches!(&axes.deferred, AxisCargo::Carried(_))
        && matches!(&axes.bench, AxisCargo::Carried(_))
    {
        issues.push(AssemblyIssue::TwoFormsCarried);
    }
    if matches!(&axes.bench, AxisCargo::Carried(_))
        && matches!(&axes.declared, AxisCargo::Absent { .. })
    {
        issues.push(AssemblyIssue::StampedCargoAbsent {
            form: super::super::types::DeliveryForm::Benches,
        });
    }
    issues
}
fn held(axis: &AxisCargo<ProvedCargo>) -> Option<&ProvedCargo> {
    match axis {
        AxisCargo::Absent { .. } => None,
        AxisCargo::Carried(proved) => Some(proved),
    }
}
