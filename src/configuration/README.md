# Mechanical configuration

This home composes versioned resource defaults into the existing public execution and retention owners.
Select [v1](v1/README.md) explicitly.
There is no unversioned latest alias or implicit default implementation whose values can move between releases.

The constructors return the existing owners' inspectable configuration types.
Use those same owners' constructors for explicit overrides and pass the resulting values to their ordinary execution functions.
No profile object, registry, tool discovery or alternate execution engine is needed.

Semantic input profiles, decoder and check revisions, independent expected results, invocation budgets, target attribution and clock selection remain caller-supplied.
Native tools, working directories, environment entries, storage roots and requested resource controls also remain explicit.
These defaults limit resources; they cannot establish semantic completeness, tool compatibility, executable authenticity or replay authority.
