//! The versioned facade requires both its feature and independently supplied semantic facts.

use super::{StorageDependencies, surface};
use std::path::Path;

pub(super) fn observe(
    subject: &Path,
    scratch: &Path,
    profile: &Path,
    posture: &str,
    storage: StorageDependencies,
) -> Result<(), String> {
    let missing_module = (posture == "diet").then_some("E0433");
    let native = if storage == StorageDependencies::Native {
        None
    } else {
        Some(if posture == "diet" { "E0433" } else { "E0425" })
    };
    let required = Some(if posture == "diet" { "E0433" } else { "E0061" });
    for (name, source, expected) in [
        (
            "input",
            "fn main() { let _run = macroonz::configuration::v1::run::<Vec<u8>>; let _limits = macroonz::configuration::v1::input_limits(); }",
            missing_module,
        ),
        (
            "native",
            "fn main() { let _tool = macroonz::configuration::v1::process_tool; let _limits = macroonz::configuration::v1::retention_limits(); }",
            native,
        ),
        (
            "missing-invocation",
            "use macroonz::{configuration::v1, harness::{input::{BoundInput, InputBinding}, runner::{TrialTableView, SelectionPlan}}}; fn _call(view: &TrialTableView<BoundInput<u8>>, selection: &SelectionPlan, decoder: &InputBinding<u8>) { let _result = v1::run(view, selection, decoder, &[1]); } fn main() {}",
            required,
        ),
        (
            "missing-decoder",
            "use macroonz::{configuration::v1, harness::{input::BoundInput, runner::{TrialTableView, SelectionPlan, Invocation}}}; fn _call(view: &TrialTableView<BoundInput<u8>>, selection: &SelectionPlan, invocation: Invocation) { let _result = v1::run(view, selection, &[1], invocation); } fn main() {}",
            required,
        ),
        (
            "missing-target",
            "use macroonz::harness::{runner::Invocation, report::{InvocationProfile, TrialSite}, clock::HarnessClock}; fn _call(profile: InvocationProfile, site: TrialSite, clock: HarnessClock) { let _invocation = Invocation::declared(profile, site, clock); } fn main() {}",
            required,
        ),
        (
            "missing-revision",
            "use macroonz::harness::input::{InputBinding, InputProfile}; fn _call(profile: InputProfile) { let _decoder = InputBinding::<u8>::declared(profile); } fn main() {}",
            required,
        ),
        (
            "missing-check-callable",
            "use macroonz::harness::descriptor::{ExecutableAttachment, SubjectRoute, CheckRef, RevisionBinding}; fn _call(subject: SubjectRoute, check: CheckRef, revision: RevisionBinding) { let _attachment = ExecutableAttachment::<(), ()>::attached(subject, check, revision, revision); } fn main() {}",
            required,
        ),
    ] {
        surface(
            subject,
            scratch,
            profile,
            &format!("{posture}-defaults-{name}"),
            source,
            expected,
        )?;
    }
    Ok(())
}
