# archive

Owned historical data retained from the report owner's canonical identity material.

A loaded capsule records what its source claimed.
It is never a current `ReplayCapsule`, `TrialReport`, reduction binding, cache permission or human admission.
Its addresses establish integrity of the supplied preimages, not execution or writer authenticity.
An `AddressClaim` retains a nested digest whose original preimage is absent; it cannot become a `ContentAddress`.
Input profile metadata is likewise a historical claim, because the original case bytes need not be the reduced witness.

## Capsule envelope

The envelope begins with the raw thirty-two-byte address derived under `historical-replay-capsule/v1`.
Its body contains, in order:

- Format version as `u32be`, currently one.
- Record kind as `u32be`, one for a capsule.
- Custody as `u32be`, zero for unauthenticated historical data.
- The framed canonical execution-key preimage.
- One input marker byte: zero for unit input, one for input-bearing execution.
- When input-bearing: framed UTF-8 namespace, framed UTF-8 local name, `u32be` profile version and framed schema claim.
- The framed canonical failure-fingerprint preimage.
- The framed canonical replay-capsule preimage.

Framing is the identity owner's `u64be` byte length followed by those bytes.
The existing report encoders own all nested preimage grammars and address domains.
Each nested digest field has exactly thirty-two bytes.
The reader checks the fingerprint's trial against the key and the capsule's key and fingerprint addresses against their supplied preimages.
It rejects unsupported markers, invalid UTF-8, trailing material and a capsule ceiling stronger than its recorded decoder permits.
Historical profile names retain their exact spelling without creating static descriptor names.

## Bounds and custody

The caller supplies complete-envelope and per-field byte ceilings.
Every framed member, including a nested preimage, is a field for this purpose.
The reader checks the complete envelope before hashing, then each field before copying.
There are no recursive members or inferred host facts.
The writer checks the same sizes before asking existing encoders to allocate their preimages.

The original execution key and the reached witness remain distinct coordinates.
Inspection reads this data without executing it.
Fresh replay must independently bind and execute the witness through the runner; this home provides no conversion into live evidence.
