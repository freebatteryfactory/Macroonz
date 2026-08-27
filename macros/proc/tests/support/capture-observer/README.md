# capture observer

This non-published qualification workspace witnesses the actual compiler-token producer without adding an entry to `macroonz-macros`, its package artifact, or the four-package product workspace.

Its proc-macro library performs the compiler crossing, and its external test compares that result with the callable text producer at their shared normalized boundary.

Its only output is one string literal containing the lowercase hexadecimal canonical bytes of the [`CapturedInput`](macroonz_compiler::CapturedInput) made by the real host capture.
A typed host refusal is also rendered inside one string literal so the witness has no panic branch and a lawful parity case cannot mistake refusal text for hexadecimal bytes.

It owns no grammar, interpretation, identity rule, or product observation channel.
