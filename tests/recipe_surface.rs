//! The root recipe entrance through a renamed facade binding, with declaration and evidence projections consumed whole.

use core::sync::atomic::{AtomicUsize, Ordering};
use macroonz as bakery;

static OPENED: AtomicUsize = AtomicUsize::new(0);

fn record_open() {
    OPENED.fetch_add(1, Ordering::Relaxed);
}

bakery::recipe! {
    /// A caller-owned door recipe.
    pub mod door {
        use core::marker::PhantomData;

        pub use baked::typestate;

        /// A caller-authored constant preserved beside generated companions.
        pub const DOOR_KIND: &str = "door";

        /// A caller-authored result alias preserved beside generated companions.
        pub type DoorResult<T> = core::result::Result<T, baked::TransitionRefusal>;

        /// A caller-authored marker preserved beside generated companions.
        pub struct DoorMarker;

        /// A caller-authored newtype preserved beside generated companions.
        pub struct DoorId(pub u8);

        /// A caller-authored data shape carrying lifetime, type, const, bound, where, named-field and phantom seats.
        pub struct BorrowedDoor<'a, T: Clone, const N: usize>
        where
            T: 'a,
        {
            /// The borrowed caller value.
            pub value: &'a T,
            /// The const-sized caller bytes.
            pub bytes: [u8; N],
            /// The caller's exact phantom relationship.
            pub marker: PhantomData<T>,
        }

        /// The caller-owned state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The closed state.
            Closed,
            /// The open state.
            Open,
        }

        /// The caller-owned event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// The request to open.
            OpenDoor,
        }

        bake! {
            vocabularies(State, Event);
            transitions {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
                typestate;
            };
        }
    }
}

#[test]
fn the_renamed_facade_bakes_the_declared_module_without_recipe_material_ceremony() {
    assert_eq!(
        door::baked::STATE_VARIANTS,
        &[door::State::Closed, door::State::Open]
    );
    assert_eq!(door::baked::EVENT_VARIANTS, &[door::Event::OpenDoor]);
    assert_eq!(
        door::baked::TRANSITIONS,
        &[(
            door::State::Closed,
            door::Event::OpenDoor,
            door::State::Open
        )]
    );
    assert_eq!(
        door::baked::apply(door::State::Closed, door::Event::OpenDoor),
        Ok(door::State::Open)
    );
    assert_eq!(
        door::baked::apply(door::State::Open, door::Event::OpenDoor),
        Err(door::baked::TransitionRefusal::Absent)
    );
    assert!(OPENED.load(Ordering::Relaxed) > 0);
    assert_eq!(door::DOOR_KIND, "door");
    let id = door::DoorId(7);
    assert_eq!(id.0, 7);
    let value = String::from("borrowed");
    let borrowed = door::BorrowedDoor::<String, 2> {
        value: &value,
        bytes: [1, 2],
        marker: core::marker::PhantomData,
    };
    assert_eq!(borrowed.value, "borrowed");
    assert_eq!(borrowed.bytes, [1, 2]);
    let stage = door::typestate::Stage::<door::typestate::Closed>(core::marker::PhantomData);
    assert_eq!(stage, door::typestate::Stage(core::marker::PhantomData));
    let result: door::DoorResult<door::State> = Ok(door::State::Open);
    assert_eq!(result, Ok(door::State::Open));
}

bakery::recipe! {
    /// A recipe whose Rust names require raw-identifier custody.
    pub mod r#type {
        /// A caller-owned raw state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum r#state {
            /// A raw keyword member.
            r#match,
            /// An ordinary member.
            Ready,
        }

        /// A caller-owned raw event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum r#event {
            /// A raw keyword member.
            r#move,
        }

        bake! {
            vocabularies(r#state, r#event);
            transitions {
                (r#match, r#move) => Ready with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
                typestate;
            };
        }
    }
}

#[test]
fn raw_identifiers_survive_every_declaration_site_projection() {
    assert_eq!(
        r#type::baked::STATE_VARIANTS,
        &[r#type::r#state::r#match, r#type::r#state::Ready]
    );
    assert_eq!(
        r#type::baked::apply(r#type::r#state::r#match, r#type::r#event::r#move),
        Ok(r#type::r#state::Ready)
    );
    let stage = r#type::baked::typestate::Stage::<r#type::baked::typestate::r#match>(
        core::marker::PhantomData,
    );
    assert_eq!(
        stage,
        r#type::baked::typestate::Stage(core::marker::PhantomData)
    );
}

#[test]
fn generated_dispatch_agrees_with_an_independent_model_and_rejects_a_planted_defect() {
    let cases = [
        (door::State::Closed, door::Event::OpenDoor),
        (door::State::Open, door::Event::OpenDoor),
    ];
    for (state, event) in cases {
        let generated = door::baked::apply(state, event);
        assert_eq!(generated, handwritten_dispatch(state, event));
        assert_ne!(generated, planted_dispatch(state, event));
    }
}

fn handwritten_dispatch(
    state: door::State,
    event: door::Event,
) -> Result<door::State, door::baked::TransitionRefusal> {
    match (state, event) {
        (door::State::Closed, door::Event::OpenDoor) => Ok(door::State::Open),
        (door::State::Open, door::Event::OpenDoor) => Err(door::baked::TransitionRefusal::Absent),
    }
}

fn planted_dispatch(
    state: door::State,
    event: door::Event,
) -> Result<door::State, door::baked::TransitionRefusal> {
    match (state, event) {
        (door::State::Closed, door::Event::OpenDoor) => Err(door::baked::TransitionRefusal::Absent),
        (door::State::Open, door::Event::OpenDoor) => Ok(door::State::Open),
    }
}
