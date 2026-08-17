# depot — the machine's bank of data-shaped truth

The depot holds facts, as Rust: error prose, golden vectors, hostile-case and
fault-scenario shapes, roster tables — each entry a doc-commented constant
with a typed identity, readable by every crate that depends on the machine
and checkable by the compiler itself. A typo'd field refuses to compile; an
unconsumed entry is flagged dead; the diff is the decision.

The depot holds data only — never behavior, never anything that branches. An
entry has one owner and states one fact; the code that acts on a fact lives
with the actor. Bulk vectors ride as line-format files bound to constants
with `include_str!`, so a vector file is part of the crate's declared input.

The depot is authored specification. Runtime evidence — a report, a finding,
a panic artifact — never writes it; what enters the bank enters by authorship.

Families materialize as modules when their first entries land; adding a
family is adding a module, and the diff is the decision.
