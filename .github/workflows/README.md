# Hosted qualification

This home observes the repository wall on declared cloud hosts.
It reports another evidence plane and never replaces the local enforcement surface or the human acceptance decision.

## Host roles

The primary Linux lane owns the complete hosted wall because source-wide formatting, documentation, dependency-policy, and Wasm questions do not become stronger when repeated on every operating system.
The other lanes own host and architecture crossings: native compilation, strict linting, external tests, doctests, and the stable-rustc coverage example.

Blacksmith supplies the Linux x64 and Linux ARM64 runners so those compiler-heavy lanes receive fast single-threaded CPUs, colocated dependency caching, machine metrics, searchable logs, and structured test analytics.
GitHub supplies the standard public Windows and macOS runners so the first pulse gains independent provider and operating-system diversity without spending premium runner minutes.

Every hosted result names its actual operating system and architecture.
A cloud runner never establishes a physical-host claim.

## Reproducibility and custody

The workflow installs the repository's exact stable Rust toolchain and exact auxiliary tool versions.
Every compilation uses the committed lockfile.
One declared dependency-fetch step precedes offline Cargo execution, so a build cannot acquire a new graph halfway through qualification.

Third-party actions are pinned by immutable commit identity, checkout does not retain credentials, and the workflow receives read-only repository contents.
The dependency cache contains Cargo registry packages and Git database material only; compiled targets, qualification scratch, verdicts, and accepted evidence never derive authority from a cache hit.

Nextest writes structured JUnit output for the runner's test analytics while ordinary step logs retain the complete human-readable failure.
The hosted observation becomes durable only when a compact receipt or promoted regression enters Git through ordinary review.

## Deliberate limits

The first pulse is manual, reporting-only, bounded by job timeouts, and cancels an older run for the same ref.
It has no secret, publication, release, attestation, artifact-upload, scheduled, push, or pull-request path.
It creates no required check, branch rule, or automatic retry loop.

Docker caches and service-container acceleration have no subject in this repository.
Sticky build disks, cached checkouts, provider-specific Windows or macOS duplicates, and agent Testboxes require measured need and a separate custody decision before they join this home.
