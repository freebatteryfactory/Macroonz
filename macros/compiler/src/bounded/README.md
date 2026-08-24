# bounded — the compiler's own collections

Three lists that carry their ceiling in their type, and the two ways a list refuses to be built.

Every list the compiler holds has a bound.
Written here, once, the bound stops being something a downstream seat has to remember and starts being something it cannot get wrong.

## The three

| Type | Holds | Refuses with |
| --- | --- | --- |
| `Bounded<T, N>` | zero to `N` items | `Overflow` |
| `NonEmpty<T, N>` | one to `N` items | `NonEmptyError` — `Empty` or `Overflow` |
| `Capped<T, N>` | one to `N` items, and how it was capped | nothing |

`Bounded` is the ordinary list.
`NonEmpty` is the list whose first item is a fact rather than a lookup, so `first()` hands back a `&T` and no caller writes a branch for a case that cannot happen.
`Capped` is the shape of a report body: everything that fit, plus an honest count of what did not.

## The ceiling is a number

`N` is a plain const-generic parameter and the limit behind it is a plain constant on the home that owns the thing being bounded — `pub const REPAIR_LIMIT: usize = 8;` written beside the repairs it governs, and spelled into the type as `Bounded<Repair, REPAIR_LIMIT>`.

There is no limit trait, no family, no magnitude, no authority, and no profile that admits one.
A ceiling belongs to whoever declared the collection, at the seat where a reader is already looking.

## Refusing and capping are different questions

`Bounded::new` and `NonEmpty::new` refuse.
Too many items comes back as an `Overflow` carrying both numbers — what fits, and what was offered — and the caller decides what that means.

`Capped::first_n` does not refuse.
It keeps what fits and writes down how many it dropped, because a report that would rather say nothing than say the first eight of nine issues is not a report.
`Capping` is that record — `Complete`, or `Truncated { omitted }` — and it is derived from the items themselves, never supplied by the caller.

## What is not here

No `push`, no `iter_mut`, no `Deref`, no `From<Vec<T>>`.

A bounded list is built once, through a constructor that establishes its bound, and read from then on.
That is the whole reason the bound is structural rather than remembered: there is no second road in.
