# Facade source

This directory implements the root `macroonz` library.
Start with [lib.rs](lib.rs) for the exported doors, recipe wrapper and feature gates.
The [root README](../README.md) owns the product contract, crate map and [feature postures](../README.md#the-three-postures).

## Follow the responsibility

| If the change concerns | Read |
| --- | --- |
| Reusable source patterns | [Pattern](pattern/README.md) |
| Input execution, selection, retention or fresh replay composition | [Workflow](workflow/README.md) |
| Versioned mechanical defaults | [Configuration](configuration/README.md) |
| Displaying reports and owner-specific failures | [Presentation](presentation/README.md) |
| Native measurement, child processes or filesystem custody | [Clock](native_clock/README.md), [process](native_process/README.md) or [storage](native_storage/README.md) |
| Executing compiler fixtures, coverage tools or mutation backends | [Compiler execution](native_compiler/README.md), [coverage](native_coverage/README.md) or [mutation](native_mutation/README.md) |
| Preparing, checking or installing generated files | [Publication](native_publication/README.md) |

These links select the owner to read; they do not describe package dependency edges or prescribe an execution sequence.
For compiler or harness semantics exposed through the facade, follow the [product task map](../README.md#find-the-owner) to that package's owner.

## Independent observations

The [facade surface](../tests/facade_surface.rs) and [harness surface](../tests/facade_harness_surface.rs) exercise the exported entrances.
The [native-policy lane](../tests/native_policy/README.md) routes feature isolation, host effects and workflow-composition checks.
Read the affected lane's assertions alongside the implementation; the [contribution contract](../CONTRIBUTING.md#local-wall) determines qualification from the change's effects.
