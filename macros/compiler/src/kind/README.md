# kind

What you are generating, declared once, in your own crate.

The compiler is generic over a kind from the first step of the road to the last.
It never enumerates the kinds that exist, never registers one, and holds no list you have to get onto.
Five open traits are the whole contract, and you implement them where your grammar already lives.

## The five

A **kind** is what one request produces: a declared name, the canonically encoded content a request carries beyond its tokens, the seats its rendering fills, and the questions it owes.

**Canonical content** is the complete semantic encoding of those kind-specific facts.
The kind owns that encoding because only its declaring adapter knows which facts its renderer may read; the compiler frames the result and binds it to the exact capture and owner-qualified kind before planning.

A **role** is one seat.
A rendered unit is matched to a planned one by role, so a rendering that produced the right number of units in the wrong seats is caught by the seat rather than by a count.
`ALL` is the quantifier every walk over a rendering uses, and a role says for itself where its unit lands.

A **question** is something a kind owes an answer to beyond what every kind owes, and an **answer** is the typed value that answers one — its canonical bytes and the sentence a person reads.

```ignore
impl Kind for GreetImpl {
    const NAME: &'static str = "greet.impl";
    type Content = Greeting;
    type Role = SoleRole;
    type Question = NoQuestions;
}
```

`SoleRole` is the roster of a kind that renders exactly one unit, at the declaration site.
`NoQuestions` is the roster of a kind that owes nothing past the universal questions; it is uninhabited, so it is also its own answer.

## Where a unit lands is the role's own fact

`Destination` has four rows — the declaration site, the test carrier, the bench carrier, and a publication artifact — and a role names one of them.

A destination is a property of the seat and not of a particular plan, so two plans of one kind cannot disagree about where their units go, and a reader asking which build compiles a unit reads the role rather than tracing the value.

## Dispositions

`Disposition` is what happened to one kind that could have been generated.

Silence is not one of its answers.
Where a projection is absent the absence has a name and cites the fact that caused it, so a reader asking why one kind is missing from a set is never handed a gap.
There is no refused answer, because a request that fails any step of the road is refused whole and produces a diagnostic rather than a set.

`KindSet` binds a declared set of kinds to the record that carries one required seat per kind.
A seat that nobody filled is a struct field nobody wrote, which stops the compiler; a kind added to a set breaks every construction of the record again, which is the point of stating the set once.

## What is not here

There is no seal.
Implementing `Kind`, `Role`, `Question`, or `Answer` is an ordinary implementation in your crate, and the compiler learns of your kind the moment you hand it a request.

The universal questions — what you are, which owner required you, which declaration caused you, which profile you were decided under, which output identity and digest you are, which assumptions you rest on, what invalidates you, why a related projection was not generated, and what repairs a refusal — are the compiler's own and live in `explanation/`.
They are never restated in a kind's roster, so a kind cannot narrow what it must explain by forgetting a row.

The compiler cannot establish that an adopter's canonical encoding is complete.
That is the declaring adapter's authority, and [`CanonicalContent`](crate::CanonicalContent) makes the required operation explicit instead of substituting `Debug`, `Hash`, or the captured bytes for a value the renderer may interpret more narrowly.

## The two stamps

`roster!` writes a plain vocabulary down: an enum, its `ALL`, and one declared name per row, from a single declaration.
It is for a closed list of names — this home's own `Destination` is written with it.
A role is not written with it, because a role also names a destination; the six lines of an `impl Role` say that better than a stamp with an extra column.

`kinds!` writes a set down: one marker type and its `Kind` implementation per row, the enumerated set, its `KindSet` implementation, and the disposition record with one required seat per row.
One declaration, so the marker, the set, and the record cannot drift apart.

## The seats

`types.rs` declares the five traits, the two compiler-owned rosters, the destination, the disposition, the set contract, and the two stamps.
`type_contract.rs` implements the traits for the two rosters this home owns.
Nothing here holds a private field, so there is no invariant nucleus and no `type_guard.rs`.
