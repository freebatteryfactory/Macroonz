# Input execution and retained replay

This home composes the existing input, runner and archive owners through the root library.
The `harness` feature supplies `workflow::run`; `native-tooling` additionally supplies retention, loading and stored-witness replay.

## Execute

Call `run` with the complete typed table view, its selection plan, the current decoder, specimen bytes, invocation and input limits.
The input owner packs and decodes once, then the existing runner executes the selection over the complete table.
The returned `InputRun` pairs that report with the original admitted input.
Read the ordinary `RunReport` for conclusions, budgets, measurements and every unselected row.
This home invents no verdict, revision, semantic configuration or clock policy.

## Retain and load

Pass an explicit native storage root, batch name, bounds and any earned replay capsules to `InputRun::retain`.
Capsules must match selected failing rows by complete execution key and failure fingerprint, with at most one capsule per row.
No capsule is inferred for a failure that has not earned one through the reduction owner.
The complete input envelope, complete-run archive and supplied capsule archives are published in one storage transaction.
The bytes remain the existing format owners' canonical encodings.

The mechanical members are `input`, `run` and `capsule-N`, where N is the original census position in canonical unsigned decimal without leading zeros.
`StoredRun::load` receives the original input profile independently, delegates bounded byte admission to each owner, and refuses missing, unknown or unrelated members.
The original case and profile must agree with the historical run, and every capsule must agree with its historical selected refusal.
The retained original input remains distinct from a reduced reached witness.
Inspecting a `StoredRun` performs no trial execution and creates no live report, replay capsule or human admission.

Storage failures remain separate from input/format refusals and from the report's subject conclusions.
An unpublished attempt may be retried through `recover_retention` with the same explicit run and replacement capsule roster; the [storage owner](../native_storage/README.md) owns that operation's custody and limits.
A successful run followed by a failed write remains available in the caller's `InputRun`.
No persistence failure relabels a subject result.

## Fresh replay

After loading, select an original census position with a retained capsule and call `StoredRun::replay` with independently supplied current trial, decoder, invocation and input limits.
The existing [runner replay](../../harness/src/runner/README.md#saved-witness-replay) decodes and executes the saved reached witness once.
Its `ReplayedTrial` keeps the current report and the report owner's historical comparison, including coordinate movement and non-reproduction distinctions.
The historical file never selects current code or reruns reduction.
To rerun the original specimen over a whole current table, pass `stored.input().payload()` to `workflow::run` with the independently selected current decoder and invocation.

This home owns composition, not process supervision, publication installation, archive authenticity or a second report/replay engine.
