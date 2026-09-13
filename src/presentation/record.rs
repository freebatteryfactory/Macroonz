//! Current typed report readings retain the complete table and attempt axes.

use super::value::{array, hex, object, optional, tagged};
use super::{Presentation, clock, context, outcome};
use crate::harness::report::{
    ReplayCapsule, RunReport, SelectionDisposition, TrialAccounting, TrialReport,
};
use serde_json::Value;

/// Project every census row of the supplied run.
pub fn run(value: &RunReport) -> Presentation {
    Presentation::projected(
        "run",
        "macroonz-harness/report",
        "recorded",
        run_value(value),
    )
}

pub(super) fn run_value(value: &RunReport) -> Value {
    object([
        ("denominator", value.denominator().into()),
        ("census", array(value.census().iter().map(accounting))),
        ("posture", context::table(value.posture())),
        ("selection", outcome::selection(value.selection())),
        ("invocation", context::invocation(value.invocation())),
        ("target", context::target(value.target())),
        ("input", optional(value.input(), context::input)),
    ])
}

fn accounting(value: &TrialAccounting) -> Value {
    let disposition = match value.disposition() {
        SelectionDisposition::Selected(report) => tagged("selected", trial_value(report)),
        SelectionDisposition::NotSelected { trial: _, reason } => {
            tagged("not-selected", outcome::not_selected(*reason))
        }
    };
    object([
        ("trial", hex(value.trial().address().as_bytes())),
        ("row", hex(value.row().address().as_bytes())),
        (
            "subject",
            hex(value.revisions().subject().address().as_bytes()),
        ),
        ("check", hex(value.revisions().check().address().as_bytes())),
        ("claim", context::declared_name(value.claim().name())),
        ("disposition", disposition),
    ])
}

/// Project one trial's complete execution standing and independent measurement.
pub fn trial(value: &TrialReport) -> Presentation {
    Presentation::projected(
        "trial",
        "macroonz-harness/report",
        "recorded",
        trial_value(value),
    )
}

pub(super) fn trial_value(value: &TrialReport) -> Value {
    let site = value.site();
    object([
        ("key", context::execution(value.standing().key())),
        ("posture", context::posture(value.standing().replay())),
        (
            "site",
            object([
                ("module_path", site.module_path().into()),
                ("file", site.file().into()),
                ("line", site.line().into()),
                ("name", site.name().into()),
            ]),
        ),
        ("attempt", outcome::attempt(value.trial(), value.attempt())),
        ("measurement", clock::measurement(value.measurement())),
        (
            "clock_attribution",
            clock::attribution(value.clock_attribution()),
        ),
    ])
}

/// Project a reached replay witness without executing it or claiming reproduction.
pub fn capsule(value: &ReplayCapsule) -> Presentation {
    Presentation::projected(
        "capsule",
        "macroonz-harness/report",
        "recorded",
        object([
            ("identity", hex(value.identity().as_bytes())),
            ("key", context::execution(value.key())),
            ("fingerprint", outcome::fingerprint(value.fingerprint())),
            ("input", hex(value.input())),
            (
                "generation",
                profile(value.generation().name(), value.generation().version()),
            ),
            (
                "minimization",
                profile(value.minimization().name(), value.minimization().version()),
            ),
            ("schema", hex(value.schema().address().as_bytes())),
            ("posture", context::posture(value.posture())),
        ]),
    )
}

pub(super) fn profile(name: &str, version: u32) -> Value {
    object([("name", name.into()), ("version", version.into())])
}
