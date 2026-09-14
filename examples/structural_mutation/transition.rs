//! Recipe target and effect alternatives with independently authored output checks.

use macroonz::compiler::descriptor::mutation::{
    EFFECT_BINDING_FAMILY, Surface, TRANSITION_TARGET_FAMILY, completed_from_effect_bindings,
    completed_from_transition_targets,
};
use macroonz::compiler::recipe::HarnessPosture;
use macroonz::compiler::{CrateBinding, Door, Producer, TextCapture};

const DOOR: Door = Door::declared(
    "structural-example",
    "structural-example.grammar",
    "structural-example::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "structural-example",
        name: "recipe",
    },
);

const RECIPE: &str = r"
pub mod machine {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State { Idle, Busy, Done }
    pub enum Event { Start, Stop }
    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Idle, Start) => Busy with(result) {
                *work = work.saturating_add(1u64);
                Ok(result)
            };
            (Busy, Stop) => Idle with(result) {
                *work = work.saturating_add(10u64);
                Ok(result)
            };
        };
        absence(refused);
        projections {
            dispatch(current, event) {
                pub fn advance(work: &mut u64, current: State, event: Event)
                    -> Result<State, TransitionRefusal>;
            };
        };
    }
}
";

pub(super) const OBSERVER: &str = r#"
use std::io::Write;

fn main() -> std::io::Result<()> {
    use machine::{State, Event};
    let mut work = 0u64;
    let reached = machine::baked::advance(&mut work, State::Idle, Event::Start);
    let mut other_work = 0u64;
    let other = machine::baked::advance(&mut other_work, State::Busy, Event::Stop);
    let absent = machine::baked::advance(&mut other_work, State::Done, Event::Start);
    let agrees = reached == Ok(State::Busy) && work == 1u64
        && other == Ok(State::Idle) && other_work == 10u64 && absent.is_err();
    writeln!(std::io::stdout(), "{}", u64::from(agrees))
}
"#;

pub(super) fn targets() -> Result<Surface, String> {
    let captured = TextCapture::read(RECIPE).map_err(|error| format!("{error:?}"))?;
    completed_from_transition_targets(
        super::declaration::declared(TRANSITION_TARGET_FAMILY)?,
        captured.input(),
        HarnessPosture::Unavailable,
        0,
        &DOOR,
    )
    .map_err(|error| error.to_string())
}

pub(super) fn effects() -> Result<Surface, String> {
    let captured = TextCapture::read(RECIPE).map_err(|error| format!("{error:?}"))?;
    completed_from_effect_bindings(
        super::declaration::declared(EFFECT_BINDING_FAMILY)?,
        captured.input(),
        HarnessPosture::Unavailable,
        0,
        &DOOR,
    )
    .map_err(|error| error.to_string())
}
