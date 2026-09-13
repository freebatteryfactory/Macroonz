//! Independent resource ownership and failure recovery through the public recipe entrance.

use macroonz as bakery;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Refusal {
    Owner,
    Revision,
    Phase,
    Effect,
    Payload,
}

#[derive(Debug)]
pub(crate) struct Resource {
    owner: u32,
    revision: u32,
    label: String,
}

pub(crate) struct Runtime {
    owner: u32,
    revision: u32,
    phase: press::Phase,
    checks: usize,
    effects: usize,
    behavior: Behavior,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Behavior {
    Apply,
    Refuse,
    Partial,
    FalseSuccess,
}

pub(crate) fn validate(
    runtime: &mut Runtime,
    resource: &Resource,
    phase: press::Phase,
) -> Result<(), Refusal> {
    runtime.checks = runtime.checks.saturating_add(1);
    if runtime.owner != resource.owner {
        return Err(Refusal::Owner);
    }
    if runtime.revision != resource.revision {
        return Err(Refusal::Revision);
    }
    if runtime.phase != phase {
        return Err(Refusal::Phase);
    }
    Ok(())
}

pub(crate) fn advance(
    runtime: &mut Runtime,
    resource: &mut Resource,
    target: press::Phase,
) -> Result<press::Phase, Refusal> {
    runtime.effects = runtime.effects.saturating_add(1);
    if runtime.behavior == Behavior::Refuse {
        return Err(Refusal::Effect);
    }
    if runtime.behavior == Behavior::FalseSuccess {
        return Ok(target);
    }
    runtime.phase = target;
    runtime.revision = runtime.revision.saturating_add(1);
    resource.revision = runtime.revision;
    resource.label.push('!');
    if runtime.behavior == Behavior::Partial {
        Err(Refusal::Effect)
    } else {
        Ok(target)
    }
}

bakery::recipe! {
    pub(crate) mod press {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub(crate) enum Phase { Draft, Checked, Published }
        pub(crate) enum Event { Check, Publish }
        bake! {
            vocabularies { Phase; Event; };
            transitions(Phase, Event) {
                (Draft, Check) => Checked with(target) {
                    crate::consuming::advance(engine, record, target)
                };
                (Checked, Publish) => Published with(target) {
                    if !approval { return Err(crate::consuming::Refusal::Payload); }
                    crate::consuming::advance(engine, record, target)
                };
            };
            absence(refused);
            projections {
                typestate(Phase) {
                    wrapper(Ticket);
                    resource(record: crate::consuming::Resource);
                    runtime(engine: &mut crate::consuming::Runtime);
                    refusal(crate::consuming::Refusal);
                    validate(crate::consuming::validate);
                    methods { Check => check(); Publish => publish(approval: bool); };
                };
            };
        }
    }
}

fn initial() -> (Runtime, Resource) {
    (
        Runtime {
            owner: 7,
            revision: 10,
            phase: press::Phase::Draft,
            checks: 0,
            effects: 0,
            behavior: Behavior::Apply,
        },
        Resource {
            owner: 7,
            revision: 10,
            label: "draft".to_owned(),
        },
    )
}

fn expected(phase: press::Phase, event: press::Event) -> Option<press::Phase> {
    match (phase, event) {
        (press::Phase::Draft, press::Event::Check) => Some(press::Phase::Checked),
        (press::Phase::Checked, press::Event::Publish) => Some(press::Phase::Published),
        _ => None,
    }
}

#[test]
fn consuming_methods_validate_source_and_target_and_keep_owned_resources() -> Result<(), String> {
    use press::baked::typestate::{Draft, Ticket};
    let (mut runtime, resource) = initial();
    let draft = Ticket::<Draft>::restore(&mut runtime, resource)
        .map_err(|(_, error)| format!("{error:?}"))?;
    assert_eq!(runtime.checks, 1);
    let checked = draft
        .check(&mut runtime)
        .map_err(|(_, error)| format!("{error:?}"))?;
    assert_eq!(runtime.checks, 3);
    assert_eq!(
        Some(runtime.phase),
        expected(press::Phase::Draft, press::Event::Check)
    );
    assert_eq!(checked.resource().label, "draft!");
    let published = checked
        .publish(&mut runtime, true)
        .map_err(|(_, error)| format!("{error:?}"))?;
    assert_eq!(runtime.checks, 5);
    assert_eq!(runtime.effects, 2);
    assert_eq!(runtime.phase, press::Phase::Published);
    assert_eq!(
        Some(runtime.phase),
        expected(press::Phase::Checked, press::Event::Publish)
    );
    assert_eq!(expected(press::Phase::Draft, press::Event::Publish), None);
    assert_eq!(published.into_resource().label, "draft!!");
    Ok(())
}

#[test]
fn a_successful_effect_cannot_skip_actual_target_validation() -> Result<(), String> {
    use press::baked::typestate::{Draft, Ticket};
    let (mut runtime, resource) = initial();
    let draft = Ticket::<Draft>::restore(&mut runtime, resource)
        .map_err(|(_, error)| format!("{error:?}"))?;
    runtime.behavior = Behavior::FalseSuccess;
    let (returned, error) = draft
        .check(&mut runtime)
        .err()
        .ok_or("unchanged phase must refuse")?;
    assert_eq!(error, Refusal::Phase);
    assert_eq!(runtime.checks, 3);
    assert_eq!(runtime.effects, 1);
    assert_eq!(runtime.phase, press::Phase::Draft);
    assert_eq!(returned.label, "draft");
    Ok(())
}

#[test]
fn consuming_methods_refuse_wrong_owner_and_stale_revision_before_effects() -> Result<(), String> {
    use press::baked::typestate::{Draft, Ticket};
    let (mut runtime, resource) = initial();
    let draft = Ticket::<Draft>::restore(&mut runtime, resource)
        .map_err(|(_, error)| format!("{error:?}"))?;
    runtime.owner = 8;
    let (foreign_resource, foreign_error) = draft
        .check(&mut runtime)
        .err()
        .ok_or("foreign owner must refuse")?;
    assert_eq!(foreign_error, Refusal::Owner);
    assert_eq!(runtime.effects, 0);
    runtime.owner = 7;
    let restored_draft = Ticket::<Draft>::restore(&mut runtime, foreign_resource)
        .map_err(|(_, error)| format!("{error:?}"))?;
    runtime.revision = runtime.revision.saturating_add(1);
    let (stale_resource, stale_error) = restored_draft
        .check(&mut runtime)
        .err()
        .ok_or("stale revision must refuse")?;
    assert_eq!(stale_error, Refusal::Revision);
    assert_eq!(stale_resource.label, "draft");
    assert_eq!(runtime.effects, 0);
    Ok(())
}

#[test]
fn failure_recovery_requires_actual_runtime_phase_and_retains_partial_effects() -> Result<(), String>
{
    use press::baked::typestate::{Checked, Draft, Ticket};
    let (mut runtime, resource) = initial();
    let draft = Ticket::<Draft>::restore(&mut runtime, resource)
        .map_err(|(_, error)| format!("{error:?}"))?;
    runtime.behavior = Behavior::Refuse;
    let (unchanged, effect_error) = draft
        .check(&mut runtime)
        .err()
        .ok_or("effect must refuse")?;
    assert_eq!(effect_error, Refusal::Effect);
    let restored_draft = Ticket::<Draft>::restore(&mut runtime, unchanged)
        .map_err(|(_, error)| format!("{error:?}"))?;
    runtime.behavior = Behavior::Partial;
    let (partially_changed, partial_error) = restored_draft
        .check(&mut runtime)
        .err()
        .ok_or("partial effect must refuse")?;
    assert_eq!(partial_error, Refusal::Effect);
    assert_eq!(partially_changed.label, "draft!");
    let (current, phase_error) = Ticket::<Draft>::restore(&mut runtime, partially_changed)
        .err()
        .ok_or("old phase cannot be restored")?;
    assert_eq!(phase_error, Refusal::Phase);
    let checked = Ticket::<Checked>::restore(&mut runtime, current)
        .map_err(|(_, error)| format!("{error:?}"))?;
    let (retained, payload_error) = checked
        .publish(&mut runtime, false)
        .err()
        .ok_or("missing approval must refuse")?;
    assert_eq!(payload_error, Refusal::Payload);
    assert_eq!(retained.label, "draft!");
    assert_eq!(runtime.effects, 2);
    Ok(())
}
