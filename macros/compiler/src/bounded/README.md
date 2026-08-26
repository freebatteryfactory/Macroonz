# bounded — collection shape under an owned ceiling

This home makes a collection's maximum cardinality part of its Rust type and keeps every field behind the constructors that establish that cardinality.

The const parameter is only a ceiling.
The semantic home holding a bounded collection owns the meaning of that ceiling and names the constant supplied to it.

```mermaid
flowchart LR
    offered[[Offered items]]
    bounded{"May empty be lawful?"}
    required{"Must one item exist?"}
    retained[[Retained prefix]]
    refusal[[Typed refusal]]

    offered --> bounded
    offered --> required
    offered --> retained
    bounded -->|"yes, within N"| B["Bounded&lt;T, N&gt;"]
    bounded -->|"more than N"| refusal
    required -->|"one through N"| NE["NonEmpty&lt;T, N&gt;"]
    required -->|"zero or more than N"| refusal
    retained --> C["Capped&lt;T, N&gt;"]
    C --> posture{"What happened?"}
    posture --> complete[Complete]
    posture --> truncated["Truncated { omitted }"]

    classDef value fill:#d9f3ff,stroke:#087e8b,color:#102a43
    classDef decision fill:#fff2cc,stroke:#c27c0e,color:#3d2b00
    classDef refused fill:#ffe0e0,stroke:#b42318,color:#4a1010
    class B,NE,C,offered,retained value
    class bounded,required,posture decision
    class refusal refused
```

## Refusal and capping answer different questions

[`Bounded::new`](Bounded::new) and [`NonEmpty::new`](NonEmpty::new) admit the complete offering or return a typed refusal.

[`Overflow`] carries the ceiling and the offered count.
[`NonEmptyError`] distinguishes an absent required item from an offering wider than its ceiling.

[`Capped::first_n`](Capped::first_n) deliberately keeps the prefix that fits and records the exact omitted count as [`Capping`].
The caller never supplies that capping posture.

## Construction and reading

[`Bounded`] may begin empty and may grow only through [`Bounded::try_push`], which refuses before changing the held sequence when the next item would exceed the ceiling.
[`Bounded::from_array`] settles a fixed offering's fit at compile time.

[`NonEmpty`] stores its first item separately, so [`NonEmpty::first`] is total and no empty branch exists for a reader to write.
[`Capped`] can be constructed only from a lawful non-empty collection or by the prefix-capping road.

There is no unchecked `Vec` conversion, mutable iterator, or dereference escape hatch.

## Ownership boundary

This home owns cardinality shape, retained order, capping posture, and construction refusals.
It does not own a canonical byte encoding for arbitrary `T`.
Each semantic holder that derives identity or bytes from one of these collections owns that encoding and consumes the public ordered readers.
