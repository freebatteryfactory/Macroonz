# Native coverage

This home supplies bounded native execution to the existing [coverage owner](../../harness/src/fuzz/README.md).
The caller supplies an instrumented target, a coverage request, an explicit `ProcessTool` and separate target limits.
The selected compiler must agree with the tool policy.
Readiness queries, both LLVM version queries, target execution, profile merge and coverage export all use [`native_process`](../native_process/README.md).
The harness retains source identity, tool matching, candidate standing, novelty, budgets and refusal meaning.
`NativeCoverage::ready` exposes the [coverage owner's target selection and established tool facts](../../harness/src/fuzz/README.md#claim) without another discovery or version-matching layer.

## Execution and cleanup

Tool invocations reuse the supplied working directory and complete explicit environment.
The target adds its case-owned `LLVM_PROFILE_FILE`; a conflicting caller entry refuses rather than overriding the case path.
The coverage-export ceiling and the native capture ceiling both apply.
Target deadlines and output exhaustion become the existing target execution classes, which cannot earn novelty.
An interrupted query, merge or export is infrastructure failure and supplies no coverage observation.

`NativeCoverageFailure` retains the exact failed request, actual process result and any case-directory cleanup.
Pending process cleanup keeps its case files in place.
`finish_cleanup` retries the process owner before removing the case directory, retaining removal failure for another attempt.
It completes cleanup of a failed observation; it does not resume that observation or refund an attempted candidate.
After cleanup, a caller may execute another attempt within the declared campaign budget.
Dropping an unfinished failure does not claim completed cleanup or remove its case files.
Its display names the failed phase and cleanup state without dumping captured output or the environment; the typed cause retains that detail.

## Caller road

The [native coverage example](../../examples/coverage_workflow/README.md) compiles an instrumented Rust subject and crosses declared candidates into retained corpus and fresh target-process replay.
`CompilerRequest::instrumented` in the [compiler owner](../native_compiler/README.md) also selects Cargo instrumentation for the target's Rust dependencies.
`CompilerRequest::instrumented_target` selects only the final Cargo target when dependency coverage is outside the caller's question.
Declare every physical source root that LLVM may export, including dependency sources, and give the complete export an explicit budget.
Zero-count source records still owe an unambiguous mapping.

## Limits

The selected compiler, target and source files remain caller trust inputs.
An explicit coverage target must match the caller's actual compilation selection; readiness does not establish cross-target execution support.
Matching tool versions and a successful profile do not authenticate an artifact against concurrent replacement.
Stable line coverage can map multiple generated operations to one macro expansion location, including a registry facade location.
Different decoder outcomes can therefore have identical coverage points; caller-authored semantic checks remain necessary.
The native process owner's supported mechanisms and cleanup limits apply to every child.
The ordinary harness entrances retain their declared executor boundary; enabling native-tooling selects this concrete enforcement path through the root library.
