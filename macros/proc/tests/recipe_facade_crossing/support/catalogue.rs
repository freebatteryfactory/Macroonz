//! Co-located and invocation-supplied bindings crossing the same renamed facade.

#[path = "catalogue_input.rs"]
mod input;

use super::write_specimen;
use crate::scratch::{cargo_with_target, command_refusal, lock_from_repository, repository_root};
use std::path::Path;

const ROWS: &str = r#"
    support = catalogue_support,
    module = catalogue,
    table = named("arithmetic", "successor"),
    suite whole = named("arithmetic", "laws") {
        co_located {
            claim = named("arithmetic", "successor-law"),
            subject = named("arithmetic", "successor"),
            check = named("arithmetic", "seven"),
            population = named("arithmetic", "six"),
            binding = {
                subject_revision = { $consumer::revision(b"successor-v1") },
                check_revision = { $consumer::revision(b"six-becomes-seven") },
                call = { $consumer::independent_check },
            },
        },
        attached_at_invocation {
            claim = named("arithmetic", "successor-law"),
            subject = named("arithmetic", "successor"),
            check = named("arithmetic", "seven-again"),
            population = named("arithmetic", "six"),
        },
    },
"#;

const SUBJECT: &str = r"
pub use bakery as front;
pub fn successor(value: u32) -> u32 { value.saturating_add(1) }
";

const CONSUMER: &str = r#"#![deny(warnings)]
use renamed_recipe_adopter::{catalogue_support, front as library};
use crate::library::harness::descriptor::{DerivedRevision, RevisionBinding};
use crate::library::harness::runner::Invocation;
use crate::library::harness::report::TrialConclusion;

fn revision(material: &[u8]) -> RevisionBinding {
    RevisionBinding::derived(DerivedRevision::from_material(material))
}

fn independent_check(_: &Invocation) -> TrialConclusion {
    assert_eq!(renamed_recipe_adopter::successor(6), 7);
    TrialConclusion::Passed
}

catalogue_support! {
    DECLARING_CLAUSE
    harness: crate::library::harness,
    consumer: crate,
    invocation: crate::library::harness::report::InvocationProfile::declared(
        crate::library::harness::report::CaseBudget::declared(1),
        crate::library::harness::report::ByteBudget::declared(64),
        crate::library::harness::report::TimeBudget::declared(1),
    ),
    target: crate::library::harness::report::TargetBinding::bound(
        crate::library::harness::report::TargetTriple::declared("declared-consumer-target"),
        crate::library::harness::report::ToolchainIdentity::declared("declared-consumer-toolchain"),
    ),
    clock: crate::library::harness::clock::HarnessClock::unavailable(),
    attached_at_invocation_subject_revision: crate::revision(b"successor-v1"),
    attached_at_invocation_check_revision: crate::revision(b"six-becomes-seven-again"),
    attached_at_invocation_call: crate::independent_check,
}

#[test]
fn inventory_has_both_bindings() {
    let table = catalogue::table().expect("both explicit binding roads are lawful");
    let checks = table.bindings().iter()
        .map(|binding| binding.row().check().name().stem().written()).collect::<Vec<_>>();
    assert_eq!(checks, ["seven", "seven-again"]);
}
"#;

pub(crate) fn observe_catalogue(scratch: &Path) -> Result<(), String> {
    let direct = format!("{SUBJECT}\n#[bakery::macros::trials({ROWS})]\npub struct Declared;");
    let recipe = format!(
        "{SUBJECT}\nbakery::recipe! {{ pub mod declared {{ bake! {{ evidence {{ trials {{ {ROWS} }}; }}; }} }} }}"
    );
    let direct_consumer = CONSUMER.replace("DECLARING_CLAUSE", "");
    write_specimen(
        scratch,
        ", features = [\"harness\"]",
        &direct,
        &direct_consumer,
    )?;
    let locked = lock_from_repository(scratch)?;
    if !locked.status.success() {
        return Err(command_refusal("catalogue lock", &locked));
    }
    for (producer, declaring) in [(direct, ""), (recipe, "declaring: renamed_recipe_adopter,")] {
        let consumer = CONSUMER.replace("DECLARING_CLAUSE", declaring);
        observe_entrance(scratch, &producer, &consumer)?;
        input::observe(scratch, &producer, &consumer)?;
    }
    Ok(())
}

fn observe_entrance(scratch: &Path, producer: &str, consumer: &str) -> Result<(), String> {
    std::fs::write(scratch.join("src/lib.rs"), producer).map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("tests/recipe.rs"), consumer).map_err(|error| error.to_string())?;
    let target = repository_root()?.join("target");
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
    if !tested.status.success() {
        return Err(command_refusal("co-located consumer", &tested));
    }
    observe_gate_bindings(scratch, consumer, &target)?;
    observe_path_refusals(scratch, consumer, &target)?;
    observe_refusals(scratch, producer, consumer, &target)?;
    std::fs::write(
        scratch.join("src/lib.rs"),
        producer.replace("saturating_add(1)", "saturating_add(2)"),
    )
    .map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("tests/recipe.rs"), consumer).map_err(|error| error.to_string())?;
    let challenged = cargo_with_target(
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
    let stdout = String::from_utf8_lossy(&challenged.stdout);
    if challenged.status.success() || !stdout.contains("catalogue::whole ... FAILED") {
        return Err(command_refusal(
            "independent successor disagreement",
            &challenged,
        ));
    }
    Ok(())
}

fn observe_gate_bindings(scratch: &Path, consumer: &str, target: &Path) -> Result<(), String> {
    let facade = consumer
        .replace(
            "catalogue_support! {",
            "crate::library::support! { catalogue_support {",
        )
        .replace("harness: crate::library::harness,", "")
        .replace("\n}\n\n#[test]", "\n} }\n\n#[test]");
    let reexport = format!(
        "{}\nmod gate_reexport {{
            pub use crate::library::harness::generated_support;
            pub mod descriptor {{ pub struct GeneratedSupportSchemaId; }}
        }}
        const _: Option<gate_reexport::descriptor::GeneratedSupportSchemaId> = None;",
        consumer.replace(
            "harness: crate::library::harness,",
            "harness: crate::gate_reexport,"
        ),
    );
    for (source, question) in [
        (&facade, "facade support"),
        (&reexport, "gate-owned binding"),
    ] {
        std::fs::write(scratch.join("tests/recipe.rs"), source)
            .map_err(|error| error.to_string())?;
        let tested = cargo_with_target(
            scratch,
            target,
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
        if !tested.status.success() {
            return Err(command_refusal(question, &tested));
        }
    }
    let conflicting = facade.replace(
        "consumer: crate,",
        "harness: crate::library::compiler, consumer: crate,",
    );
    std::fs::write(scratch.join("tests/recipe.rs"), conflicting)
        .map_err(|error| error.to_string())?;
    let checked = cargo_with_target(
        scratch,
        target,
        &["check", "--locked", "--offline", "--test", "recipe"],
    )?;
    if checked.status.success()
        || !String::from_utf8_lossy(&checked.stderr).contains("no rules expected")
    {
        return Err(command_refusal(
            "facade conflicting harness refusal",
            &checked,
        ));
    }
    Ok(())
}

fn observe_path_refusals(scratch: &Path, consumer: &str, target: &Path) -> Result<(), String> {
    let mut attempts = vec![
        (
            consumer.replace("harness: crate::library::harness,", ""),
            "no rules expected",
        ),
        (
            consumer.replace(
                "harness: crate::library::harness,",
                "harness: crate::library::harness, harness: crate::library::harness,",
            ),
            "no rules expected",
        ),
        (
            consumer.replace(
                "harness: crate::library::harness,",
                "harness: crate::library::compiler,",
            ),
            "generated_support",
        ),
    ];
    if consumer.contains("declaring: renamed_recipe_adopter,") {
        attempts.extend([
            (
                consumer.replace("declaring: renamed_recipe_adopter,", ""),
                "declaring",
            ),
            (
                consumer.replace(
                    "declaring: renamed_recipe_adopter,",
                    "declaring: crate::library,",
                ),
                "could not find",
            ),
        ]);
    }
    for (damaged, expected) in attempts {
        std::fs::write(scratch.join("tests/recipe.rs"), damaged)
            .map_err(|error| error.to_string())?;
        let checked = cargo_with_target(
            scratch,
            target,
            &["check", "--locked", "--offline", "--test", "recipe"],
        )?;
        if checked.status.success() || !String::from_utf8_lossy(&checked.stderr).contains(expected)
        {
            return Err(command_refusal("carrier path refusal", &checked));
        }
    }
    Ok(())
}

fn observe_refusals(
    scratch: &Path,
    producer: &str,
    consumer: &str,
    target: &Path,
) -> Result<(), String> {
    for (damaged_producer, damaged_consumer, expected) in [
        (
            producer.to_owned(),
            consumer.replace("consumer: crate,", ""),
            "no rules expected",
        ),
        (
            producer.to_owned(),
            consumer.replace("consumer: crate,", "consumer: crate, consumer: crate,"),
            "no rules expected",
        ),
        (
            producer.replace(
                "call = { $consumer::independent_check }",
                "call = { false }",
            ),
            consumer.to_owned(),
            "mismatched types",
        ),
        (
            producer.replace(
                "subject_revision = { $consumer::revision(b\"successor-v1\") }",
                "subject_revision = { false }",
            ),
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
            target,
            &["check", "--locked", "--offline", "--test", "recipe"],
        )?;
        if checked.status.success() || !String::from_utf8_lossy(&checked.stderr).contains(expected)
        {
            return Err(command_refusal("catalogue compile refusal", &checked));
        }
    }
    Ok(())
}
