# composition — the descriptor composition root

The one place that says which providers of descriptor material exist.

## Why a hand-authored root is lawful and an unchecked inventory is not

Composition carries meaning. Naming, in one declaration, exactly which providers
compose is a statement somebody made and can be held to. An unchecked inventory
carries none: it is a list that happens to be right today, and nothing refuses
when it stops being right. The machine's format law bans hand-maintained
inventories for precisely that reason, and this root is not one — it is a
composition declaration, and every claim it makes is joined against derived
facts.

## Local facts generate local products; global facts compose through here

A local fact — one obligation, one work formula, one port — generates its own
local product beside it. A global fact — the full set of test descriptors, the
documentation index, the public-surface inventory — composes through THIS root
and nowhere else. There is no ambient registration, no scan of the tree, no
link-time collection, and no attribute that quietly enrols a provider: a provider
that is not declared here does not participate.

## The bidirectional join this root is owed

Detecting an omitted provider or one that exists only in the root is a JOIN, and
the join is repository-level and has no owner today. Its shape is
stated here so the obligation is not vague: a provider exists ↔ it appears
exactly once in this root ↔ it has a disposition ↔ it has an obligation. Omission
fails, phantom fails, duplicate fails. This home owns the duplicate end of that
join structurally — [`CompositionRoot::declared`] refuses one — and the omission
and phantom ends land when the providers themselves exist. Sequencing
the join is not deferring it: the shape above is the check, written down.

## The seats

`types.rs` declares. Its own child `type_guard.rs` holds every road that reaches
a private seat — the root's provider set and the refusal body's one seat — which
is what makes the duplicate-free claim structural: there is no second seam that
can build a root, so no root exists that the duplicate scan did not run over, and
no refusal body exists that the scan did not produce. `type_contract.rs` states
the refusal family's declared shape. `establish.rs` is the duplicate scan itself —
a pure pass, reading providers through the same answers any caller gets, reaching
no private seat.

This home's qualification obligations live in the crate README's tooling-obligation
blocks.
