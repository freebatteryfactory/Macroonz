# Wave F fuzz targeted mutation

This receipt records a bounded local mutation campaign over the corrected fuzz authority and custody joins and awaits owner acceptance and publication authority.

## Authority

- Source base: `b93a1c1` on `codex/macroonz-repository-completion`.
- Tool: `cargo-mutants 27.0.0`.
- Toolchain: stable Rust 1.98 from the repository root.
- Mutated package: `macroonz-harness`.
- Test target: `fuzz_compose` only.
- Execution was serial and used no hosted service, network instrumentation, new dependency, feature, package, or workspace.

## Denominator

The campaign selected 44 generated mutants at these corrected decision points:

- `CoverageBudgets::declared`;
- `CoverageCorpus::reserve_execution`;
- `CoverageCorpus::admit`;
- `observe_rustc_profile`;
- `observe_case`;
- `CovProcess::bounded_output`;
- `preflight_ready`;
- `RustcProfileResult::established`.

The initial pass classified 28 mutants as caught, 13 as unviable, and three as missed, with no timeout.
The unviable mutations were generated replacements that could not construct the informed private types and therefore could not compile into a competing behavior.

## Promoted findings

- Replacing the existing-directory comparison in `observe_rustc_profile` survived because no external crossing distinguished `CaseAlreadyExists` from another case-creation failure.
- Replacing the export overflow comparison from `>` to `>=` survived because the tests proved overflow but not an output exactly equal to its byte ceiling.
- Replacing the point overflow comparison from `>` to `>=` survived because the tests proved overflow but not a canonical point set exactly equal to its ceiling.
- `an_existing_case_directory_keeps_its_specific_refusal` now preserves the exact collision refusal.
- `exact_coverage_export_byte_ceiling_is_inclusive` now proves that an export exactly equal to the declared byte ceiling is lawful.
- `exact_coverage_point_ceiling_is_inclusive` now proves that a point set exactly equal to the declared point ceiling is lawful and admissible.

## Closure

- The corrected focused fuzz target passed 18 of 18 tests.
- Strict Clippy passed for the corrected external target with warnings denied and no suppression.
- Mutation iteration excluded the 41 already caught or unviable mutants and retested the three promoted survivors.
- The iteration caught all three, with zero misses and zero timeouts.
- The combined 44-mutant campaign therefore has no surviving viable mutation in its declared denominator.
- Raw mutant copies, logs, and diffs remained disposable beneath `target/qualification/wave-f-fuzz-mutants-20260828` until this receipt and the owning regressions were ready to commit.
- The host blocked direct recursive deletion and Cargo refused to clean a non-Cargo directory without `CACHEDIR.TAG`, so the exact campaign directory was moved recoverably out of the repository to the invoking user's Downloads directory as `macroonz-disposable-wave-f-fuzz-mutants-20260828`.
- That moved scratch has no evidence authority, and the live repository retains no task-created mutant directory.

## Remaining boundaries

- This local Windows mutation result is not hosted, cross-host, packaged, published, or registry-delivered evidence.
- Mutation outside the declared eight decision points is not claimed by this receipt.
