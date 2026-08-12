# `explanation_protocol` — the typed answers, and the coverage that admits no partial view

The explanation protocol's machinery: the typed answers that carry the questions,
and the coverage check that admits no incomplete view.

The question roster itself is a leaf vocabulary and lives in [`crate::question`],
which both this home and the planning home import. Nothing here restates it.

## The protocol is mandatory, and the shape enforces it

A projection kind declares which questions its plans answer, and every kind
carries the universal roster whether it lists it or not. A
[`ProjectionExplanationView`] is complete only when every applicable question has
exactly one answer — an unanswered seat, a doubled seat, and an answer to a
question the kind does not admit are all refused, each naming the question. No
kind ducks the protocol by answering fewer questions than its roster, because the
roster is what the view is checked against.

## Answers reference identities, not prose

Every [`ExplanationAnswer`] carries typed values and exact identities. The human
projection riding alongside is for a person to read and is derived from those
values; nothing reads it back. The question an answer belongs to is derived from
the answer itself, so a mismatched question-and-answer pair is unrepresentable
rather than validated.

## The seats

`types.rs` declares. Its own child `type_guard.rs` holds every road that reaches
a private field — an explanation's question, answer, and human projection, and
the view's own seats — which is what makes "the question comes from the answer"
structural: there is no seam anywhere that files a true answer under a question
somebody supplied. `type_contract.rs` states the refusal family's shape and the
closed table that maps an answer to its question. `establish.rs` is the coverage
pass, the admission answer a caller asks for one kind, and the body the
established issues amount to.
