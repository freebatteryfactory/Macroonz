# `origin_graph` — where a generated thing came from

The origin graph and the citation machinery: which owner fact decided each step,
and what the plane explicitly does not claim about the result.

## The structural orphan law

Every generated-unit type in the plane carries an [`OriginTrail`], and a trail is
structurally non-empty. A generated node with no origin is therefore
unrepresentable rather than validated: there is no road that produces one, so no
check has to catch one.

A source span is not an origin. A span says where bytes sat; an origin says which
authored declaration, which pattern instantiation, which profile selection, and
which rendering act stand between that declaration and this unit. A generated
unit that offers only a span has answered a different question than the one
asked.

## What a trace is, and is not

A [`DecisionTrace`] preserves selection order — the order the plane made the
decisions, never a sorted or prettified order. A check that did not run is
recorded as [`TraceDecision::NotRun`] and is never confused with one that ran and
passed. No protected source material enters a trace: entries name subjects and
owner facts by identity, and identities carry no spelling.

## The seats

`types.rs` declares. Its own child `type_guard.rs` holds every road that reaches
a private field — the trail's edges and the trace's entries — which is what makes
the orphan law and the selection-order law structural rather than reviewed: there
is no other seam that can draw a trail or record a trace. `type_contract.rs`
states the decision roster's own discriminant table. `encode.rs` writes the
canonical bytes every value here contributes to a transcript, over the public
walk alone, so an encoding can never see more than a reader can.
