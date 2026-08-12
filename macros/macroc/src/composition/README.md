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
the join belongs to xtask, which already owns the derived-fact side. Its shape is
stated here so the obligation is not vague: a provider exists ↔ it appears
exactly once in this root ↔ it has a disposition ↔ it has an obligation. Omission
fails, phantom fails, duplicate fails. This home owns the duplicate end of that
join structurally — [`CompositionRoot::declared`] refuses one — and the omission
and phantom ends land with xtask when the providers themselves exist. Sequencing
the join is not deferring it: the shape above is the check, written down.

## The seats

`types.rs` declares. Its own child `type_guard.rs` holds the road that reaches
the root's private provider set, which is what makes the duplicate-free claim
structural: there is no second seam that can build a root, so no root exists that
the duplicate scan did not run over. `type_contract.rs` states the refusal
family's declared shape. `establish.rs` is the duplicate scan itself and the body
the established issues amount to — a pure pass, reading providers through the
same answers any caller gets.
