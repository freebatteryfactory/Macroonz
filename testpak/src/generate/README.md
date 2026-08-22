# generate — the generation contract

The harness's own generation machinery, one home, consumed by properties and muterprater alike. The generator owns only generation facts; execution facts belong to the runner, and verdict facts to the check — three owned axes, never one status blob.

  - The typed generation dispositions, so the denominator cannot silently shrink: generated;

  - bytes-insufficient; precondition-rejected — COUNTED, because a rejection that silently burns budget shrinks the denominator;

  - generator-refused; generator-contract-violated; generation-budget-exhausted.

The runner's execution axis has its own roster, stated here once beside its sibling: a run attempt is executed, skipped with its reason, timed out, or infrastructure-failed.

The harness keeps several honest censuses — rows, selected trials, generated cases, mutants, bench samples — sharing primitives; each denominator answers its own question and none flattens into another.

Generation has its owning values. A generation plan binds the population identity, the generation profile and version, the root seed or exact supplied bytes, the case budget, the byte budget, the rejection budget, and the size progression.

A reduction plan binds the reduction profile and version, the generic byte reducer, optional semantic reducers, the required fingerprint preservation, and the reduction budget. The paved recommendation, realization free: a deterministic counter-addressed byte source over the admitted identity substrate.

The replay capsule binds the report instrument's complete contract — that home is the one owner and nothing is restated here — and is never a naked integer seed.

Declared bounds yield generated boundary populations for free:

  - empty, one, just-below, at-the-limit, just-above, maximum lawful, first unlawful.

The escalation ladder by domain:

- a tiny closed domain is enumerated ALL; a large closed domain gets generated cases plus metamorphic pressure; open and byte-shaped domains get the fuzz lane; stateful subjects get sequences plus faults.

Minimization preserves the fingerprint:

  - a shrunk input must carry the same failure fingerprint, or the shrink is rejected — no minimizing into a different bug.

Command sequences are structured inputs, and one shared sequence driver serves temporal properties, metamorphic relations, sequence mutation, and chaos scheduling.

Budgets live here — the generator's own — and on bench rows; no per-trial budget field exists, because the invocation's budgets suffice and a row field would be a second budget authority.
