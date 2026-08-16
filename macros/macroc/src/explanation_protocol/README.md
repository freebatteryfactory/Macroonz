# explanation_protocol — the typed answers, and the coverage that admits no partial view

The question roster itself is a leaf vocabulary and lives in [`crate::question`],
which both this home and the planning home import. Nothing here restates it.

## The mandatory protocol

A projection kind declares which questions its plans answer, and every kind
carries the universal roster whether it lists it or not. A
[`ProjectionExplanationView`] is complete only when every applicable question has
exactly one answer — an unanswered seat, a doubled seat, and an answer to a
question the kind does not admit are all refused, each naming the question. No
kind ducks the protocol by answering fewer questions than its roster, because the
roster is what the view is checked against.

## Identities, not prose

Every [`ExplanationAnswer`] carries typed values and exact identities. Two things
are derived from the answer and neither is a seat a caller fills: the question it
belongs to, and the human projection a person reads. A mismatched
question-and-answer pair is unrepresentable rather than validated, and so is a
sentence that contradicts its answer — the rendering is composed from the answer
when it is asked for and is never stored, so there is no second value to keep in
agreement. Nothing reads the rendering back.

## The seats

`types.rs` declares. Its own child `type_guard.rs` holds every road that reaches
a private field, which is what makes "the question comes from the answer"
structural: there is no seam anywhere that files a true answer under a question
somebody supplied. `project.rs` renders one answer for a person, exhaustively
and from static literals proven at compile time, which is the same claim for the
sentence. `type_contract.rs` states the refusal family's shape and the closed
table that maps an answer to its question. `establish.rs` is the coverage pass
and the admission answer a caller asks for one kind — reaching no private seat,
because the body the established issues amount to is built beside the seat it
fills.
