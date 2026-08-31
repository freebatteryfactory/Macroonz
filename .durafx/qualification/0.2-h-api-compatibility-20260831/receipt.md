# Macroonz 0.2 Wave H local API-compatibility receipt

This Git-tracked receipt accepts the local Windows portion of Wave H against the exact published 0.1.0 packages and the clean current package candidates.

## Authority

- Candidate repository HEAD: `73ffa61b8555290deebee2d463f207ba10651036`.
- Published baseline tag: `v0.1.0` at `446b8a75ad1967df060cb89e4c1b4099b8b3526b`.
- Host plane: local Microsoft Windows on `x86_64-pc-windows-msvc`.
- Rust compiler: `rustc 1.98.0 (88d9e12ae 2026-08-18)` with LLVM `22.1.8`.
- Cargo: `cargo 1.98.0 (797e8a9bc 2026-08-05)`.
- Analyzer: task-local `cargo-semver-checks 0.50.0` with executable SHA-256 `69D6A38347DB5E2C0AD8BCF50DB518DF6DF76D51B26750ABE2F77C3D408065D6`.
- Analyzer scratch: `target/qualification/0.2-wave-h-semver-pilot-20260831`.
- Analyzer installation scratch: `target/qualification/0.2-wave-h-semver-tool-20260830`.
- The host-global analyzer remained unchanged at its pre-existing version.
- No nightly command, `RUSTC_BOOTSTRAP`, product edit, dependency edit, feature edit, tracked test edit, global tool installation, or new compatibility subsystem entered this observation.

## Exact package denominator

| Package | Published 0.1.0 bytes | Published SHA-256 | Candidate bytes | Candidate entries | Candidate SHA-256 |
| --- | ---: | --- | ---: | ---: | --- |
| `macroonz` | 128,355 | `DBE80FF4EB844508DC9FF69A4938729C6CAD98F145C895D9A7D14FF57FB255FC` | 128,357 | 20 | `B3E5BDF0490E11A580961A6C5438ABB27D3B9A1305E9536A95F4F6284E5A5B6D` |
| `macroonz-compiler` | 393,098 | `6896202A7ECF4F4A8017029C364D707D21C6D02AFCB29A42322BE655D37C0D74` | 408,178 | 312 | `5C4D7A7BC70E19B547ACD1857D38CE8646A93B9EF3BAFBAAF57CD9D0F114D070` |
| `macroonz-harness` | 441,544 | `D4047EA57BAD927E8CBA4913F6A54DD9DF095114F3E3723A62EAFD2C7D919BAA` | 460,813 | 377 | `6CB879B11168691CA44E74BD590790060D039040A2B2CD08BEFAFC637BC0157B` |
| `macroonz-macros` | 20,825 | `61950E4BDF0BBD46211413B7A461A0BD24F2F2381E150DCB8C960D4E1DB7BEF7` | 20,824 | 32 | `4731975F381C8339B4B8343F7DC2EDC0B1A708244B185F4BED4FECC09B2204AF` |

- Every published archive was retrieved afresh from crates.io and reproduced its accepted Q0 hash and byte count.
- Every candidate archive was produced serially with stable Rust 1.98 and `cargo package --locked` from the clean candidate HEAD without `--allow-dirty`.
- Every candidate `.cargo_vcs_info.json` names the candidate HEAD and carries no dirty key.
- Every baseline and candidate normalized `Cargo.toml.orig` is byte-identical for its corresponding package, so the feature, dependency, target, package-metadata, version, and MSRV declarations did not move in this candidate.
- The facade package remains at 20 entries, with the inert nested documentation entry `.github/README.md` replaced by `.github/workflows/README.md` under the existing unanchored `README.md` include pattern.
- The compiler package adds exactly eight source, example, and external-test entries.
- The harness package adds exactly eleven external compile-refusal and mutation-custody entries.
- The proc-macro package retains its exact 32-entry roster.

## Symmetric analyzer topology

- An initial analyzer attempt refused because packages extracted beneath the repository target were captured by the repository workspace.
- Baseline and candidate extracted manifests therefore received the same empty `[workspace]` table before analysis.
- The eight manifest diffs add only that table and change no package, dependency, feature, target, profile, or lint stanza.
- The original `.crate` archives remain the package-byte authority, and the annotated extractions are analyzer inputs rather than package-identity claims.
- The analyzer used package-local dependency resolution and reached the crates.io index, so this receipt makes no fully offline analyzer claim.

## Analyzer matrix

Every completed row used the default v0.50.0 lint policy, explicit baseline root, explicit current manifest, explicit `--release-type minor`, and target `x86_64-pc-windows-msvc`.
Every successful row reported `196 checks: 196 pass, 58 skip` and required no SemVer update.
The 58 skipped checks remain explicit non-observations.

| Package | Feature posture | Result class | Exit |
| --- | --- | --- | ---: |
| `macroonz-compiler` | default with `host` off | completed with no deny-level break | 0 |
| `macroonz-compiler` | all features with `host` on | completed with no deny-level break | 0 |
| `macroonz-harness` | default with `preemption` off | completed with no deny-level break | 0 |
| `macroonz-harness` | all features with `preemption` on | completed with no deny-level break | 0 |
| `macroonz` | no explicit features | completed with no deny-level break | 0 |
| `macroonz` | explicit `harness` | completed with no deny-level break | 0 |
| `macroonz` | default `full` | completed with no deny-level break | 0 |
| `macroonz` | all features, equivalent to `full` | completed with no deny-level break | 0 |
| `macroonz-macros` | featureless proc-macro target | analyzer could not complete | 101 |

- The proc-macro refusal is exact: no ordinary library target was selected, and `macroonz-macros` was skipped because it has no ordinary library target.
- Parent independently reran all nine rows and reproduced the same eight green classes and the same proc-macro refusal class.

## Public-path and package-source census

- The root facade manifest and `src/lib.rs` are unchanged from the published baseline.
- The proc-macro manifest and `src/lib.rs` are unchanged from the published baseline.
- The compiler manifest is unchanged and its additive public source consists of checked keyed rosters, exact keyed assignments, their structural refusals, and two conventional borrowed-slice projectors.
- The additive compiler export roster is `KeyedRoster`, `KeyedRosterAssignment`, `KeyedRosterError`, `KeyedRosterAssignmentError`, `DuplicateKey`, `ForeignRosterReference`, `UnassignedRosterMember`, `keyed_roster_slice`, and `keyed_assignment_slice`.
- The keyed types expose only documented informing constructors, retained-order readers, key lookups, and exact assignment readers over their private structural fields.
- The harness manifest is unchanged and its additive public source consists of exact compiler-diagnostic anchors, exact compilation declarations and observations, exact disagreements and verdicts, `compared_compilation`, and `CompiledVerdict::concluded`.
- The additive harness export roster is `RustcErrorCode`, `RustcErrorCodeRefusal`, `RelativeSourcePath`, `RelativeSourcePathRefusal`, `SourcePosition`, `SourcePositionRefusal`, `PrimarySourceSpan`, `PrimarySourceSpanRefusal`, `DiagnosticAnchor`, `DeclaredCompilation`, `ObservedCompilation`, `CompilationDisagreement`, `CompilationVerdict`, and `compared_compilation`.
- No published public path was removed, renamed, narrowed, or moved in the handwritten source census.
- The analyzer independently found no deny-level public API break across every supported package and feature posture.

## Package-shaped generated and facade crossings

- One downstream source invokes the public `network!` proc macro through a renamed `bakery` facade dependency and names the harness through the same facade.
- The exact published 0.1.0 packages compile that source under minimal, harness, and full facade postures on native Windows and `wasm32-unknown-unknown`.
- The exact current package candidates compile the same source under the same three postures and two targets.
- All twelve baseline/current posture-target rows used scratch-local locks and locked, offline stable Rust 1.98 checks.
- The minimal row separately observes the renamed facade's compiler-owner path without requiring the harness feature.
- The harness and full rows observe the renamed proc-macro path, facade-qualified harness path, generated topology function, generated fault-schedule function, and generated fault type.
- This crossing establishes the exact retained `network!` consumer shape and does not claim every possible generated declaration.

## Planted controls

### Handwritten breaking control

- The scratch candidate changes only `Bounded::try_push` from `pub` to `pub(crate)`.
- The analyzer exits 100 under lint `inherent_method_missing`.
- The finding names `Bounded::try_push` at baseline `src/bounded/type_guard.rs:71`.
- The analyzer requires a new major version with one major and zero minor checks failed.

### Additive control

- The scratch candidate adds only one documented public `WAVE_H_ADDITIVE_CONTROL: u8` constant at the compiler root.
- The analyzer exits 0 and requires no SemVer update.

### Proc-macro and generated-API gap control

- The scratch proc package changes only the public entry name `network` to `network_removed_control`.
- The analyzer still exits 101 before comparison because it cannot represent the proc-macro target.
- The current package-shaped consumer passes unchanged.
- The otherwise identical consumer against the renamed proc candidate refuses with primary `E0433` at `tests/facade_harness_surface.rs:3:19` because `network` is absent from `macros`.
- Three later unresolved generated-module errors are consequences of that missing entry rather than independent findings.
- The gap is therefore explicit and crossed by an existing ordinary downstream compile witness rather than a new product analyzer.

## Analyzer and dependency-policy ceilings

- The task-local locked analyzer installation warned that its upstream lock selected yanked `chacha20 0.10.1`.
- The analyzer is qualification tooling outside the Macroonz product graph, and this receipt neither suppresses nor relabels that warning.
- No unlock, patch, vendor copy, dependency substitution, or productization was authorized or performed.
- The warning is an exact external-tool dependency-policy ceiling for this observation.
- Cargo-semver-checks explicitly does not cover every breaking API change, cannot compare the proc-macro target here, and does not establish generated expansion compatibility by itself.
- Green analyzer rows are joined to package rosters, source/public-path census, renamed package-only consumers, generated compile crossings, and planted controls rather than treated as standalone proof.

## Parent QA and resource disposition

- Parent independently reproduced all published and candidate archive hashes, candidate entry counts, clean VCS identities, archive-roster deltas, manifest equality, and one-edit control diffs.
- Parent read the complete delegated result, analyzer outputs, control outputs, consumer outputs, manifest-topology diff, archive-roster diff, and all retained holder sources.
- Parent independently reproduced the eight supported analyzer outcomes, the breaking and additive outcomes, the proc-macro refusal, the lawful current consumer, the broken generated consumer, and the complete baseline/current renamed facade matrix.
- The first expanded posture run filled the C drive and refused during the harness row with operating-system error 112.
- That resource refusal occurred after both minimal rows passed and is not a product verdict.
- PowerShell recursive cleanup was rejected by host policy before execution.
- Cargo-native cleanup then removed only exact verified task-created target directories, and the remaining compile-only matrix passed.
- Build objects remain disposable and have no repository authority.

## Compact custody

- This receipt retains the irreducible analyzer, package-roster, generated-consumer, and planted-control observations.
- The historical source payload was removed during the repository evidence-retention correction because its consumer source and package inputs are reconstructable from Git, the published registry artifacts, and the hashes above.
- The delegated scratch result had SHA-256 `C8379EC23C8B0739473E16E78615EEEDF114F3FE19FA88FE6C9E3C130B510ACB` at observation time.
- Raw analyzer logs, downloaded package archives, extracted packages, analyzer binaries, source laboratories, and build outputs remain disposable.
- This receipt contains no personal identity or absolute host path.

## Acceptance and remaining ceilings

- The local Windows portion of Wave H is accepted at candidate HEAD `73ffa61b8555290deebee2d463f207ba10651036`.
- One planted public break is detected, one additive control is admitted, and the proc-macro scanner gap is explicit and independently detected by a package-shaped consumer.
- The exact published baseline and current candidate preserve the observed renamed facade and generated `network!` surface across minimal, harness, and full native/Wasm compile postures.
- This receipt establishes no Linux, macOS, physical-host, hosted-as-delivered, registry-delivered 0.2, or human-acceptance claim.
- Hosted-as-delivered and registry-delivered compatibility remain later Wave N evidence planes.
- Wave C remains independently blocked on its named real-defect acceptance sentence while Wave I and other independent work may continue.
