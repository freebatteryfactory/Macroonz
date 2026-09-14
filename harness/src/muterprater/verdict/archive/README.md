# archive

Complete historical mutation records, ordered runs and reusable target and activation members.

A loaded record retains what its source claimed, with no conversion to live mutation, baseline qualification, demonstration, activation or trust evidence.
Its integrity address does not authenticate a producer or establish execution.

## Complete mutation envelope

The envelope starts with the raw thirty-two-byte integrity address under `historical-mutation-report/v1`.
Its body contains u32be format one, u32be kind one and u32be historical custody zero, followed by:

- Framed target member under the member grammar below.
- One baseline byte: qualified zero, failed one or not run two.
- One materialization byte: built zero, unviable one or tool failed two.
- Framed activation member under the member grammar below.
- One execution byte: completed zero, not executed one, timed out two, crashed three or infrastructure failed four.
- One outcome byte and its arm's retained evidence.
- One equivalence byte: not assessed zero, proven in scope one, refuted two or inconclusive three.

Outcome zero is killed, followed by rejection byte zero for demonstrated or one for backend-reported.
A demonstrated rejection contains the [report archive](../../../report/archive/)'s finding grammar: framed fingerprint preimage, framed UTF-8 file, u32be line and optional foreign material.
The fingerprint preimage retains the exact trial, cause and failure class; no second trial claim can disagree with it.
A backend-reported rejection contains the same owner's foreign-material grammar and requires its present arm.
It holds no trial or fingerprint.
Outcome one is survived and contains no additional members.
Outcome two is inconclusive followed by one cause byte: baseline not qualified zero, not materialized one, not activated two, witness incomplete three, unobservable and unrejected four or proven equivalent in scope five.

A historical kill requires qualified baseline, built materialization, observed or backend-unobservable activation and completed execution.
A historical survivor requires qualified baseline, built materialization, observed activation and completed execution.
Inconclusive records retain every supplied axis and cause without imposing a new correspondence between them.
Target, activation and rejection coordinates establish no extra joins beyond the live verdict owner's constructors.

Composed interpreted archives additionally join the retained trial to the mutation through this owner's inverse consistency check.
That check preserves the [live interpreted constructor](../type_guard.rs)'s baseline, materialization, observed activation, execution, outcome and equivalence relationship, including the entire demonstrated finding.
It reads existing claims and constructs no new live mutation outcome.

Independent complete-envelope and field ceilings apply before allocation, including each framed target, activation and fingerprint preimage.
The writer preflights all fields before allocating preimages; the reader bounds the complete envelope before hashing and each field before copying.
Unknown slots, unsupported framing, absent required rejection text and undeclared trailing bytes refuse.

These members preserve the verdict owner's retained coordinates without constructing `MutationTarget`, `ActiveSelection` or `ActivationEvidence`.
External target and selected-alternative addresses remain historical claims because their original preimages are absent.
A declared family spelling remains a historical bank attribution, without lookup in the current bank.
Reported source coordinates retain their exact file, line and column.
No target identity implies a source/point join that the live target constructor does not establish.

## Complete-run envelope

The run envelope starts with the raw thirty-two-byte integrity address under `historical-mutation-run/v1`.
Its body contains u32be format one, u32be kind two and u32be historical custody zero, then baseline byte zero for a qualified historical pass, a u64be report population and that many framed complete mutation envelopes in original order.
Failed baseline one and absent baseline two refuse, and every other baseline slot is unknown.
Each nested envelope retains its own integrity address and obeys both byte ceilings.
The independent report ceiling is checked before walking or allocating the population; no untrusted count reserves storage.

An empty roster and repeated targets remain lawful historical runs.
The run's qualified-baseline claim does not impose an additional equality on each record's independently retained baseline axis.
The verdict owner's saturating census is derived from every retained outcome, without a separately encoded summary that could disagree.
That accounting carries no current baseline, exact-roster campaign acceptance or trust-opening authority.

## Member grammar

The enclosing record supplies framing, version, custody and integrity.
Every variable member uses the identity owner's length framing.
Names use the descriptor owner's namespace and stem grammar, and address claims contain exactly thirty-two bytes.

A target contains identity, family, site and owner in that order.
Identity zero is external with a framed address claim.
Identity one is interpreted and two is compiled projection; both contain a point name and framed alternative claim.
Family zero is outside the bank; one adds a framed nonempty UTF-8 family slug.
Site zero is reported and contains a framed nonempty UTF-8 file, u32be line and u32be column.
Site one contains a declared activation name.
Owner zero is unmapped; one adds the historical claim name.

Activation zero contains framed surface claim, point name, framed alternative claim, framed witness claim and positive u32be firings.
Activation one is not observed and two is unobservable under the backend.
Positive counts remain callback reports, without instrumentation authority.
Each member must be fully consumed, and unknown slots or excess field bounds refuse.
