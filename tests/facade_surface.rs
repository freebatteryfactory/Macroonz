//! The facade observed from an ordinary outside holder: compiler, procedural declaration, and harness remain under their owning modules.

use core::mem::size_of;

#[cfg(feature = "harness")]
macroonz::macros::network! {
    harness = macroonz::harness,
    module = facade_net,
    namespace = "facade",
    nodes = [client, server],
    link request = client to server,
    schedule quiet = [],
}

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

/// A facade-qualified proc declaration may name the facade-qualified harness and produce ordinary working cargo.
#[cfg(feature = "harness")]
#[test]
fn macros_and_harness_compose_through_the_facade() -> Result<(), facade_net::Fault> {
    let topology = facade_net::topology()?;
    assert_eq!(topology.nodes().len(), 2usize);
    assert_eq!(topology.links().len(), 1usize);
    assert!(facade_net::quiet()?.disciplines().is_empty());
    Ok(())
}
