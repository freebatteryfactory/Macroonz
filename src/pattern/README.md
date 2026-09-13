# Reusable source patterns

This home composes ordinary compiler tokens into reusable stamp patterns for caller-owned declarations.
It supplies repeated structural mechanics; the compiler stamp owner retains matcher/invocation agreement, bounded sites, visibility transport and publication records.
No pattern writes files or acquires publication authority.
The root facade exposes this composition in every feature posture as `macroonz::pattern`.

## Admitted newtypes

`admitted_newtype(parameters, arguments, predicates)` returns an ordinary `compiler::stamp::Pattern`.
Each slice contains exact completed token fragments for one generic parameter, its corresponding argument, or one where predicate.
Empty slices select a nongeneric pattern.
The chosen generic shape belongs to the pattern and applies to every site that adopts it; rustc owns syntax, bounds, lifetime validity and parameter/argument agreement.
The [admitted-types example](../../examples/admitted_types/README.md) renders nongeneric and borrowed generic forms and supplies a standalone caller.

The pattern declares six site arguments, in this order: exported type name, private home name, representation attribute, stored type, refusal type and admission function path.
The site's visibility is the stamp's existing reach coordinate.
Its generated invocation has this shape:

```text
chosen_stamp! {
    pub Identifier in identifier_home;
    layout transparent;
    value u64;
    refusal crate::AdmissionError;
    admit crate::admit_identifier;
}
```

The private home contains one nominal `Value` type with a private stored field, reexported under the site's chosen name and visibility.
The admission function receives the stored type by value and must return `Result<Stored, Refusal>`.
`try_new` invokes that function before constructing the wrapper; `as_inner` shares the admitted representation, and `into_inner` consumes the wrapper to return it.
There is no generated unchecked constructor, mutable accessor, Default, Clone, Copy, conversion trait or wire format.
The caller may write further implementations through these public operations while preserving its own validation and encoding policy.

The layout clause supplies the argument of Rust's `repr` attribute, such as `transparent`, `C` or `Rust`.
Rustc refuses a layout incompatible with the stored type and shared accessor, including an unaligned reference from a packed field.
Stored, refusal, validator and predicate paths must resolve inside the private home; explicit crate paths avoid accidental invocation-site imports.
The home name must be distinct within its containing module, and a caller generic named `Value` collides with the pattern's internal type and refuses in rustc.

Checked construction does not establish that a caller's admission function is truthful or that a representation with interior mutability preserves a semantic invariant forever.
Independent consumer tests own those claims.
The pattern carries no identity, clock, namespace or subject-specific validation policy of its own.
For addressed publication, pass the returned pattern through the stamp owner's [publication road](../../macros/compiler/src/stamp/README.md#runnable-publication).

## Fixture registration

Typed fixtures and their registration use the existing [trial declaration](../../macros/compiler/src/descriptor/trial/README.md) and [trial-table owner](../../harness/src/descriptor/README.md).
The [trial workflow](../../examples/trial_workflow/README.md) supplies the ordinary root entrance with independent callable checks.
This home does not introduce another fixture runner or registry.
