# diagnostic — one typed value per observation

A request that fails any step is refused whole, and this is the value that says why.

A `Diagnostic` is one complete typed observation of the step, site, expected contract, observed difference, related issues, declared repairs, and reproduction route.
Every rendering is a projection of that value and may differ in shape, ordering, or verbosity without differing in what it claims.

## One line, one grammar

Every diagnostic line is composed from the typed observation and the request-owned door through the one projection this home owns.
The exact line grammar belongs to that projection's rustdoc and external observer.

## The door

The [`request`](../request/README.md) home owns `Door` and its construction contract.
This home consumes the door when it projects a refusal and reexports that request type at `diagnostic::Door` as a compatibility path, not as a second declaration.

## Refusing generically

Each step of the road refuses in the vocabulary of the home that owns it and implements `Refused` to say how that vocabulary reads: its class, its first issue, its observed classification, what its line is a summary of, the canonical material of every issue it established, and the repairs their owners declared.

`Diagnostic::refused` is the one road for caller-placed refusals.
Where the refusing act itself establishes the site, the refusal type exposes a typed diagnostic road that accepts no second placement.
So a user of three different derives built on this compiler reads three diagnostics shaped one way, and no home composes a sentence of its own shape.

Two facts ride on the error's *type* rather than on the call: the phase it is raised at, and the family tag its related identities derive under.
A step refuses at one phase of the road, and a type whose refusals span two steps is two types.
A family tag is preimage material, so a call site free to name one is a call site that can derive this refusal's issues in another refusal's space.

`Refused::related` answers with the canonical material of every issue the body established, in the order it established them, and with nothing where the refusal enumerated none.
The completeness of that encoding remains the refusing home's contract.

## Repairs are cited, never composed

Every `Repair` carries the owner fact that declares it.
This home reports which declared repair applies; it does not write advice.

And the standing prohibition: no repair ever suggests deleting a declared capability so that generation compiles.
Making a program smaller until the compiler stops complaining is a silent narrowing of what that program promised, not a repair.

## Related-set evidence ceiling

A related set distinguishes the whole established body from each issue within it and derives both levels together from the refusing home's canonical material.
Its exact derivation and capping contracts belong to `RelatedSet` rustdoc and external observers.
