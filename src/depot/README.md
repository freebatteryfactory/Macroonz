# depot — the machine's bank of data-shaped truth

The depot holds facts, as Rust: error prose, golden vectors, hostile-case and
fault-scenario shapes, roster tables — each entry a doc-commented constant
with a typed identity, readable by every crate that depends on the machine
and checkable by the compiler itself. A typo'd field refuses to compile; the
diff is the decision. An entry names its consumer beside itself — the one
family whose consumer lives outside this crate is the golden vectors, read by
the harness's oracle and by nothing here.

The depot holds data only — never behavior, never anything that branches. An
entry has one owner and states one fact; the code that acts on a fact lives
with the actor. Bulk vectors ride as line-format files bound to constants
with `include_str!`, so a vector file is part of the crate's declared input.

The depot is authored specification. Runtime evidence — a report, a finding,
a panic artifact — never writes it; what enters the bank enters by authorship.

Families materialize as modules when their first entries land; adding a
family is adding a module, and the diff is the decision. The error bank is
the first citizen: refusal prose with typed identities, projected into
diagnostics by the generation services and proven by the harness — every
entry reachable, every refusal rendered, no orphans — so one compile-refusal
snapshot can carry three signals at once: the compiler's prose, this bank's
prose, and the typed identity.
