//! Actual authored and staged executions reuse the archive's declared input and invocation.

use macroonz_harness::descriptor::{
    AuthoredTableName, Binding, DerivedRevision, ExecutableAttachment, Origin, PopulationRef,
    Provenance, RevisionBinding, Row,
};
use macroonz_harness::report::{RunReport, TrialConclusion};
use macroonz_harness::runner::{
    Invocation, Selection, SelectionPlan, TrialBinding, TrialTable, run_all,
};

pub(super) fn binding<Input>(
    population: &'static str,
    origin: Origin,
    call: fn(&Invocation<Input>) -> TrialConclusion,
) -> Result<TrialBinding<Input>, ()> {
    let template = super::super::row()?;
    let row = Row::declared(
        template.claim(),
        template.execution_suite(),
        template.classification().clone(),
        template.subject(),
        template.check(),
        PopulationRef::named("archive-run", population).map_err(|_| ())?,
        origin,
    )
    .map_err(|_| ())?;
    let revision = RevisionBinding::derived(DerivedRevision::from_material(include_bytes!(
        "run_fixture.rs"
    )));
    let attachment =
        ExecutableAttachment::attached(row.subject(), row.check(), revision, revision, call);
    Binding::bound(row, attachment, Provenance::Unproduced).map_err(|_| ())
}

pub(super) fn table<Input>(bindings: Vec<TrialBinding<Input>>) -> Result<TrialTable<Input>, ()> {
    TrialTable::authored(
        AuthoredTableName::named("archive-run", "world").map_err(|_| ())?,
        Provenance::Unproduced,
        bindings,
    )
    .map_err(|_| ())
}

pub(super) fn typed() -> Result<RunReport, ()> {
    let table = table(vec![
        binding("first", Origin::HandWritten, |_| TrialConclusion::Passed)?,
        binding("second", Origin::HandWritten, |_| TrialConclusion::Passed)?,
    ])?;
    Ok(run_all(
        &table.view(),
        &SelectionPlan::of(Selection::All),
        &super::fixture::typed_invocation(&[2, 3])?,
    ))
}
