# 08_schema — the schema plane and the construction-refusal register

Band 08. Imports identity, refusal, value, and the root calculus. The largest
register in the machine: five identity-class production instantiations, the
four value-shape axes, nine refinement kinds, the seven-stage pipeline,
migration's twelve boundaries, compatibility, codec profiles (seated here by
band math — a codec binds a schema relationship, and bytes cannot import
schema), and the seven collection-shaped construction-refusal families with
their complete rosters and compile-time bounds.

## Variant-spelling law (verbatim, mechanical)

Negated adjective where the book states it (`Unresolved`, `Contradictory`,
`Unbounded`, `Ownerless`, `MechanismDefined`, `Ambiguous`, `Overlapping`,
`OutOfRange`, `WrongRole`, `WrongProfile`, `NonReconstructable`,
`Nondeterministic`); `Not` before a positively stated property (`NotTotal`,
`NotInspectable`, `NotLanguageNeutral`, `NotIndependentlyEvaluable`); the
prohibited act itself (`HiddenIoOrEffect`, `ContextualClaimDeclared`,
`CompositionWeakensChild`, `TransformOutsideBoundedLane`,
`AuthorityClaimedFromArrangement`, `SelectiveAccessSkipsRequiredValidation`,
`HostRepresentationAsCanonicalBytes`, `AcceptedHistoryRewriteDeclared`,
`BoundaryUnderstated`, `ShredMappedToAbsence`, `VersionOrderSubstitution`,
`ParserBehaviorSubstitution`); bounds in exactly two spellings (`Unbounded`,
`<thing>BoundsMissing` — always plural). Never a third invented form.
Deliberate cross-family spelling collisions over distinct types are lawful:
`SourceOrTargetMissing` and `LossPostureMissing` (migration & compatibility),
`Nondeterministic` and `HiddenIoOrEffect` (refinement & migration),
`TransformOutsideBoundedLane` (schema & migration), `Unbounded` (contract,
refinement, layout).

## The permanent hostile corpus (binding here; contents are the evidence home's)

26 named cases: unknown-required member accepted; optional extension silently
discarded despite a preservation promise; opaque extension interpreted as
authority; colliding or recycled field identities; default silently inserted
during validation; normalization hidden inside decoding; Rust closure treated
as a portable refinement; contextual lookup hidden inside validation; borrowed
view escaping its extent; selective access bypassing required validation; issue
enumeration exceeding its bound; generated and reference validators
disagreeing; wrong-codec-role bytes accepted; semantic-invalid value encoded
canonically; noncanonical value accepted as canonical; trailing/overlapping
hidden object; dynamic migration value carrying live authority; skipped
migration edge; lossy migration reported as lossless; codec re-encoding
mislabeled as schema migration; re-encryption drifting protected identity;
ciphertext digest substituted for a protected semantic commitment; public
low-entropy plaintext digest; shred converted into missing/null/empty; current
reader removed while its published compatibility promise remains.

## Representation gates (minted, unresolved by design)

Value-carrier gate (this home) · refusal-carrier gate (repository plane) ·
presentation-value gate (dead with the language; revives with a frontend) ·
numeric-width route (evidence-selected). A universal opaque-storage token is
forbidden — gates are owner-named, never generic.

## Flags carried

Codec roster 14-vs-15 count discrepancy (the primary's own arithmetic — the
"which profile item" payload stays unreportable until settled).
`MeaningOrBindingNotPreserved` authored as seven preservation objects
(null-and-extension read as one law); the 5-vs-7 ambiguity flagged.

## Obligations

```yaml
home: 08_schema
obligations:
  - id: schema.validation-pipeline-is-seven-ordered
    challenge_kind: compile-law
    green: laws.rs schema::validation_pipeline_is_seven_ordered
    red: owed-to-testpak
  - id: schema.value-shape-axes-are-four-closed-enums
    challenge_kind: compile-law
    green: laws.rs schema::value_shape_axes_are_four_closed_enums
    red: owed-to-testpak
  - id: schema.refinement-kinds-are-nine-and-properties-nine
    challenge_kind: compile-law
    green: laws.rs schema::refinement_kinds_are_nine_and_properties_nine
    red: owed-to-testpak
  - id: schema.migration-boundaries-are-twelve
    challenge_kind: compile-law
    green: laws.rs schema::migration_boundaries_are_twelve
    red: owed-to-testpak
  - id: schema.protected-transformations-are-six
    challenge_kind: compile-law
    green: laws.rs schema::protected_transformations_are_six
    red: owed-to-testpak
  - id: schema.seven-families-are-collection-shaped-with-roster-bounds
    challenge_kind: compile-law
    green: laws.rs schema::seven_families_are_collection_shaped_with_roster_bounds
    red: owed-to-testpak
  - id: schema.nested-causes-nest-distinct-families
    challenge_kind: compile-law
    green: laws.rs schema::nested_causes_nest_distinct_families
    red: owed-to-testpak
  - id: schema.identity-instantiations-declare-two-columns
    challenge_kind: compile-law
    green: laws.rs schema::identity_instantiations_declare_two_columns
    red: owed-to-testpak
  - id: schema.dual-agreement
    challenge_kind: differential
    green: owed — the generated-vs-reference validator comparison lands with testpak
    red: owed-to-testpak
```
