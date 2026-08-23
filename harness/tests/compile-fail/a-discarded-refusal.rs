//! A refusal that was established and then discarded is a check that did not
//! happen.
//!
//! Every value that carries an outcome — a refusal, a diagnostic, a verdict, a
//! receipt, a decision, a plan, a closure, a proof — is marked with a semantic
//! `#[must_use]` whose message states what the value carries. The attribute is
//! diagnostics, so its bite is a denied warning, and this fixture is where that
//! bite is executed rather than asserted.
//!
//! The value below is the real thing: `compile_refusal_text` is the callable
//! route, the declaration handed to it is one the authored grammar does not
//! admit, and what comes back is the services' own refusal.
//!
//! # What this fixture proves, exactly
//!
//! It proves that for one representative marked refusal, under this toolchain,
//! discarding the value fails the build, and that the message the attribute
//! carries is what the caller is shown. It does NOT claim non-vacuity for every
//! marked type: one fixture exercises one type, and the rest stand on the
//! attribute being the same attribute.
//!
//! # Why the crate-level `deny` is here, and why the two discards differ
//!
//! trybuild compiles each fixture as its own generated crate, and that generated
//! manifest carries no `[lints]` table — so the workspace lint wall does not
//! reach this file, and `unused_must_use` would arrive as a warning nobody fails
//! on. The `deny` below restores exactly the wall the workspace already
//! declares; it grants nothing the wall does not.
//!
//! The two discards below are not equivalent, and the difference is the honest
//! scope of this reversal:
//!
//! - `refused();` — a discarded expression statement. `unused_must_use` is a
//!   rustc lint, it reads the attribute on the type, and it is what fails this
//!   build.
//! - `let _ = refused();` — an explicit discard, and the very repair rustc's own
//!   help text suggests below. Only clippy's `let_underscore_must_use` sees it,
//!   and clippy does not run over trybuild fixtures at all. That lint is denied
//!   on the workspace wall and enforced over the real tree by
//!   `cargo clippy --workspace --all-targets`, which is a different gate than
//!   this one. The line stands here so the split is visible in the source rather
//!   than remembered.

#![deny(unused_must_use)]

use threadpak_macroc::TextCompileRefusal;
use threadpak_macroc::derive_refusal::compile_refusal_text;

/// The refusal the callable text route hands back for a declaration its grammar
/// does not admit.
fn refused() -> TextCompileRefusal {
    compile_refusal_text("struct NotAnEnumAtAll;").unwrap_err()
}

fn main() {
    // Invisible to rustc: an explicit `_` binding is a use, and only clippy's
    // own lint reads it. It compiles here, and it is refused on the wall.
    let _ = refused();

    // The discard the attribute refuses.
    refused();
}
