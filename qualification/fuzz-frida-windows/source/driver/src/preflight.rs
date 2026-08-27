//! Typed Windows cold-shell choreography preflight for the F0 Frida final exam.
//!
//! Discovery uses vendor-supported mechanisms only: Microsoft `vswhere` + `vcvarsall`,
//! and `rustc --print` for sysroot / target-libdir / host-tuple.
//! This module never mutates process environment (Safe Rust / no `set_var`).
//! A cold-shell launcher applies the composed env before build and run.

use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

/// Expected SHA-256 of the pinned Frida Gum Windows x86_64 devkit archive (uppercase hex).
pub(crate) const FRIDA_DEVKIT_SHA256: &str =
    "07E0DF78E2EF962D8228A3C9866F97B6D9BEEA310434377DCCCFA402B01F9DE1";

/// One named capability the adopter preflight must resolve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Capability {
    VsWhere,
    VcVarsAll,
    ComposedMsvcSdkEnv,
    Rustc198,
    RustHostTuple,
    RustSysroot,
    RustTargetLibdir,
    RustStdDll,
    LlvmReported,
    FridaGumLib,
    FridaGumHeader,
    FridaDevkitHash,
}

impl Capability {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::VsWhere => "vswhere",
            Self::VcVarsAll => "vcvarsall",
            Self::ComposedMsvcSdkEnv => "composed-msvc-sdk-env",
            Self::Rustc198 => "rustc-1.98",
            Self::RustHostTuple => "rustc-host-tuple",
            Self::RustSysroot => "rustc-sysroot",
            Self::RustTargetLibdir => "rustc-target-libdir",
            Self::RustStdDll => "rust-std-dll",
            Self::LlvmReported => "llvm-reported",
            Self::FridaGumLib => "frida-gum-lib",
            Self::FridaGumHeader => "frida-gum-header",
            Self::FridaDevkitHash => "frida-devkit-hash",
        }
    }
}

/// Typed available/unavailable result with one actionable fact.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PreflightStatus {
    Available { fact: String },
    Unavailable { fact: String },
}

impl PreflightStatus {
    pub(crate) const fn tag(&self) -> &'static str {
        match self {
            Self::Available { .. } => "available",
            Self::Unavailable { .. } => "unavailable",
        }
    }

    pub(crate) fn fact(&self) -> &str {
        match self {
            Self::Available { fact } | Self::Unavailable { fact } => fact.as_str(),
        }
    }
}

/// One preflight row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PreflightRow {
    pub(crate) capability: Capability,
    pub(crate) status: PreflightStatus,
}

/// Environment composition produced by Microsoft's vcvars road.
#[derive(Debug, Clone, Default)]
pub(crate) struct ComposedEnv {
    pub(crate) path: Option<String>,
    pub(crate) lib: Option<String>,
    pub(crate) include: Option<String>,
}

/// Run cold-shell discovery without mutating this process environment.
#[must_use]
pub(crate) fn probe_cold(workspace: &Path, devkit: &Path) -> (Vec<PreflightRow>, ComposedEnv) {
    let mut rows = Vec::new();
    let (vswhere_row, vs_install) = probe_vswhere();
    rows.push(vswhere_row);

    let (vcvars_row, vcvars) = match vs_install {
        Some(install) => probe_vcvarsall(&install),
        None => (
            PreflightRow {
                capability: Capability::VcVarsAll,
                status: PreflightStatus::Unavailable {
                    fact: "skipped: vswhere did not yield an installation path".to_owned(),
                },
            },
            None,
        ),
    };
    rows.push(vcvars_row);

    let (compose_row, composed) = match vcvars {
        Some(path) => compose_vcvars_env(&path),
        None => (
            PreflightRow {
                capability: Capability::ComposedMsvcSdkEnv,
                status: PreflightStatus::Unavailable {
                    fact: "skipped: vcvarsall.bat was not located".to_owned(),
                },
            },
            ComposedEnv::default(),
        ),
    };
    rows.push(compose_row);

    rows.push(probe_rustc());
    rows.push(probe_rust_print(
        Capability::RustHostTuple,
        &["+1.98.0", "--print", "host-tuple"],
    ));
    rows.push(probe_rust_print(
        Capability::RustSysroot,
        &["+1.98.0", "--print", "sysroot"],
    ));
    rows.push(probe_rust_print(
        Capability::RustTargetLibdir,
        &["+1.98.0", "--print", "target-libdir"],
    ));
    rows.push(probe_std_dll_from_target_libdir());
    rows.push(probe_llvm_reported());
    rows.push(probe_file(
        Capability::FridaGumLib,
        &devkit.join("frida-gum.lib"),
    ));
    rows.push(probe_file(
        Capability::FridaGumHeader,
        &devkit.join("frida-gum.h"),
    ));
    rows.push(probe_frida_hash(workspace));
    (rows, composed)
}

/// Persist composed env lines for the cold-shell launcher (PATH/LIB/INCLUDE).
pub(crate) fn write_composed_env(
    writer: &mut impl Write,
    composed: &ComposedEnv,
) -> io::Result<()> {
    writeln!(writer, "key\tvalue")?;
    if let Some(path) = &composed.path {
        writeln!(writer, "PATH\t{path}")?;
    }
    if let Some(lib) = &composed.lib {
        writeln!(writer, "LIB\t{lib}")?;
    }
    if let Some(include) = &composed.include {
        writeln!(writer, "INCLUDE\t{include}")?;
    }
    Ok(())
}

fn probe_vswhere() -> (PreflightRow, Option<PathBuf>) {
    let candidate =
        PathBuf::from(r"C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe");
    if !candidate.is_file() {
        return (
            PreflightRow {
                capability: Capability::VsWhere,
                status: PreflightStatus::Unavailable {
                    fact: format!("missing {}", candidate.display()),
                },
            },
            None,
        );
    }
    match Command::new(&candidate)
        .args([
            "-latest",
            "-products",
            "*",
            "-requires",
            "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
            "-property",
            "installationPath",
        ])
        .output()
    {
        Ok(output) if output.status.success() => {
            let install = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            if install.is_empty() {
                (
                    PreflightRow {
                        capability: Capability::VsWhere,
                        status: PreflightStatus::Unavailable {
                            fact: "vswhere returned an empty installationPath".to_owned(),
                        },
                    },
                    None,
                )
            } else {
                (
                    PreflightRow {
                        capability: Capability::VsWhere,
                        status: PreflightStatus::Available {
                            fact: format!("{} -> {install}", candidate.display()),
                        },
                    },
                    Some(PathBuf::from(install)),
                )
            }
        }
        Ok(output) => (
            PreflightRow {
                capability: Capability::VsWhere,
                status: PreflightStatus::Unavailable {
                    fact: format!("vswhere exited {}", output.status),
                },
            },
            None,
        ),
        Err(error) => (
            PreflightRow {
                capability: Capability::VsWhere,
                status: PreflightStatus::Unavailable {
                    fact: format!("vswhere could not start: {error}"),
                },
            },
            None,
        ),
    }
}

fn probe_vcvarsall(install: &Path) -> (PreflightRow, Option<PathBuf>) {
    let vcvars = install
        .join("VC")
        .join("Auxiliary")
        .join("Build")
        .join("vcvarsall.bat");
    if vcvars.is_file() {
        (
            PreflightRow {
                capability: Capability::VcVarsAll,
                status: PreflightStatus::Available {
                    fact: vcvars.display().to_string(),
                },
            },
            Some(vcvars),
        )
    } else {
        (
            PreflightRow {
                capability: Capability::VcVarsAll,
                status: PreflightStatus::Unavailable {
                    fact: format!("missing {}", vcvars.display()),
                },
            },
            None,
        )
    }
}

fn compose_vcvars_env(vcvars: &Path) -> (PreflightRow, ComposedEnv) {
    let batch = std::env::temp_dir().join(format!(
        "macroonz-f0-vcvars-{}.bat",
        std::process::id()
    ));
    let script = format!(
        "@echo off\r\ncall \"{}\" x64\r\nset PATH\r\nset LIB\r\nset INCLUDE\r\n",
        vcvars.display()
    );
    if let Err(error) = fs::write(&batch, script) {
        return (
            PreflightRow {
                capability: Capability::ComposedMsvcSdkEnv,
                status: PreflightStatus::Unavailable {
                    fact: format!("could not write temp vcvars batch: {error}"),
                },
            },
            ComposedEnv::default(),
        );
    }
    let system_root = std::env::var_os("SystemRoot").unwrap_or_else(|| r"C:\Windows".into());
    let system_drive = std::env::var_os("SystemDrive").unwrap_or_else(|| "C:".into());
    let comspec = PathBuf::from(&system_root).join("System32").join("cmd.exe");
    let path_min = format!(
        "{};{}",
        PathBuf::from(&system_root).join("System32").display(),
        PathBuf::from(&system_root).display()
    );
    let result = Command::new(&comspec)
        .args(["/d", "/c", &batch.display().to_string()])
        .env_clear()
        .env("SystemRoot", &system_root)
        .env("SYSTEMROOT", &system_root)
        .env("SystemDrive", &system_drive)
        .env("windir", &system_root)
        .env("ComSpec", &comspec)
        .env("PATH", &path_min)
        .output();
    let _ = fs::remove_file(&batch);
    match result {
        Ok(output) if output.status.success() => {
            let text = String::from_utf8_lossy(&output.stdout);
            let mut composed = ComposedEnv::default();
            for line in text.lines() {
                if let Some(value) = line.strip_prefix("PATH=") {
                    composed.path = Some(value.to_owned());
                } else if let Some(value) = line.strip_prefix("LIB=") {
                    composed.lib = Some(value.to_owned());
                } else if let Some(value) = line.strip_prefix("INCLUDE=") {
                    composed.include = Some(value.to_owned());
                }
            }
            let lib_ok = composed
                .lib
                .as_deref()
                .is_some_and(|lib| lib_has_msvc(lib) && lib_has_sdk_um(lib));
            if lib_ok {
                (
                    PreflightRow {
                        capability: Capability::ComposedMsvcSdkEnv,
                        status: PreflightStatus::Available {
                            fact: format!(
                                "vcvarsall x64 composed via clean cmd env from {}",
                                vcvars.display()
                            ),
                        },
                    },
                    composed,
                )
            } else {
                (
                    PreflightRow {
                        capability: Capability::ComposedMsvcSdkEnv,
                        status: PreflightStatus::Unavailable {
                            fact:
                                "vcvarsall ran but LIB lacked MSVC and/or Windows Kits um\\x64 roots"
                                    .to_owned(),
                        },
                    },
                    composed,
                )
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            (
                PreflightRow {
                    capability: Capability::ComposedMsvcSdkEnv,
                    status: PreflightStatus::Unavailable {
                        fact: format!(
                            "vcvarsall composition exited {}; stderr={}",
                            output.status,
                            stderr.trim()
                        ),
                    },
                },
                ComposedEnv::default(),
            )
        }
        Err(error) => (
            PreflightRow {
                capability: Capability::ComposedMsvcSdkEnv,
                status: PreflightStatus::Unavailable {
                    fact: format!("vcvarsall composition could not start: {error}"),
                },
            },
            ComposedEnv::default(),
        ),
    }
}

fn lib_has_msvc(lib: &str) -> bool {
    std::env::split_paths(lib).any(|path| {
        let text = path.to_string_lossy();
        text.contains("VC") && text.contains("Tools") && text.contains("MSVC") && path.is_dir()
    })
}

fn lib_has_sdk_um(lib: &str) -> bool {
    std::env::split_paths(lib).any(|path| {
        let text = path.to_string_lossy().to_ascii_lowercase();
        text.contains("windows kits") && text.contains("um") && text.contains("x64") && path.is_dir()
    })
}

fn probe_rustc() -> PreflightRow {
    match Command::new("rustc").args(["+1.98.0", "--version"]).output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            if version.contains("1.98.0") {
                PreflightRow {
                    capability: Capability::Rustc198,
                    status: PreflightStatus::Available { fact: version },
                }
            } else {
                PreflightRow {
                    capability: Capability::Rustc198,
                    status: PreflightStatus::Unavailable {
                        fact: format!("rustc +1.98.0 reported `{version}`"),
                    },
                }
            }
        }
        Ok(output) => PreflightRow {
            capability: Capability::Rustc198,
            status: PreflightStatus::Unavailable {
                fact: format!("rustc +1.98.0 exited {}", output.status),
            },
        },
        Err(error) => PreflightRow {
            capability: Capability::Rustc198,
            status: PreflightStatus::Unavailable {
                fact: format!("rustc +1.98.0 could not start: {error}"),
            },
        },
    }
}

fn probe_rust_print(capability: Capability, args: &[&str]) -> PreflightRow {
    match Command::new("rustc").args(args).output() {
        Ok(output) if output.status.success() => {
            let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            if value.is_empty() {
                PreflightRow {
                    capability,
                    status: PreflightStatus::Unavailable {
                        fact: format!("rustc {} returned empty stdout", args.join(" ")),
                    },
                }
            } else {
                PreflightRow {
                    capability,
                    status: PreflightStatus::Available { fact: value },
                }
            }
        }
        Ok(output) => PreflightRow {
            capability,
            status: PreflightStatus::Unavailable {
                fact: format!("rustc {} exited {}", args.join(" "), output.status),
            },
        },
        Err(error) => PreflightRow {
            capability,
            status: PreflightStatus::Unavailable {
                fact: format!("rustc {} could not start: {error}", args.join(" ")),
            },
        },
    }
}

fn probe_std_dll_from_target_libdir() -> PreflightRow {
    match Command::new("rustc")
        .args(["+1.98.0", "--print", "target-libdir"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let dir = PathBuf::from(String::from_utf8_lossy(&output.stdout).trim());
            match fs::read_dir(&dir) {
                Ok(entries) => {
                    let found = entries.filter_map(Result::ok).any(|entry| {
                        entry.file_name().to_string_lossy().starts_with("std-")
                            && entry.path().extension().is_some_and(|ext| ext == "dll")
                    });
                    if found {
                        PreflightRow {
                            capability: Capability::RustStdDll,
                            status: PreflightStatus::Available {
                                fact: dir.display().to_string(),
                            },
                        }
                    } else {
                        PreflightRow {
                            capability: Capability::RustStdDll,
                            status: PreflightStatus::Unavailable {
                                fact: format!("no std-*.dll under {}", dir.display()),
                            },
                        }
                    }
                }
                Err(error) => PreflightRow {
                    capability: Capability::RustStdDll,
                    status: PreflightStatus::Unavailable {
                        fact: format!("target-libdir unreadable: {error}"),
                    },
                },
            }
        }
        Ok(output) => PreflightRow {
            capability: Capability::RustStdDll,
            status: PreflightStatus::Unavailable {
                fact: format!("rustc --print target-libdir exited {}", output.status),
            },
        },
        Err(error) => PreflightRow {
            capability: Capability::RustStdDll,
            status: PreflightStatus::Unavailable {
                fact: format!("rustc --print target-libdir could not start: {error}"),
            },
        },
    }
}

fn probe_llvm_reported() -> PreflightRow {
    match Command::new("rustc").args(["+1.98.0", "-vV"]).output() {
        Ok(output) if output.status.success() => {
            let text = String::from_utf8_lossy(&output.stdout);
            match text
                .lines()
                .find_map(|line| line.strip_prefix("LLVM version: "))
            {
                Some(version) => PreflightRow {
                    capability: Capability::LlvmReported,
                    status: PreflightStatus::Available {
                        fact: version.trim().to_owned(),
                    },
                },
                None => PreflightRow {
                    capability: Capability::LlvmReported,
                    status: PreflightStatus::Unavailable {
                        fact: "rustc -vV lacked an LLVM version line".to_owned(),
                    },
                },
            }
        }
        Ok(output) => PreflightRow {
            capability: Capability::LlvmReported,
            status: PreflightStatus::Unavailable {
                fact: format!("rustc -vV exited {}", output.status),
            },
        },
        Err(error) => PreflightRow {
            capability: Capability::LlvmReported,
            status: PreflightStatus::Unavailable {
                fact: format!("rustc -vV could not start: {error}"),
            },
        },
    }
}

fn probe_file(capability: Capability, path: &Path) -> PreflightRow {
    if path.is_file() {
        PreflightRow {
            capability,
            status: PreflightStatus::Available {
                fact: path.display().to_string(),
            },
        }
    } else {
        PreflightRow {
            capability,
            status: PreflightStatus::Unavailable {
                fact: format!("missing {}", path.display()),
            },
        }
    }
}

fn probe_frida_hash(workspace: &Path) -> PreflightRow {
    let archive = workspace
        .join("devkit")
        .join("frida-gum-devkit-17.9.5-windows-x86_64.tar.xz");
    if !archive.is_file() {
        return PreflightRow {
            capability: Capability::FridaDevkitHash,
            status: PreflightStatus::Unavailable {
                fact: format!("missing {}", archive.display()),
            },
        };
    }
    let script = format!(
        "(Get-FileHash -Algorithm SHA256 -LiteralPath '{}').Hash",
        archive.display()
    );
    let powershell = PathBuf::from(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe");
    match Command::new(&powershell)
        .args(["-NoProfile", "-Command", &script])
        .output()
    {
        Ok(output) if output.status.success() => {
            let sha = String::from_utf8_lossy(&output.stdout).trim().to_owned();
            if sha.eq_ignore_ascii_case(FRIDA_DEVKIT_SHA256) {
                PreflightRow {
                    capability: Capability::FridaDevkitHash,
                    status: PreflightStatus::Available {
                        fact: format!("sha256={sha}; path={}", archive.display()),
                    },
                }
            } else {
                PreflightRow {
                    capability: Capability::FridaDevkitHash,
                    status: PreflightStatus::Unavailable {
                        fact: format!("sha256 mismatch: got {sha}, expected {FRIDA_DEVKIT_SHA256}"),
                    },
                }
            }
        }
        Ok(output) => PreflightRow {
            capability: Capability::FridaDevkitHash,
            status: PreflightStatus::Unavailable {
                fact: format!("Get-FileHash exited {}", output.status),
            },
        },
        Err(error) => PreflightRow {
            capability: Capability::FridaDevkitHash,
            status: PreflightStatus::Unavailable {
                fact: format!("Get-FileHash could not start: {error}"),
            },
        },
    }
}

/// Write preflight rows as machine-readable lines.
pub(crate) fn write_rows(writer: &mut impl Write, rows: &[PreflightRow]) -> io::Result<()> {
    for row in rows {
        writeln!(
            writer,
            "preflight\t{}\t{}\t{}",
            row.capability.as_str(),
            row.status.tag(),
            row.status.fact()
        )?;
    }
    Ok(())
}

/// Whether every capability is available.
#[must_use]
pub(crate) fn all_available(rows: &[PreflightRow]) -> bool {
    rows.iter()
        .all(|row| matches!(row.status, PreflightStatus::Available { .. }))
}
