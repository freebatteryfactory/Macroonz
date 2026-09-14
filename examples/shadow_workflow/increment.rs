//! Two separately authored ways to update one counter.

use super::synchronization::{AtomicUsize, Ordering};

pub(super) fn separate(counter: &AtomicUsize) {
    let observed = counter.load(Ordering::SeqCst);
    counter.store(observed.saturating_add(1usize), Ordering::SeqCst);
}

pub(super) fn indivisible(counter: &AtomicUsize) {
    counter.fetch_add(1usize, Ordering::SeqCst);
}
