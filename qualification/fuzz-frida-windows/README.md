# fuzz-frida-windows

First-party Windows runnable road for the selected `LibAFL` + Frida backend behind `macroonz-harness::fuzz`.

## Boundary

This directory is durable qualification tooling.
It is not a published package, not a root-workspace member, and not product source.
Engine crates (`libafl`, `frida-gum`) stay here; they do not enter default `macroonz-harness` dependencies.

The Macroonz semantic shell remains [`harness/src/fuzz/`](../../harness/src/fuzz/README.md): backend selection, named ceilings, declared preflight facts, and `compose_reduce_replay`.
This driver reuses those roads and the F0-proven EventSink + LibAFL loop rather than cloning engine internals into Macroonz.

## Accepted ceilings (carry with every run)

1. Residual LNK4098 coexistence; `/IGNORE:4098` is experimental choreography only.
2. `LIB` must append MSVC and Windows SDK `um`/`ucrt` x64 roots after the Frida devkit.
3. Rust 1.98 `std-*.dll` directory must be on `PATH` when the driver and target import those DLLs.
4. Linux and macOS Frida execution remain unproven until their native Wave F hosts run.

## Prerequisites

1. Rust 1.98.0 as in the repository toolchain.
2. Visual Studio Build Tools with the MSVC x64 toolset (`vswhere` must find `vcvarsall.bat`).
3. Frida Gum Windows x86-64 **17.9.5** archive under `target/qualification/fuzz-frida-windows/devkit/` (see [`devkit-pin.tsv`](devkit-pin.tsv)).
   The cold shell verifies the archive SHA-256, extracts into a clean directory, and records linked `.lib`/`.h` hashes before Cargo builds.

## Run

From an ordinary PowerShell session (no Developer Command Prompt required):

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File qualification\fuzz-frida-windows\cold-shell-run.ps1
```

Build and evidence land under `target/qualification/fuzz-frida-windows/` only.
Seal retained runs into `.durafx/fuzz/...` with `qualification/durafx-sealer` when a campaign needs durable custody.

## Provenance

Promoted from the sealed F0 ACCEPT bundle:

`.durafx/fuzz/e4c87115…/x86_64-pc-windows-msvc/f0-frida-accepted-20260827-0c5332ae…`

Composition now calls `macroonz_harness::fuzz::compose_reduce_replay` for the Macroonz reduction/replay handoff.
