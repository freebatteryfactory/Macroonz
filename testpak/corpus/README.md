# corpus — seed-packs for warm-start fuzzing

One compressed seed-pack per subject: length-prefixed binary, no parser
dependency, machine-written and machine-read. A pack warm-starts the fuzz
lane's search; it holds nothing durable — a find that matters is promoted
into a regression descriptor row with its reproduction seed, and that row is
the record. Packs are exploration state, not specification and not evidence.
