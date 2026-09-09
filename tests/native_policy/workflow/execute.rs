//! Direct owner calls and independent counters observe the root execution composition.

use super::fixture::{self, mapped};
use macroonz::harness::input::{InputBinding, InputRefusal, pack};
use macroonz::harness::report::{RunAttempt, SkipReason};
use macroonz::harness::runner::run_all;
use macroonz::workflow;

#[test]
fn root_execution_matches_explicit_owners_and_keeps_unselected_rows() -> Result<(), String> {
    let decoder = fixture::decoder()?;
    let table = fixture::table()?;
    let selection = fixture::selection()?;
    fixture::reset();
    let composed = fixture::run(&[7, 1, 9])?;
    assert_eq!(fixture::observations(), (1, 1, 0));
    assert_eq!(composed.report().denominator(), 2);
    assert!(
        composed
            .report()
            .census()
            .get(1)
            .ok_or("missing census row")?
            .disposition()
            .report()
            .is_none()
    );
    let envelope = mapped(pack(decoder.profile(), &[7, 1, 9], fixture::INPUT_LIMITS))?;
    let invocation = fixture::invocation(1).with_input(mapped(decoder.decode(envelope.clone()))?);
    assert_eq!(
        composed.report(),
        &run_all(&table.view(), &selection, &invocation)
    );
    assert_eq!(composed.input(), &envelope);
    let capsule = fixture::capsule(&composed)?;
    assert_eq!(capsule.input(), &[1]);
    assert_ne!(capsule.input(), composed.input().payload());
    Ok(())
}

#[test]
fn incomplete_decoder_and_execution_budget_refuse_without_false_green() -> Result<(), String> {
    let decoder = fixture::decoder()?;
    let incomplete = InputBinding::declared(decoder.profile(), fixture::revision(), |_source| {
        Ok(Vec::<u8>::new())
    });
    fixture::reset();
    assert!(matches!(
        workflow::run(
            &fixture::table()?.view(),
            &fixture::selection()?,
            &incomplete,
            &[1],
            fixture::invocation(1),
            fixture::INPUT_LIMITS
        ),
        Err(InputRefusal::TrailingInputBytes { count: 1 })
    ));
    assert_eq!(fixture::observations(), (0, 0, 0));
    let skipped = mapped(workflow::run(
        &fixture::table()?.view(),
        &fixture::selection()?,
        &decoder,
        &[1],
        fixture::invocation(0),
        fixture::INPUT_LIMITS,
    ))?;
    assert_eq!(fixture::observations(), (1, 0, 0));
    assert_eq!(
        fixture::selected(&skipped)?.attempt(),
        &RunAttempt::SkippedWithReason(SkipReason::BudgetExhausted)
    );
    Ok(())
}
