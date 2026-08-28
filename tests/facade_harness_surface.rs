//! Harness-enabled facade composition observed from an ordinary outside holder.

macroonz::macros::network! {
    harness = macroonz::harness,
    module = facade_net,
    namespace = "facade",
    nodes = [client, server],
    link request = client to server,
    schedule quiet = [],
}

/// A facade-qualified proc declaration may name the facade-qualified harness and produce ordinary working cargo.
#[test]
fn macros_and_harness_compose_through_the_facade() -> Result<(), facade_net::Fault> {
    let topology = facade_net::topology()?;
    assert_eq!(topology.nodes().len(), 2usize);
    assert_eq!(topology.links().len(), 1usize);
    assert!(facade_net::quiet()?.disciplines().is_empty());
    Ok(())
}
