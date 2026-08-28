//! A diagnostic name cannot bypass kebab-case admission.
//!
//! The tuple field is private, so an external caller cannot construct a declared diagnostic name without the typed guard.

use macroonz_compiler::diagnostic::DiagnosticName;

fn main() {
    let _ = DiagnosticName("Not-Kebab");
}
