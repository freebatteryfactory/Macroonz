# Compiler-core refusal

This fixture observes the absence of the optional proc-host module from the compiler core.
The parent driver requires refusal when the compiler's `host` feature is disabled and successful compilation of the same source when it is enabled.
