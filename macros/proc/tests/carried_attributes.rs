//! The three attributes applied for real: each expands where it stands, and the item survives beside its carrier.
//!
//! Compiling this file is most of the claim.
//! Each attribute below ran as an actual proc macro, walked the whole road — capture, request, render, close, explain, bind — and emitted an exported carrier this crate now defines; a refusal anywhere would be a `compile_error!` and this target would not build.
//! The carriers stay inert on purpose: invoking one is the consumption target's act, performed where that target's harness and host facts live.

#[macroonz_macros::trials(
    support = greet_support,
    module = greet_trials,
    table = named("proc", "greet-table"),
    suite checks = named("proc", "unit") {
        greet_answers {
            claim = named("proc", "greet-answers"),
            subject = named("proc", "greet"),
            check = named("proc", "exact"),
            population = named("proc", "smalls"),
        },
    },
)]
mod greeted {
    /// The one fact the trial declaration stands beside.
    pub(crate) const ANSWER: &str = "hello";
}

/// The declared order one mutation surface presses: three members, so two adjacent transpositions exist.
#[macroonz_macros::mutations(
    module = pressed,
    refusal = PressRefusal,
    support = press_support,
    family = named("proc", "refusals"),
    point = named("proc", "press-point"),
    fact = named("proc", "cause-order"),
    map named("proc", "cause-order") = named("proc", "order-held"),
    permit named("proc", "order-held") = ["declared-order-permutation"],
)]
pub enum Cause {
    /// The first cause in the declared order.
    First,
    /// The second cause in the declared order.
    Second,
    /// The third cause in the declared order.
    Third,
}

#[macroonz_macros::bench(
    support = pace_support,
    table_function = pace_table,
    table = named("proc", "pace-table"),
    reporter = pace_reporter,
    encode_pace {
        workload = named("proc", "encode"),
        preflight = named("proc", "encode-correct"),
        planted_worse = named("proc", "encode-worse"),
        complexity = named("proc", "linear"),
        axis = [2, 4, 8],
        samples = 16,
        warmups = 4,
        ratio_numerator = 3,
        ratio_denominator = 1,
        observe = [named("proc", "bytes-touched")],
    },
)]
mod paced {
    /// The one fact the bench declaration stands beside.
    pub(crate) const WORKLOAD: &str = "encode";
}

/// The one shadow declaration a crate writes: both faces of every chosen name, emitted where it stands.
///
/// This build carries no `loom` configuration, so the ordinary faces are what compile here — each name below resolves to its standard-library path through this module.
mod shadowed {
    macroonz_macros::shadow! {
        loom = loom,
        names = [Arc, AtomicUsize, Barrier, Mutex, Ordering, mpsc, spin_loop, thread, thread_local],
    }
}

/// The decorated items reach this test untouched, which is the half of the contract compilation alone cannot state.
#[test]
fn the_items_survive_beside_their_carriers() {
    assert_eq!(greeted::ANSWER, "hello");
    assert_eq!(paced::WORKLOAD, "encode");
    let held = Cause::Second;
    assert!(matches!(held, Cause::First | Cause::Second | Cause::Third));
}

/// The shadowed names resolve to their ordinary faces off the loom configuration, and behave as the standard library.
#[test]
fn the_shadowed_names_stand_for_std_off_loom() {
    let value = shadowed::Arc::new(7u8);
    assert_eq!(*value, 7u8);
    let counter = shadowed::AtomicUsize::new(0usize);
    counter.fetch_add(2usize, shadowed::Ordering::SeqCst);
    assert_eq!(counter.load(shadowed::Ordering::SeqCst), 2usize);
    let guarded = shadowed::Mutex::new("held");
    if let Ok(reached) = guarded.lock() {
        assert_eq!(*reached, "held");
    }
    let spawned = shadowed::thread::spawn(|| 5u8);
    assert_eq!(spawned.join().ok(), Some(5u8));
    let (sender, receiver) = shadowed::mpsc::channel::<u8>();
    assert!(sender.send(3u8).is_ok());
    assert_eq!(receiver.recv().ok(), Some(3u8));
    shadowed::spin_loop();
    shadowed::thread_local! {
        static HELD: u8 = const { 9u8 };
    }
    HELD.with(|held| assert_eq!(*held, 9u8));
    let gate = shadowed::Barrier::new(1usize);
    assert!(gate.wait().is_leader());
}
