# `macroonz` — public contracts

This crate owns the small public contract algebra shared by Macroonz compilers, derives, and generated consumers.

Its limit families make capacity authority and compile-time admission explicit. `Bounded` and `NonEmptyBounded` preserve the governing family in the type, while `AdmittedLimit` and `PositiveLimit` carry the compiler-established facts required by checked construction.

Its refusal-family contracts distinguish body shape, stable cause identity, typed cause order, textual projection, and admitted coverage. Macroonz derives emit `SELECTION_ORDER` and `DECLARED_ORDER` from one captured declaration. Admission first checks shape coherence and then proves that the typed order projects to the emitted textual order.

`AdmittedPrefix` binds a non-empty issue body to the coverage posture produced by the same construction. `Commitment` is an opaque domain-tagged identity shape. `FieldCardinality` is the shared value-cardinality roster. `closed_register!` declares a closed enum together with its roster, stable names, descriptions, and positions from one row list.

The crate contains contracts only. It performs no capture, planning, rendering, expansion, qualification, runtime work, filesystem access, or product-specific interpretation.
