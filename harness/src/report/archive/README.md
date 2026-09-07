# archive

Owned historical data retained from the report owner's canonical identity material.

A loaded record states what its source claimed.
It is never a current `ReplayCapsule`, `TrialReport`, `RunReport`, reduction binding, cache permission or human admission.
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

## Trial envelope

The trial envelope derives its leading address under `historical-trial-report/v1`.
It uses format one, kind two and historical custody zero, followed by the same execution preimage and input metadata as the capsule envelope.
The remaining fields occur in this order:

- One replay-ceiling byte using `ReplayPosture::slot`.
- Framed UTF-8 module path, framed UTF-8 file, `u32be` line and framed UTF-8 display name.
- One attempt byte and its arm's members.
- One measurement byte and its arm's members.

Attempt zero is executed/pass and has no members.
Attempt one is executed/refusal: framed canonical fingerprint preimage, framed UTF-8 refusal file, `u32be` refusal line and optional foreign material.
The fingerprint trial must equal the execution trial.
Attempt two is skipped, followed by one reason byte: budget exhausted zero, target unsupported one, prerequisite absent two or satisfied by cached execution three.
Attempt three is timed out and has no members.
Attempt four is infrastructure failure, followed by one fault byte and optional foreign material.
Infrastructure slots are generation unavailable zero, support absent one, capture failed two, backend unavailable three, backend initialization failed four and backend execution unresolved five.

Optional foreign material begins with zero for absent or one for present.
Present material contains framed exact bytes, one truncation byte and one fidelity byte.
Truncation zero is complete; truncation one inserts `u64be` admitted and offered counts before fidelity.
Fidelity zero is exact UTF-8 and one is lossy replacement.
The reader checks fidelity against the retained bytes and enforces the report owner's foreign-byte bound.
A truncated record must retain exactly that bound, name its actual retained length and state a strictly larger offered count.
Offered counts remain portable `u64` claims without allocating the missing material.

Measurement zero is observed duration followed by `u64be` nanoseconds; one is unavailable; two is failure followed by one failure byte.
Clock failure slots are opening refused zero, closing refused one, opening unwound two, closing unwound three and regressed four.
Regression additionally retains `u64be` opening and closing readings and requires closing to precede opening.
No reader calls a clock or constructs a current measurement tick.
Measurement is retained independently of the attempt and never upgrades its conclusion.
The envelope address covers retained bytes for integrity; adding a measurement or foreign-text field does not change the canonical execution key or failure fingerprint.

The reader rejects undeclared trailing material and every unknown discriminant.
The historical replay ceiling cannot exceed its recorded decoder ceiling.
Neither a retained pass nor a claimed ceiling establishes that the producer actually executed a subject.

## Complete-run envelope

The complete-run envelope derives its leading address under `historical-run-report/v1`.
It uses format one, kind three and historical custody zero, followed by these members:

- Framed invocation and target context: `u32be` case budget, `u64be` byte budget, `u64be` time budget, framed UTF-8 target and framed UTF-8 toolchain.
- Input marker zero for unit input, or one followed by framed input coordinates and profile metadata.
- Table posture byte zero for authored, or one followed by the staged parent's framed UTF-8 namespace and stem.
- One selection-outcome byte.
- Census population as `u64be`, followed by that many framed rows in the original order.

Context encoding reuses the invocation and target fields of the canonical execution-key writer.
Input coordinates reuse its two framed thirty-two-byte case and decoder claims and decoder-posture byte.
Profile metadata is the same namespace, local name, version and schema sequence used by trial and capsule envelopes.
The run retains context and input even when no row is selected or the census is empty.
Selection slots are satisfied zero, unsatisfied by empty selection one, empty as carried over from a previous run two and empty as asking what the world holds three.

Each row contains framed thirty-two-byte trial, row-revision, subject-revision and check-revision claims, then framed UTF-8 claim namespace and stem, then one disposition byte.
Disposition zero is selected and adds the framed entire trial envelope, including its own integrity address.
Disposition one is outside selection and two is suite not run, with no trial member.
Historical names use the [descriptor archive](../../descriptor/archive/)'s owned vocabulary and retain its name boundary.

The reader refuses duplicate semantic trial claims, even if their row-revision claims differ.
Selected trial, subject and check coordinates must match their census row.
Every selected trial's invocation, target and complete optional input must match the run context.
Satisfied selection requires at least one selected row; every empty-selection outcome requires none.
A selected skip, timeout or infrastructure failure still satisfies selection without claiming execution or a passing conclusion.

Row revisions and claim spellings remain historical claims because the report does not retain their descriptor preimages.
It likewise holds neither an authored-table name nor individual subject and check revision postures to reconstruct.
Integrity and internal joins do not establish that the claimed census was actually complete or authentic.
Coverage and live comparison continue to require a current `RunReport`.

## Bounds and custody

The caller supplies complete-envelope and per-field byte ceilings.
Every framed member, including a nested preimage, is a field for this purpose.
Run limits independently bound census rows before walking or allocating that population, and each framed row and nested trial envelope obeys the field ceiling.
The reader never reserves storage from an untrusted census count.
The reader checks the complete envelope before hashing, then each field before copying.
There are no recursive members or inferred host facts.
The writer checks the same sizes before asking existing encoders to allocate their preimages.

The original execution key and the reached witness remain distinct coordinates.
Inspection reads this data without executing it.
Fresh replay must independently bind and execute the witness through the runner; this home provides no conversion into live evidence.
