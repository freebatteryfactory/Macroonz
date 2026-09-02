//! Package-shaped effect forms and their independently stated behavior.

pub(super) const EFFECT_PRODUCER: &str = r"#![deny(warnings)]

bakery::recipe! {
    /// Zero-argument path shorthand.
    pub mod zero_argument {
        use core::sync::atomic::{AtomicUsize, Ordering};

        static CALLS: AtomicUsize = AtomicUsize::new(0);

        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// No effect has run.
            Idle,
            /// The effect completed.
            Done,
        }

        /// One event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Run the effect.
            Run,
        }

        fn effect() {
            CALLS.fetch_add(1, Ordering::SeqCst);
        }

        /// Reads how many effects ran.
        pub fn calls() -> usize {
            CALLS.load(Ordering::SeqCst)
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Idle, Run) => Done with(crate::zero_argument::effect);
            };
            absence(refused);
            projections { dispatch(apply); };
        }
    }
}

bakery::recipe! {
    /// Shared borrowing, methods, and generic calls through exact rows.
    pub mod shared {
        /// Shared caller context.
        pub struct Context {
            /// One observed value.
            pub value: usize,
        }

        impl Context {
            fn method(&self, state: State, event: Event) -> usize {
                self.value + state.slot() + event.slot()
            }
        }

        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The initial state.
            Idle,
            /// The completed state.
            Done,
        }

        impl State {
            const fn slot(self) -> usize {
                match self {
                    Self::Idle => 0,
                    Self::Done => 1,
                }
            }
        }

        /// Three shared effect forms.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// A shared free function.
            Shared,
            /// A method call.
            Method,
            /// A generic free function.
            Generic,
        }

        impl Event {
            const fn slot(self) -> usize {
                match self {
                    Self::Shared => 1,
                    Self::Method => 2,
                    Self::Generic => 3,
                }
            }
        }

        fn observe(context: &Context, state: State, event: Event) -> usize {
            context.value + state.slot() + event.slot()
        }

        fn generic<T, const N: usize>(context: &Context) -> usize {
            context.value + core::mem::size_of::<T>() + N
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Idle, Shared) => Done with(target) {
                    let _observed = crate::shared::observe(context, current, event);
                    Ok(target)
                };
                (Idle, Method) => Done with(target) {
                    let _observed = context.method(current, event);
                    Ok(target)
                };
                (Idle, Generic) => Done with(target) {
                    let _observed = crate::shared::generic::<u16, 4>(context);
                    Ok(target)
                };
            };
            absence(refused);
            projections {
                dispatch(current, event) {
                    pub fn apply(
                        context: &super::Context,
                        current: State,
                        event: Event,
                    ) -> Result<State, TransitionRefusal>;
                };
            };
        }
    }
}

bakery::recipe! {
    /// Mutable and fallible effects.
    pub mod mutable {
        /// Mutable caller context.
        pub struct Context {
            /// Completed effect count.
            pub calls: usize,
            /// Whether the fallible effect refuses.
            pub fail: bool,
        }

        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The initial state.
            Idle,
            /// The completed state.
            Done,
        }

        /// Two mutable effect forms.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// A mutable infallible call.
            Mutate,
            /// A mutable fallible call.
            Try,
        }

        fn mutate(context: &mut Context) {
            context.calls = context.calls.saturating_add(1);
        }

        fn fallible(context: &mut Context) -> Result<(), ()> {
            if context.fail {
                Err(())
            } else {
                context.calls = context.calls.saturating_add(1);
                Ok(())
            }
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Idle, Mutate) => Done with(target) {
                    crate::mutable::mutate(context);
                    Ok(target)
                };
                (Idle, Try) => Done with(target) {
                    crate::mutable::fallible(context)
                        .map_err(|()| TransitionRefusal::Absent)?;
                    Ok(target)
                };
            };
            absence(refused);
            projections {
                dispatch(state, event) {
                    pub fn apply(
                        context: &mut super::Context,
                        state: State,
                        event: Event,
                    ) -> Result<State, TransitionRefusal>;
                };
            };
        }
    }
}

bakery::recipe! {
    /// A consuming context returning a replacement context.
    pub mod consuming {
        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The initial state.
            Idle,
            /// The completed state.
            Done,
        }

        /// One event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Consume the context.
            Run,
        }

        /// One consumed and returned caller value.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Context {
            /// The context's resulting state.
            pub state: State,
        }

        fn consume(mut context: Context, target: State) -> Context {
            context.state = target;
            context
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Idle, Run) => Done with(target) {
                    Ok(crate::consuming::consume(context, target))
                };
            };
            absence(refused);
            projections {
                dispatch(state, event) {
                    pub fn apply(
                        context: super::Context,
                        state: State,
                        event: Event,
                    ) -> Result<super::Context, TransitionRefusal>;
                };
            };
        }
    }
}

bakery::recipe! {
    /// An asynchronous effect returning the declared target.
    pub mod asynchronous {
        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The initial state.
            Idle,
            /// The completed state.
            Done,
        }

        /// One event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Await the effect.
            Run,
        }

        async fn effect(target: State) -> State {
            target
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Idle, Run) => Done with(target) {
                    Ok(crate::asynchronous::effect(target).await)
                };
            };
            absence(refused);
            projections {
                dispatch {
                    pub async fn apply(
                        state: State,
                        event: Event,
                    ) -> Result<State, TransitionRefusal>;
                };
            };
        }
    }
}

bakery::recipe! {
    /// An effect that returns the next state.
    pub mod returning {
        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The initial state.
            Idle,
            /// The completed state.
            Done,
        }

        /// One event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Return the next state.
            Run,
        }

        const fn effect(target: State) -> State {
            target
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Idle, Run) => Done with(target) {
                    Ok(crate::returning::effect(target))
                };
            };
            absence(refused);
            projections {
                dispatch {
                    pub fn apply(
                        state: State,
                        event: Event,
                    ) -> Result<State, TransitionRefusal>;
                };
            };
        }
    }
}

bakery::recipe! {
    /// A lifetime-bearing borrowed output.
    pub mod borrowed {
        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The initial state.
            Idle,
            /// The completed state.
            Done,
        }

        /// One event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Borrow the output.
            Read,
        }

        /// Caller-owned borrowed material.
        pub struct Context {
            /// The borrowed label.
            pub label: String,
        }

        fn effect(context: &Context) -> &str {
            context.label.as_str()
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Idle, Read) => Done with(target) {
                    let _ = target;
                    Ok(crate::borrowed::effect(context))
                };
            };
            absence(refused);
            projections {
                dispatch(state, event) {
                    pub fn apply<'a>(
                        context: &'a super::Context,
                        state: State,
                        event: Event,
                    ) -> Result<&'a str, TransitionRefusal>;
                };
            };
        }
    }
}

bakery::recipe! {
    /// A typestate carrier distinct from the declared state enum.
    pub mod typestate_result {
        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The initial state.
            Closed,
            /// The completed state.
            Open,
        }

        /// One event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Open the carrier.
            OpenDoor,
        }

        /// The distinct completed carrier.
        #[derive(Debug, PartialEq, Eq)]
        pub struct Open;

        const fn effect(_target: State) -> Open {
            Open
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Closed, OpenDoor) => Open with(target) {
                    Ok(crate::typestate_result::effect(target))
                };
            };
            absence(refused);
            projections {
                dispatch {
                    pub fn apply(
                        state: State,
                        event: Event,
                    ) -> Result<super::Open, TransitionRefusal>;
                };
            };
        }
    }
}

bakery::recipe! {
    /// One explicit caller-owned unsafe boundary.
    pub mod explicit_boundary {
        /// One state vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum State {
            /// The initial state.
            Idle,
            /// The completed state.
            Done,
        }

        /// One event vocabulary.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Event {
            /// Cross the explicit boundary.
            Run,
        }

        unsafe fn effect(value: u8) -> u8 {
            value.saturating_add(1)
        }

        bake! {
            vocabularies { State; Event; };
            transitions(State, Event) {
                (Idle, Run) => Done with(target) {
                    let _ = target;
                    Ok(unsafe { crate::explicit_boundary::effect(value) })
                };
            };
            absence(refused);
            projections {
                dispatch(state, event) {
                    pub unsafe fn apply(
                        value: u8,
                        state: State,
                        event: Event,
                    ) -> Result<u8, TransitionRefusal>;
                };
            };
        }
    }
}
";

pub(super) const EFFECT_CONSUMER: &str = r#"#![deny(warnings)]

#[test]
fn zero_argument_side_effect_matches_the_handwritten_result() {
    use renamed_recipe_adopter::zero_argument::{Event, State, baked, calls};

    assert_eq!(baked::apply(State::Idle, Event::Run), Ok(State::Done));
    assert_eq!(calls(), 1);
}

#[test]
fn shared_method_and_generic_effects_match_declared_targets() {
    use renamed_recipe_adopter::shared::{Context, Event, State, baked};

    let context = Context { value: 7 };
    for event in [Event::Shared, Event::Method, Event::Generic] {
        assert_eq!(baked::apply(&context, State::Idle, event), Ok(State::Done));
    }
}

#[test]
fn mutable_and_fallible_effects_preserve_context_and_failure() {
    use renamed_recipe_adopter::mutable::{Context, Event, State, baked};

    let mut context = Context { calls: 0, fail: false };
    assert_eq!(
        baked::apply(&mut context, State::Idle, Event::Mutate),
        Ok(State::Done)
    );
    assert_eq!(context.calls, 1);
    assert_eq!(
        baked::apply(&mut context, State::Idle, Event::Try),
        Ok(State::Done)
    );
    assert_eq!(context.calls, 2);
    context.fail = true;
    assert_eq!(
        baked::apply(&mut context, State::Idle, Event::Try),
        Err(baked::TransitionRefusal::Absent)
    );
}

#[test]
fn consuming_effect_returns_the_replacement_context() {
    use renamed_recipe_adopter::consuming::{Context, Event, State, baked};

    let context = Context { state: State::Idle };
    assert_eq!(
        baked::apply(context, State::Idle, Event::Run),
        Ok(Context { state: State::Done })
    );
}

#[test]
fn asynchronous_effect_completes_to_the_declared_target() {
    use core::future::Future;
    use core::task::{Context, Poll, Waker};
    use renamed_recipe_adopter::asynchronous::{Event, State, baked};

    let mut future = Box::pin(baked::apply(State::Idle, Event::Run));
    let mut context = Context::from_waker(Waker::noop());
    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(Ok(State::Done)));
}

#[test]
fn returned_state_matches_the_handwritten_effect() {
    use renamed_recipe_adopter::returning::{Event, State, baked};

    assert_eq!(baked::apply(State::Idle, Event::Run), Ok(State::Done));
}

#[test]
fn borrowed_output_keeps_its_caller_lifetime() {
    use renamed_recipe_adopter::borrowed::{Context, Event, State, baked};

    let context = Context { label: String::from("ready") };
    assert_eq!(baked::apply(&context, State::Idle, Event::Read), Ok("ready"));
}

#[test]
fn typestate_effect_returns_the_distinct_carrier() {
    use renamed_recipe_adopter::typestate_result::{Event, Open, State, baked};

    assert_eq!(baked::apply(State::Closed, Event::OpenDoor), Ok(Open));
}

#[test]
fn explicit_boundary_remains_explicit_at_the_generated_call() {
    use renamed_recipe_adopter::explicit_boundary::{Event, State, baked};

    assert_eq!(unsafe { baked::apply(4, State::Idle, Event::Run) }, Ok(5));
}
"#;
