//! The facade observed from an ordinary outside holder: compiler, procedural declaration, and harness remain under their owning modules.

use core::mem::size_of;

/// The compiler remains reachable without flattening its types into the facade root.
#[test]
fn compiler_stays_under_its_owner() {
    assert_eq!(size_of::<macroonz::compiler::NoQuestions>(), 0usize);
    assert_eq!(
        size_of::<macroonz::compiler::codec::CodecProjection>(),
        0usize
    );
    assert_eq!(macroonz::compiler::codec::MEMBER_CONTRACT.len(), 5usize);
}
