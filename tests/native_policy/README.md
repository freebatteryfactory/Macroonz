# Native-effect qualification

This lane derives the permitted root Clippy profile from the repository's canonical strict configuration.
The derivation removes exactly the declared Instant entry and preserves every other source byte.
Unexpected policy shape refuses preparation.

The explicitly selected wall checks compiler, proc and harness packages strictly, checks every ordinary root feature posture strictly, and checks the native root posture with the derived profile.
All selections use the same source checkout.
Nested Cargo commands use the parent test's target directory so a fresh compiler subject does not create a separate dependency cache.
Independent disposable compiler subjects distinguish clock permission from still-forbidden environment and SystemTime operations.
The relaxed pass alone is insufficient.
Switching the independent clock subject from strict to native and back leaves its source and target untouched, so the final refusal cannot depend on rewriting its source to force a rebuild.

Independent consumer manifests require both native clock and storage entrances to be absent in diet, harness-only and ordinary full postures and present with only native-tooling selected.
The opted-in consumer executes on the current host and compiles for Wasm; cross-compilation alone does not execute the unavailable-target branch.
Normal dependency graphs independently cover diet, harness, full, native-only and full-plus-native consumer manifests on all targets and on Wasm.
The storage dependency family must appear only in an opted-in native-target graph.

The native dependency crossings exercise directory-relative reads and writes, traversal and pre-existing link escape refusal, exclusive creation, non-overwriting hard-link publication, bounded reading and exclusive file-lock custody.
These observations qualify the selected filesystem primitives on the current host; they do not establish a complete storage transaction, crash recovery or power-loss durability.

Public storage controls independently exercise whole-batch readback, aggregate bounds, collision refusal, unpublished recovery, unexpected-file preservation and directory-link escape refusal.
A child process writes the first payload through the public transaction API, then the parent terminates it and requires explicit recovery before readback.
The ignored child entry is invoked only by that control and is not a standalone campaign.
These controls observe process interruption, not power-loss durability.
An independent compiler consumer accepts the public constructors and one consuming commit, then refuses forged names, unadmitted batches and reuse after commit.

Root workflow controls compare composition with explicit input/runner/archive calls while independently observing decoder, subject and reduction-probe counts.
The complete census, original input and earned reached witness survive one native batch; fresh child processes independently select defective and fixed implementations and execute the saved witness once.
Corrupt records, valid but unrelated inputs/capsules, write failure and interrupted retention preserve their separate refusal planes.

This is repository qualification through an existing package test target.
Derived configuration and compiler subjects remain disposable under target/qualification.
The lane recreates them from repository source and requires no preserved campaign output after cargo clean.
It defines no public library API or adopter service, and it is not a process-deadline enforcement mechanism.
