# 20_derived — DataBlocks, masks, materialization, physical plans

Band 20. Imports bytes (ContentRegionId — extent identity IS the Tier-1
region, no parallel notion), history (FederationCutEntries mechanism,
CommitPoint via it), identity, refusal, and the root calculus. The derived
plane: never authority, always rebuildable, four core rules (R1–R4) in the
module docs.

## The settled decisions

- **The two-seat hybrid**: preimage-derived `RowDomainId` (seat 1, gating
  every composition)
  + fresh `OccurrenceId` (seat 2, naming which build), neither doing double
  duty; `RowDomainEqualityUnproven` is a DISTINCT fail-closed outcome from a
  proven mismatch — fail-closed is not permission to report the stronger one.
- **Protected index (D59, resolved to option 3)**: no first-party family
  ships; bounded decrypt-and-scan or application-owned indexing with no
  machine authority; the nine-item REVERSIBLE STANDING BAR is carried as a
  machine-readable const — a standing bar, not an admitted capability, and
  never re-flattened into a permanent ban.

## Nuances carried in the code

`SelectionMaskConstruction`'s check order is NORMATIVE LAW (dependent checks;
the gate propagates; first-established wins; declaration order is never the
rule) with the six deliberate absences documented on the family;
`SourceBinding`'s two FORMS make the pinned bodies of causes 3 and 4
structural (form-disagreement vs both-generation-form); `LengthMismatch` may
carry both lengths canonically while the release law names row counts a
cardinality disclosure no boundary emits absent contract (the ingress
decision's typed-redacted-diagnostic default is the neighboring answer — flag
carried for the release owner at 22); the seven validity conditions never
collapse into one bit; `MaterializationSourceCuts` is a role-distinct newtype
over history's carrier (one mechanism, never one meaning — the DRY law makes
the sharing the default); the three materialization axes plus staleness as
the fourth (evidence) axis — staleness never occupies a presence or
availability variant; `MaterializationCoverage` rides root `Completeness`
(non-erasable domain); the `PlanTemplate`/`PlanBinding` split with the
ephemeral-bound-specialization rule; scratch-consumed-or-dropped; the
construction-lifecycle vocabulary renames executed per the repository
vocabulary law (the record is the evidence home's, at 23).

Mechanism-diagnostic rosters are per-host-profile declarations, never core
content: `DATA_SEMANTIC_WORK` is the portable surface, and the law that a
diagnostic never becomes semantic work is carried on it.

## The parity contracts (the navigation heartbeat's derived half — testpak's
campaigns)

P1 DataBlock ↔ journal evaluation (a derived result equals a simple
accepted-history fold over the same admitted cut) · P2 scalar-route parity ·
P3 mask-model parity · P4 late-materialization plain-value parity · P5
durable-byte independent-reader parity · P6 differential parity per optimized
kernel · P7 fusion's TWO invariants (algebraic fold-combination + concrete
per-consumer preservation — proving the first does not prove the second) ·
P8 semantic-work invariance across realizations · P9 profile equivalence at
one frozen cut · P10 deterministic-order parity. Full-denominator honesty:
every attempted proposal, target, workload, hostile, timeout, refusal,
unsupported row, infrastructure failure, and not-run row stays in the
denominator; benchmark victory excuses nothing; a Cargo feature is not proof
of support.
