# `.durafx` sealer

This standalone qualification utility copies one declared staging tree into a new immutable `.durafx` run bundle.
It is not a published package, a root-workspace member, product source, or an ambient evidence reader.

## Boundary

The process reads one bounded, versioned declaration record from standard input and never reads process arguments, environment variables, the current directory, or an ambient repository locator.
The record explicitly names the repository, command, and every campaign value it needs, including the navigation label and the entry and byte ceilings; no field has a default.
The label is navigation material rather than identity authority.
Seal paths are absolute, lexically normalized UTF-8 paths, and the staging path must remain a strict descendant of the declared repository after filesystem resolution.
The warehouse is always the declared repository's direct `.durafx` child and is never supplied as a second path.
Verify likewise requires the repository and one absolute, lexically normalized run path and derives no run through discovery.
The utility accepts an ordinary staging directory or one explicitly named run below `target/qualification/` as the disposable-to-durable handoff.
It refuses `target` or a Cargo profile tree wholesale, nested `target` and `.durafx` directories, recognizable Cargo build directories and compiled artifacts, filesystem indirection and Windows reparse points, non-files, empty directories, reserved protocol paths, repository overlap, and an existing final destination.
The staging root must contain one nonempty caller-authored `receipt.md` that states the campaign's proposition, command and tool posture, observed result, and evidence ceiling at the detail the campaign requires.
Missing, empty, directory-shaped, or symlinked semantic receipt material refuses before the warehouse is changed.

The utility generates `DURAFX-RECEIPT.txt` only from the declared placement facts and binds it to the caller-authored `receipt.md` path.
That generated placement/custody receipt cannot substitute for the campaign semantics, command, tool version, result, or evidence ceiling owned by `receipt.md`.
`DURAFX-MANIFEST.blake3` lists every retained payload file in UTF-8 path order with its normalized slash-separated path, byte length, and full BLAKE3 hash.
The manifest is the structural index and is the only retained file excluded from its own finite payload denominator.
The run identity is a domain-separated BLAKE3 transcript over the evidence manifest, plane, source revision, host-target posture, declared entry and byte ceilings, and semantic-receipt path.
The generated placement receipt and navigation label are excluded from that transcript, so relabeling identical evidence changes navigation and the full manifest without changing evidence identity.

The final seat is:

```text
.durafx/<plane>/<source-revision>/<host-target>/<label>-<run-digest>/
```

The utility atomically claims the final run path with a no-clobber directory creation, populates it with create-new file writes, recenses the staging tree, applies the Unix advisory guard where it is meaningful, and rereads and verifies the stored receipts, manifest, path, and payload.
Its last semantic filesystem action creates one empty `DURAFX-COMPLETE` directory.
A missing, non-directory, redirected, or nonempty completion marker means the final-named seat is incomplete and verification refuses it.
A precommit failure attempts to remove its incomplete seat, and a cleanup failure leaves a blocking but unverifiable reservation rather than overwriting or blessing it.
The run root remains writable so the completion directory can be the final atomic transition.
Unix descendants have write bits removed; Windows supplies no safe standard-library permission transition that is both useful for directories and reversible under the repository's no-lint-escape law, so Windows relies on complete hash verification rather than pretending its read-only file attribute is an access boundary.
The guard is not a substitute for the receipt hashes or an operating-system access boundary.
There is no portable filesystem transaction joining that final transition to process exit or standard-output delivery, so a lost acknowledgment after publication is resolved by explicitly verifying the deterministic run path rather than resealing over it.

`verify` checks the completion state, repository custody, canonical receipt and manifest encodings, declared ceilings, every expected payload hash and byte length, missing files, additional files and directories, filesystem indirection, and agreement between the declared placement and the directory key.
It never discovers a run through a `latest` pointer or by scanning the warehouse.

The caller-authored receipt is retained bytes rather than prose this utility can semantically judge.
The host target and other placement fields are declared facts rather than ambient machine observations.
BLAKE3 establishes byte agreement and collision-resistant naming rather than origin authenticity.
The staging producer must quiesce its tree while sealing; a second complete census detects intervening additions, removals, and changes, but this utility does not claim a hostile-writer filesystem snapshot.

## Source ownership

The executable has one directional source graph:

```text
main.rs -> arguments.rs
main.rs -> seal.rs
main.rs -> verify.rs
seal.rs -> arguments.rs, manifest.rs, storage.rs, verify.rs
verify.rs -> manifest.rs, storage.rs
storage.rs -> manifest.rs
```

`main.rs` is only the bounded standard-input process door and result writer.
`arguments.rs` owns the declared protocol, `manifest.rs` owns canonical receipt and manifest encodings, `seal.rs` owns the state transition into a final seat, `verify.rs` owns bundle judgment, and `storage.rs` owns the filesystem traversal and custody effects genuinely consumed by both sealing and verification.
There is no common, utility, or core catch-all module.

## Lint wall

The isolated package cannot inherit `[workspace.lints]` from the root workspace without becoming a member of the product graph.
Its manifest therefore seats the complete current root Rust and Clippy lint table in its own isolated workspace while the repository's one root `clippy.toml` remains the shared threshold and disallowed-operation configuration discovered by Clippy.
Cargo provides no cross-workspace lint-table inheritance, so the duplicated manifest table is an unavoidable isolation cost and must change in the same accepted tooling edit whenever the root table changes.
No local lint relaxation, `allow`, or `expect` is present.

## Qualification commands

Run these from the repository root when a Cargo executor is authorized:

```text
cargo +1.98.0 check --locked --offline --manifest-path qualification/durafx-sealer/Cargo.toml --target-dir target/qualification/durafx-sealer
cargo +1.98.0 clippy --locked --offline --manifest-path qualification/durafx-sealer/Cargo.toml --target-dir target/qualification/durafx-sealer --all-targets -- -D warnings
cargo +1.98.0 test --locked --offline --manifest-path qualification/durafx-sealer/Cargo.toml --target-dir target/qualification/durafx-sealer
cargo +1.98.0 fmt --manifest-path qualification/durafx-sealer/Cargo.toml -- --check
```

The isolated lockfile carries the exact `blake3 1.8.6` closure already pinned by the canonical root lockfile.
Cargo must still confirm that hand-seated lockfile before this packet earns a compiled or executed claim.

## Declaration protocol

Version 1 is one tab-separated UTF-8 record with one LF or CRLF terminator.
Tabs, embedded newlines, other control characters, missing or reordered fields, additional fields, non-UTF-8 bytes, relative paths, and `.` or `..` path components refuse.
The fixed seal record is:

```text
durafx-sealer-request-v1<TAB>command=seal<TAB>repository=<absolute-repository><TAB>staging=<absolute-path><TAB>plane=<plane><TAB>source-revision=<revision><TAB>host-target=<host-target><TAB>entry-limit=<positive-decimal><TAB>byte-limit=<positive-decimal><TAB>label=<label><LF-or-CRLF>
```

The fixed verify record is:

```text
durafx-sealer-request-v1<TAB>command=verify<TAB>repository=<absolute-repository><TAB>run=<absolute-run-path><LF-or-CRLF>
```

PowerShell can pipe a declared seal record without exposing any value through the child argument vector:

```powershell
$staging = (Resolve-Path -LiteralPath 'target/qualification/replay-posture-mutants').Path
$repository = (Get-Location).Path
$revision = '<full-source-revision>'
$request = @(
    'durafx-sealer-request-v1'
    'command=seal'
    "repository=$repository"
    "staging=$staging"
    'plane=mutation'
    "source-revision=$revision"
    'host-target=windows-x86_64-pc-windows-msvc'
    'entry-limit=<declared-entry-ceiling>'
    'byte-limit=<declared-byte-ceiling>'
    'label=replay-posture'
) -join "`t"
$request | cargo +1.98.0 run --locked --offline --manifest-path qualification/durafx-sealer/Cargo.toml --target-dir target/qualification/durafx-sealer
```

Bash can supply the same record with `printf`:

```bash
printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
  'durafx-sealer-request-v1' 'command=seal' \
  "repository=$repository" "staging=$staging" 'plane=mutation' \
  "source-revision=$revision" "host-target=$host_target" \
  "entry-limit=$entry_limit" "byte-limit=$byte_limit" 'label=replay-posture' \
  | cargo +1.98.0 run --locked --offline --manifest-path qualification/durafx-sealer/Cargo.toml --target-dir target/qualification/durafx-sealer
```

## Legacy disposition

This minimum utility has no adoption mode and does not rewrite existing `.durafx` evidence.
Future legacy adoption requires an explicit owner disposition for the source bundle, its pre-existing receipt authority, the staging projection, and the destination placement.
A lawful adoption copies the selected legacy material into a new declared staging tree, seals a new collision-safe bundle, verifies it, and records the old-to-new hash roster without mutating or blessing the legacy directory in place.
