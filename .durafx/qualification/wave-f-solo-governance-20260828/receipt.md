# Solo-maintainer hosted governance

## Standing

- Repository: `freebatteryfactory/Macroonz`.
- Visibility: public.
- Default branch: `main`.
- Synchronized default-branch source at mutation time: `be990fb11b8968ab13944c2aee746b200ce929c8`.
- Governance branch source before this receipt: `c8551f05129a5ddf365a75815beb165d1e990f01`.
- Campaign-plan snapshot used for the audit and owner decision: SHA-256 `BC63FE853F74EA586AFE0A3870A0DBFDE3919F8A622BCD76748D68F548EAA3B4`.
- The owner explicitly authorized all five settings after a read-only audit named the two contradictions and three optional hardening choices.

## Prior state

- Merge commits, squash merges, and rebase merges were all enabled.
- Private vulnerability reporting was disabled while `SECURITY.md` directed reporters to that road.
- Repository Actions were enabled for all actions, but full commit-SHA pinning was not required by GitHub settings.
- Secret scanning and secret-scanning push protection were disabled.
- The repository had no branch protection and no ruleset.

## Applied state

- Merge commits remain enabled.
- Squash merges and rebase merges are disabled.
- Private vulnerability reporting is enabled.
- Repository Actions remain enabled for all actions and now require full commit-SHA pins.
- Default workflow permissions remain read-only, and workflows remain unable to approve pull requests.
- Secret scanning and secret-scanning push protection are enabled.
- Dependabot security updates, non-provider-pattern scanning, validity checks, automatic merge, and automatic branch deletion remain disabled.
- Active repository ruleset `21761340`, named `Protect main history`, targets only `~DEFAULT_BRANCH`.
- The ruleset has no bypass actor and contains only `deletion` and `non_fast_forward` rules.

## Repository compatibility

- GitHub's applied-rule projection for `main` contains exactly `deletion` and `non_fast_forward`.
- The ruleset carries no pull-request requirement, required check, review count, linear-history rule, merge queue, signed-commit rule, CODEOWNERS rule, deployment gate, or organization-role machinery.
- Direct fast-forward updates and owner-created merge commits remain outside the two prohibited operations.
- The active `Hosted pulse` workflow remains identifier `344858202` at `.github/workflows/hosted-pulse.yml`.
- All nine current `uses:` declarations carry full forty-hex commit identities, and the repository contains zero unpinned action use.
- The workflow retains manual dispatch, read-only contents permission, same-ref cancellation, no secret, no artifact upload, no publication, and no required-check authority.

## Verification

- Independent repository readback returned merge commits enabled and squash and rebase merges disabled.
- Independent security readback returned private vulnerability reporting enabled, secret scanning enabled, and push protection enabled.
- Independent Actions readback returned Actions enabled, all actions allowed, and full commit-SHA pinning required.
- Independent ruleset readback returned active ruleset `21761340`, default-branch targeting, no bypass actor, and only deletion and non-fast-forward rules.
- Independent applied-rules readback for `main` returned only deletion and non-fast-forward.

## Plane limits

- This receipt records GitHub repository administration state observed on 2026-08-28; repository administrators can change that external state later.
- It does not establish crate publication, registry delivery, attestation, release acceptance, immutable releases, physical Linux for the current exact-child source, or any unrecorded organization-level policy.
- No repository source, public API, dependency, feature, workflow, check name, trigger, identity, encoded byte, or product behavior changed in this governance pass.
