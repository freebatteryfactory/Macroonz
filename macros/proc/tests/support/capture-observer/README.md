# capture observer

This non-published repository-test proc-macro package exists because Rust requires a real proc-macro compilation boundary to witness the actual compiler-token producer.
It does not enter `macroonz-macros`, its package artifact, or the four-package product workspace.

Its proc-macro library performs the compiler crossing, and its external test compares that result with the callable text producer at their shared normalized boundary.

Its only output is one string literal containing the lowercase hexadecimal canonical bytes of the [`CapturedInput`](macroonz_compiler::CapturedInput) made by the real host capture.
A typed host refusal is also rendered inside one string literal so the witness has no panic branch and a lawful parity case cannot mistake refusal text for hexadecimal bytes.

It owns no grammar, interpretation, identity rule, or product observation channel.
Publishing it would add no consumer capability because its only question is whether the real proc-macro host capture agrees with the callable text producer.
