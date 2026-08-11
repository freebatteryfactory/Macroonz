# 12_port — the port contract algebra

Band 12. Imports schema, identity, refusal, and the root calculus. What a port
IS: one typed semantic boundary contract per family, the seventeen-fact
operation contract, outbound external operations, the foreign-claim admission
seam, the one-shot response-binding refusal, and the host-obligation shapes
mechanisms qualify against. The port-family-and-host-mechanism matrix is owned
here; the image, runtime, and Bvisor homes consume it and never re-mint it.

## The four-way law (binding, verbatim structure)

Capability authorizes a port operation; the port or authority-backend contract
proves its physical postconditions; the boundary supervisor mediates and
observes one Attempt; runtime interprets the evidence.

## The three-home split this anchors

12_port owns the contract/declaration shapes. 18_bvisor owns the live
crossing: requests, Attempts, validation, physical observations — it imports
this home's `PortPostcondition` and enforces this home's `ResponseBinding`
(authored here, applied there — the firewall pattern; its enforcement
checklist factors these same causes and is never a second family).
21_application owns the invocation profiles and the remote face. The one-shot
continuation MODEL is the execution home's; `DeadlinePolicy` is time's;
protected resolution is authority's — all referenced, never re-minted.

## Authored fresh here (flagged in the code)

`PortEffectPosture` (the restricted-query law needs posture as data), the
`ResponseBinding` selection order (DeadAttempt → SecondResume → WrongRequest →
Duplicate → WrongFamily → WrongType → WrongCapability → WrongSource →
WrongGeneration → OverBound → Expired → Late), and the `ForeignClaim` /
`AdmittedForeign` seam shape realizing "foreign material has exactly one
morphism: firewall admission."

## Shape laws carried in the docs

Least authority (the port receives only what that request needs);
"unsupported is only ever an answer" (no request vocabulary can express
unsupported/none/best-effort — a request cannot pre-weaken itself to pass);
mechanism success flags are evidence input, never automatic proof; every
mechanism implements only the operations and postconditions it can actually
establish, refusing rather than borrowing a stronger filesystem / browser /
network / device / clock / secret / durability analogy; no universal request
envelope, response enum, host trait, or dynamic dispatcher.

## Obligations

```yaml
home: 12_port
obligations:
  - id: port.family-version-rides-authority-position
    challenge_kind: compile-refusal
    green: laws.rs port::family_version_rides_authority_position
    red: owed-to-testpak — cross-family version compare must not typecheck
  - id: port.roles-are-thirteen
    challenge_kind: compile-law
    green: laws.rs port::roles_are_thirteen
    red: owed-to-testpak
  - id: port.response-binding-owes-its-order
    challenge_kind: compile-law
    green: laws.rs port::response_binding_owes_its_order
    red: owed-to-testpak
  - id: port.foreign-claim-admits-only-through-evidence
    challenge_kind: compile-refusal
    green: laws.rs port::foreign_claim_admits_only_through_evidence
    red: owed-to-testpak — any other unwrap of a ForeignClaim must not compile
  - id: port.operation-contract-composes
    challenge_kind: compile-law
    green: laws.rs port::operation_contract_composes
    red: owed-to-testpak
  - id: port.rosters-hold
    challenge_kind: compile-law
    green: laws.rs port::rosters_hold
    red: owed-to-testpak
  - id: port.unsupported-is-only-an-answer
    challenge_kind: compile-refusal
    green: none — the absence of unsupported/none/best-effort in every request
      vocabulary IS the law
    red: owed-to-testpak — a request type carrying an Unsupported variant must
      not compile
```
