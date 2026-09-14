# Native mutation execution

This home connects explicitly selected cargo-mutants execution to the existing [backend reader and custody owner](../../harness/src/muterprater/backend/README.md).
The root native-tooling feature supplies the entrance.
The caller supplies the absolute backend, Cargo and rustc executables, a complete environment, the project working directory, declared source files, a source-byte bound, output and target directories, and the selected target triple.
The backend executable receives its required leading mutants argument directly; no shell or executable discovery is introduced.

## Execution

`MutationRequest` binds Cargo and rustc through explicit CARGO and RUSTC environment entries, refusing conflicting entries.
Every version query and backend execution uses the existing [native process owner](../native_process/README.md).
Version queries initially share the selected process limits; `version_queries` can select a separate explicit budget before execution.
The selected profile requires cargo-mutants 27.0.0 and the existing Rust 1.98.1 coverage baseline.
The compiler identity comes from the selected rustc executable's version output; the target argument is passed to every backend Cargo build and test.

The command runs the unmutated baseline, requests every outcome class, disables backend configuration-file loading, retains source order, and uses one backend job and one Cargo job.
Exact path filters accompany the backend's filename globs so a nested file with the same basename cannot widen the mutation roster.
Paths containing whitespace or control characters refuse because this console grammar cannot represent those coordinates faithfully.
Cargo builds use locked, offline resolution and a caller-selected target directory.
Offline resolution is not network isolation for code executed by Cargo.
The backend uses its ordinary source-copy mode, does not copy target or version-control directories, and does not relax the source's lint policy.
Its temporary source copies are backend-owned disposable work, never an implementation checkout.
Use a target directory reserved for mutation output and clean it before qualifying unchanged source against its artifacts.

Declared files are read within one canonical root with an aggregate byte bound before the mutation process starts and again after it finishes.
Files outside that root and non-regular files refuse.
An existing output directory refuses before execution, preventing the backend from rotating or deleting an earlier campaign there.
Output remains disposable and is not automatically removed by this home.
An interrupted operation retains native cleanup ownership; finishing cleanup does not turn a deadline or truncated output into a successful observation.

## Observation and retention

The immutable result retains its request, version-query outputs, original source bytes and actual process result.
Complete stdout followed by complete stderr forms the explicitly ordered console document read by the existing wrap grammar.
Both streams retain their original bytes separately; this ordering is not a claim about interleaving in time.
Non-UTF-8 or incomplete streams and an incomplete announced mutant population refuse observation.
Backend exits for missed mutants or backend test timeouts remain eligible for their owning mutation readings; infrastructure exits do not.

The manifest's source roster remains exactly the files named by parsed mutation reports.
The complete declared input roster is independently compared before and after execution, including declared files that produced no report.
Archive retention uses the existing backend archive with original console and source material.
Current-source comparison rereads the manifest's exact source roster through this home's bounded reader and delegates identity comparison to `CompiledSuiteArtifactCustody`.
No archive format, mutation grammar, verdict, clock or supervisor is duplicated here.

## Limits

The selected tools and Cargo project are caller trust inputs.
Before/after source equality does not authenticate a workspace against concurrent replacement or prove that undeclared dependencies, configuration or test files stayed fixed.
The backend may acquire the project material it needs through Cargo and its source-copy mechanism; this library reads only declared source files.
Native process deadlines and cleanup retain their documented operating-system limits.
The console reader establishes witness rejection, never observed activation or a semantic survivor.
Loading historical bytes does not create a native execution result or human acceptance.
