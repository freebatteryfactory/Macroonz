# Reporting a security issue

**Report privately, through GitHub's private vulnerability reporting**, on this
repository's Security tab. That road is private by construction, so a report and
its repair can be written in the same place without the issue being public
first. If it is unavailable to you, open an ordinary issue that says only that
you have something to report and asks for a private channel — no detail, no
reproduction.

Expect an acknowledgement. There is no service commitment behind that sentence
and there will not be one until this repository has a release to stand behind:
this is a small project, and a response time nobody can hold to is worse than no
number at all.

## What is supported

Nothing yet. Every package here is version `0.0.0`, no version has been
published, and the repository is in architecture closure — the machine's homes
carry their specifications and the tooling that checks them, and product runtime
behaviour is deliberately absent. There is no supported release, so there is no
supported-versions table; the only thing that can be reported against is the
trunk as it stands.

This section becomes a table the day a version is published, and not before. A
table listing supported versions of software nobody can install would be a claim
about a promise that does not exist.

## What is in scope

The repository as it stands, which today means its tooling and its hosted
qualification road rather than a running machine:

- the qualification road and the repository laws — a check that can be made to
  report PASS about a tree it did not read is a defect of exactly the kind this
  file is for, and so is one that can be made to write into the tree it is
  judging;
- the dependency posture — what `deny.toml` allows, what the manifests pin, and
  any road by which the resolved graph can come to differ from what the
  repository declares;
- the hosted workflows — anything that lets a change execute code the
  repository did not commit, or lets a run reach a credential;
- the metaprogramming services and the expansion shell — an expansion is
  supposed to be a function of its declared input alone, and a way to make one
  read the network, the filesystem, the environment, a clock, or entropy is a
  report.

## What is not in scope, and why

- **Anything requiring `unsafe`.** The lint wall forbids it outright, so a
  report that begins by adding it is a report about a different repository.
- **Resource exhaustion from a deliberately hostile declaration**, unless it
  crosses a bound the repository has actually declared. Bounds are being
  established home by home; where one does not exist yet, its absence is
  recorded work rather than a vulnerability.
- **Findings from a scanner, unaccompanied.** A rule identifier and a file path
  are not a report. What is needed is the road: what an attacker controls, what
  they reach, and what they get.
- **Anything about a host.** Hosts live in other repositories and pin an exact
  revision of this one. A defect in how a host uses this machine is a report to
  that host.

## What helps

The exact commit, the exact toolchain, the exact command, and the smallest
input that shows the behaviour. If the finding is a refusal that should have
happened and did not, say which refusal — this repository is built out of
refusals with names, and naming the one that stayed quiet is most of the work.
