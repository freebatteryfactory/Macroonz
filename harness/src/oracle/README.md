# oracle — the second opinion

Most of the harness judges a subject against laws its owner declared.
This home exists for the claims where that is not enough: where the only thing available to check an answer against is the logic that produced it, and agreement would establish nothing at all.

So the oracle goes and gets its answer from somewhere else.

## Four lanes, four different somewhere-elses

| Lane | Where the second opinion comes from | What it may claim |
| --- | --- | --- |
| golden vector | a pack of input-and-output pairs written from a specification | the producer rendered exactly these bytes for this input |
| independent transcript | this home's own re-encoding of a published preimage grammar | the specification, read by somebody else, names the identity that was published |
| structural read | a Rust parser that shares nothing with whatever rendered the artifact | the artifact declares this target, this contract, these members |
| compiled read-back | a compiler, which resolves and evaluates by its own rules | the artifact means what it spells, or the compiler refuses it |

A verdict is method-specific, and reporting one lane's answer as though it came from another is the collapse this home exists to refuse.
The structural read never claims a path resolves; the compiled read-back never claims anything about how the artifact is written.

## Which way a vector is allowed to travel

A vector is born from the specification and read here.
It is never exported from the thing under judgement: a vector copied out of a producer turns the lane into a mirror, and a producer that silently changes then agrees with itself forever.
The vectors belong to whoever owns the specification, and this home ships only the reader — so anyone writing vectors for their own types gets the same instrument.
`VectorPack::read` states the complete pack grammar, and nothing else in the harness reads one.

## The ceiling on the word "independent"

No comparison here earns independence by being described as independent.

These four lanes give method-specific evidence, good exactly as far as the method reaches.
Author-declared independence — the property suites' loudest arm — records what an author claims about two roads, never that anything established the claim.
A caller-supplied reference implementation is a second road like any other, and calling it a reference does not promote it to a judge.
The stronger status is a qualified implementation bound to the revision it was qualified at, with the qualification history kept behind that binding, and it is not built here.

## What is deliberately absent

There is no scan for a declared textual form in rendered output, and its absence is a decision rather than a gap.
Anchors like that are generator-invalidation data: they belong to whoever authors the rendered form, so a hand restating them beside the renderer would be maintaining a second spelling of somebody else's output.
