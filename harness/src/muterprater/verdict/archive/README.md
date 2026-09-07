# archive

Historical target and activation members for enclosing mutation and proposal records.

These members preserve the verdict owner's retained coordinates without constructing `MutationTarget`, `ActiveSelection` or `ActivationEvidence`.
External target and selected-alternative addresses remain historical claims because their original preimages are absent.
A declared family spelling remains a historical bank attribution, without lookup in the current bank.
Reported source coordinates retain their exact file, line and column.
No target identity implies a source/point join that the live target constructor does not establish.

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
