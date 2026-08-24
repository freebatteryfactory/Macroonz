# diagnostic — one typed value per observation

A request that fails any step is refused whole, and this is the value that says why.

A `Diagnostic` names the step that was running, the exact site, one plain summary line, the contract that was expected, how what was found differs from it, the other issues it points at, the repairs their owners declared, and a route to reach the same observation again without a proc macro.
Every rendering of it — the compiler error, a machine-readable report, whatever a tool puts in front of a person — is a projection of that one value.
A projection may differ in shape, ordering, and verbosity; it may never differ in what it claims.

## One line, one grammar

```text
<prefix>: <class>: <first established issue>[<body>][<site>]
```

The prefix is the consumer's, read off the door.
The class is read off `RefusalClass` rather than spelled at the seam that refused, so two steps reporting one class read as one class and a build log groups them together.
The first established issue is stated in full, the body clause says how many stand behind it and whether the set that names them is complete, and the site clause is written only where the refusal sits somewhere narrower than the declaration.

One line, because one line is what a compiler shows.
Nothing is lost to it: every established issue has its own identity in the related set, and the typed body is the value the caller of the underlying step still holds.

## The door

`Door` is the one value that says who is asking — the diagnostic prefix a consumer's users read, the stable name of its declaration grammar, the stable name of its entry point, the crate its rendered paths are rooted at, and the producer it stamps.

Say it once and every diagnostic carries it.
The prefix opens every line; the two declared names become the expected contract and the reproduction route, derived here under the declared-name grammar at the two positions this compiler assigns.
No spelling of any of the five lives in this crate, and this home reads only the first three.

## Refusing generically

Each step of the road refuses in the vocabulary of the home that owns it and implements `Refused` to say how that vocabulary reads: its class, its first issue, its observed classification, what its line is a summary of, the canonical material of every issue it established, and the repairs their owners declared.

`Diagnostic::refused` is the one road from any of them to a diagnostic.
So a user of three different derives built on this compiler reads three diagnostics shaped one way, and no home composes a sentence of its own shape.

Two facts ride on the error's *type* rather than on the call: the phase it is raised at, and the family tag its related identities derive under.
A step refuses at one phase of the road, and a type whose refusals span two steps is two types.
A family tag is preimage material, so a call site free to name one is a call site that can derive this refusal's issues in another refusal's space.

`Refused::related` answers with the canonical material of every issue the body established, in the order it established them, and with nothing at all where the refusal enumerated none.
That material is what a related identity stands over, so two bodies differing in any typed member must answer with different bytes.
The completeness of an encoding is the refusing home's to state, where it writes it.

## Repairs are cited, never composed

Every `Repair` carries the owner fact that declares it.
This home reports which declared repair applies; it does not write advice.

And the standing prohibition: no repair ever suggests deleting a declared capability so that generation compiles.
Making a program smaller until the compiler stops complaining is a silent narrowing of what that program promised, not a repair.

## Two levels over one material

A related set commits twice over one refusal: once to the whole body, which is a commitment to every issue at once, and once per established issue on its own.
They are two Rust types under two identity subjects, so seating one where the other belongs does not compile, and identical bytes at the two levels derive unrelated identities rather than one.

The set derives both levels itself, out of the issue material, in a single act.
A road that took a body identity and a set of issue identities as two arguments would take two halves that do not check each other — each honestly derived, and the pair naming one refusal's body over another refusal's issues.

Where a body outruns the declared bound the body's identity is carried alone, and the capping beside it states how many per-issue identities are missing.
That is a coarser commitment to the same refusal, never a shorter commitment to a different one, and the summary line says so out loud because the typed capping is not something a compiler shows.
