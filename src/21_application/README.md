# 21_application — applications and the remote face

Band 21. Imports history (RemovalCommitment), identity, and the root
calculus. The local face (identity model, composition, invocation profiles,
resources) and the remote face (the one global interaction contract, carrier
vocabularies, the ingress paved road).

## Executed renames (recorded decisions)

- **Serve is dead as a package; the type prefix died with it.** The contract
  types live here under plain names; the participant projections (client,
  server, session, carrier shells) are MACRO-GENERATED from the contract
  (hosts opt in via features); physical obligations are the host repos'.
  `ServeRequestId → CarrierRequestId` (approved) — it correlates protocol
  messages on a carrier, and the identity-separation wall becomes
  `CarrierRequestId ≠ PortRequestId`, related only through typed carriage.
  `ServeSessionState → SessionState`, `ServeSessionTerminal →
  SessionTerminal`, `ServeStreamState → StreamState`, `ServeDirectionState →
  DirectionState`, `ServeStreamClosure → StreamClosure`.
- **`DeliveryGuarantee`** (AUTHORED rename): the old remote-face
  `DeliveryRole {BestEffort, ResumableAtLeastOnce}` collided with the
  runtime home's four-role delivery enum — one spelling, one meaning.

## The standing ingress decision, baked

The progressive ack ladder (only `Admitted` discharges the sender's retry;
`SingleAck` is admitted-only), the no-default ack profile (symmetric,
safety-equivalent — the interface selects, per the paved-road
classification), the four-rung idempotency identity ladder with the
only-lawful-reservation-token story, effectful-ingress-with-no-identity
refuses, the typed-redacted-diagnostic default with per-reason-class opt-in
raw retention under four guardrails, and the three marked deployment
tunables (idempotency retention window, raw-retention window, per-class
capture) — tunables, never product decisions.

## Scope-guard eleven

`ActivationGeneration = AuthorityPosition<InstanceId>` — scope and order
only; which image a generation activated rides `ActivationImageBinding`, a
typed relation, never the ordinal's bytes.

## Owed onward

The sans-I/O protocol core's step machinery, the generated projections, and
the carrier design inventory (a design denominator, never a support claim) →
the metaprogramming services (macroc) + hosts; participant surfaces are
projections of the one global contract, produced by the projection engine
and exposed through macro, xtask, or host tooling surfaces. Restricted-query
eligibility derives from the operator register's declared effect and
suspension posture at 15_execution — never from a roster of operation names
kept here; the derived predicate is owed to the metaprogramming services
(macroc). The information-release contract and
firewall authorship → 22. Projection-completeness and trace-equivalence proof
→ 23 (a consumer of the contract, never its co-owner).

## Obligations

```yaml
home: 21_application
obligations:
  - id: application.activation-generation-rides-scope
    challenge_kind: compile-refusal
    green: laws.rs application::activation_generation_rides_scope
    red: owed-to-testpak — cross-instance compare must not typecheck
  - id: application.identities-do-not-merge
    challenge_kind: compile-refusal
    green: laws.rs application::identities_do_not_merge
    red: owed-to-testpak — wrong-role substitution must not compile
  - id: application.invocation-profiles-are-three
    challenge_kind: compile-law
    green: laws.rs application::invocation_profiles_are_three
    red: owed-to-testpak
  - id: application.carrier-identities-stay-apart
    challenge_kind: compile-refusal
    green: laws.rs application::carrier_identities_stay_apart
    red: owed-to-testpak — a carrier id satisfying a port id must not compile
  - id: application.session-and-stream-vocabularies
    challenge_kind: compile-law
    green: laws.rs application::session_and_stream_vocabularies
    red: owed-to-testpak — a linear half-close enum must not exist
  - id: application.ingress-ladders-hold
    challenge_kind: compile-law
    green: laws.rs application::ingress_ladders_hold
    red: owed-to-testpak — a non-admitted discharge must be unreachable
  - id: application.contract-and-rosters-hold
    challenge_kind: compile-law
    green: laws.rs application::contract_and_rosters_hold
    red: owed-to-testpak
```
