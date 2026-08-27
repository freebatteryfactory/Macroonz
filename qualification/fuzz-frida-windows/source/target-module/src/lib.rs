#![deny(missing_docs)]

//! The separately linked compiler target observed by the native Windows F0 pilot.

use macroonz_compiler::{TextCapture, TextReadCause};

/// The public target outcome retained by the qualification driver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureOutcome {
    /// The candidate bytes were not UTF-8 and therefore never entered `TextCapture`.
    NotUtf8,
    /// The candidate was read into one normalized captured input.
    Read {
        /// Canonical bytes in the normalized capture.
        canonical_bytes: usize,
        /// Top-level token trees in the normalized capture.
        top_level_trees: usize,
    },
    /// The text reader established a typed refusal.
    Refused {
        /// The established refusal cause.
        cause: TextReadCause,
        /// The source byte at which the cause was established.
        at: u64,
    },
}

/// Observe one exact candidate through the compiler's public text-capture road.
#[must_use]
pub fn observe(candidate: &[u8]) -> CaptureOutcome {
    let Ok(source) = core::str::from_utf8(candidate) else {
        return CaptureOutcome::NotUtf8;
    };
    match TextCapture::read(source) {
        Ok(capture) => CaptureOutcome::Read {
            canonical_bytes: capture.input().canonical_bytes().len(),
            top_level_trees: capture.input().trees().len(),
        },
        Err(refusal) => CaptureOutcome::Refused {
            cause: refusal.cause,
            at: refusal.at,
        },
    }
}

