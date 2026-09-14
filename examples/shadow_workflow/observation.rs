//! The caller's independent requirement on completed increments.

use super::{increment, synchronization};
use synchronization::{Arc, AtomicUsize, Ordering, thread};

pub(super) fn sequential() -> Result<(), String> {
    let counter = AtomicUsize::new(0usize);
    increment::separate(&counter);
    increment::separate(&counter);
    exact(&counter)
}

pub(super) fn concurrent(operation: fn(&AtomicUsize)) -> Result<(), String> {
    let counter = Arc::new(AtomicUsize::new(0usize));
    let handles = (0usize..2usize)
        .map(|_| {
            let shared = Arc::clone(&counter);
            thread::spawn(move || operation(&shared))
        })
        .collect::<Vec<_>>();
    for handle in handles {
        handle.join().map_err(|_| "a worker did not join")?;
    }
    exact(&counter)
}

fn exact(counter: &AtomicUsize) -> Result<(), String> {
    if counter.load(Ordering::SeqCst) == 2usize {
        Ok(())
    } else {
        Err("two completed increments must leave two".to_owned())
    }
}

#[cfg(loom)]
pub(super) fn sequential_model() -> macroonz::harness::preemption::PreemptionModelResult {
    sequential().map_err(|error| {
        macroonz::harness::preemption::PreemptionModelFailure::reported(error.as_bytes())
    })
}

#[cfg(loom)]
pub(super) fn separate_model() -> macroonz::harness::preemption::PreemptionModelResult {
    concurrent(increment::separate).map_err(|error| {
        macroonz::harness::preemption::PreemptionModelFailure::reported(error.as_bytes())
    })
}

#[cfg(loom)]
pub(super) fn indivisible_model() -> macroonz::harness::preemption::PreemptionModelResult {
    concurrent(increment::indivisible).map_err(|error| {
        macroonz::harness::preemption::PreemptionModelFailure::reported(error.as_bytes())
    })
}
