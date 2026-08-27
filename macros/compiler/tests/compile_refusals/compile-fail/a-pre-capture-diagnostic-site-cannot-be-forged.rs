//! A pre-capture diagnostic site cannot be forged beside its refusal.
//!
//! The text refusal itself owns its byte-role coordinate and exposes only its typed diagnostic road, so it does not implement the public caller-placed refusal bound.

use macroonz_compiler::{
    CrateBinding, Diagnostic, Door, Placement, Producer, TextReadCause, TextReadRefusal,
};

const DOOR: Door = Door::declared(
    "compile refusal",
    "compile.refusal.pre.capture",
    "compile_refusal::pre_capture",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "compile_refusal",
        name: "pre_capture",
    },
);

fn main() {
    let refusal = TextReadRefusal {
        cause: TextReadCause::NotBalanced,
        at: 99,
    };
    let _ = Diagnostic::refused(&refusal, &DOOR, &Placement::WholeDeclaration);
}
