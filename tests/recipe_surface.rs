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
        pub type DoorResult<T> = Result<T, baked::TransitionRefusal>;

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

        /// One caller-authored trait preserved beside generated companions.
        pub trait AuthoredStage {
            /// Reads the caller-authored state spelling.
            fn authored_name(self) -> &'static str;
        }

        impl AuthoredStage for State {
            fn authored_name(self) -> &'static str {
                match self {
                    Self::Closed => "Closed",
                    Self::Open => "Open",
                }
            }
        }

        /// The caller-owned event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// The request to open.
            OpenDoor,
            /// A declared event with no admitted transition row.
            CloseDoor,
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, OpenDoor) => Open with(crate::record_open);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
                compile_contract;
                declaration_conformance;
                typestate(State);
            };
            support(door_recipe_support);
        }
    }
}

door_recipe_support! {
    declaring: crate,
    harness: bakery::harness,
}

#[test]
fn the_renamed_facade_bakes_the_declared_module_without_recipe_material_ceremony() {
    assert_eq!(
        door::baked::STATE_VARIANTS,
        &[door::State::Closed, door::State::Open]
    );
    assert_eq!(
        door::baked::EVENT_VARIANTS,
        &[door::Event::OpenDoor, door::Event::CloseDoor]
    );
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
    assert_eq!(
        door::baked::apply(door::State::Closed, door::Event::CloseDoor),
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
    assert_eq!(
        door::AuthoredStage::authored_name(door::State::Open),
        "Open"
    );
    assert_eq!(
        <door::typestate::Closed as door::typestate::RecipeStage>::NAME,
        "Closed"
    );
    let stage = door::typestate::Stage::<door::typestate::Closed>::new();
    assert_eq!(stage, door::typestate::Stage(core::marker::PhantomData));
    assert_eq!(
        door::typestate::Stage::<door::typestate::Open>::default(),
        door::typestate::Stage(core::marker::PhantomData)
    );
    let result: door::DoorResult<door::State> = Ok(door::State::Open);
    assert_eq!(result, Ok(door::State::Open));
}

bakery::recipe! {
    /// A recipe whose Rust names require raw-identifier custody.
    pub mod r#type {
        /// A caller-owned state vocabulary beside a raw module name.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The waiting member.
            Waiting,
            /// The ready member.
            Ready,
        }

        /// A caller-owned event vocabulary beside a raw effect path.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// The request to advance.
            Advance,
        }

        /// A caller-owned effect named by a raw Rust keyword.
        pub fn r#move() {
            crate::record_open();
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Waiting, Advance) => Ready with(crate::r#type::r#move);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
                typestate(State);
            };
        }
    }
}

#[test]
fn raw_identifiers_survive_module_and_effect_path_projection() {
    assert_eq!(
        r#type::baked::STATE_VARIANTS,
        &[r#type::State::Waiting, r#type::State::Ready]
    );
    assert_eq!(
        r#type::baked::apply(r#type::State::Waiting, r#type::Event::Advance),
        Ok(r#type::State::Ready)
    );
    assert_eq!(
        <r#type::baked::typestate::Waiting as r#type::baked::typestate::RecipeStage>::NAME,
        "Waiting"
    );
    let stage = r#type::baked::typestate::Stage::<r#type::baked::typestate::Waiting>::new();
    assert_eq!(
        stage,
        r#type::baked::typestate::Stage(core::marker::PhantomData)
    );
}

#[test]
fn generated_dispatch_agrees_with_an_independent_model_and_rejects_a_planted_defect() {
    let cases = [
        (door::State::Closed, door::Event::OpenDoor),
        (door::State::Closed, door::Event::CloseDoor),
        (door::State::Open, door::Event::OpenDoor),
        (door::State::Open, door::Event::CloseDoor),
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
        (door::State::Closed, door::Event::CloseDoor)
        | (door::State::Open, door::Event::OpenDoor | door::Event::CloseDoor) => {
            Err(door::baked::TransitionRefusal::Absent)
        }
    }
}

fn planted_dispatch(
    state: door::State,
    event: door::Event,
) -> Result<door::State, door::baked::TransitionRefusal> {
    match (state, event) {
        (door::State::Closed, door::Event::OpenDoor) => Err(door::baked::TransitionRefusal::Absent),
        (door::State::Closed, door::Event::CloseDoor)
        | (door::State::Open, door::Event::OpenDoor | door::Event::CloseDoor) => {
            Ok(door::State::Open)
        }
    }
}
