//! Archive delivery is distinct from the checkout-path journeys: Cargo assembles the four packages, and only extracted source supplies the compiler, proc carrier, harness, examples, and independent facade controls.
//!
//! The aggregate root reconnects the normalized manifests with workspace membership and extracted-package patches, retaining the facade profiles and committed dependency lock.
//! One standalone scratch consumer compiles the archived skill against the delivered facade; no library source is changed.
//! This is first-publication package composition, not a claim that unpublished siblings resolve through crates.io.
//! Archive assembly permits dirty development trees and reports Cargo's source records; release qualification separately requires a clean committed source and candidate hashes.

#[path = "support/archive_delivery.rs"]
mod delivery;
#[path = "support/archive_skill.rs"]
mod skill;

use crate::scratch::{cargo_command, cargo_with_target, observed_in_scratch_for};
use std::ffi::OsStr;
use std::path::Path;

#[test]
#[ignore = "long extracted-archive delivery campaign; run explicitly"]
fn the_four_archives_resolve_and_execute_without_checkout_fallback() -> Result<(), String> {
    observed_in_scratch_for("archive_delivery", delivery::observe)
}

#[test]
fn incomplete_cargo_requests_refuse_before_launching_a_process() {
    let absent = Path::new("unavailable-archive-manifest");
    for (arguments, refusal) in [
        (&[][..], "a Cargo observation requires one subcommand"),
        (
            &["nextest"][..],
            "a Nextest observation requires its subcommand",
        ),
    ] {
        assert_eq!(
            cargo_with_target(absent, absent, arguments).map(|_| ()),
            Err(refusal.to_owned())
        );
    }
}

#[test]
fn scratch_commands_do_not_inherit_the_parent_workspace_nextest_profile() -> Result<(), String> {
    let root = Path::new("delivered");
    let target = root.join("build");
    for arguments in [&["nextest", "run"][..], &["test"][..]] {
        let command = cargo_command(root, &target, arguments)?;
        assert_eq!(
            command
                .get_envs()
                .find(|(name, _value)| *name == OsStr::new("NEXTEST_PROFILE")),
            Some((OsStr::new("NEXTEST_PROFILE"), None)),
        );
        assert_eq!(
            command
                .get_envs()
                .find(|(name, _value)| *name == OsStr::new("CARGO_TARGET_DIR")),
            Some((OsStr::new("CARGO_TARGET_DIR"), Some(target.as_os_str()))),
        );
        assert!(command.get_envs().all(|(name, _value)| {
            name != OsStr::new("CARGO_BUILD_JOBS") && name != OsStr::new("CARGO_INCREMENTAL")
        }));
    }
    Ok(())
}

#[test]
fn the_skill_consumer_requires_the_complete_named_recipe_block() -> Result<(), String> {
    let recipe = "macroonz::recipe! { mod sample {} }";
    let valid = format!("## Write one generic recipe\n\n```rust\n{recipe}\n```\n");
    assert_eq!(skill::first_recipe(&valid)?, recipe);
    for missing in [
        valid.replace("## Write one generic recipe", "## Another section"),
        valid.replace("```rust", "```text"),
        valid.replace("\n```\n", "\n"),
        valid.replace("macroonz::recipe!", "unrelated!"),
        format!("## Write one generic recipe\n## Another section\n```rust\n{recipe}\n```"),
    ] {
        assert!(skill::first_recipe(&missing).is_err(), "{missing}");
    }
    Ok(())
}

#[test]
fn archive_resolution_refuses_missing_foreign_and_checkout_packages() -> Result<(), String> {
    let root = Path::new("delivered");
    let valid = delivery::expected_graph(root).join("\n");
    delivery::check_graph(root, &valid)?;
    let native = valid.replace('/', std::path::MAIN_SEPARATOR_STR);
    delivery::check_graph(root, &native)?;
    assert!(delivery::check_graph(root, "").is_err());
    for expected in delivery::expected_graph(root) {
        assert!(delivery::check_graph(root, &valid.replace(&expected, "")).is_err());
        let checkout = expected.replace("delivered", "checkout");
        assert!(delivery::check_graph(root, &valid.replace(&expected, &checkout)).is_err());
    }
    let foreign = format!("{valid}\nmacroonz-foreign v0.2.0 (checkout)");
    assert!(delivery::check_graph(root, &foreign).is_err());
    Ok(())
}

#[test]
fn archive_rosters_refuse_escape_or_another_package() {
    assert!(delivery::check_roster("crate", "crate/Cargo.toml\ncrate/src/lib.rs\n").is_ok());
    for roster in [
        "",
        "crate/../outside",
        "crate//../outside",
        "crate/..\\outside",
        "crate/Cargo.toml\ncrate/Cargo.toml",
        "/crate/Cargo.toml",
        "other/Cargo.toml",
        "crate/src/lib.rs",
    ] {
        assert!(delivery::check_roster("crate", roster).is_err(), "{roster}");
    }
}
