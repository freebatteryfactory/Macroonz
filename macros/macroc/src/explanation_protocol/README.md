# explanation_protocol — the typed answers, the parentage they were answered over, and the coverage that admits no partial view

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

## A view is ABOUT something, and it says what

Coverage says a view answered its kind's questions. It never said WHOSE questions
those were — and a kind is not an expansion: two plans of one kind admit the same
roster, so a view written over one of them covers the other's roster exactly.
A terminal handed such a view bound plan A, closure A, and a complete,
well-formed, correct-looking account of a different expansion.

So a complete view carries its PARENTAGE. It is completed over the PLAN and the
PROVED CLOSURE themselves — the closure through the sealed [`ProvedClosure`]
contract, which the closure home alone can satisfy — and it reads both identities
off those values rather than taking two a caller could name. It then mints its
own identity over the plan, the closure, and the canonical typed answers, so the
terminal that binds it has three names to compare and one to commit to.

The seats are stored in the kind's DECLARED question order, never the caller's,
which is what makes one set of answers one explanation however it was assembled.

## Identities, not prose

Every [`ExplanationAnswer`] carries typed values and exact identities. Two things
are derived from the answer and neither is a seat a caller fills: the question it
belongs to, and the human projection a person reads. A mismatched
question-and-answer pair is unrepresentable rather than validated, and so is a
sentence that contradicts its answer — the rendering is composed from the answer
when it is asked for and is never stored, so there is no second value to keep in
agreement. Nothing reads the rendering back.

## The seats

`types.rs` declares, including the sealed proof contract a view is answered over
and the two magnitude rows this home's capacities are governed by — meaning,
number, and reason on one row, stamped through the plane's `limits!`.
Its own child `type_guard.rs` holds every road that reaches a private field,
which is what makes "the question comes from the answer" structural — there is no
seam anywhere that files a true answer under a question somebody supplied — and
it is where a view's parentage is taken and its identity minted. `project.rs`
renders one answer for a person, exhaustively and from static literals proven at
compile time, which is the same claim for the sentence. `encode.rs` writes the
canonical bytes an explanation's identity is derived over, through each typed
value's own home's spelling and with no human prose in it. `type_contract.rs`
states the refusal family's shape, the closed table that maps an answer to its
question, and the answer roster's own discriminants. `establish.rs` is the
coverage pass and the admission answer a caller asks for one kind — reaching no
private seat, because the body the established issues amount to is built beside
the seat it fills.
