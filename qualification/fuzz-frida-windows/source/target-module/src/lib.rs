#![deny(missing_docs)]

//! The separately linked compiler target observed by the native Windows Frida road.

use macroonz_compiler::{
    CapturedAtom, LiteralReadCause, TextCapture, TextReadCause, capture_literal,
};

/// The public text-capture outcome retained by the qualification driver.
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

/// The public literal-capture outcome for the second self-fuzz surface.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralOutcome {
    /// The candidate bytes were not UTF-8.
    NotUtf8,
    /// A known literal form produced an atom.
    Atom {
        /// Discriminant name of the captured atom.
        kind: &'static str,
    },
    /// The literal reader established a typed refusal.
    Refused {
        /// The established refusal cause.
        cause: LiteralReadCause,
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

/// Observe one exact candidate through the compiler's public literal-capture road.
#[must_use]
pub fn observe_literal(candidate: &[u8]) -> LiteralOutcome {
    let Ok(source) = core::str::from_utf8(candidate) else {
        return LiteralOutcome::NotUtf8;
    };
    match capture_literal(source) {
        Ok(atom) => LiteralOutcome::Atom {
            kind: atom_kind(&atom),
        },
        Err(cause) => LiteralOutcome::Refused { cause },
    }
}

fn atom_kind(atom: &CapturedAtom) -> &'static str {
    match atom {
        CapturedAtom::Word(_) => "word",
        CapturedAtom::Punct(_) => "punct",
        CapturedAtom::Text(_) => "text",
        CapturedAtom::Number(_) => "number",
        CapturedAtom::ByteText(_) => "byte_text",
        CapturedAtom::Character(_) => "character",
        CapturedAtom::Byte(_) => "byte",
        CapturedAtom::NulTerminatedText(_) => "nul_terminated_text",
        CapturedAtom::RawIdentifier(_) => "raw_identifier",
        CapturedAtom::JointPunct(_) => "joint_punct",
    }
}
