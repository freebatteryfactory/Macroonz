//! Root pattern output compiled and executed with independently authored admission and wire behavior.

use crate::scratch::{
    cargo_with_target, command_refusal, observed_in_scratch_for, repository_root,
};
use std::path::Path;
use std::process::{Command, Output};

#[test]
fn admitted_patterns_preserve_caller_validation_generics_and_wire_choices() -> Result<(), String> {
    observed_in_scratch_for("admitted_patterns", observe)
}

#[test]
fn admitted_patterns_public_standalone_consumer_executes() -> Result<(), String> {
    observed_in_scratch_for("admitted_patterns_example", |scratch| {
        std::fs::write(scratch.join("definitions.rs"), generated()?)
            .map_err(|error| error.to_string())?;
        let consumer = include_str!("../../../../examples/admitted_types/consumer.rs");
        run(scratch, consumer)?;
        refused(
            scratch,
            "expanded exported macro",
            &consumer.replace(
                "mod definitions;\nuse definitions::{BorrowedBatch, Identifier, Label};",
                "include!(\"definitions.rs\");",
            ),
            &[
                "macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths",
            ],
        )
    })
}

#[test]
fn admitted_patterns_refuse_admission_shortcuts_and_invalid_rust() -> Result<(), String> {
    observed_in_scratch_for("admitted_patterns_refusal", |scratch| {
        let declarations = generated()?;
        let failures = REFUSALS
            .iter()
            .filter_map(|(label, attempt, diagnostics)| {
                let source = format!("{declarations}\n{CONSUMER}\n{attempt}");
                refused(scratch, label, &source, diagnostics).err()
            })
            .collect::<Vec<_>>();
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures.join("\n"))
        }
    })
}

#[test]
fn admitted_patterns_preserve_every_declared_reach() -> Result<(), String> {
    observed_in_scratch_for("admitted_patterns_reach", |scratch| {
        let declarations = generated()?;
        let source = format!("{declarations}\n{CONSUMER}\n{REACHES}")
            .replace("fn main() {", "fn main() { observe_reaches();");
        run(scratch, &source)?;
        for (label, attempt) in [
            ("private sibling", "private_site::Local::try_new(1)"),
            ("module sibling", "module_site::Local::try_new(1)"),
            ("parent escape", "outer::parent_site::Local::try_new(1)"),
        ] {
            refused(
                scratch,
                label,
                &format!("{source}\npub fn forbidden() {{ let _ = {attempt}; }}"),
                &["E0603", "private"],
            )?;
        }
        cross_crate_reach(scratch, &source)
    })
}

fn generated() -> Result<String, String> {
    let root = repository_root()?;
    let generated = cargo_with_target(
        root,
        &root.join("target"),
        &[
            "run",
            "-j1",
            "--example",
            "admitted_types",
            "--no-default-features",
            "--locked",
            "--offline",
        ],
    )?;
    if !generated.status.success() {
        return Err(command_refusal("public admitted-types example", &generated));
    }
    String::from_utf8(generated.stdout).map_err(|error| error.to_string())
}

fn observe(scratch: &Path) -> Result<(), String> {
    run(scratch, &format!("{}\n{CONSUMER}", generated()?))
}

fn run(scratch: &Path, source: &str) -> Result<(), String> {
    let compiled = compile(scratch, source, &[])?;
    if !compiled.status.success() {
        return Err(command_refusal(
            "independent admitted-types consumer",
            &compiled,
        ));
    }
    let executed = Command::new(scratch.join("consumer.exe"))
        .output()
        .map_err(|error| error.to_string())?;
    if !executed.status.success() {
        return Err(command_refusal(
            "admitted-types runtime and wire controls",
            &executed,
        ));
    }
    Ok(())
}

fn refused(scratch: &Path, label: &str, source: &str, expected: &[&str]) -> Result<(), String> {
    let output = compile(scratch, source, &[])?;
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    if output.status.success() || expected.iter().any(|part| !diagnostic.contains(part)) {
        return Err(format!(
            "{label} missed its required refusal {expected:?}:\n{diagnostic}"
        ));
    }
    Ok(())
}

fn compile(scratch: &Path, source: &str, arguments: &[&str]) -> Result<Output, String> {
    let path = scratch.join("consumer.rs");
    std::fs::write(&path, source).map_err(|error| error.to_string())?;
    Command::new("rustc")
        .args([
            "+1.98.1",
            "--edition=2024",
            "--deny",
            "warnings",
            "--forbid",
            "unsafe_code",
        ])
        .arg(path)
        .arg("-o")
        .arg(scratch.join("consumer.exe"))
        .args(arguments)
        .current_dir(scratch)
        .output()
        .map_err(|error| error.to_string())
}

fn cross_crate_reach(scratch: &Path, source: &str) -> Result<(), String> {
    let library = scratch.join("libadmitted_library.rlib");
    let library_path = library.to_str().ok_or("library path is not UTF-8")?;
    let source = source.replace("fn main()", "pub fn verify()");
    let built_library = compile(
        scratch,
        &source,
        &["--crate-type=lib", "--crate-name=admitted_library"],
    )?;
    if !built_library.status.success() {
        return Err(command_refusal("admitted-pattern library", &built_library));
    }
    std::fs::rename(scratch.join("consumer.exe"), &library).map_err(|error| error.to_string())?;
    let binding = format!("admitted_library={library_path}");
    let lawful = "fn main() { let value = admitted_library::public_site::Local::try_new(2).unwrap(); assert_eq!(value.into_inner(), 2); admitted_library::verify(); }";
    let public_consumer = compile(scratch, lawful, &["--extern", &binding])?;
    if !public_consumer.status.success() {
        return Err(command_refusal(
            "public admitted-pattern consumer",
            &public_consumer,
        ));
    }
    let executed = Command::new(scratch.join("consumer.exe"))
        .output()
        .map_err(|error| error.to_string())?;
    if !executed.status.success() {
        return Err(command_refusal(
            "public admitted-pattern execution",
            &executed,
        ));
    }
    let forbidden = "fn main() { let _ = admitted_library::crate_site::Local::try_new(2); }";
    let private_consumer = compile(scratch, forbidden, &["--extern", &binding])?;
    let diagnostic = String::from_utf8_lossy(&private_consumer.stderr);
    if private_consumer.status.success()
        || !diagnostic.contains("E0603")
        || !diagnostic.contains("private")
    {
        return Err(format!(
            "crate reach escaped to an external consumer:\n{diagnostic}"
        ));
    }
    Ok(())
}

const REFUSALS: &[(&str, &str, &[&str])] = &[
    (
        "exported constructor",
        "pub fn forbidden() { let _ = Identifier(0); }",
        &["E0423", "private"],
    ),
    (
        "private constructor",
        "pub fn forbidden() { let _ = identifier_home::Value(0); }",
        &["E0603", "private"],
    ),
    (
        "private field",
        "pub fn forbidden(value: Identifier) { let _ = value.0; }",
        &["E0616", "private"],
    ),
    (
        "default bypass",
        "pub fn forbidden() { let _ = Identifier::default(); }",
        &["E0599", "default"],
    ),
    (
        "clone bypass",
        "pub fn forbidden(value: Identifier) { let _ = value.clone(); }",
        &["E0599", "clone"],
    ),
    (
        "shared mutation",
        "pub fn forbidden(value: Identifier) { *value.as_inner() = 0; }",
        &["E0594", "cannot assign"],
    ),
    (
        "mutable accessor",
        "pub fn forbidden(mut value: Identifier) { let _ = value.as_inner_mut(); }",
        &["E0599", "as_inner_mut"],
    ),
    (
        "consumed reuse",
        "pub fn forbidden(value: Identifier) { let _ = value.into_inner(); let _ = value.as_inner(); }",
        &["E0382", "moved"],
    ),
    (
        "caller bound",
        "pub fn forbidden() { let _ = BorrowedBatch::<(), 1>::try_new(&[()]); }",
        &["E0277", "Display"],
    ),
    (
        "validator output",
        "pub fn wrong(value: u64) -> Result<bool, Admission> { Ok(value != 0) } admitted_type! { pub Wrong in wrong_home; layout transparent; value u64; refusal crate::Admission; admit crate::wrong; }",
        &["E0308", "Result<u64", "Result<bool"],
    ),
    (
        "packed borrow",
        "admitted_type! { pub Packed in packed_home; layout packed; value u64; refusal crate::Admission; admit crate::admit_identifier; }",
        &["E0793", "unaligned"],
    ),
    (
        "invalid layout",
        "admitted_type! { pub Invalid in invalid_home; layout u8; value u64; refusal crate::Admission; admit crate::admit_identifier; }",
        &["`#[repr(u8)]` attribute cannot be used on structs", "enums"],
    ),
    (
        "unchecked derive seat",
        "admitted_type! { #[derive(Default)] pub Unchecked in unchecked_home; layout transparent; value u64; refusal crate::Admission; admit crate::admit_identifier; }",
        &["no rules expected", "#"],
    ),
    (
        "opaque reach",
        "macro_rules! forward { ($reach:vis $name:ident) => { admitted_type! { $reach $name in opaque_home; layout transparent; value u64; refusal crate::Admission; admit crate::admit_identifier; } }; } forward!(pub Opaque);",
        &["opaque", "visibility"],
    ),
];

const REACHES: &str = r"
mod private_site {
    admitted_type! { Local in local_home; layout transparent; value u64; refusal crate::Admission; admit crate::admit_identifier; }
    pub fn observe() { let value = Local::try_new(1).unwrap(); assert_eq!(*value.as_inner(), 1); assert_eq!(value.into_inner(), 1); }
}
mod module_site {
    admitted_type! { pub(self) Local in local_home; layout C; value u64; refusal crate::Admission; admit crate::admit_identifier; }
    pub fn observe() { let value = Local::try_new(2).unwrap(); assert_eq!(*value.as_inner(), 2); assert_eq!(value.into_inner(), 2); }
}
pub mod outer {
    pub mod parent_site {
        admitted_type! { pub(super) Local in local_home; layout Rust; value u64; refusal crate::Admission; admit crate::admit_identifier; }
    }
    pub fn observe() { let value = parent_site::Local::try_new(3).unwrap(); assert_eq!(*value.as_inner(), 3); assert_eq!(value.into_inner(), 3); }
}
pub mod crate_site {
    admitted_type! { pub(crate) Local in local_home; layout transparent; value u64; refusal crate::Admission; admit crate::admit_identifier; }
}
pub mod public_site {
    admitted_type! { pub Local in local_home; layout transparent; value u64; refusal crate::Admission; admit crate::admit_identifier; }
}
pub fn observe_reaches() {
    private_site::observe(); module_site::observe(); outer::observe();
    let value = crate_site::Local::try_new(4).unwrap(); assert_eq!(*value.as_inner(), 4); assert_eq!(value.into_inner(), 4);
    let value = public_site::Local::try_new(5).unwrap(); assert_eq!(*value.as_inner(), 5); assert_eq!(value.into_inner(), 5);
}
";

const CONSUMER: &str = r#"
#[derive(Debug, PartialEq, Eq)]
pub enum Admission { Empty }

pub fn admit_identifier(value: u64) -> Result<u64, Admission> {
    if value == 0 { Err(Admission::Empty) } else { Ok(value) }
}

pub fn admit_label(value: String) -> Result<String, Admission> {
    let admitted = value.trim().to_owned();
    if admitted.is_empty() { Err(Admission::Empty) } else { Ok(admitted) }
}

pub fn admit_batch<T: core::fmt::Display, const N: usize>(value: &[T; N]) -> Result<&[T; N], Admission> {
    if N == 0 { Err(Admission::Empty) } else { Ok(value) }
}

impl Identifier {
    pub fn little_endian(&self) -> [u8; 8] { self.as_inner().to_le_bytes() }
    pub fn big_endian(&self) -> [u8; 8] { self.as_inner().to_be_bytes() }
}

struct Payload<'a> { text: &'a str, custody: std::rc::Rc<()> }

impl core::fmt::Display for Payload<'_> {
    fn fmt(&self, out: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        out.write_str(self.text)
    }
}

fn main() {
    assert_eq!(Identifier::try_new(0).err(), Some(Admission::Empty));
    let identifier = Identifier::try_new(258).expect("nonzero value");
    assert_eq!(identifier.little_endian(), [2, 1, 0, 0, 0, 0, 0, 0]);
    assert_eq!(identifier.big_endian(), [0, 0, 0, 0, 0, 0, 1, 2]);
    assert_eq!(identifier.into_inner(), 258);
    assert_eq!(Label::try_new("   ".to_owned()).err(), Some(Admission::Empty));
    let label = Label::try_new("  cargo  ".to_owned()).expect("caller normalizes text");
    assert_eq!(label.as_inner().as_bytes(), b"cargo");
    assert_eq!(label.into_inner(), "cargo");
    let text = String::from("porcelain");
    let custody = std::rc::Rc::new(());
    let values = [Payload { text: &text, custody: std::rc::Rc::clone(&custody) }];
    let batch = BorrowedBatch::<Payload<'_>, 1>::try_new(&values).expect("one borrowed item");
    assert_eq!(batch.as_inner()[0].to_string(), "porcelain");
    assert!(std::rc::Rc::ptr_eq(&batch.as_inner()[0].custody, &custody));
    assert!(core::ptr::eq(batch.into_inner(), &values));
    assert_eq!(BorrowedBatch::<Payload<'_>, 0>::try_new(&[]).err(), Some(Admission::Empty));
}
"#;
