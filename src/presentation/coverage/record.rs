//! Complete coverage records without new execution, interpretation or frontier admission.

use super::axes;
use crate::harness::fuzz::{
    CoverageAdmission, CoverageBudgets, CoverageCampaign, CoverageCorpus, CoveragePoint,
    CoverageSourceRoot, CoverageStanding, CoverageTool, ReadyPreflight, RustcProfileResult,
};
use crate::presentation::{
    Presentation, context,
    path::path,
    value::{array, hex, object, optional, tagged},
};
use serde_json::Value;

/// Project coverage readiness with original declarations and established host facts.
pub fn coverage_readiness(record: &ReadyPreflight) -> Presentation {
    Presentation::projected(
        "coverage-readiness",
        "macroonz-harness/fuzz",
        "recorded",
        readiness(record),
    )
}

pub(in crate::presentation) fn readiness(record: &ReadyPreflight) -> Value {
    let request = record.request();
    object([
        ("standing", standing(record.standing())),
        ("rustc", path(record.rustc())),
        ("release", record.release().into()),
        ("host", record.host().into()),
        ("sysroot", path(record.sysroot())),
        ("llvm_version", record.llvm_version().into()),
        ("tool_version", record.tool_version().into()),
        ("profdata", path(record.tool_path(CoverageTool::Profdata))),
        ("cov", path(record.tool_path(CoverageTool::Cov))),
        (
            "canonical_source_roots",
            array(record.source_roots().iter().map(source_root)),
        ),
        (
            "request",
            object([
                ("rustc", path(request.rustc())),
                (
                    "target",
                    object([
                        ("executable", path(request.target().executable())),
                        (
                            "arguments",
                            array(
                                request
                                    .target()
                                    .arguments()
                                    .iter()
                                    .map(|value| value.as_str().into()),
                            ),
                        ),
                        (
                            "triple",
                            optional(request.target().triple(), |value| value.spelling().into()),
                        ),
                    ]),
                ),
                (
                    "source_roots",
                    array(request.source_roots().iter().map(source_root)),
                ),
                ("scratch", path(request.scratch())),
                ("campaign", campaign(request.campaign())),
            ]),
        ),
    ])
}

fn source_root(value: &CoverageSourceRoot) -> Value {
    object([
        ("logical", context::declared_name(value.logical())),
        ("checkout", path(value.checkout())),
    ])
}

fn standing(value: &CoverageStanding) -> Value {
    object([
        ("campaign", campaign(value.campaign())),
        ("target", context::target(value.target())),
    ])
}

fn campaign(value: CoverageCampaign) -> Value {
    object([
        (
            "population",
            context::declared_name(value.population().name()),
        ),
        ("revision", context::revision(value.revision())),
        (
            "profile",
            object([
                ("name", context::declared_name(value.profile().name())),
                ("version", value.profile().version().into()),
            ]),
        ),
        ("budgets", budgets(value.budgets())),
    ])
}

fn budgets(value: CoverageBudgets) -> Value {
    object([
        ("executions", value.executions().cases().into()),
        ("input_bytes", value.input_bytes().bytes().into()),
        ("export_bytes", value.export_bytes().into()),
        ("points", value.points().into()),
        ("retained_cases", value.retained_cases().cases().into()),
        ("retained_bytes", value.retained_bytes().bytes().into()),
    ])
}

/// Project one joined candidate, target classification and complete coverage observation.
pub fn coverage_result(record: &RustcProfileResult) -> Presentation {
    Presentation::projected(
        "coverage-result",
        "macroonz-harness/fuzz",
        "recorded",
        object([
            ("case", record.case().into()),
            ("candidate", hex(record.candidate())),
            ("execution", axes::execution(record.execution())),
            (
                "points",
                array(record.observation().points().iter().map(point)),
            ),
            ("standing", standing(record.standing())),
        ]),
    )
}

fn point(value: &CoveragePoint) -> Value {
    let (kind, source, line, branch) = match value {
        CoveragePoint::Line { source, line } => ("line", source, line, Value::Null),
        CoveragePoint::Branch {
            source,
            line,
            block,
            branch,
        } => (
            "branch",
            source,
            line,
            object([("block", (*block).into()), ("branch", (*branch).into())]),
        ),
    };
    tagged(
        kind,
        object([
            (
                "source",
                object([
                    ("root", context::declared_name(source.root())),
                    ("relative", source.relative().into()),
                ]),
            ),
            ("line", (*line).into()),
            ("branch", branch),
        ]),
    )
}

/// Project the accumulated frontier and retained candidates without inventing attempt history.
pub fn coverage_corpus(record: &CoverageCorpus) -> Presentation {
    Presentation::projected(
        "coverage-corpus",
        "macroonz-harness/fuzz",
        "recorded",
        object([
            ("standing", standing(record.standing())),
            ("attempted_cases", record.attempted_cases().into()),
            (
                "attempted_input_bytes",
                record.attempted_input_bytes().into(),
            ),
            ("observed", array(record.observed().iter().map(point))),
            (
                "interesting",
                array(
                    record
                        .interesting()
                        .iter()
                        .map(|value| hex(value.as_bytes())),
                ),
            ),
            ("retained_bytes", record.retained_bytes().into()),
        ]),
    )
}

/// Project the novelty decision without turning it into a semantic verdict.
pub fn coverage_admission(record: &CoverageAdmission) -> Presentation {
    let decision = match record {
        CoverageAdmission::Known => tagged("known", Value::Null),
        CoverageAdmission::Interesting(value) => tagged("interesting", hex(value.as_bytes())),
    };
    Presentation::projected(
        "coverage-admission",
        "macroonz-harness/fuzz",
        "recorded",
        decision,
    )
}
