# Staged compilation

This home compiles a complete prepared publication in a privately created tree with explicitly supplied authored inputs.
`StagingPlan::declared` applies the inventory owner's shared path, file-count and byte rules to the entire union before filesystem access.
Generated and authored files cannot alias or occupy each other's parent paths.
The caller owns the compilation entrypoint, manifests, lockfiles, dependency bindings and other authored bytes; no source scan invents them.

`stage` opens an explicitly supplied existing parent through the storage root capability and exclusively creates `.macroonz-stage-NAME`.
Only declared directories and files are populated, using exclusive file creation and flushing.
The resulting directory capability contains later reads, and the existing storage owner supplies bounded byte reads.
Existing stage names refuse.
A failed population leaves its explicitly named private tree for caller-owned disposable cleanup; it never overwrites or recursively deletes that tree.

`cached` names a disposable tree from the complete sorted path/physical-byte roster, using the compiler's framing and identity owners under the private staging-cache profile.
It creates missing material exclusively and reuses existing material only after the same complete roster, kind and byte comparison.
A changed or incomplete cache refuses rather than being overwritten or trusted.
Canonical publication identity is unchanged, and a cache hit supplies no compiler observation: `compile` still applies every native compilation and dependency requirement.
Removing disposable caches changes reuse only; every required source byte comes from the new explicit plan.

`compile` requires the selected native compiler request to use that source directory, a staged diagnostic locus and any staged Cargo manifest.
Compiler outputs and the selected dependency file must be outside the source tree.
The complete staged roster and exact bytes are compared before execution and after finished cleanup; missing, additional, changed, linked and non-file entries refuse.
Comparison checks both the retained directory capability and the directory currently visible at the compiler's selected source path.
Only declared directory prefixes are traversed during this comparison.

Successful qualification additionally requires actual compilation acceptance and the existing compiler owner's dependency observation.
Every prepared file must occur in the selected artifact's dependency roster.
For Cargo, the selected dependency file must be the `.d` neighbor of the reported artifact.
Cargo freshness retains the compiler owner's distinction between a successful cached build and a newly executed rustc invocation.
Dependency membership does not establish semantic coverage of every conditional branch or infer the meaning of the caller's compilation fixture.
The selected tool, authored fixture and external dependencies remain caller trust inputs; before/after equality does not authenticate concurrent replacement.

Pending compiler cleanup retains the source tree and original compiler context and exposes an explicit retry.
A completed refusal retains its actual compiler result and source context.
`CompiledPublication` retains the original prepared bytes and compiler observation; it grants no destination ownership or installation receipt.
Unix and Windows select private directory operations; other targets refuse staging effects.
