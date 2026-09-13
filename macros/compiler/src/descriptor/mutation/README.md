# `mutation` — the kind that carries a mutation surface into a test target

One declaration in; one module out, delivered as deferred cargo the consumer's test target invokes.

The module carries three things: the policy the surface is lowered under, the site whose alternatives a selection chooses between, and the dispatch that answers an evaluation.

## Every fact here is the declaration's

An owner fact is a name the consumer declares.
An operator family is a slug the consumer declares.
An alternative is DATA: the semantic bytes that identify the operation, and the value it means.

Nothing here holds a roster of facts that exist or families that can be applied, and nothing in this vocabulary invents an alternative.
A producer that computed pressure from what a declaration MEANS would be a producer that knew the thing it does not know.
The structural completion roads below compute alternatives from an author's declared members and bindings without choosing which behavior is correct.

What this home checks is shape: a mapping is one fact and one claim, a permission is one claim and a non-empty roster of families, two alternatives never carry one operation, and every roster stands inside its declared magnitude.

## Two halves, and where they meet

A helper body states what an author can state: the module, the refusal type, the optional support address, the evaluation family, the point, the owner fact, the mappings, and the permissions.

Everything else a site carries — the type its alternatives are values of, the production the unchanged declaration answers with, the unchanged operation bytes, and the alternatives themselves — is token material and semantic bytes computed by the door that captured the declaration this helper sits on.

[`Declaration::completed`] joins the two halves into a [`Surface`].
[`door::mutations_from_surface`](crate::descriptor::door::mutations_from_surface) delivers that informed material through the ordinary sealed carrier when the caller supplies the corresponding helper and item captures.

## The declared-order door, for the door that owns no semantics

[`completed`] is this home's own computation of the second half, for the generic attribute: the item the helper sits on must be an enum, its variant list in authored order is the declared order, the unchanged operation is that order as written, and each alternative is one adjacent transposition of it under the [`DECLARED_ORDER_FAMILY`] operator family.

The reading is structural on purpose — variant names and their order, never fields or meanings — because transposing two neighbors of an order somebody declared requires no knowledge of what the declaration means.
What a transposition surviving says is the consumer's to judge, exactly as every other survivor is.

A door that does know its declaration's semantics — an adopter's own derive — computes richer site material and calls [`Declaration::completed`] directly.

## Codec pressure

[`completed_from_codec_order`] consumes one informed [`CodecContent`](crate::codec::CodecContent) and transposes each adjacent member pair under the same [`DECLARED_ORDER_FAMILY`] operator.
It preserves each member's name, type, shape and cardinality, along with the codec's owner, direction, assembly, placement, schema, byte role and assumptions.
The codec owner admits every changed shape and renders the unchanged and selected source through its ordinary paired roads.
The operation bytes frame `codec-member-order/v1` followed by that codec's complete canonical content, using the identity owner's byte framing.
An unpressable roster or refused rendering returns a typed error before a partial surface is delivered.

The completed surface's value type is `&'static str`, containing generated codec Rust source.
The caller supplies the owning item, member implementations and independent witnesses when materializing that source through the existing specimen or native compiler road.
A generated evaluation's firing observes selection of this source, not execution of the generated codec.
It cannot establish subject activation, equivalence, a behavioral kill or survival; those claims require the relevant harness evidence and actual caller execution.
In particular, reordered assembly arguments may fail to compile or may remain type-correct while changing behavior.
Neither outcome is inferred by the producer, and no oracle is generated.

## Transition and effect pressure

[`completed_from_transition_targets`] selects one zero-based authored transition row and replaces its destination with each other member of the declared source vocabulary.
[`completed_from_effect_bindings`] selects one such row and substitutes each distinct complete effect binding already stated by sibling rows, retaining first occurrence in authored order.
The owner supplies a separate point, fact and claim policy for the selected row through the ordinary [`Declaration`].
The respective operator slugs are [`TRANSITION_TARGET_FAMILY`] and [`EFFECT_BINDING_FAMILY`], owned by the harness bank.
Neither operation invents a destination, effect, permission or witness.

Each changed recipe is re-admitted through [`RecipeEdit`](crate::recipe::RecipeEdit) and rendered through the same recipe compiler as the unchanged source.
The alternatives preserve the selected row's source and trigger; target substitution preserves its effect, and effect substitution preserves its declared target.
The operation bytes frame `recipe-structural-change/v1`, the operator slug and the complete changed recipe canonical content.
Unchanged operations and byte-identical duplicates offer no additional structural selection; no semantic equivalence is inferred for different operations.
An absent site, empty alternative set, exceeded alternative bound or failed re-admission withholds the complete surface with a typed refusal rather than silently omitting failed candidates.

The surface carries complete generated recipe source as `&'static str` under the same source-selection evidence ceiling as codec pressure.
Selected source may fail rustc or execute differently because an effect's new context need not satisfy its original bindings.
Only actual materialization, subject execution and independent judgment can establish those outcomes.

## The alternatives and their meanings travel together

Each alternative carries both its operation bytes and the tokens of the value it means.

A rendering that carried the meaning without the bytes would emit a value nothing could select.
One that carried the bytes without the meaning would select a value nothing renders.
One type, both seats required, and neither case is writable.

## An unmapped site is still a site

Where the policy maps the site's owner fact to a claim, the rendered discovery carries that claim.
Where it does not, the discovery carries the unmapped posture and says so.

Neither is a refusal here.
Whether an unmapped discovery may become executable is the harness's ruling, made where the lowering is admitted, and a producer that decided it would be a second authority.

## Ownership

This home owns the mutation declaration shape, structural completions, the carried policy and site material, the deferred projection, and every refusal required to establish them.
The consumer owns the meaning of facts, claims, operators, alternatives, and evaluation; the harness owns whether a rendered discovery becomes executable evidence.

## Authored grammar

```text
#[<helper>(
    module = <module name>,
    refusal = <refusal type name>,
    support = <exported name>,
    family = named("<namespace>", "<stem>"),
    point = named("<namespace>", "<stem>"),
    fact = named("<namespace>", "<stem>"),

    map named("<namespace>", "<fact>") = named("<namespace>", "<claim>"),
    permit named("<namespace>", "<claim>") = ["<family slug>", ...],
)]
```

`support` is the one optional clause: a declaration whose carrier another helper already addressed states none.

### Caller-owned names

An owner fact and an operator family are the CONSUMER's declarations.
This reading resolves neither against a roster it owns, because a producer that knew which facts exist would be a producer that knew what the consumer's declaration means.
What it checks is shape: a mapping is one fact and one claim, a permission is one claim and a non-empty roster of family slugs, and nothing states one fact or one claim twice.

### Door-owned material

The site's own material — the type its alternatives are values of, the production the unchanged declaration answers with, the operation bytes, and the alternatives themselves — is computed by the door that captured the declaration this helper sits on.
[`Declaration::completed`] is where the two meet.
