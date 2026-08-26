# reduction

The reduction owner searches for a smaller input that still produces the fingerprint carried by one refused trial report.

A reduction plan binds an ordered semantic-reducer roster, the generic byte reducer, and a finite attempt budget.
A report-bound probe refuses reports that did not fail, and every proposed candidate must be strictly smaller than its predecessor before the candidate can be observed.
Semantic candidates run first; generic chunk removal and zeroing then run at halving widths until a fixed point or budget exhaustion.

The evidence records reducer executions, attempt counts, the original and reduced fingerprints, the resulting bytes, and the halt.
A replay capsule is minted only from that evidence and preserves the report-owned identity and encoding contract.

This home does not interpret input bytes, choose semantic candidates, execute a subject, or claim that the reached result is globally minimal.
