//! The verification pass over one carrier's axes, and nothing else.
//!
//! The carried set is the quantifier: every carried axis is examined against the assembly's declaration and against the axes before it, so "every carried axis was examined" is a fact about the loops rather than a claim about them.
//!
//! Nothing here reaches a private field.
//! The pass reads each axis through the same answers any caller gets, before any assembly exists, and the roads that CONSUME it live in `type_guard.rs` — because building an assembly and building a refusal body are what must stay unreachable.

use super::{AssemblyIssue, AxisCargo, CargoAxis, DeliveryForm, ProvedCargo, SupportAxes};
use crate::identity::{self, Identity};

/// Every axis carrying one terminal's proved cargo, in roster order.
///
/// The declared axis is not among them and cannot be: its material is a body somebody wrote rather than a delivery somebody proved, so there is no terminal for it to be compared against.
pub(super) fn carried_axes(axes: &SupportAxes) -> Vec<(CargoAxis, &ProvedCargo)> {
    [
        (CargoAxis::Deferred, held(&axes.deferred)),
        (CargoAxis::Bench, held(&axes.bench)),
    ]
    .into_iter()
    .filter_map(|(axis, cargo)| cargo.map(|proved| (axis, proved)))
    .collect()
}

/// Every carried axis whose terminal stands over another declaration, reported once per axis.
///
/// Both declarations travel on the issue and neither is elected: which one the caller meant is the caller's own fact, and a carrier composing two declarations' cargo is one exported name delivering material from two places whichever one was intended.
pub(super) fn root_issues(
    root: Identity<identity::CapturedDeclaration>,
    carried: &[(CargoAxis, &ProvedCargo)],
) -> Vec<AssemblyIssue> {
    carried
        .iter()
        .filter(|(_, proved)| proved.root() != root)
        .map(|(axis, proved)| AssemblyIssue::RootsDisagree {
            axis: *axis,
            stated: root,
            carried: proved.root(),
        })
        .collect()
}

/// Every carried value seated on an axis other than the one its proved destination belongs to.
///
/// Promotion proves that the value came from the destination named at that call, while the public axes decide where the value is later seated.
/// Rechecking the relationship here prevents a proved test delivery from being moved into the benchmark field, or the reverse, after promotion erased the call's axis argument.
pub(super) fn destination_issues(carried: &[(CargoAxis, &ProvedCargo)]) -> Vec<AssemblyIssue> {
    carried
        .iter()
        .filter(|(axis, proved)| axis.reads_from() != Some(proved.destination()))
        .map(
            |(axis, proved)| AssemblyIssue::CargoReachesASecondDestination {
                axis: *axis,
                destination: proved.destination(),
            },
        )
        .collect()
}

/// Every terminal delivery two axes read, reported at its SECOND occurrence.
///
/// The second rather than the first, because the first reading is the lawful one and what is being established is that something read it again: a caller repairing a doubled consumption drops the later axis, and pointing at the earlier one would send it to the reading it means to keep.
///
/// Counted by walking each axis against the axes before it, so the pass runs the carried set to the end and one pair raises one issue rather than two.
pub(super) fn consumption_issues(carried: &[(CargoAxis, &ProvedCargo)]) -> Vec<AssemblyIssue> {
    let mut issues: Vec<AssemblyIssue> = Vec::new();
    for (position, (_, proved)) in carried.iter().enumerate() {
        let earlier = carried.iter().take(position).any(|(_, other)| {
            other.source() == proved.source() && other.destination() == proved.destination()
        });
        if earlier {
            issues.push(AssemblyIssue::CargoConsumedTwice {
                source: proved.source(),
                destination: proved.destination(),
            });
        }
    }
    issues
}

/// Whether the axes name one delivery form, and whether that form's seats are filled.
///
/// One carrier is one gate invocation and one gate invocation is one coupled pair, so two proved axes are two carriers and the repair is to compose them as two.
/// The bench form's stamped seat is required because the gate's own transcription of that seat has no empty row; the trial form's is not, and a delivery whose whole cargo is deferred writes it empty.
pub(super) fn form_issues(axes: &SupportAxes) -> Vec<AssemblyIssue> {
    let mut issues: Vec<AssemblyIssue> = Vec::new();
    let deferred = matches!(&axes.deferred, AxisCargo::Carried(_));
    let bench = matches!(&axes.bench, AxisCargo::Carried(_));
    if deferred && bench {
        issues.push(AssemblyIssue::TwoFormsCarried);
    }
    if bench && matches!(&axes.declared, AxisCargo::Absent { .. }) {
        issues.push(AssemblyIssue::StampedCargoAbsent {
            form: DeliveryForm::Benches,
        });
    }
    issues
}

/// The proved cargo one axis carries, where it carries any.
fn held(axis: &AxisCargo<ProvedCargo>) -> Option<&ProvedCargo> {
    match axis {
        AxisCargo::Absent { .. } => None,
        AxisCargo::Carried(proved) => Some(proved),
    }
}
