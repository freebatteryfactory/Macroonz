# 16_image — ProgramImage and admission

Band 16. Imports execution (KernelRequirementSet, the kernel-version pair
amended there), bytes (ContentRegionId), identity, and the root calculus. The
self-explaining executable package: identities, the component table, packaging,
the affine validation ladder, and the sixteen-stage admission pipeline.

## The component table is fixed at the bytes

The `img` frame row in the byte-profile register fixes the
component table AT THE BYTES: `u32 count × { role u16, profile u16, content
digest 32 B, length u64 }` — so `ComponentRole`'s wire form is a registered
`u16` and the component profile is a registered `u16`, typed accordingly.
Self-contained packaging inlines each component as a frame under the physical
cap; a larger component must be a content region — the tiering law expressed
in packaging.

## Authored fresh here (flagged): the ComponentRole roster

The image's eighteen bound facts are complete; which component roles carry
them is decided here. AUTHORED as nineteen roles —
one per separable bound-fact carrier, identities riding the root frame header
— with the three optional roles (origin maps, authenticity, qualification)
admitted only where a profile admits them.

## The identities

`ImageDigest` (Class B, exact bytes — never the semantic home's meaning
digest), `ProgramImageRef` (Class E riding the root evidence-reference
shape), `ImageFamilyFormatVersion` and `ImageProfileVersion` (the seventh and
eighth scope-guard instantiations; `SemanticKernelVersion` — the ninth —
amended into the execution home where kernels live), `AdmittedProgramId`
(Class D, minted only by the pipeline's final stage — admission does not
mutate image identity). Supplying one identity where another is required is a
compile-time wrong-role refusal; unknown required meaning refuses, never
silently ignored.

## The selected default (Decision D-IMG-2)

SelfContained is the selected paved-road default — offline verification,
regulated/air-gapped deployment, agent handoff, reproducibility. ImmutableBound
and Hybrid stay first-class. Selecting the default narrows nothing.

## The ladder's minting monopoly

`AgreementCheckedImage` and `ExecutableImage` are minted ONLY by the
independent agreement verifier — never by literal. `ImageValidation` is the
durable record of the reached phase, not the live handle; a decoded record
re-enters live use only through re-validation.

## Owed upward

ApplicationImage composition/interface/instance/lifecycle → 21 (this home
owns the shared image-family packaging law). The invocation-admission
authority-bound value, ExecutingAttempt, and the continuation record → 17/18/19.

## Obligations

```yaml
home: 16_image
obligations:
  - id: image.identities-ride-scope-guards
    challenge_kind: compile-refusal
    green: laws.rs image::identities_ride_scope_guards
    red: owed-to-testpak — cross-family/cross-profile compare must not typecheck
  - id: image.component-roster-is-authored-nineteen
    challenge_kind: compile-law
    green: laws.rs image::component_roster_is_authored_nineteen
    red: owed-to-testpak
  - id: image.program-image-composes
    challenge_kind: compile-law
    green: laws.rs image::program_image_composes
    red: owed-to-testpak
  - id: image.validation-ladder-is-five-and-minted
    challenge_kind: compile-refusal
    green: laws.rs image::validation_ladder_is_five_and_minted
    red: owed-to-testpak — literal construction of the two verifier-minted
      rungs must not compile
  - id: image.admission-pipeline-is-sixteen
    challenge_kind: compile-law
    green: laws.rs image::admission_pipeline_is_sixteen
    red: owed-to-testpak
```
