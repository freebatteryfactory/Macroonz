# 18_bvisor — the boundary supervisor

Band 18. Imports port (PortFamilyVersion, PortPostcondition — the planted
seat collecting), identity (ConstraintSourcePair), semantic, bounds, refusal,
and the root calculus. The physical membrane: admission, the Attempt
lifecycle, the reservation contract, physical observations, witnesses, the
port crossing, and containment.

## Band-forced seat executed: AttemptId declares here

The old reading-order note put the Attempt identity with the runtime chapter;
band math and the minting law force it here — admission is the ONLY minting
site (`AdmissionOutcome::Admitted`), a refusal creates no Attempt at all, and
the runtime home imports the identity for lineage. Flagged as superseding the
reading-order note, same class as the SourceClosure seat.

## The fourteen-issue family's nuances (all carried in the code)

The inversion rule fixes the collection shape; `EarlyStopped` is the NORMAL
posture (the dependency-order halt is the stated reason, so the collection is
a singleton whenever the order stopped at a single-subject station); members
hold the issue vocabulary's canonical order, never evaluation order — the
evaluation order is security-sensitive and rendering it would republish it;
the canonical refusal is not the released refusal (constant-cardinality
grouped projections under hostile threat profiles; a grouped projection may
not authorize what a hidden canonical issue forbids); issue 6 alone carries a
constraint-source pair; issue 12 imports the port home's `PortPostcondition`
(no twin); stale is never wrong; missing is never stale.

## The Attempt-existence line

`ReservationObservation` is the one home of "the host could not satisfy an
otherwise lawful bounded request." The admission issue binds it to answer why
NO Attempt was admitted; the runtime's resource-exhausted outcome binds it to
answer how an ADMITTED operation ended. No conversion exists in either
direction — a failed admission creates no Attempt.

## The lifecycle's affine spine

PlannedInvocation is pre-Attempt; Admitted → Running → LiveSuspended →
Terminal are non-Clone, non-serializable live handles (raw-pointer phantom);
`TerminalAttempt::seal` CONSUMES the terminal state and mints the immutable
`AttemptReport` (sealing is not a phase; reconciliation is never a phase);
`AttemptState` is only the persisted projection, re-entering live custody
through validation, never construction.

## Obligations

```yaml
home: 18_bvisor
obligations:
  - id: bvisor.admission-family-holds-fourteen
    challenge_kind: compile-law
    green: laws.rs bvisor::admission_family_holds_fourteen
    red: owed-to-testpak
  - id: bvisor.attempt-minting-is-admissions-alone
    challenge_kind: compile-refusal
    green: laws.rs bvisor::attempt_minting_is_admissions_alone
    red: owed-to-testpak — an Attempt identity from any other route must not
      compile
  - id: bvisor.lifecycle-is-affine-and-sealed
    challenge_kind: compile-law
    green: laws.rs bvisor::lifecycle_is_affine_and_sealed
    red: owed-to-testpak — resuming a terminal Attempt must not compile
  - id: bvisor.reservation-has-one-home
    challenge_kind: compile-law
    green: laws.rs bvisor::reservation_has_one_home
    red: owed-to-testpak — a conversion across the Attempt-existence line
      must not compile
  - id: bvisor.containment-is-two-coordinates
    challenge_kind: compile-law
    green: laws.rs bvisor::containment_is_two_coordinates
    red: owed-to-testpak
  - id: bvisor.port-crossing-binds
    challenge_kind: compile-law
    green: laws.rs bvisor::port_crossing_binds
    red: owed-to-testpak
```
