//! Advanced Rust and explicit unsafe custody observed through one disposable renamed-facade adopter.

use macroonz_compiler::{
    GeneratedToken, GeneratedTree, decorated, documentation, function_item, function_signature,
};
use std::path::Path;
use std::process::Output;

#[path = "support/scratch.rs"]
mod scratch;

use scratch::{cargo, command_refusal, manifest_path, observed_in_scratch_for, repository_root};

#[derive(Clone, Copy)]
enum Boundary {
    Safe,
    Unsafe,
}

#[derive(Clone, Copy)]
enum SafetyContract {
    Documented,
    Missing,
}

#[test]
fn advanced_rust_and_explicit_unsafe_remain_caller_and_rustc_authority() -> Result<(), String> {
    observed_in_scratch_for("advanced_rust", observe_matrix)
}

fn observe_matrix(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch)?;
    let safe_preset = generated_function(Boundary::Safe, SafetyContract::Missing)?;
    if safe_preset.contains("unsafe fn") {
        return Err("a safe function preset emitted an unsafe boundary".to_owned());
    }
    write_generated_bins(scratch)?;

    observe_lawful_package(scratch)?;
    observe_generated_unsafe(scratch)?;
    observe_rustc_refusals(scratch)
}

fn observe_lawful_package(scratch: &Path) -> Result<(), String> {
    require_success(
        "advanced-Rust lock generation",
        &cargo(scratch, &["generate-lockfile", "--offline"])?,
    )?;
    require_success(
        "advanced-Rust package tests",
        &cargo(scratch, &["test", "--locked", "--offline"])?,
    )?;
    require_success(
        "advanced-Rust strict Clippy",
        &cargo(
            scratch,
            &[
                "clippy",
                "--lib",
                "--tests",
                "--locked",
                "--offline",
                "--",
                "-Dclippy::missing-safety-doc",
            ],
        )?,
    )?;
    require_success(
        "advanced-Rust Wasm posture",
        &cargo(
            scratch,
            &[
                "check",
                "--lib",
                "--locked",
                "--offline",
                "--target",
                "wasm32-unknown-unknown",
            ],
        )?,
    )
}

fn observe_generated_unsafe(scratch: &Path) -> Result<(), String> {
    let documented = scratch.join("generated-documented");
    let missing = scratch.join("generated-missing");
    require_success(
        "documented generated unsafe lock generation",
        &cargo(&documented, &["generate-lockfile", "--offline"])?,
    )?;
    require_success(
        "missing-doc generated unsafe lock generation",
        &cargo(&missing, &["generate-lockfile", "--offline"])?,
    )?;
    require_success(
        "documented generated unsafe API",
        &cargo(
            &documented,
            &[
                "clippy",
                "--locked",
                "--offline",
                "--",
                "-Dclippy::missing-safety-doc",
            ],
        )?,
    )?;
    require_refusal(
        "generated unsafe safety documentation",
        &cargo(
            &missing,
            &[
                "clippy",
                "--locked",
                "--offline",
                "--",
                "-Dclippy::missing-safety-doc",
            ],
        )?,
        &[
            "unsafe function's docs are missing a `# Safety` section",
            "clippy::missing_safety_doc",
        ],
    )
}

fn observe_rustc_refusals(scratch: &Path) -> Result<(), String> {
    require_refusal(
        "borrow authority",
        &cargo(
            scratch,
            &[
                "check",
                "--features",
                "hostile_borrow",
                "--bin",
                "borrow-refusal",
                "--locked",
                "--offline",
            ],
        )?,
        &["E0515", "cannot return reference to local variable"],
    )?;
    require_refusal(
        "unsafe operation authority",
        &cargo(
            scratch,
            &[
                "check",
                "--features",
                "hostile_unsafe_op",
                "--bin",
                "unsafe-operation-refusal",
                "--locked",
                "--offline",
            ],
        )?,
        &["E0133", "unsafe_op_in_unsafe_fn"],
    )?;
    require_refusal(
        "unsafe implementation authority",
        &cargo(
            scratch,
            &[
                "check",
                "--features",
                "hostile_unsafe_impl",
                "--bin",
                "unsafe-impl-refusal",
                "--locked",
                "--offline",
            ],
        )?,
        &["E0200", "requires an `unsafe impl` declaration"],
    )?;
    require_refusal(
        "unsafe external-block authority",
        &cargo(
            scratch,
            &[
                "check",
                "--features",
                "hostile_unsafe_extern",
                "--bin",
                "unsafe-extern-refusal",
                "--locked",
                "--offline",
            ],
        )?,
        &["extern blocks must be unsafe"],
    )?;
    require_refusal(
        "unsafe attribute authority",
        &cargo(
            scratch,
            &[
                "check",
                "--features",
                "hostile_unsafe_attribute",
                "--bin",
                "unsafe-attribute-refusal",
                "--locked",
                "--offline",
            ],
        )?,
        &["unsafe attribute used without unsafe", "no_mangle"],
    )?;
    Ok(())
}

fn require_success(label: &str, output: &Output) -> Result<(), String> {
    if output.status.success() {
        Ok(())
    } else {
        Err(command_refusal(label, output))
    }
}

fn require_refusal(label: &str, output: &Output, anchors: &[&str]) -> Result<(), String> {
    if output.status.success() {
        return Err(format!("{label} unexpectedly compiled"));
    }
    let reading = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    for anchor in anchors {
        if !reading.contains(anchor) {
            return Err(format!(
                "{label} omitted diagnostic anchor `{anchor}`\n{reading}"
            ));
        }
    }
    Ok(())
}

fn write_specimen(scratch: &Path) -> Result<(), String> {
    let facade = manifest_path(repository_root()?)?;
    for relative in ["src", "src/bin", "tests"] {
        std::fs::create_dir(scratch.join(relative)).map_err(|error| error.to_string())?;
    }
    let manifest = ADVANCED_MANIFEST.replace("{facade}", facade.as_str());
    for (relative, source) in [
        ("Cargo.toml", manifest.as_str()),
        ("src/lib.rs", ADVANCED_PRODUCER),
        ("tests/advanced.rs", ADVANCED_CONSUMER),
        ("src/bin/borrow_refusal.rs", BORROW_REFUSAL),
        (
            "src/bin/unsafe_operation_refusal.rs",
            UNSAFE_OPERATION_REFUSAL,
        ),
        ("src/bin/unsafe_impl_refusal.rs", UNSAFE_IMPL_REFUSAL),
        ("src/bin/unsafe_extern_refusal.rs", UNSAFE_EXTERN_REFUSAL),
        (
            "src/bin/unsafe_attribute_refusal.rs",
            UNSAFE_ATTRIBUTE_REFUSAL,
        ),
    ] {
        std::fs::write(scratch.join(relative), source).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn write_generated_bins(scratch: &Path) -> Result<(), String> {
    for (directory, name, source) in [
        (
            "generated-documented",
            "generated-documented",
            generated_function(Boundary::Unsafe, SafetyContract::Documented)?,
        ),
        (
            "generated-missing",
            "generated-missing",
            generated_function(Boundary::Unsafe, SafetyContract::Missing)?,
        ),
    ] {
        let package = scratch.join(directory);
        std::fs::create_dir_all(package.join("src")).map_err(|error| error.to_string())?;
        let manifest = GENERATED_MANIFEST.replace("{name}", name);
        std::fs::write(package.join("Cargo.toml"), manifest).map_err(|error| error.to_string())?;
        std::fs::write(package.join("src/main.rs"), source).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn generated_function(
    boundary: Boundary,
    safety_contract: SafetyContract,
) -> Result<String, String> {
    let qualifiers = match boundary {
        Boundary::Safe => Vec::new(),
        Boundary::Unsafe => vec![GeneratedToken::word("unsafe")],
    };
    let signature = function_signature(
        qualifiers,
        GeneratedToken::word("generated_boundary"),
        Vec::new(),
        Vec::new(),
        None,
        Vec::new(),
    )
    .map_err(|error| error.to_string())?;
    let item = function_item(signature, Vec::new()).map_err(|error| error.to_string())?;
    let attributes = match safety_contract {
        SafetyContract::Documented => vec![documentation(
            "One generated caller-owned boundary.\n\n# Safety\n\nThe caller must uphold the declared external contract.",
        )
        .map_err(|error| error.to_string())?],
        SafetyContract::Missing => Vec::new(),
    };
    let item = decorated(attributes, vec![GeneratedToken::word("pub")], item);
    let generated = GeneratedTree::assembled(item).map_err(|error| error.to_string())?;
    Ok(format!(
        "#![deny(warnings)]\n#![deny(clippy::missing_safety_doc)]\n{}\nfn main() {{}}\n",
        generated.inspected()
    ))
}

const ADVANCED_MANIFEST: &str = r#"[package]
name = "advanced-recipe-adopter"
version = "0.0.0"
edition = "2024"
rust-version = "1.98.1"
publish = false
autobins = false
autoexamples = false
autotests = false
autobenches = false
build = false

[lib]
path = "src/lib.rs"

[[test]]
name = "advanced"
path = "tests/advanced.rs"

[features]
hostile_borrow = []
hostile_unsafe_op = []
hostile_unsafe_impl = []
hostile_unsafe_extern = []
hostile_unsafe_attribute = []

[[bin]]
name = "borrow-refusal"
path = "src/bin/borrow_refusal.rs"
required-features = ["hostile_borrow"]
test = false
bench = false

[[bin]]
name = "unsafe-operation-refusal"
path = "src/bin/unsafe_operation_refusal.rs"
required-features = ["hostile_unsafe_op"]
test = false
bench = false

[[bin]]
name = "unsafe-impl-refusal"
path = "src/bin/unsafe_impl_refusal.rs"
required-features = ["hostile_unsafe_impl"]
test = false
bench = false

[[bin]]
name = "unsafe-extern-refusal"
path = "src/bin/unsafe_extern_refusal.rs"
required-features = ["hostile_unsafe_extern"]
test = false
bench = false

[[bin]]
name = "unsafe-attribute-refusal"
path = "src/bin/unsafe_attribute_refusal.rs"
required-features = ["hostile_unsafe_attribute"]
test = false
bench = false

[dependencies]
bakery = { package = "macroonz", path = "{facade}", default-features = false }

[lints.rust]
warnings = "deny"
unsafe_op_in_unsafe_fn = "deny"

[workspace]
"#;

const GENERATED_MANIFEST: &str = r#"[package]
name = "{name}"
version = "0.0.0"
edition = "2024"
rust-version = "1.98.1"
publish = false
autobins = true
autoexamples = false
autotests = false
autobenches = false
build = false

[lints.rust]
warnings = "deny"
unsafe_op_in_unsafe_fn = "deny"

[workspace]
"#;

const ADVANCED_PRODUCER: &str = r#"#![deny(warnings)]
#![deny(unsafe_op_in_unsafe_fn)]

bakery::recipe! {
    pub mod advanced {
        use core::marker::PhantomData;
        use core::pin::Pin;

        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct Batch<'a, T, const N: usize>
        where
            T: Copy,
        {
            value: T,
            marker: PhantomData<&'a [T; N]>,
        }

        impl<'a, T, const N: usize> Batch<'a, T, N>
        where
            T: Copy,
        {
            pub const fn new(value: T) -> Self {
                Self {
                    value,
                    marker: PhantomData,
                }
            }

            pub const fn shared(&self) -> &T {
                &self.value
            }

            pub const fn exclusive(&mut self) -> &mut T {
                &mut self.value
            }

            pub const fn consuming(self) -> T {
                self.value
            }

            pub fn pinned(self: Pin<&mut Self>) -> usize {
                let _pinned = self;
                N
            }
        }

        pub trait View<'a> {
            type Borrowed<'b>: Copy
            where
                Self: 'b,
                Self: 'a;

            const WIDTH: usize;

            fn borrowed(&'a self) -> Self::Borrowed<'a>;
        }

        impl<'a, T, const N: usize> View<'a> for Batch<'a, T, N>
        where
            T: Copy + 'a,
        {
            type Borrowed<'b> = &'b T
            where
                Self: 'b,
                Self: 'a;

            const WIDTH: usize = N;

            fn borrowed(&'a self) -> Self::Borrowed<'a> {
                &self.value
            }
        }

        pub async fn async_copy<T>(value: T) -> T
        where
            T: Copy,
        {
            value
        }

        pub fn precise<'a, T>(value: &'a T) -> impl Copy + use<'a, T>
        where
            T: Copy + 'a,
        {
            value
        }

        pub fn map_for_all<T, F>(value: &T, map: F) -> &T
        where
            F: for<'a> Fn(&'a T) -> &'a T,
        {
            map(value)
        }

        pub mod r#type {
            pub const fn r#match() -> usize {
                3
            }
        }

        /// Reads one caller-owned pointer.
        ///
        /// # Safety
        ///
        /// The pointer must be aligned, initialized, and valid for one read.
        pub unsafe fn read_raw<T>(pointer: *const T) -> T
        where
            T: Copy,
        {
            unsafe { pointer.read() }
        }

        pub fn read_safe<T>(value: &T) -> T
        where
            T: Copy,
        {
            let pointer = value as *const T;
            unsafe { read_raw(pointer) }
        }

        #[doc = "Reads through one explicit caller-owned raw boundary.\n\n# Safety\n\nImplementations must read only from a valid pointer of the requested type."]
        pub unsafe trait RawView {
            /// Reads one pointer.
            ///
            /// # Safety
            ///
            /// The pointer must be aligned, initialized, and valid for one read.
            unsafe fn read<T: Copy>(pointer: *const T) -> T;
        }

        pub struct RawReader;

        unsafe impl RawView for RawReader {
            unsafe fn read<T: Copy>(pointer: *const T) -> T {
                unsafe { pointer.read() }
            }
        }

        unsafe extern "C" {
            pub safe fn abs(value: i32) -> i32;
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn macroonz_advanced_marker() -> u8 {
            1
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Stage {
            Empty,
            Full,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Action {
            Fill,
        }

        fn fill() {}

        bake! {
            vocabularies { Stage; Action; };
            transitions(Stage, Action) {
                (Empty, Fill) => Full with(super::fill);
            };
            absence(refused);
            projections {
                companions;
                dispatch(apply);
                typestate(Stage);
            };
        }
    }
}
"#;

const ADVANCED_CONSUMER: &str = r##"#![deny(warnings)]
#![deny(unsafe_op_in_unsafe_fn)]

use bakery::compiler::TextCapture;
use core::pin::Pin;

fn same<T>(value: &T) -> &T {
    value
}

#[test]
fn exact_advanced_rust_and_generated_typestate_share_one_recipe_room() -> Result<(), String> {
    use advanced_recipe_adopter::advanced::{Action, Batch, RawReader, RawView, Stage, View, baked};

    let mut shared = Batch::<'static, u8, 4>::new(7);
    assert_eq!(*shared.shared(), 7);
    *shared.exclusive() = 8;
    assert_eq!(View::borrowed(&shared), &8);
    assert_eq!(<Batch<'static, u8, 4> as View<'static>>::WIDTH, 4);
    let mut pinned = Batch::<'static, u8, 4>::new(9);
    assert_eq!(Batch::pinned(Pin::new(&mut pinned)), 4);
    assert_eq!(Batch::<'static, u8, 4>::new(10).consuming(), 10);
    let future = advanced_recipe_adopter::advanced::async_copy(11_u8);
    core::mem::drop(future);
    let precise = advanced_recipe_adopter::advanced::precise(&12_u8);
    let _retained = precise;
    assert_eq!(advanced_recipe_adopter::advanced::map_for_all(&13_u8, same), &13);
    assert_eq!(advanced_recipe_adopter::advanced::r#type::r#match(), 3);
    assert_eq!(advanced_recipe_adopter::advanced::read_safe(&14_u8), 14);
    let raw = 15_u8;
    let pointer = &raw as *const u8;
    // SAFETY: the pointer was derived from the live `raw` value immediately above.
    assert_eq!(unsafe { RawReader::read(pointer) }, 15);
    assert_eq!(advanced_recipe_adopter::advanced::macroonz_advanced_marker(), 1);
    assert_eq!(baked::apply(Stage::Empty, Action::Fill), Ok(Stage::Full));
    let _stage = baked::typestate::Stage::<baked::typestate::Empty>::new();

    let captured = TextCapture::read(EXPLICIT_UNSAFE).map_err(|error| error.to_string())?;
    let item = captured
        .input()
        .authored_item()
        .map_err(|error| error.to_string())?;
    let inspected = item
        .preserved()
        .generated()
        .map_err(|error| error.to_string())?
        .inspected();
    assert!(item.unsafe_token().is_some());
    assert!(inspected.contains("# Safety"));
    assert!(inspected.contains("unsafe fn read_raw"));
    assert!(inspected.contains("unsafe { pointer . read ( ) }"));
    Ok(())
}

const EXPLICIT_UNSAFE: &str = r#"#[doc = "Reads one pointer.\n\n# Safety\n\nThe pointer must be valid."] pub unsafe fn read_raw<T: Copy>(pointer: *const T) -> T { unsafe { pointer.read() } }"#;
"##;

const BORROW_REFUSAL: &str = r"#![deny(warnings)]

fn borrowed<'a>() -> &'a u8 {
    let local = 1_u8;
    &local
}

fn main() {
    let _borrow = borrowed();
}
";

const UNSAFE_OPERATION_REFUSAL: &str = r"#![deny(warnings)]
#![deny(unsafe_op_in_unsafe_fn)]

unsafe fn read(pointer: *const u8) -> u8 {
    pointer.read()
}

fn main() {}
";

const UNSAFE_IMPL_REFUSAL: &str = r"#![deny(warnings)]

unsafe trait Contract {}

struct Local;

impl Contract for Local {}

fn main() {}
";

const UNSAFE_EXTERN_REFUSAL: &str = r#"#![deny(warnings)]

extern "C" {
    fn abs(value: i32) -> i32;
}

fn main() {}
"#;

const UNSAFE_ATTRIBUTE_REFUSAL: &str = r#"#![deny(warnings)]

#[no_mangle]
pub extern "C" fn marker() {}

fn main() {}
"#;
