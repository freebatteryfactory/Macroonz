//! The subject-panic catch at the trial boundary: one unwind catch, one chained
//! panic hook installed once, and the safe payload and observed origin copied
//! into a typed finding.
//!
//! A panic from the subject is a verdict about the subject, so it is caught
//! here and recorded as the finding it is. Two mechanisms are needed because
//! neither is enough alone: the unwind catch returns the payload but not where
//! the panic was raised, and the hook sees the location but cannot stop the
//! unwind.
//!
//! # The process-global fact
//!
//! A panic hook is process-wide, and this file installs one. It is installed
//! once, on the first trial that runs, and it CHAINS: it copies the origin it
//! observed and then calls the hook that was standing when it was installed, so
//! whatever the host printed before it still prints — this home writes nothing
//! itself. A hook any other party installs afterwards replaces this one, and
//! findings then carry the payload without an origin. Correlation is per
//! thread: the slot is cleared before the callable runs and taken after it
//! unwinds, on the same thread, so the origin a finding carries is the origin of
//! the unwind that reached this boundary.
//!
//! # The unwind-safety assertion
//!
//! [`AssertUnwindSafe`] is asserted over the call, and what it asserts is
//! narrow. The closure environment contains shared references to the attachment and invocation, and the subject is a function pointer with no captured state, so this boundary mutates no captured value across the catch. The assertion claims no semantic purity for the subject and nothing about state the subject reaches on its own: a subject that mutates a global or its own interior mutability leaves that state exactly as its panic left it, and this boundary neither inspects nor repairs it.
//!
//! # Nonclaims
//!
//! An unwind is what is caught. An abort — a build that aborts on panic, an
//! explicit abort, an allocation failure — and a stack overflow are not
//! unwinds: they end the process, no finding is produced, and no sentence here
//! claims otherwise. A panic on a thread the subject spawned reaches this
//! boundary only if the subject's own call unwinds because of it.

use super::types::{Invocation, SUBJECT_PANIC_CAUSE};
use crate::descriptor::ExecutableAttachment;
use crate::report::{FailureClass, FindingLocation, ForeignText, TrialConclusion, TrialFinding};
use std::any::Any;
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, Location, catch_unwind};
use std::sync::OnceLock;

/// Where one panic was raised, as the hook observed it.
///
/// The file is owned rather than borrowed because a hook sees the location for
/// the length of the call and no longer.
struct PanicOrigin {
    file: String,
    line: u32,
}

/// The one installation of this home's panic hook.
static CAPTURE_HOOK: OnceLock<()> = OnceLock::new();

thread_local! {
    /// The origin this thread's last observed panic was raised at.
    static PANIC_ORIGIN: Cell<Option<PanicOrigin>> = const { Cell::new(None) };
}

/// The conclusion one attachment reaches, with a subject panic caught at the
/// boundary and returned as a refusal.
///
/// The caught-payload mapping reads the payload only in the two shapes it is safely readable in, reaches the origin slot through a fallible borrow, and gives every absence its own arm. This is a claim about the mapping after hook operations return, not panic-freedom of process-global hook installation, the chained prior hook, or allocation.
pub(super) fn caught_conclusion(
    attachment: &ExecutableAttachment<Invocation, TrialConclusion>,
    invocation: &Invocation,
) -> TrialConclusion {
    install_capture_hook();
    store_origin(None);
    match catch_unwind(AssertUnwindSafe(|| attachment.conclude(invocation))) {
        Ok(conclusion) => conclusion,
        Err(payload) => {
            let origin = taken_origin();
            TrialConclusion::Refused(panic_finding(payload.as_ref(), origin.as_ref()))
        }
    }
}

/// The typed finding one caught panic becomes.
///
/// The finding's own location is this boundary, which is where the refusal was
/// raised. The subject's origin rides the foreign text instead of the location
/// seat: a location a hook observes is borrowed for that call, and the record
/// vocabulary's location seat is over text that lives for the program.
fn panic_finding(payload: &(dyn Any + Send), origin: Option<&PanicOrigin>) -> TrialFinding {
    TrialFinding::established(
        FailureClass::SubjectPanic,
        SUBJECT_PANIC_CAUSE,
        FindingLocation::at(file!(), line!()),
        foreign_material(origin, payload_text(payload)),
    )
}

/// The panic payload, in the two shapes a payload is safely readable in.
///
/// A payload of any other type is read as absent rather than rendered through a
/// road that would have to guess at its bytes.
fn payload_text(payload: &(dyn Any + Send)) -> Option<&str> {
    payload
        .downcast_ref::<&str>()
        .copied()
        .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
}

/// The foreign text one caught panic carries in.
///
/// The origin and the payload are outside material both, so they ride one
/// bounded foreign-text field. Nothing in the harness reads it back: the facts
/// a reading needs are the typed class and the typed cause.
fn foreign_material(origin: Option<&PanicOrigin>, payload: Option<&str>) -> Option<ForeignText> {
    let material = match (origin, payload) {
        (Some(place), Some(text)) => format!("{}:{}: {text}", place.file, place.line),
        (Some(place), None) => format!("{}:{}", place.file, place.line),
        (None, Some(text)) => text.to_owned(),
        (None, None) => return None,
    };
    Some(ForeignText::admitted(material.as_bytes()))
}

/// Install this home's hook, once for the process.
fn install_capture_hook() {
    let _installed = CAPTURE_HOOK.get_or_init(chain_capture_hook);
}

/// Take the standing hook and set one that captures the origin before
/// delegating to it.
fn chain_capture_hook() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        store_origin(info.location().map(observed));
        previous(info);
    }));
}

/// One observed location, copied into an owned origin.
fn observed(location: &Location<'_>) -> PanicOrigin {
    PanicOrigin {
        file: location.file().to_owned(),
        line: location.line(),
    }
}

/// Record what the hook observed on this thread.
///
/// A thread tearing down has no slot left to write to, and the finding then
/// carries its payload without an origin.
fn store_origin(origin: Option<PanicOrigin>) {
    let _stored = PANIC_ORIGIN.try_with(|slot| slot.set(origin));
}

/// Take the origin this thread's hook last observed, leaving the slot empty.
fn taken_origin() -> Option<PanicOrigin> {
    PANIC_ORIGIN.try_with(Cell::take).unwrap_or(None)
}
