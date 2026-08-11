# 22_security — security machinery

Band 22. Imports authority (CapabilityGrantId — the value algebra, grants,
claims, the meet, KeyScope, ProtectedResolution, and the release contract all
live at 06), identity, and the root calculus. This band collects the
lifecycle machinery band math forced upward: the lease (consumes 09's
deadline policy), revocation distribution, shred, secrets, mechanism
standing, trust-boundary disclosure, and the supply-chain law.

## The band-forced seat collected

`CapabilityLease` — flagged at 06 as seated here because it binds the time
home's `DeadlinePolicy`, three bands above the grant algebra. Grant validity
is answered through the canonical `Truth` (never a second three-valued enum,
never a revoked flag; `Pending` narrows fail-closed). Renewal is a named
authority-bearing morphism carried by `LeaseRenewalAuthority`.

## Nuances carried

Revocation as a distributed-time problem: observation ≠ acknowledgement ≠
evidence freshness (three facts, per participant, with the explicit
denominator — never bare "done"; the escaped bearer claim structurally does
not exist because no-bearer-tokens is already law). The four paved
revocation defaults per authority class (asymmetric default-plus-override).
Shred: the four progress facts never collapse into each other or the
resolution outcome; the completion denominator names every protected
derivative with six honest row statuses visible; anti-resurrection; secure
ciphertext deletion never claimed from key shred alone. `SecretUseHandle`
carries NO `Clone`/`Copy`/`Debug`/`Display`/serde by law (`Debug` and
`Display` are named release surfaces — the morphism refused by not
existing), `!Send`/`!Sync` structurally. Mechanism standing is DERIVED from
four append-only fact families (never one mutable status enum; the old
first-state word renamed to PROPOSED); a historical fact is never erased.
The trust-boundary acronym is dead — the substance is TRUST-BOUNDARY
DISCLOSURE, claim-local. The safe-Rust floor is repository policy, enforced
by the workspace lint wall; the named-receipted-loosening law
(declassification, widening-as-new-grant, KDF-realizes-attenuation) is stated
as this home's own law in the docs.

Permanent hostile families are not a roster here: the qualifying owner
(testpak) declares them, and 23_evidence already carries that assignment.

## Flags carried

`ProtectedResolution` variant drift (prose elsewhere lists `Unavailable` and
backend-unreachable as peers the eight-variant enum does not carry — the
eight are law; any second roster is unowned, flagged). The
release-projection default: refusal releases inherit the
typed-redacted-diagnostic default posture — one answer, never a second per
family (reconciling 20's length-disclosure flag).

## Obligations

```yaml
home: 22_security
obligations:
  - id: security.lease-collects-the-banded-seat
    challenge_kind: compile-law
    green: laws.rs security::lease_collects_the_banded_seat
    red: owed-to-testpak
  - id: security.revocation-axes-stay-apart
    challenge_kind: compile-law
    green: laws.rs security::revocation_axes_stay_apart
    red: owed-to-testpak — a fused observed/acknowledged token must not
      compile
  - id: security.shred-progress-never-collapses
    challenge_kind: compile-law
    green: laws.rs security::shred_progress_never_collapses
    red: owed-to-testpak
  - id: security.mechanism-standing-is-append-only
    challenge_kind: compile-law
    green: laws.rs security::mechanism_standing_is_append_only
    red: owed-to-testpak — a mutable status enum must not exist
  - id: security.secret-handle-refuses-the-morphism
    challenge_kind: compile-refusal
    green: laws.rs security::secret_handle_refuses_the_morphism
    red: owed-to-testpak — Debug/Display/serde/Send on the handle must not
      compile
  - id: security.firewall-and-rosters-hold
    challenge_kind: compile-law
    green: laws.rs security::firewall_and_rosters_hold
    red: owed-to-testpak
```
