# sgrules — the construction-phase QA rule pack

Rules are data files, part of the toolchain layer like `clippy.toml`. This
pack is the zero-compile phase's QA instrument: while nothing runs cargo,
these rules and read-only review are the enforcement surface. The pack is
construction-phase material — when the phase ends and the toolchain stands,
rules whose claims the compiler or clippy own retire to those owners.

Graduated trust: a NEW rule enters advisory (`warning`/`hint`) and earns
`error` only when its signal quality has been observed — a rule that cries
wolf trains people to ignore the pack. Rules restating standing law (the
lint wall, the zero-compile guards) enter at `error` because their claims
were already blocking elsewhere.

What ast-grep cannot express structurally lives here as the review
checklist instead of a fake rule — a rule that pretends to check what it
cannot is worse than a stated manual check:

- **The spine's illegal shortcuts** (each skipped arrow is a named defect,
  checked on diffs during review, never a standing registry):
  - a Row plus a callable reaching execution without a Binding
    establishing the attachment;
  - a Selection reaching execution without the complete Table;
  - one Binding treated as a denominator;
  - a RunReport writing an authored Row (evidence authoring spec);
  - content reaching rendering without a plan;
  - a plan reaching tokens without the closure and the closed expansion;
  - rendering gaining semantic identity;
  - an explanation minting an owner fact.
- **Behavior coupling:** would this test or design survive a lawful
  refactor? Owner-coupled tests get flagged.
- **"promote" appears in no filename**; the proposal road's file is
  `propose.rs`.
- **Test scaffolding as specification is a finding, anywhere** —
  no `laws.rs`, no proof-surface module,
  no assertion file collecting claims across unrelated homes,
  no `#[cfg(test)]` item inside a library.
  A structural claim is enforced by a type;
  a lane in a crate's `tests/` observes behavior a type cannot state,
  named for that behavior, reached through the public surface.
- **Hand-maintained population lists** are a finding — populations are
  depot data or generated.
- **File-grammar conformance:** every semantic home is README + mod +
  types, with `type_guard.rs`/`type_contract.rs` earned and role-named
  pure-function files otherwise; a `pub` type outside the owning
  `types.rs` is a finding; files exist only when they have content.
