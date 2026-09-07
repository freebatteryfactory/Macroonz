# archive

This home retains a completed reduction account as owned historical data.
It reuses the [report archive](../../../report/archive/README.md) for the capsule and historical names, digest claims, ceilings and bounded framing.
Loading creates no live reduction evidence, capsule, probe binding or human admission.

The retained capsule preserves the original report's execution key and the reached witness as distinct coordinates.
The surrounding account retains the actual declared candidate-probe budget, original report ceiling, probe revision and posture, invoked semantic reducers, generic-reducer reach, candidate census and halt.
Only invoked semantic reducers are retained; the original bytes, uninvoked plan roster and individual candidate observations are not reconstructed.
Historical source claims and internal consistency neither authenticate a producer nor establish that the claimed execution occurred.

## Envelope

The leading thirty-two bytes are the address derived under `historical-reduction/v1`.
The body contains, in order:

- Format `u32be` one, kind `u32be` four and historical custody `u32be` zero.
- A framed complete historical capsule envelope, including its own address.
- One original-report replay-ceiling byte.
- A framed thirty-two-byte probe-revision claim and one probe-posture byte.
- The declared candidate-probe budget as `u32be`.
- The invoked semantic-reducer population as `u64be`, followed by that many framed records.
- One generic-reducer byte: zero for executed chunk removal and zeroing, one for unreached because semantic probes spent the budget.
- Accepted, moved-fingerprint and no-failure census counts, each as `u32be`.
- One halt byte: zero for fixed point, one for budget exhausted.

Each semantic record contains framed UTF-8 namespace and stem, a framed thirty-two-byte revision claim, one posture byte, then offered-candidate and probed-candidate counts as `u64be`.
Names must have nonempty components and retain their exact historical spelling.
Every posture byte uses zero for derived, one for declared and two for untracked.
Framing is the identity owner's `u64be` byte length followed by the bytes.

## Admission

The caller independently bounds total envelope bytes, every framed member and the invoked-reducer population.
The writer checks those bounds before capsule capture and encoding allocate; the reader checks the envelope before hashing and fields before copying.
No storage is reserved from an untrusted population count.

The checked census sum cannot exceed the declared positive budget.
Each invoked semantic reducer must have remaining budget and must have probed the lesser of its offered count and that remaining budget.
Semantic names are unique, their counts remain ordered, and semantic probes cannot exceed the complete census.
An unreached generic phase requires semantic probes to have spent the whole budget and a budget-exhausted halt.
An executed generic phase requires remaining budget at entry.
Budget-exhausted halt requires the whole candidate budget to have been spent.
A fixed point requires generic execution but may coincide with the last allowed probe.
The baseline probe is excluded from these counts.

The original report ceiling cannot exceed its recorded decoder ceiling.
The capsule ceiling must equal the meet of the original report, probe and every invoked semantic reducer.
These joins retain the reduction owner's actual distinctions without proving historical callables, candidate purity or global minimality.
