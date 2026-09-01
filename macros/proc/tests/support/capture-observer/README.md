# capture observer

This non-published repository-test proc-macro package exists because Rust requires a real proc-macro compilation boundary to witness the actual compiler-token producer.
It does not enter `macroonz-macros`, its package artifact, or the four-package product workspace.

Its proc-macro library performs the compiler crossing, and its external test compares that result with the callable text producer at their shared normalized boundary.

Its canonical observer outputs one string literal containing the lowercase hexadecimal canonical bytes of the [`CapturedInput`](macroonz_compiler::CapturedInput) made by the real host capture.
Its round-trip observer projects that same captured fragment through Macroonz's generated-token vocabulary and the real compiler-token emitter.
Typed capture, fragment-generation, and host-emission refusals become `compile_error!` output, so the witness has no panic branch.

It owns no grammar, interpretation, identity rule, or product observation channel.
Publishing it would add no consumer capability because its only questions are whether the real proc-macro host capture agrees with the callable text producer and whether preserved tokens return through the real proc-token boundary.
