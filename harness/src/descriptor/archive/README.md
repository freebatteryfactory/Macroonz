# archive

Owned historical descriptor data without a live declaration mint.

`ArchivedName` retains exact, nonempty UTF-8 namespace and stem components.
Its caller bounds each component before construction.
It cannot become a static `NamespacedName` or an executable binding.
The report archive re-exports this same type for its historical names.

## Candidate records

`retain_candidate` reads an existing row's already-owned canonical bytes.
`read_candidate` admits those same bytes from caller-owned storage under independent limits.
Both preserve all candidate fields and the exact canonical preimage, without constructing `Row`, `CanonicalRowBytes`, `NamespacedName` or an executable attachment.
The descriptor [encoder](../encode.rs) owns the version, field order, framing and discriminants.
There is no second row writer or separate row format here.

Only the candidate origin is admitted.
Its synthesis opening is retained as a historical survivor point or proof gap, without establishing that the mutation or gap exists.
Roles and tags must already be strictly ordered by namespace and stem, as the canonical writer emits them.
Duplicate or out-of-order labels refuse instead of being normalized.
Every member must be consumed; unknown versions, origins and synthesis slots refuse.

## Complete row records

The `read_row` and `retain_row` operations retain every canonical row origin through the same bounded name, label and prefix readers as candidates.
`ArchivedRow` owns its readable fields and canonical bytes, while `ArchivedOrigin` records hand-written, generated, candidate, replay-admission or discharge-admission claims.
Candidate reading remains restricted to the candidate origin and keeps its existing refusals.
The descriptor encoder remains the sole owner of the row grammar; no second writer or row envelope is introduced.

Historical replay admissions retain exact thirty-two-byte proposal and replay claims, the recorded replay-bearing ground and destination name.
Discharge admissions retain a proposal claim and destination, with no ground byte or replay seat.
Generated origins retain the declaration door and projection; producer and schema provenance belong to the binding and are absent from canonical row bytes.
The complete reader refuses unknown origins, malformed address widths and a discharge or unknown ground in the replay-bearing arm.
It does not require an admission destination to equal the row's execution suite, because the live row constructor establishes no such join.
No historical origin constructs a live `Origin`, proposal, replay reference or human admission.

## Binding records

`retain_binding` and `read_binding` retain a complete row beside its attachment subject/check names, separate revision claims and producer provenance.
Subject and check must match the row, in that order, and a generated row must carry produced provenance.
Produced provenance on other origins remains lawful.
`ArchivedRevisionBinding` retains address bytes and the original posture without original derivation material or a live revision constructor.
`ArchivedProvenance` retains unproduced standing or the producer name and schema address claim without asserting schema currency.
No callable, function address or reconstruction handle is retained.

The nested binding format is `u32be(1)`, framed canonical row bytes, attachment subject name, attachment check name, subject revision record, check revision record, then provenance.
A name uses the descriptor encoder's namespace/stem framing.
A revision record is `bytes(bytes(address) || posture)`, with exactly thirty-two address bytes and posture `0` derived, `1` declared or `2` untracked.
The inner revision body is forty-one bytes.
Provenance is slot `0` unproduced, or slot `1` followed by the producer name and framed thirty-two-byte schema claim.
The binding has no separate integrity envelope; its enclosing historical record owns that relationship.
Unknown versions, slots, malformed widths and trailing material refuse.

## Bounds and custody

The caller independently bounds complete canonical bytes, each name component and each role or tag population.
`RowArchiveLimits` shares those ceilings with candidate reading and applies the field ceiling to historical address fields as well.
`BindingArchiveLimits` applies the same ceilings to complete binding bytes, each name/address field and each nested row label roster.
The nested row remains subject to its owning byte and field checks.
The reader checks the complete byte ceiling before reading, each name component before copying and each roster count before walking it.
It never reserves from a supplied count and allocates no missing material.
The result owns its bytes and readable fields independently of the source buffer.

Canonical row bytes carry no archive integrity address, producer authentication or custody claim by themselves.
An enclosing historical record owns its own envelope and joins; this reader cannot establish those facts.
No loaded value authorizes execution, table admission, caching or human approval.
