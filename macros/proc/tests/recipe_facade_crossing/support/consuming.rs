//! A caller-owned lifecycle compiled independently of the repository feature graph.

use super::write_specimen;
use crate::scratch::{cargo_with_target, command_refusal, lock_from_repository, repository_root};
use std::path::Path;

const PRODUCER: &str = r"
pub use bakery as front;
pub struct Record { pub owner: u8, pub count: u8 }
pub struct Host { pub owner: u8, pub phase: workflow::Phase }
#[derive(Debug, PartialEq)]
pub enum Error { Owner, Phase, Effect }
pub fn inspect(host: &mut Host, record: &Record, phase: workflow::Phase) -> Result<(), Error> {
    if host.owner != record.owner { return Err(Error::Owner); }
    if host.phase != phase { return Err(Error::Phase); }
    Ok(())
}
front::recipe! {
    pub mod workflow {
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Phase { Draft, Checked, Sent }
        pub enum Event { Check, Send }
        bake! {
            vocabularies { Phase; Event; };
            transitions(Phase, Event) {
                (Draft, Check) => Checked with(target) {
                    host.phase = target;
                    record.count += 1;
                    Ok(())
                };
                (Checked, Send) => Sent with(target) {
                    host.phase = target;
                    record.count += amount;
                    Ok(())
                };
            };
            absence(refused);
            projections {
                typestate(Phase) {
                    wrapper(Ticket);
                    resource(record: crate::Record);
                    runtime(host: &mut crate::Host);
                    refusal(crate::Error);
                    validate(crate::inspect);
                    methods { Check => check(); Send => send(amount: u8); };
                };
            };
        }
    }
}
pub use workflow::baked::typestate::{Ticket, Draft, Checked, Sent};
pub fn initial() -> Result<(Host, Ticket<Draft>), Error> {
    let mut host = Host { owner: 7, phase: workflow::Phase::Draft };
    let record = Record { owner: 7, count: 0 };
    let ticket = Ticket::<Draft>::restore(&mut host, record).map_err(|(_, error)| error)?;
    Ok((host, ticket))
}
";

const CONSUMER: &str = r"
use renamed_recipe_adopter::{initial, Error};
#[test]
fn ordinary_consuming_journey() -> Result<(), Error> {
    let (mut host, draft) = initial()?;
    let checked = draft.check(&mut host).map_err(|(_, error)| error)?;
    let sent = checked.send(&mut host, 4).map_err(|(_, error)| error)?;
    assert_eq!(sent.into_resource().count, 5);
    Ok(())
}
";

pub(crate) fn observe_consuming(scratch: &Path) -> Result<(), String> {
    write_specimen(scratch, "", PRODUCER, CONSUMER)?;
    let locked = lock_from_repository(scratch)?;
    if !locked.status.success() {
        return Err(command_refusal("consuming lock", &locked));
    }
    let target = repository_root()?.join("target");
    let tested = cargo_with_target(
        scratch,
        &target,
        &["test", "--locked", "--offline", "--test", "recipe"],
    )?;
    if !tested.status.success() {
        return Err(command_refusal("ordinary consuming adopter", &tested));
    }
    observe_expression_and_path(scratch, &target)?;
    std::fs::write(scratch.join("src/lib.rs"), PRODUCER).map_err(|error| error.to_string())?;
    std::fs::write(scratch.join("tests/recipe.rs"), CONSUMER).map_err(|error| error.to_string())?;
    let wasm = cargo_with_target(
        scratch,
        &target,
        &[
            "check",
            "--lib",
            "--locked",
            "--offline",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    if !wasm.status.success() {
        return Err(command_refusal("consuming Wasm compile", &wasm));
    }
    observe_consumer_refusals(scratch, &target)?;
    observe_binding_refusals(scratch, &target)?;
    std::fs::write(scratch.join("src/lib.rs"), PRODUCER).map_err(|error| error.to_string())?;
    let restored = cargo_with_target(
        scratch,
        &target,
        &["test", "--locked", "--offline", "--test", "recipe"],
    )?;
    if !restored.status.success() {
        return Err(command_refusal("restored consuming lawful twin", &restored));
    }
    Ok(())
}

fn observe_expression_and_path(scratch: &Path, target: &Path) -> Result<(), String> {
    let expression = PRODUCER.replace(
        "validate(crate::inspect);",
        "validate({ if host.owner == 0 { return Err(crate::Error::Owner); } crate::inspect });",
    );
    let expression_consumer = format!(
        "{CONSUMER}\n#[test] fn expression_return_retains_resource() {{ let mut host = renamed_recipe_adopter::Host {{ owner: 0, phase: renamed_recipe_adopter::workflow::Phase::Draft }}; let record = renamed_recipe_adopter::Record {{ owner: 0, count: 9 }}; let result = renamed_recipe_adopter::Ticket::<renamed_recipe_adopter::Draft>::restore(&mut host, record); assert_eq!(result.err().map(|(record, error)| (record.count, error)), Some((9, Error::Owner))); }}"
    );
    let path = PRODUCER.replace("with(target) {\n                    host.phase = target;\n                    record.count += 1;\n                    Ok(())\n                }", "with(crate::refuse_check)");
    let path =
        format!("{path}\npub fn refuse_check() -> Result<(), Error> {{ Err(Error::Effect) }}");
    let path_consumer = r"
use renamed_recipe_adopter::{initial, Error};
#[test]
fn path_refusal_returns_the_owned_resource() -> Result<(), Error> {
    let (mut host, draft) = initial()?;
    let refused = draft.check(&mut host).err().map(|(record, error)| (record.count, error));
    assert_eq!(refused, Some((0, Error::Effect)));
    Ok(())
}
";
    for (source, consumer) in [
        (expression, expression_consumer.as_str()),
        (path, path_consumer),
    ] {
        std::fs::write(scratch.join("src/lib.rs"), source).map_err(|error| error.to_string())?;
        std::fs::write(scratch.join("tests/recipe.rs"), consumer)
            .map_err(|error| error.to_string())?;
        let tested = cargo_with_target(
            scratch,
            target,
            &["test", "--locked", "--offline", "--test", "recipe"],
        )?;
        if !tested.status.success() {
            return Err(command_refusal(
                "consuming expression/path custody",
                &tested,
            ));
        }
    }
    Ok(())
}

fn observe_consumer_refusals(scratch: &Path, target: &Path) -> Result<(), String> {
    for (body, expected) in [
        (
            "let (mut host, draft) = initial()?; let _result = draft.send(&mut host, 4);",
            "no method named `send`",
        ),
        (
            "let (mut host, draft) = initial()?; let _first = draft.check(&mut host); let _second = draft.check(&mut host);",
            "use of moved value: `draft`",
        ),
        (
            "let (_host, draft) = initial()?; let _copy = draft.clone();",
            "no method named `clone`",
        ),
        (
            "let _later = renamed_recipe_adopter::Ticket::<renamed_recipe_adopter::Sent>::default();",
            "no associated function or constant named `default`",
        ),
        (
            "let _later = renamed_recipe_adopter::Ticket::<renamed_recipe_adopter::Sent> { __macroonz_resource: renamed_recipe_adopter::Record { owner: 7, count: 0 }, __macroonz_phase: core::marker::PhantomData };",
            "are private",
        ),
        (
            "let (mut host, draft) = initial()?; let checked = draft.check(&mut host).map_err(|(_, error)| error)?; let _result = checked.send(&mut host, true);",
            "mismatched types",
        ),
        (
            "let (mut host, draft) = initial()?; let borrowed = draft.resource(); let _result = draft.check(&mut host); assert_eq!(borrowed.count, 0);",
            "cannot move out of `draft` because it is borrowed",
        ),
    ] {
        let body = body.replace("initial()", "renamed_recipe_adopter::initial()");
        let consumer = format!(
            "use renamed_recipe_adopter::Error; #[test] fn hostile() -> Result<(), Error> {{ {body} Ok(()) }}"
        );
        std::fs::write(scratch.join("tests/recipe.rs"), consumer)
            .map_err(|error| error.to_string())?;
        refused(scratch, target, expected)?;
    }
    Ok(())
}

fn observe_binding_refusals(scratch: &Path, target: &Path) -> Result<(), String> {
    std::fs::write(scratch.join("tests/recipe.rs"), CONSUMER).map_err(|error| error.to_string())?;
    for (source, expected) in [
        (
            PRODUCER
                .replace(
                    "phase: workflow::Phase) -> Result<(), Error>",
                    "phase: workflow::Phase) -> Result<bool, Error>",
                )
                .replace("    Ok(())\n}", "    Ok(true)\n}"),
            "mismatched types",
        ),
        (
            PRODUCER.replace(
                "record.count += amount;\n                    Ok(())",
                "record.count += amount;\n                    7u8",
            ),
            "mismatched types",
        ),
        (
            PRODUCER.replace("validate(crate::inspect);", "validate(crate::missing);"),
            "cannot find value `missing`",
        ),
        (
            PRODUCER.replace("wrapper(Ticket)", "wrapper(Draft)"),
            "distinct from every phase marker",
        ),
        (
            PRODUCER.replace(
                "methods { Check => check(); Send => send(amount: u8); };",
                "methods { Check => check(); };",
            ),
            "one declared consuming method",
        ),
    ] {
        std::fs::write(scratch.join("src/lib.rs"), source).map_err(|error| error.to_string())?;
        refused(scratch, target, expected)?;
    }
    Ok(())
}

fn refused(scratch: &Path, target: &Path, expected: &str) -> Result<(), String> {
    let checked = cargo_with_target(
        scratch,
        target,
        &["check", "--locked", "--offline", "--test", "recipe"],
    )?;
    if checked.status.success() || !String::from_utf8_lossy(&checked.stderr).contains(expected) {
        return Err(command_refusal(expected, &checked));
    }
    Ok(())
}
