# Hosted Mermaid rendering receipt

This Git-tracked receipt records a visual rendering check of every Mermaid diagram on the public GitHub completion branch.

## Denominator

- Hosted source snapshot: `8c184279189b83e5a94d0cba3108ac561227bd87` on `codex/macroonz-repository-completion`.
- Local comparison snapshot: `72ecfae28d977e29e9faece04f36a137a901e7cc` on the same branch.
- No Mermaid-owning Markdown file differs between those snapshots.
- The repository contains sixteen Mermaid blocks across fourteen Markdown files.
- Every block carries one accessible title and one accessible description.

## Visual observation

- Microsoft Edge loaded each public GitHub Markdown page in the dark theme from the live completion branch.
- All sixteen diagrams rendered as graphics with their nodes, labels, and connections visible.
- The root and runner pages each rendered both of their diagrams.
- The harness, bench, fuzz, generate, interleave, preemption, properties, compiler, bounded, expansion, kind, and stamp pages each rendered their one diagram.
- Some first loads displayed GitHub's `Unable to render rich display` fallback while the hosted renderer was loading.
- A warmed retry of every affected page rendered the diagram successfully without a repository change.
- A captured network trace for the harness page observed a successful response from GitHub's hosted Mermaid rendering endpoint.
- No Mermaid source correction was required.

## Custody and cleanup

- The exact disposable observation path was `target/qualification/wave-f-github-mermaid-20260828`.
- Before cleanup, that path contained 9,673 files totaling 478,979,908 bytes, including isolated browser profiles, screenshots, and one network trace.
- The raw browser profiles, screenshots, and network trace are observation exhaust rather than repository authority.
- This readable receipt retains the accepted observation after the exact disposable path is removed.
- Guarded Cargo cleanup removed 9,675 files totaling 456.8 MiB, including the temporary cache tag required for safe cleanup.
- A direct existence check confirmed that the exact disposable path no longer exists.

## Boundary

- This receipt proves one successful hosted visual rendering observation for the sixteen diagrams present at the named public source snapshot.
- It does not claim continuous availability of GitHub's hosted renderer.
- It is not hosted CI, hosted security, branch governance, physical Linux, macOS, ARM64, package publication, registry delivery, merge, or release acceptance.
- No workflow, ruleset, protection, publication, or remote ref was changed by this check.
