# Compiler source dependencies

This home captures a bounded source-dependency rule from an explicitly selected compiler output file.
`CompilerRequest::with_dependencies` requests rustc link and dependency emission together, or selects the existing Cargo dependency file beside its final artifact.
Cargo's dependency path is supplied explicitly; no build-directory scan or internal-cache layout discovery is performed.

Capture runs after completed native cleanup and successful selected compilation.
It retains the exact dependency bytes and requires one rule naming an artifact actually reported by that compiler invocation.
The [documented Cargo dependency format](https://doc.rust-lang.org/cargo/reference/build-cache.html#dep-info-files) supplies Makefile-like rules for the artifact's input files.
The reader accepts one whole target spelling per rule, including literal or backslash-escaped spaces, and source paths with literal native separators and backslash-escaped spaces.
Phony empty rules and comment lines confer no source membership.
Repeated selected rules, duplicate sources, control characters, incomplete text, ambiguous paths and exceeded bounds refuse.
Relative source paths use the same declared working-directory basis as compiler diagnostics.

This records source-file dependency membership, not semantic coverage of every conditional branch, execution, source-byte authenticity or a guarantee that every supplied file was compiled.
A consumer must compare its required source roster with the observed paths and retain its own before/after source custody.
The original compiler result remains independently available when dependency capture refuses.
Tools, project configuration and output files remain caller trust inputs; capture does not authenticate concurrent replacement or override Cargo's reported artifact freshness.
