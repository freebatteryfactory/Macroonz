//! Explicit caller declarations and independently stated subject observations.

use macroonz_compiler::descriptor::mutation::{
    Address, Declaration, FactMapping, FamilySlug, Permission, Policy,
};
use macroonz_compiler::descriptor::{ModuleName, Name, TypeName};
use macroonz_compiler::recipe::{HarnessPosture, RecipeBake, bake};
use macroonz_compiler::{CrateBinding, Door, GeneratedToken, Producer, TextCapture};

pub(super) const DOOR: Door = Door::declared(
    "recipe-pressure",
    "recipe-pressure.grammar",
    "recipe-pressure::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "recipe-pressure",
        name: "recipe",
    },
);

pub(super) const RECIPE: &str = r"
pub mod machine {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum State { Idle, Busy, Done }
    pub enum Event { Start, Stop }
    pub const AUTHORED: u8 = 37;

    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Idle, Start) => Busy with(result) {
                *log += 1;
                Ok(result)
            };
            (Busy, Stop) => Idle with(destination) {
                *log += 10;
                Ok(destination)
            };
        };
        absence(refused);
        projections {
            companions;
            dispatch(current, event) {
                pub fn advance(
                    log: &mut u8,
                    current: State,
                    event: Event,
                ) -> Result<State, TransitionRefusal>;
            };
        };
    }
}
";

pub(super) const WITNESS: &str = r#"
fn main() {
    use machine::{Event, State};
    assert_eq!(machine::AUTHORED, 37);
    let mut other_log = 0;
    assert_eq!(
        machine::baked::advance(&mut other_log, State::Busy, Event::Stop)
            .expect("the other row must remain available"),
        State::Idle,
    );
    assert_eq!(other_log, 10);
    let mut log = 0;
    assert!(machine::baked::advance(&mut log, State::Done, Event::Start).is_err());
    let reached = machine::baked::advance(&mut log, State::Idle, Event::Start)
        .expect("the declared event must execute");
    assert_eq!(reached, State::Busy, "independent-transition-target");
    assert_eq!(log, 1, "independent-transition-effect");
}
"#;

pub(super) fn declaration(family: &str) -> Result<Declaration, String> {
    let name = |stem| Name::named("recipe-pressure", stem).map_err(|refusal| refusal.to_string());
    let fact = name("first-row")?;
    let claim = name("first-event-behavior")?;
    let permission = Permission::permitted(
        claim.clone(),
        vec![FamilySlug::declared(family).map_err(|refusal| refusal.to_string())?],
    )
    .map_err(|refusal| refusal.to_string())?;
    let policy = Policy::declared(
        name("machine")?,
        vec![FactMapping {
            fact: fact.clone(),
            claim,
        }],
        vec![permission],
    )
    .map_err(|refusal| refusal.to_string())?;
    Ok(Declaration::captured(
        Address {
            module: ModuleName::declared("pressure").map_err(|refusal| refusal.to_string())?,
            refusal: TypeName::declared("PressureRefusal")
                .map_err(|refusal| refusal.to_string())?,
            support: None,
        },
        policy,
        name("selected-row")?,
        fact,
    ))
}

pub(super) fn captured(source: &str) -> Result<TextCapture, String> {
    TextCapture::read(source).map_err(|refusal| format!("{refusal:?}"))
}

pub(super) fn ordinary(source: &str) -> Result<RecipeBake, String> {
    bake(
        captured(source)?.input(),
        HarnessPosture::Unavailable,
        &DOOR,
    )
    .map_err(|refusal| refusal.summary().to_owned())
}

pub(super) fn source(tokens: &[GeneratedToken]) -> Result<&str, String> {
    match tokens {
        [GeneratedToken::Text(source)] => Ok(source),
        _ => Err("recipe mutation material must be one source literal".to_owned()),
    }
}
