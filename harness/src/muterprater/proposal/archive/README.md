# archive

Complete historical offers for the proposal owner's three concrete grounds.

The live `ProposalDocument` storage seam remains open.
This archive format has its own closed historical ground roster and implements neither `ProposalDocument` nor `ReplayBearingProposal`.
Loading never constructs a live candidate, demonstrated execution, activation, stored proposal reference or human admission receipt.

## Identity and integrity

The existing [proposal identity writer](../encode.rs) derives identity from candidate canonical bytes, ground and destination.
The archive recomputes it through that same writer and refuses a disagreeing stored claim.
Concrete grounds and comparisons remain outside proposal identity, as its [owning contract](../types.rs) requires.
The archive envelope separately addresses every retained byte, so evidence can change while proposal identity remains stable.
Neither address authenticates a producer, storage destination or human ruling.

## Envelope

The leading thirty-two bytes address the body under historical-proposal/v1.
The body starts with u32be format one, u32be kind one and u32be historical custody zero.
It then contains the framed canonical candidate descriptor, one `AdmissionGround` slot, destination name and framed proposal address claim.
Framing and names use the existing identity and descriptor grammars.
The descriptor archive reads the candidate without minting a live row.
The concrete ground members follow, with no undeclared trailing material.

A mutant-killed ground contains framed historical target, framed historical activation, framed capsule envelope, framed complete-run envelope, framed demonstrating-trial claim and u64be known-fingerprint population.
That population is followed by framed canonical fingerprint preimages in original comparison order.
The [verdict archive](../../verdict/archive/) owns target and activation members.
The [report archive](../../../report/archive/) owns every report, capsule, execution and fingerprint grammar.
The complete run must be staged and contain the named selected trial executing a typed refusal.
The capsule key and fingerprint must agree with that trial and finding.
The candidate survivor and target points must agree where both are present, and a not-observed activation cannot support a kill.
Known fingerprints may repeat one another but cannot equal the demonstrated candidate failure.
The selected trial and finding are derived from the retained census rather than encoded as competing copies.
The live offer does not independently join candidate descriptor coordinates to that trial; historical loading does not add such a claim.

A claim-pinned ground contains the pinned claim name, framed capsule envelope, u64be before count and strictly greater u64be after count.
Its no-comparison reason is forced to `GroundCarriesNoFailure` and has no redundant encoding.
The claim and capsule retain the live owner's actual joins, without inferred equality to the candidate's coordinates.

An obligation-discharged ground contains the owed claim name, framed nonempty UTF-8 opening condition, lane byte, framed trial claim and the report archive's execution-key preimage plus input metadata.
Lane zero is `TestRow`, one is `FuzzSeed` and two is `ChaosScenario`.
The prior-discharge comparison is forced empty for this owed claim, and no replay capsule is present.
The separately recorded discharge trial is not forced to equal the execution key's trial because `DischargeEvidence` does not establish that join.

## Resource and authority boundaries

The caller independently bounds the envelope, every framed field, each candidate label roster, the staged census and the known-failure roster.
Nested envelopes are framed fields and retain their own existing limits and integrity checks.
The writer admits complete sizes before allocating nested preimages; the reader checks the envelope before hashing and each field before copying.
Population limits precede roster walks and no untrusted count is used to reserve storage.
Portable proof counts retain u64 claims without minting a current `ProofDelta`.
Every unknown slot, invalid required name, impossible ground, duplicate candidate failure or inconsistent internal identity refuses.

Inspection requires no execution or ambient host access.
Original material absent from the live ground remains absent: a proposal archive is not a complete backend campaign, source archive, process record or custody receipt.
