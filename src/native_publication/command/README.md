# Publication commands

This home composes a caller-registered generation operation with publication preparation, staged compilation and destination custody.
Call `bake` with an explicit `BakeCommand` and a closure that produces the caller's `Publication` from its declared inputs.
Registration is the supplied Rust operation; there is no ambient registry, source discovery, dynamic plugin loading or inferred application grammar.
The callback runs once for prepare, inspect, check and generate, and does not run during recovery.
Its typed error remains a declaration error distinct from native infrastructure observations.

`Prepare` returns the sealed canonical inventory without native effects.
`Inspect` produces optional formatted physical bytes, retaining their original canonical commitments and formatter observations.
`Check` prepares the same expected bytes and returns the destination owner's complete read-only comparison; a comparison containing discrepancies is a completed check, never a current-output claim.
`Generate` prepares those bytes, stages the complete explicit authored/generated input union, relocates the admitted compiler request, compiles, preflights and installs through the existing destination owner.
`Recover` completes historical installation intent and makes no fresh-compilation claim.

Preparation requires an explicitly selected disposable workspace and complete output bounds.
The workspace must be outside the checked or installed destination and separate from any authored source authority.
Before workspace writes, the command resolves the declared roots and refuses a workspace equal to or below the destination, including an alias through a link.
Generation also requires the compiler output root, direct link artifact and selected dependency file to be disjoint from the destination tree, resolving existing path prefixes without creating missing output directories.
An output root enclosing the destination refuses because it grants the compiler writes below that root.
This admission does not authenticate roots against concurrent replacement by unrelated software.
An exclusive storage lease coordinates use of the workspace and its reserved formatter input file; callers retain coordination responsibility for separately selected compiler outputs.
Every invocation reconstructs its sources from declared input; exactly compared staged-source caching only avoids rewriting identical disposable source bytes.
It never substitutes for the required compiler invocation or upgrades Cargo freshness.

The tool budget bounds the selected process count, the sum of declared execution/cleanup allowances and the sum of stdout/stderr retention bounds for the entire reached tool family.
Admission includes the formatter version query, every selected file format and the selected compiler before any process starts.
These are aggregate native-process allowances, not a deadline on caller Rust code, filesystem operations or operating-system startup.
The native process owner retains its actual deadline, capture and cleanup guarantees.

Errors preserve their owner-specific cause and any pending query, formatter or staged compiler resources.
While a pending error is retained, it also holds the exclusive workspace lease.
`finish_cleanup` keeps that lease if cleanup remains pending and releases it after finished cleanup.
Dropping the error relinquishes workspace custody under the [process owner's drop limits](../../native_process/README.md#execution-and-cleanup); it does not establish finished cleanup.
Cleanup does not resume generation, install output or convert a failed command into a success.
The caller may inspect the retained observations and retry the command after cleanup; source and destination changes still require their owners' ordinary admission.

Application command-line or stdin handling is a thin adapter over these same operations.
The library reads no arguments or inherited environment, scans no authored tree and adds no installer or supervisor to the adopter.
