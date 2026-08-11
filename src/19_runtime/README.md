# 19_runtime — the Turn and the Stitch

Band 19. Imports bvisor (AttemptId, ReservationObservation), history
(CommitKnowledge), identity, and the root calculus. The sync-first logical
machine: the Stitch contract, the Turn and its fourteen phases, the identity
quartet, Attempt lineage as message-passing, the durable checkpoint, effect
recovery, reconciliation, the cancellation fact model, delivery, driving,
and supervision.

## The Turn/Stitch altitude (settled vocabulary applied)

The STITCH is the transition contract (state + one observation → one bounded
deterministic transition → one of seven outputs); the TURN is the identity of
one such transition over frozen inputs. The Stitch trait's concrete Rust
shape (the corpus's own open) lands with the runtime machinery — the seven
outputs, the fifteen-item driver-invariance list, and the seven driver
freedoms are law now.

## TurnId is the machine's first DERIVED Class-D production identity

Replay-stable under the derived-seat law: replay is the named consumer of
convergence and the runtime custodies the seven-part preimage — replaying the
same transition reconstructs the same identity. AttemptId (18's, imported)
stays fresh-per-effort; the quartet never merges.

## Nuances carried

TurnPhase fourteen flat (the prose's pairs documented on the enum; persisted
vocabulary is flat); BoundedCauseSet is a SET (order-insensitive membership;
deterministic storage order is canonical-emission-only); the `…Edge` spelling
stays reserved and a bare `Cause` is refused; the checkpoint's eight closed
non-reasons; ProcessStateRole is the AUTHORED name for the four fixed roles;
the recovery profile's five pair-fact axes are records with named sub-axes,
never booleans, every claimed property bound BEFORE the irreversible Attempt;
the freshness-requirement sub-axis is deliberately NOT the root freshness
axis (demands vs is); ReconciliationDisposition lives ONLY inside
`Complete(_)` — disposition-without-completion is unrepresentable; the old
two-variant cancellation outcome is RETIRED in favor of the fact model;
`BoundOutcome::ResourceExhausted` binds 18's observation across the
Attempt-existence line with no conversion; Permit custody-is-not-proof with
no generic released flag; supervision strategy names deliberately not frozen.

## Owed onward

Serve/session/DeliveryIndex carriage → 21. The live Mailbox/Broadcast
mechanisms, capacity profiles, and the Stitch trait → this home's machinery
phase. The calibration pair → 23.

## Obligations

```yaml
home: 19_runtime
obligations:
  - id: runtime.stitch-contract-and-driver-invariance
    challenge_kind: compile-law
    green: laws.rs runtime::stitch_contract_and_driver_invariance
    red: owed-to-testpak
  - id: runtime.turn-identity-quartet
    challenge_kind: compile-law
    green: laws.rs runtime::turn_identity_quartet
    red: owed-to-testpak — a quartet merger must not compile
  - id: runtime.attempt-lineage-is-message-passing
    challenge_kind: compile-law
    green: laws.rs runtime::attempt_lineage_is_message_passing
    red: owed-to-testpak
  - id: runtime.checkpoint-advances-only-on-prerequisites
    challenge_kind: compile-law
    green: laws.rs runtime::checkpoint_advances_only_on_prerequisites
    red: owed-to-testpak
  - id: runtime.effect-recovery-has-nine-axes
    challenge_kind: compile-law
    green: laws.rs runtime::effect_recovery_has_nine_axes
    red: owed-to-testpak
  - id: runtime.reconciliation-and-cancellation-axes
    challenge_kind: compile-law
    green: laws.rs runtime::reconciliation_and_cancellation_axes
    red: owed-to-testpak — a disposition outside Complete must not compile
  - id: runtime.delivery-and-bound-outcomes
    challenge_kind: compile-law
    green: laws.rs runtime::delivery_and_bound_outcomes
    red: owed-to-testpak
```
