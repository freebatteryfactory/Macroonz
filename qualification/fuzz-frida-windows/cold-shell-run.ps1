# Cold-shell launcher for the first-party Windows Frida/LibAFL fuzz road.
# Ordinary PowerShell -> clean vcvarsall -> rustc prints -> build/run.
# Ambient Developer Command Prompt is not required.
# Ambient Cursor/shell env is cleared for the vcvars child (inherited PATH can break vcvarsall).

$ErrorActionPreference = "Stop"
$Root = $PSScriptRoot
if (-not (Test-Path (Join-Path $Root "source\Cargo.toml"))) {
  throw "missing source workspace under $Root"
}
$RepoRoot = (Resolve-Path (Join-Path $Root "..\..")).Path
$Work = Join-Path $RepoRoot "target\qualification\fuzz-frida-windows"
$Source = Join-Path $Root "source"
$DevkitRoot = Join-Path $Work "devkit"
$Archive = Join-Path $DevkitRoot "frida-gum-devkit-17.9.5-windows-x86_64.tar.xz"
$ExpectedArchiveSha = "07E0DF78E2EF962D8228A3C9866F97B6D9BEEA310434377DCCCFA402B01F9DE1"
$Devkit = Join-Path $DevkitRoot "frida-gum-17.9.5"
$Build = Join-Path $Work "build"
$Evidence = Join-Path $Work "evidence\final-exam"
New-Item -ItemType Directory -Force -Path $Evidence | Out-Null
New-Item -ItemType Directory -Force -Path $DevkitRoot | Out-Null

$Pin = Join-Path $Root "devkit-pin.tsv"
if (Test-Path $Pin) {
  foreach ($line in Get-Content -LiteralPath $Pin) {
    if ($line -match "^sha256`t([0-9A-Fa-f]{64})$") {
      $ExpectedArchiveSha = $Matches[1].ToUpperInvariant()
    }
  }
}

if (-not (Test-Path -LiteralPath $Archive)) {
  throw "Frida Gum Windows x86-64 17.9.5 archive missing at $Archive; download the pinned archive named in devkit-pin.tsv"
}
$ArchiveSha = (Get-FileHash -Algorithm SHA256 -LiteralPath $Archive).Hash.ToUpperInvariant()
if ($ArchiveSha -ne $ExpectedArchiveSha) {
  throw "Frida archive SHA-256 mismatch: got $ArchiveSha expected $ExpectedArchiveSha"
}

# Always extract from the verified archive into a clean directory so linked .lib/.h cannot diverge from the hash.
if (Test-Path -LiteralPath $Devkit) {
  Remove-Item -LiteralPath $Devkit -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $Devkit | Out-Null
tar -xf $Archive -C $Devkit
$Lib = Join-Path $Devkit "frida-gum.lib"
$Header = Join-Path $Devkit "frida-gum.h"
if (-not (Test-Path -LiteralPath $Lib)) { throw "frida-gum.lib missing after extract from verified archive" }
if (-not (Test-Path -LiteralPath $Header)) { throw "frida-gum.h missing after extract from verified archive" }
$LibSha = (Get-FileHash -Algorithm SHA256 -LiteralPath $Lib).Hash.ToUpperInvariant()
$HeaderSha = (Get-FileHash -Algorithm SHA256 -LiteralPath $Header).Hash.ToUpperInvariant()
@(
  "archive`t$Archive"
  "archive-sha256`t$ArchiveSha"
  "frida-gum.lib-sha256`t$LibSha"
  "frida-gum.h-sha256`t$HeaderSha"
  "extract`tclean-from-verified-archive"
) | Set-Content -Encoding ascii (Join-Path $Evidence "devkit-auth.tsv")

$VsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $VsWhere)) { throw "vswhere missing: $VsWhere" }
$Install = & $VsWhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (-not $Install) { throw "vswhere returned no installationPath" }
$VcVars = Join-Path $Install "VC\Auxiliary\Build\vcvarsall.bat"
if (-not (Test-Path $VcVars)) { throw "vcvarsall missing: $VcVars" }

$Batch = Join-Path $env:TEMP "macroonz-fuzz-frida-vcvars-dump.bat"
@(
  "@echo off"
  "call `"$VcVars`" x64"
  "set PATH"
  "set LIB"
  "set INCLUDE"
) | Set-Content -Encoding ascii -LiteralPath $Batch

$psi = New-Object System.Diagnostics.ProcessStartInfo
$psi.FileName = "$env:SystemRoot\System32\cmd.exe"
$psi.Arguments = "/d /c `"$Batch`""
$psi.UseShellExecute = $false
$psi.RedirectStandardOutput = $true
$psi.RedirectStandardError = $true
$psi.CreateNoWindow = $true
$psi.EnvironmentVariables.Clear()
$psi.EnvironmentVariables["SystemRoot"] = $env:SystemRoot
$psi.EnvironmentVariables["SYSTEMROOT"] = $env:SystemRoot
$psi.EnvironmentVariables["SystemDrive"] = $env:SystemDrive
$psi.EnvironmentVariables["windir"] = $env:SystemRoot
$psi.EnvironmentVariables["ComSpec"] = "$env:SystemRoot\System32\cmd.exe"
$psi.EnvironmentVariables["PATH"] = "$env:SystemRoot\System32;$env:SystemRoot"
$p = [Diagnostics.Process]::Start($psi)
$Dump = $p.StandardOutput.ReadToEnd()
$Err = $p.StandardError.ReadToEnd()
$p.WaitForExit()
Remove-Item -LiteralPath $Batch -Force -ErrorAction SilentlyContinue
if ($p.ExitCode -ne 0) { throw "vcvarsall failed: $Err" }

$EnvMap = @{}
foreach ($line in ($Dump -split "`r?`n")) {
  if ($line -match "^(PATH|LIB|INCLUDE)=(.*)$") {
    $EnvMap[$Matches[1]] = $Matches[2]
  }
}
if (-not $EnvMap.ContainsKey("LIB")) { throw "vcvarsall did not produce LIB" }

$EnvMap["LIB"] = "$Devkit;$($EnvMap['LIB'])"
$EnvMap["INCLUDE"] = if ($EnvMap.ContainsKey("INCLUDE")) { "$Devkit;$($EnvMap['INCLUDE'])" } else { $Devkit }

$TargetLibdir = (& rustc +1.98.0 --print target-libdir).Trim()
$CargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $env:USERPROFILE ".cargo" }
$RustupHome = if ($env:RUSTUP_HOME) { $env:RUSTUP_HOME } else { Join-Path $env:USERPROFILE ".rustup" }
$EnvMap["PATH"] = "$TargetLibdir;$CargoHome\bin;$($EnvMap['PATH'])"

$env:PATH = $EnvMap["PATH"]
$env:LIB = $EnvMap["LIB"]
$env:INCLUDE = $EnvMap["INCLUDE"]
$env:CARGO_TARGET_DIR = $Build
$env:CARGO_HOME = $CargoHome
$env:RUSTUP_HOME = $RustupHome
$env:MACROONZ_FUZZ_FRIDA_WORK = $Work

@(
  "vswhere`t$Install"
  "vcvarsall`t$VcVars"
  "vcvars-mode`tclean-child-env"
  "target-libdir`t$TargetLibdir"
  "frida-lib`t$Devkit"
  "frida-archive-sha256`t$ArchiveSha"
  "work`t$Work"
) | Set-Content -Encoding ascii (Join-Path $Evidence "cold-shell-launcher.tsv")

Set-Location $Source
$ErrorActionPreference = "Continue"
cargo +1.98.0 build -p macroonz-fuzz-frida-driver
$buildExit = $LASTEXITCODE
$ErrorActionPreference = "Stop"
if ($buildExit -ne 0) { throw "cargo build failed" }
& "$Build\debug\macroonz-fuzz-frida-driver.exe"
if ($LASTEXITCODE -ne 0) { throw "driver failed: $LASTEXITCODE" }
