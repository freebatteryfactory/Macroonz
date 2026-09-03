# Macroonz 0.2.0 release receipt

## Standing

- This page is the single receipt for the Macroonz `0.2.0` release plane.
- No `0.2.0` package has been published, no `v0.2.0` tag exists, and no GitHub Release exists at the time this page was opened.
- The first section below records local shape-lot custody that tests cannot retain; every later section appends only after its own hosted, registry, tag, release, or acceptance authority exists.

## Compiled-pressure source custody

The harness mutation lanes read three retained Cargo Mutants console artifacts and compare them with the tracked source they were recorded against.
The source copies that once sat beside those consoles are gone; each console now joins the live tracked source, and the exact revision each campaign ran against is recorded here.

### Current wrapped-backend campaign

- Console: `harness/tests/trust_opening_evidence/current-compiled-pressure-artifact/cargo-mutants-27.0.0-console.txt`, recorded at `bc99da3`.
- Source at the campaign: `harness/src/muterprater/backend/wrap.rs`, Git blob `53b19ddb28e5e54ac11ed50744d5feb8554fb405`, SHA-256 `EBF016D030A184CDB2879B85FBDAE2648E50CBFF07008CC869C57B0D651CAD4F`, `19,435` bytes.
- Reconstruction: `git show bc99da3:harness/src/muterprater/backend/wrap.rs`.
- Observation: one selected mutant, `replace != with == in roster_count` at line `351`, column `13`, reported caught by Cargo Mutants `27.0.0` on `x86_64-pc-windows-msvc` under `rustc 1.98.0 (88d9e12ae 2026-08-18)`.
- The lane pins the harness-derived revision identity of that source, so an edit to the wrapped backend refuses the campaign join until the campaign is rerun and this section is updated.

### Historical wrapped-backend campaign

- Console: `harness/tests/trust_opening_evidence/compiled-pressure-artifact/cargo-mutants-27.0.0-console.txt`, recorded at `ff0a1b3`.
- Source at the campaign: `harness/src/muterprater/wrap.rs`, Git blob `0ce1fd82b1e2c51018bfc048367f625c86b32f26`, SHA-256 `5E6172E62E5A3B14A1A2F10E672226C5C473A01C7BD652C6FB8F4F11D9B29323`, `19,349` bytes.
- Reconstruction: `git show 40581a0:harness/tests/trust_opening_evidence/compiled-pressure-artifact/wrap.rs`.
- Observation: one selected mutant, `replace != with == in roster_count` at line `348`, column `13`, reported caught by the same backend, target, and toolchain.
- The source coordinate no longer exists in the tree; the lane observes that the historical console still reads, that its coordinate differs from the current one, and that joining it to the current source refuses.

### Compile-contract campaign

- Console: `harness/tests/trust_opening_evidence/compile-contract-pressure-artifact/cargo-mutants-27.0.0-console.txt`, recorded at `4109a6d`.
- Sources at the campaign, all under `harness/src/oracle/compiled/`:
  - `compare.rs`, Git blob `94f49b28c960535e4fd3f0dd8c2dfb627a89d958`, SHA-256 `92E76CC471DE1077CF6D0CEAD2940DAEC4D5FB680A4DEEE7E2E79E10B4700B5D`, `6,474` bytes;
  - `conclude.rs`, Git blob `44b7e8b0f7e5140bdc2b4ca1241e19632d7185ca`, SHA-256 `DD80AE731903821063D4D889FC7E144724B3660C46B555E4C9E065E31BE29ACC`, `2,893` bytes;
  - `type_guard.rs`, Git blob `e6849fa64133c5beef7e69f7d49fd34f313d2c51`, SHA-256 `98A0024FC87B300597BADE99476ABC655B8E8097F57F1EEB6A80B5DF754EEA4E`, `6,841` bytes.
- Reconstruction: `git show 4109a6d:harness/src/oracle/compiled/<file>`.
- Observation: `49` selected mutants, `28` reported caught and `21` unviable, zero survivors, every target owner-unmapped.
- The lane pins the harness-derived revision identity of each source, so an edit to any of the three refuses the campaign join until the campaign is rerun and this section is updated.
