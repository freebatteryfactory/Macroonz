# Documentation diagrams

This home owns authored Mermaid sources, their delivery roster and one pinned native rendering procedure.
The source owners linked from the [repository map](../../README.md#find-the-owner) decide whether the diagrams describe the system correctly.
Byte freshness establishes no semantic truth.
Rendering is a maintainer workflow from a repository checkout; packaged readers consume the generated assets without installing this toolchain.

## Edit and check

Use Node 24.18.0 and Bun 1.4.2 from the repository root.
The lockfile pins the dependency graph; rendering uses Node, while Bun installs it with lifecycle scripts disabled.
Provision only Puppeteer's pinned headless browser in disposable build storage:

```powershell
bun install --cwd assets/diagrams --frozen-lockfile --ignore-scripts
$env:PUPPETEER_CACHE_DIR = Join-Path $pwd 'target/qualification/documentation-navigation/browser'
node assets/diagrams/node_modules/puppeteer/lib/puppeteer/node/cli.js browsers install chrome-headless-shell
node assets/diagrams/run.mjs generate
node assets/diagrams/run.mjs check
node --test assets/diagrams/controls.test.mjs
```

The qualified rendering platform is native Windows x64; the prepared CI job selects Windows separately and requires its own hosted observation.
Other operating systems have no byte-equivalence claim.
The browser version, viewport and Mermaid configuration live in `render.json`; the embedded Noto Sans font comes from the locked package and retains its [license](OFL.txt).
Both identifier and shape-drawing seeds are fixed: deterministic identifiers alone do not stabilize every shape's SVG path.
Pages render offline, using local tool resources and embedded font bytes.
Do not substitute a system browser, fetch an undeclared CLI, or strip differences to make freshness pass.

Edit the root `.mmd` source, then regenerate before staging its complete delivery set.
`catalog.json` maps each source to its README and package; it is a presentation roster, not a second architecture specification.
Generation renders the complete set before writing any delivery and fails on orphaned outputs; remove obsolete catalog entries and their obsolete generated files together.
A filesystem write failure is a failed generation, and the subsequent check must pass before the batch is accepted.
Checking renders in memory and changes no tracked file or Git state.
Missing sources or images, orphaned images, changed inputs and renderer failures refuse rather than silently skipping material.

## Delivery

README reference links select local SVG files on GitHub and in extracted packages.
Standalone READMEs link the adjacent Mermaid source; rustdoc-included owners name its path in the source package, since a relative source-file link would not resolve on the generated documentation server.
Compiler and harness package assets are generated delivery copies of this home's sources and SVGs.
Owner modules prepend generated `diagrams.md` reference definitions when including their README in rustdoc.
Those definitions embed the same SVG bytes, so rustdoc needs neither a network image nor a copied web-server asset directory.
READMEs themselves remain authored prose.
Generated sources, images and definitions must not be independently edited.
Cargo package selection must exclude the renderer, dependencies and browser/cache material even before cleanup; inspect both required assets and forbidden leakage with dependencies installed.

## Staged checks and CI

Install the tracked hook once with `git config core.hooksPath .githooks` after checking that no existing hook configuration would be replaced.
Run `node assets/diagrams/staged.mjs` to check the proposed commit explicitly.
The hook reads its driver from the index, exports only the indexed diagram inputs and delivery documents into a disposable snapshot, and runs that snapshot's checker.
It reuses the installed dependency graph only when the indexed lock matches; install the indexed dependency selection when they differ.
Neither path stashes, repairs, stages or commits files.
Relevant commits with absent dependencies fail with a tool error; install the declared tools before retrying.
Unrelated commits do not invoke the renderer.

The [documentation workflow](../../.github/workflows/documentation.yml) always reports a job and selects expensive work from the complete changed revision range.
It uses the same checker and hostile-input controls, grants read-only contents access, and carries no publication step or secret.
Installing a workflow does not make it a required branch check or establish hosted success.

After checks and commits finish, remove this home's exact `node_modules` directory if no active process uses it.
The browser cache remains disposable under `target` and may be cleaned under repository custody rules.
Deleting dependencies is housekeeping; it does not undo executed code or clear Bun's shared cache.
Keep the manifest, lockfile and authored sources.
