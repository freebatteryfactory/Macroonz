# Caller-admitted types

Run `cargo run --example admitted_types --no-default-features` from the repository root.
The generator writes definitions and their declared invocations to standard output.
Its root [pattern composition](../../src/pattern/README.md) owns the invocation grammar and generated operations.
This example declares a nonzero integer, normalized text and a borrowed generic array; their admission functions remain ordinary caller code.

The [standalone consumer](consumer.rs) loads that generated source through `mod definitions;` from `definitions.rs` beside itself.
Save the generator's standard output with that name in a disposable directory, copy the consumer there, then compile it with `rustc --edition=2024 consumer.rs` and run the resulting binary.
The consumer requires no Macroonz dependency because it consumes ordinary generated Rust.
The generated macro definitions belong in an ordinary source module; Rust refuses same-crate absolute calls to exported macros introduced through `include!` expansion.
The repository's outside-consumer lane recreates these files from the example and executes this exact consumer alongside independently authored behavior and compile-refusal controls.

`admit_label` owns whitespace normalization, and `admit_batch` owns the nonempty-array rule.
The example's caller supplies an explicit little-endian encoding through shared readback; the pattern supplies no wire format.
The borrowed form keeps its lifetime, element bound and const length in ordinary Rust.

The generator only renders source material.
It does not mint a publication record, compile output, install files or report that caller validation is correct.
The stamp owner's [publication example](../../macros/compiler/src/stamp/README.md#runnable-publication) supplies the addressed and sealed publication road for a caller that needs it.
