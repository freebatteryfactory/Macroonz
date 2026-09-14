//! The existing direct and recipe catalogue controls with decoder-admitted specimens.

use crate::scratch::{cargo_with_target, command_refusal, repository_root};
use std::path::Path;

const SPECIMEN: &str = r#"
use crate::library::harness::input::{self, BoundInput, InputBinding, InputLimits, InputProfile, InputRefusal};
use crate::library::harness::identity::{ContentAddress, DomainTag, IdentityProfileVersion};
use crate::library::harness::descriptor::NamespacedName;

fn specimen() -> Result<BoundInput<u8>, InputRefusal> {
    let name = NamespacedName::named("arithmetic", "operand").map_err(|_| InputRefusal::ProfileMismatch)?;
    let schema = ContentAddress::derived(
        DomainTag::declared("arithmetic-input", IdentityProfileVersion::declared(1)),
        b"one unsigned operand byte",
    );
    let decoder = InputBinding::declared(
        InputProfile::declared(name, 1, schema),
        revision(b"read-one-u8-v1"),
        |source| source.arbitrary::<u8>(),
    );
    decoder.decode(input::pack(decoder.profile(), &[6], InputLimits::declared(4096, 64))?)
}
"#;

pub(super) fn observe(scratch: &Path, producer: &str, consumer: &str) -> Result<(), String> {
    let producer = producer.replace(
        "support = catalogue_support,",
        "support = catalogue_support, input = { u8 },",
    );
    let consumer = consumer
        .replace("_: &Invocation", "invocation: &Invocation<BoundInput<u8>>")
        .replace("successor(6)", "successor(u32::from(*invocation.input().value()))")
        .replace(
            "clock: crate::library::harness::clock::HarnessClock::unavailable(),",
            "clock: crate::library::harness::clock::HarnessClock::unavailable(), specimen: crate::specimen(),",
        );
    let consumer = format!("{consumer}\n{SPECIMEN}");
    super::observe_entrance(scratch, &producer, &consumer)?;
    observe_admission_refusals(scratch, &producer, &consumer)?;
    observe_runtime_refusals(scratch, &producer, &consumer)
}

fn observe_admission_refusals(
    scratch: &Path,
    producer: &str,
    consumer: &str,
) -> Result<(), String> {
    let target = repository_root()?.join("target");
    for (damaged_producer, damaged_consumer, expected) in [
        (
            producer.to_owned(),
            consumer.replace("specimen: crate::specimen(),", ""),
            "no rules expected",
        ),
        (
            producer.to_owned(),
            consumer.replace(
                "specimen: crate::specimen(),",
                "specimen: crate::specimen(), specimen: crate::specimen(),",
            ),
            "no rules expected",
        ),
        (
            producer.to_owned(),
            consumer.replace(
                "specimen: crate::specimen(),",
                "specimen: Ok::<(), crate::library::harness::input::InputRefusal>(()),",
            ),
            "`?` operator has incompatible types",
        ),
        (
            producer.replace("input = { u8 }", "input = { u16 }"),
            consumer.to_owned(),
            "mismatched types",
        ),
    ] {
        std::fs::write(scratch.join("src/lib.rs"), damaged_producer)
            .map_err(|error| error.to_string())?;
        std::fs::write(scratch.join("tests/recipe.rs"), damaged_consumer)
            .map_err(|error| error.to_string())?;
        let checked = cargo_with_target(
            scratch,
            &target,
            &["check", "--locked", "--offline", "--test", "recipe"],
        )?;
        if checked.status.success() || !String::from_utf8_lossy(&checked.stderr).contains(expected)
        {
            return Err(command_refusal("typed catalogue admission", &checked));
        }
    }
    Ok(())
}

fn observe_runtime_refusals(scratch: &Path, producer: &str, consumer: &str) -> Result<(), String> {
    let target = repository_root()?.join("target");
    std::fs::write(scratch.join("src/lib.rs"), producer).map_err(|error| error.to_string())?;
    for (damaged_consumer, expected) in [
        (
            consumer.replace("&[6], InputLimits", "&[6, 9], InputLimits"),
            "InputNotBound(TrailingInputBytes",
        ),
        (
            consumer.replace("CaseBudget::declared(1)", "CaseBudget::declared(0)"),
            "BudgetExhausted",
        ),
        (
            consumer.replace("ByteBudget::declared(64)", "ByteBudget::declared(0)"),
            "BudgetExhausted",
        ),
    ] {
        std::fs::write(scratch.join("tests/recipe.rs"), damaged_consumer)
            .map_err(|error| error.to_string())?;
        let tested = cargo_with_target(
            scratch,
            &target,
            &[
                "test",
                "--locked",
                "--offline",
                "--test",
                "recipe",
                "--",
                "--include-ignored",
            ],
        )?;
        let observed = format!(
            "{}\n{}",
            String::from_utf8_lossy(&tested.stdout),
            String::from_utf8_lossy(&tested.stderr)
        );
        if tested.status.success()
            || !observed.contains("catalogue::whole ... FAILED")
            || !observed.contains(expected)
        {
            return Err(command_refusal(
                "typed catalogue execution refusal",
                &tested,
            ));
        }
    }
    Ok(())
}
