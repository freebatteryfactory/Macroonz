# Compiler and procedural host

This directory groups the callable compiler and its procedural host.
It is a workspace directory, not a Cargo package or a shared semantic owner.
The [root product map](../README.md#the-bakery) owns the crate map; the [dependency law](../AGENTS.md) owns permitted imports.

| If the change concerns | Read |
| --- | --- |
| Declaration grammar, informed structure, projection, tokens, diagnostics or sealed expansion | [Compiler](compiler/README.md) and its linked semantic home |
| Procedural entry points, token conversion, span custody or emission into rustc | [Proc host](proc/README.md) |
| The public facade entrance or root workflow composition | [Facade source](../src/README.md) |

The compiler README distinguishes its callable road from the first-party recipe road.
The proc README explains the carrier boundary and caller-owned projection hosts.
Their shared parent supplies no additional library layer.

For independent cross-package observations, start with the [facade crossings](proc/tests/recipe_facade_crossing.rs); follow the [task map](../README.md#find-the-owner) for the affected owner's other checks.
