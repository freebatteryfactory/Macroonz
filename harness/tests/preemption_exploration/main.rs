//! The target-qualified preemption road, exercised from outside on both sides of its availability boundary.
//!
//! Supported targets execute the pinned backend and its hostile controls, while unsupported targets retain and type-check the unavailable result plane without compiling that backend.

#[cfg(any(
    all(
        unix,
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "x86_64",
            target_arch = "loongarch64",
            target_arch = "riscv64",
            all(target_arch = "powerpc64", target_endian = "little"),
        ),
    ),
    all(windows, any(target_arch = "x86_64", target_arch = "aarch64")),
))]
mod supported;

#[cfg(not(any(
    all(
        unix,
        any(
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "x86_64",
            target_arch = "loongarch64",
            target_arch = "riscv64",
            all(target_arch = "powerpc64", target_endian = "little"),
        ),
    ),
    all(windows, any(target_arch = "x86_64", target_arch = "aarch64")),
)))]
mod unavailable;
