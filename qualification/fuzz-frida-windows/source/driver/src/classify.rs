//! Typed execution-class labels for the F0 TextCapture pilot.

use libafl::executors::ExitKind;
use macroonz_f0_target::CaptureOutcome;

/// One Fuzz-acceptance class from the completion program's F0 denominator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ExecutionClass {
    /// `TextCapture` admitted a normalized input.
    LawfulSuccess,
    /// `TextCapture` established a typed refusal.
    TypedRefusal,
    /// Bytes were not UTF-8 and never entered `TextCapture`.
    NotUtf8,
    /// The executor reported a timeout.
    Timeout,
    /// The executor reported a crash or abort.
    Crash,
    /// The harness returned an exit kind this pilot does not map further.
    AmbiguousPartial,
}

impl ExecutionClass {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::LawfulSuccess => "lawful-success",
            Self::TypedRefusal => "typed-refusal",
            Self::NotUtf8 => "not-utf8",
            Self::Timeout => "timeout",
            Self::Crash => "crash",
            Self::AmbiguousPartial => "ambiguous-partial-acceptance",
        }
    }
}

/// Classify one observed target outcome under one executor exit kind.
#[must_use]
pub(crate) fn classify(outcome: CaptureOutcome, exit: ExitKind) -> ExecutionClass {
    match exit {
        ExitKind::Timeout => ExecutionClass::Timeout,
        ExitKind::Crash | ExitKind::Oom => ExecutionClass::Crash,
        ExitKind::Ok => match outcome {
            CaptureOutcome::Read { .. } => ExecutionClass::LawfulSuccess,
            CaptureOutcome::Refused { .. } => ExecutionClass::TypedRefusal,
            CaptureOutcome::NotUtf8 => ExecutionClass::NotUtf8,
        },
        _ => ExecutionClass::AmbiguousPartial,
    }
}

/// Stable label for a `CaptureOutcome` retained in handoff material.
#[must_use]
pub(crate) fn outcome_label(outcome: CaptureOutcome) -> &'static str {
    match outcome {
        CaptureOutcome::Read { .. } => "read",
        CaptureOutcome::Refused { .. } => "refused",
        CaptureOutcome::NotUtf8 => "not-utf8",
    }
}
