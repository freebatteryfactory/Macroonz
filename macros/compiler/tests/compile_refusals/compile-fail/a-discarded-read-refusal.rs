//! A refusal that was established and then discarded is a check that did not happen.
//!
//! Every value carrying an outcome is marked with a semantic `#[must_use]` whose message says what the value carries.
//! The attribute is diagnostics, so its bite is a denied warning, and this fixture is where that bite is executed rather than asserted.
//!
//! trybuild compiles each fixture as its own generated crate and that manifest carries no lint table, so the `deny` below restores exactly the wall the workspace already declares and grants nothing it does not.
//!
//! The two discards are not equivalent, and the difference is this reversal's honest scope: the expression statement is what `unused_must_use` reads and what fails this build, while the explicit `_` binding is a use rustc does not lint and only clippy's own lint sees, over the real tree, on a different gate.

#![deny(unused_must_use)]

use macroonz::{TextReadCause, TextReadRefusal};

/// The refusal the callable text route hands back for a text it could not read.
fn refused() -> TextReadRefusal {
    TextReadRefusal {
        cause: TextReadCause::NotTerminated,
        at: 0,
    }
}

fn main() {
    // Invisible to rustc: an explicit `_` binding is a use.
    let _ = refused();

    // The discard the attribute refuses.
    refused();
}
