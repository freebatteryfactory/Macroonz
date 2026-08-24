# Working law

This file binds every person, model, and agent who edits this repository.
It is the only such file; `CLAUDE.md` is a pointer to it.
The [README](README.md) owns the product and the crate map, and nothing here repeats it.

> There is no CI and no gate.
> The toolchain at the root, run locally, is the enforcement surface: the lint wall in `Cargo.toml`, `clippy.toml`, `deny.toml`, `.cargo/config.toml`, the compiler itself, and the lanes.
> Checks report.
> A human decides.

---

## 1 · Macroonz knows nothing

Macroonz is generic machinery — a compiler that bakes what a request asks for, a harness that judges what it is handed, a proc host that carries tokens.
It has no product.

- No downstream type, trait, roster, identity rule, bound, or error family is defined here, copied here, or depended on here.
- Nothing is moved here because several files happen to import it. Shared is not ours.
- The compiler's own errors are plain diagnostics with ordinary fields. Its own collections are its own. Its own rosters are plain enums. None of them is somebody else's theory of refusal, admission, or commitment wearing a new name.
- A library that uses Macroonz keeps every one of its primitives and writes its own derives on the compiler's public API.

If the compiler or the harness cannot do its job without knowing the product it serves, the request surface is wrong, and it is repaired here, generically.

---

## 2 · Dependencies point one way

```text
macroonz-macros   ──▶ macroonz
macroonz-harness  ┈┈▶ macroonz   tests only
```

Every other edge is forbidden, and the diagram is the whole claim: an arrow a manifest carries that no line here draws is a defect in one of the two.

---

## 3 · The repository is the specification

There is no separate book.
A semantic fact has one owner; weaker seats cite the owner and never restate it.
A false sentence anywhere in the tree is a broken build, and code rises to the docs — never the reverse.

Git owns history.
No migration story, origin story, status line, or population count lives in the tree.

A structural invariant lives in the type and the smart constructor that make its violation unwritable.
A lane in `tests/` observes what no type can state: behavior, composition across crates, encodings, diagnostics, and the compile refusal that shows a violation really is unwritable from outside.
There is no `laws.rs`, no proof-surface module, and no `#[cfg(test)]` item inside a library.
A claim no seat can establish is not claimed.

---

## 4 · The shape of a home

Every home is a directory.

| File | Owns |
| --- | --- |
| `README.md` | Why the home exists, what it claims, what it does not — for someone reading it the first time |
| `mod.rs` | The door: declarations and re-exports, nothing else |
| `types.rs` | Every public type of the home. A `pub` type anywhere else refuses |
| `type_guard.rs` | Smart constructors and the invariant nucleus; declared from `types.rs` as `#[path = "type_guard.rs"] mod guard;` so it sees private fields |
| `type_contract.rs` | Trait implementations |
| `capture.rs`, `plan.rs`, `render.rs`, `encode.rs`, `diagnose.rs`, … | One role each, pure functions over values already informed by their types |

There is no `validate.rs`.
A check over raw input returns an informed type or a diagnostic, never a boolean that leaves the input standing.
A file exists only when it has content.
An empty seat is a directory with one README that states its question and the condition that fills it.

A home is sized for a reader.
A function past a hundred lines is two operations.
Four levels of nesting is a type that has not been written yet.

---

## 5 · Prose

Prose is a reading surface, in Markdown and in doc comments alike.

- **One sentence per source line.** A line ends at the end of a sentence, or at a colon before a list — never inside a phrase, a type name, a link, or a code span.
- **No column limit.** Editors soft-wrap; rustdoc renders paragraphs. No wrapping formatter is ever configured.
- **Real structure.** Blank line between paragraphs, `-` for lists, `>` for quotations, fences for code and diagrams, tables where a table reads better than a list. Never four-space indentation — Markdown reads it as code.
- **Formatting is not rewriting.** A formatting-only change preserves every word. A change of wording is a semantic edit, reviewed as one.

A doc comment begins with one plain sentence saying what the item is.
Further sections are earned by the item's actual contract — construction, bounds, ordering, refusals, errors, examples — only where the caller needs the distinction.

> If deleting the item would make the sentence false, the sentence is the item's doc comment.
> If deleting the item would leave it true, it belongs once in the home's README, under an anchor the item cites.

Remove on sight: restatements of the signature, restatements of a linked owner's contract, history, counts, procedure, and the defect that caused the type to exist.
Doc lines are never a larger body than the code they document.
A README has no yaml blocks, no ledgers, no status tables.

---

## 6 · Hard rules

- **Safe Rust only, no hatches.** The lint wall is declared once in the root `Cargo.toml` and inherited by every member. `#[allow]` and `#[expect]` are both forbidden: a complaint the wall raises is repaired in the design until the complaint is impossible, never quieted at the site. The tree carries zero of either, and a change that needs one is a change whose design is not finished.
- **No Python.** Ever. Tooling is Rust.
- **Declared input only.** Expansion and judgment are functions of what they were handed: no network, no filesystem scan, no environment, no clock, no entropy. The harness may read the host facts it needs to run; they never enter an identity or a verdict.
- **The proc host carries.** Token conversion, span custody, one compiler call, diagnostic placement, emission. It owns no grammar.
- **Nothing sneaks in.** No probes, no symlinks, no `build.rs`. LF everywhere. No person's name in any file.
- **Red is a sensor.** A failing lane, lint, metric, or search result is evidence; the owner rules on the substance before code changes to answer it. No code is edited to silence a finding.
- **Unused is not dead.** `unused`, `uncalled`, and `unreached` are wiring facts, never deletion authority.
- **Read whole.** Every assigned file, top to bottom, before classifying or editing it. Search output enumerates candidates; it is never the denominator.
- **Name the plane.** "Complete", "closed", and "proven" are never written bare. Local green is not hosted, packaged, or accepted.
- **Humans commit.** Commits, pushes, branch rewrites, and recovery operations are a human's. An agent never initiates one.
