#![forbid(unsafe_code)]

use core::sync::atomic::{AtomicUsize, Ordering};

static EFFECTS: AtomicUsize = AtomicUsize::new(0);

fn record_effect() {
    EFFECTS.fetch_add(1, Ordering::SeqCst);
}

macroonz::recipe! {
    pub mod subject {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State { Closed, Open }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event { OpenDoor }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, OpenDoor) => Open with(crate::record_effect);
            };
            absence(refused);
            projections { companions; dispatch(apply); };
        }
    }
}

fn main() -> Result<(), String> {
    let arguments = std::env::args().collect::<Vec<_>>();
    let [_, expected] = arguments.as_slice() else {
        return Err("one independent expected target is required".to_owned());
    };
    let target = match expected.as_str() {
        "open" => subject::State::Open,
        "closed" => subject::State::Closed,
        _ => return Err("unknown independent expectation".to_owned()),
    };
    for state in [subject::State::Closed, subject::State::Open] {
        let expected = match state {
            subject::State::Closed => Ok(target),
            subject::State::Open => Err(subject::baked::TransitionRefusal::Absent),
        };
        assert_eq!(subject::baked::apply(state, subject::Event::OpenDoor), expected);
    }
    assert_eq!(EFFECTS.load(Ordering::SeqCst), 1);
    println!("adopter-domain=2 expected-target={expected} effects=1");
    Ok(())
}
