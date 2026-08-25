# fault

Adversity you schedule, not adversity the harness invents.

A fault here is a value you wrote: some behavior of yours, joined to the postcondition you promise still holds when that behavior refuses.
The harness never calls it and never looks inside it.
What the harness owns is the placement — which of your adapters sits at which command, under which named schedule, and whether that schedule is coherent enough to be worth running.

## The road

Place adapters at zero-based positions, name the schedule, gather schedules into a campaign, select one by name, and inject it into an ordinary sequence of your own commands.

```text
let control = FaultSchedule::declared(schedule_name("quiet-control")?, Vec::new());
let hostile = FaultSchedule::declared(
    schedule_name("capacity-at-the-second-write")?,
    vec![ScheduledFault::at(
        SequencePosition::at(1),
        FaultAdapter::declared(WriteFault::Capacity, WritePostcondition::StateUnchanged),
    )],
);
let campaign = FaultCampaign::declared(vec![control, hostile])?;
let selected = campaign.select(schedule_name("capacity-at-the-second-write")?)?;
let injected = inject(&selected, commands())?;
```

Back comes your sequence, each command carrying the adapters scheduled at its position, in the order you wrote them.
Running them is yours.

## What it refuses

- A campaign with no schedule, and a campaign whose schedules are all empty controls: each declares pressure and applies none.
- Two schedules under one name, which would leave a selection with two answers.
- A name the campaign never declared.
- A position past the end of the sequence, refused before a single adapter is cloned, so a schedule is never half-injected.

An empty schedule beside a hostile one is not a refusal.
That is the control you compare against.

## What it does not do

It defines no port trait, keeps no registry, installs no hook, executes no adapter, and reaches no verdict.
An injected sequence is a history; what your behavior did with it, and whether your postcondition survived, is read on the harness's ordinary `properties` and `runner` roads.

Out-of-memory needs no allocator hook here.
An adapter that refuses at its own declared capacity is that experiment, written in safe Rust.
