//! The concurrency declaration's generated crossing, observed through the actual proc entry and harness surface.

macroonz_macros::concurrency! {
    namespace = "concurrency-projection",
    harness = mh,
    sampled {
        seed = 19,
        population = "sampled-orders",
        samples = 3,
        interleavings = 1,
    },
    module = generated,
    exhaustive {
        samples = 3,
        interleavings = 2,
        population = "all-orders",
        seed = 7,
    },
}

macroonz_macros::concurrency! {
    harness = mh,
    module = refused_bounds,
    namespace = "concurrency-projection",
    no_interleavings { population = "orders", interleavings = 0, samples = 1, seed = 1 },
    no_samples { population = "orders", interleavings = 1, samples = 0, seed = 1 },
}

macroonz_macros::concurrency! {
    harness = mh,
    module = refused_namespace,
    namespace = "",
    row { population = "orders", interleavings = 1, samples = 1, seed = 1 },
}

macroonz_macros::concurrency! {
    harness = mh,
    module = refused_population,
    namespace = "concurrency-projection",
    row { population = "", interleavings = 1, samples = 1, seed = 1 },
}

mod generated_explorations;
mod harness_refusals;
mod identity_content;
mod support;
