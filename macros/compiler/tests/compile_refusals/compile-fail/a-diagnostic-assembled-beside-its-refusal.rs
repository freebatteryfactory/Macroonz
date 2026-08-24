//! A diagnostic is what a refusal was projected into, and `Diagnostic::refused` is the one road there.
//!
//! Every seat a diagnostic owes — the phase, the site, the summary, the expected contract, the observed classification, the related set, the repairs, the route — is composed on that road out of the refusal, the door, and the placement.
//! A diagnostic assembled field by field would be a complaint wearing the shape of an answer: a summary naming one position beside a site holding another, over a related set nobody derived.
//!
//! No value is constructed below; the struct expression alone is the proof.

use macroonz::{Diagnostic, Phase};

fn main() {
    let _minted = Diagnostic {
        phase: Phase::Planning,
    };
}
