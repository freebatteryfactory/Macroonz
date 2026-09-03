//! The harness package's named shape debt, held at its exact current denominator until each owner closes it.

use std::fs;
use std::path::{Path, PathBuf};

fn rust_sources(root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

fn occurrence_paths(root: &Path, needle: &str) -> Result<Vec<String>, std::io::Error> {
    let mut observed = Vec::new();
    for path in rust_sources(root)? {
        let source = fs::read_to_string(&path)?;
        let relative = path
            .strip_prefix(root)
            .map_err(|error| std::io::Error::other(error.to_string()))?
            .to_string_lossy()
            .replace('\\', "/");
        for _occurrence in source.match_indices(needle) {
            observed.push(relative.clone());
        }
    }
    observed.sort();
    Ok(observed)
}

fn paths_named(root: &Path, name: &str) -> Result<Vec<String>, std::io::Error> {
    let mut observed = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory)? {
            let path = entry?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.file_name().is_some_and(|candidate| candidate == name) {
                observed.push(
                    path.strip_prefix(root)
                        .map_err(|error| std::io::Error::other(error.to_string()))?
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    observed.sort();
    Ok(observed)
}

fn assert_occurrences(root: &Path, needle: &str, expected: &[&str]) -> Result<(), std::io::Error> {
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(occurrence_paths(root, needle)?, expected, "{needle}");
    Ok(())
}

fn assert_single_owner_stamps(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "u64::try_from(length).unwrap_or(u64::MAX)",
        &["identity/encode.rs"],
    )?;
    assert_occurrences(
        root,
        "map_err(|_| EncodeRefusal::LengthPastEncodingWidth)",
        &["descriptor/encode.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! namespaced_reference",
        &["descriptor/guard_name.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! artifact_mutation_bank",
        &["depot/artifact_mutation/bank.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! generated_support_field_banks",
        &["depot/producer_field/bank.rs"],
    )?;
    assert_occurrences(root, "macro_rules! declare_census", &["census/stamp.rs"])?;
    assert_occurrences(root, "macro_rules! implement_census", &["census/stamp.rs"])?;
    assert_occurrences(
        root,
        "macro_rules! with_generation_dispositions",
        &["generate/generation/types.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! with_shrink_verdicts",
        &["generate/reduction/types.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! with_mutation_verdicts",
        &["muterprater/verdict/types.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! with_network_census_seats",
        &["network/simulation/types.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! declare_change_pair",
        &["report/stamp.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! implement_copy_change_pair",
        &["report/stamp.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! implement_borrowed_change_pair",
        &["report/stamp.rs"],
    )?;
    assert_occurrences(root, "impl RowRevisionChange", &[])?;
    assert_occurrences(root, "impl ExecutionRevisionChange", &[])?;
    assert_occurrences(root, "impl InvocationProfileChange", &[])?;
    assert_occurrences(root, "impl TargetBindingChange", &[])?;
    assert_occurrences(root, "impl ConclusionFlip", &[])?;
    Ok(())
}

fn assert_closed_shape_debt(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(root, "let mut killed: u32", &[])?;
    assert_occurrences(root, "self.census.sends =", &[])?;
    assert_occurrences(root, "ArtifactMutation::OrderPermuted,", &[])?;
    assert_occurrences(
        root,
        "\"candidate_alternatives\"",
        &["depot/producer_field/bank.rs"],
    )?;
    assert_occurrences(
        root,
        "\"planted_worse_falsifier\"",
        &["depot/producer_field/bank.rs"],
    )?;
    assert_occurrences(root, "struct BodyReader<'body>", &[])?;
    assert_occurrences(
        root,
        "struct BodyReader<'body, Refusal>",
        &["identity/types.rs"],
    )?;
    assert_occurrences(
        root,
        "fn addressed_body<Address, Refusal>(",
        &["identity/read.rs"],
    )?;
    assert_occurrences(root, "struct AbsolutePath(PathBuf);", &["fuzz/types.rs"])?;
    assert_occurrences(
        root,
        "path.as_os_str().is_empty()",
        &["fuzz/guard_declaration.rs"],
    )?;
    Ok(())
}

fn assert_content_address_denominator(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(root, "(ContentAddress);", &[])?;
    assert_occurrences(
        root,
        "macro_rules! content_address_reference",
        &["identity/type_guard.rs"],
    )?;
    assert_occurrences(
        root,
        "content_address_reference!",
        &[
            "bench/declaration/type_guard.rs",
            "bench/declaration/types.rs",
            "corpus/type_guard.rs",
            "corpus/types.rs",
            "descriptor/guard_row.rs",
            "descriptor/guard_row.rs",
            "descriptor/guard_row.rs",
            "descriptor/guard_schema.rs",
            "descriptor/types.rs",
            "descriptor/types.rs",
            "descriptor/types.rs",
            "descriptor/types.rs",
            "generate/generation/type_guard.rs",
            "generate/generation/types.rs",
            "muterprater/backend/type_guard.rs",
            "muterprater/backend/type_guard.rs",
            "muterprater/backend/types.rs",
            "muterprater/backend/types.rs",
            "muterprater/discovery/guard_lowering.rs",
            "muterprater/discovery/guard_lowering.rs",
            "muterprater/discovery/guard_lowering.rs",
            "muterprater/discovery/guard_policy.rs",
            "muterprater/discovery/types.rs",
            "muterprater/discovery/types.rs",
            "muterprater/discovery/types.rs",
            "muterprater/discovery/types.rs",
            "muterprater/specimen/type_guard.rs",
            "muterprater/specimen/types.rs",
            "muterprater/verdict/type_guard.rs",
            "muterprater/verdict/types.rs",
            "network/transcript/type_guard.rs",
            "network/transcript/types.rs",
            "report/guard_identity.rs",
            "report/guard_identity.rs",
            "report/guard_identity.rs",
            "report/guard_identity.rs",
            "report/types.rs",
            "report/types.rs",
            "report/types.rs",
            "report/types.rs",
        ],
    )
}

fn assert_wrap_custody(root: &Path) -> Result<(), std::io::Error> {
    assert_eq!(
        paths_named(root, "wrap.rs")?,
        ["src/muterprater/backend/wrap.rs"]
    );
    let evidence = root.join("tests/trust_opening_evidence");
    for copied_source in ["wrap.rs", "compare.rs", "conclude.rs", "type_guard.rs"] {
        assert_eq!(
            paths_named(&evidence, copied_source)?,
            Vec::<String>::new(),
            "{copied_source}"
        );
    }
    let campaign_pin = ["fn campaign_", "source()"].concat();
    assert_occurrences(
        root,
        &campaign_pin,
        &["tests/trust_opening_evidence/support.rs"],
    )?;
    let receipt = root
        .parent()
        .map(|repository| repository.join(".durafx/package/release-0.2.0/receipt.md"));
    assert!(
        receipt.is_some_and(|receipt| receipt.is_file()),
        "the 0.2.0 release receipt records the campaign source custody"
    );
    Ok(())
}

fn assert_test_support(root: &Path) -> Result<(), std::io::Error> {
    let declaration = ["macro_rules! declare_exploration_", "lane_failure"].concat();
    assert_occurrences(
        root,
        &declaration,
        &["tests/support/exploration_lane_failure.rs"],
    )?;
    let invocation = ["test_support::declare_exploration_", "lane_failure!"].concat();
    assert_occurrences(
        root,
        &invocation,
        &[
            "tests/interleave_exploration/main.rs",
            "tests/network_discipline/support.rs",
        ],
    )?;
    let manual_debug = ["impl core::fmt::Debug for ", "LaneFailure"].concat();
    assert_occurrences(
        root,
        &manual_debug,
        &["tests/network_transcript/support.rs"],
    )?;
    let fixture_owner = ["struct Trial", "Fixture"].concat();
    assert_occurrences(root, &fixture_owner, &["tests/support/trial_fixture.rs"])?;
    let fixture_import = ["use trial_fixture::Trial", "Fixture;"].concat();
    assert_occurrences(
        root,
        &fixture_import,
        &[
            "tests/fuzz_compose/main.rs",
            "tests/reduction_instrument/main.rs",
        ],
    )?;
    for relative in [
        "tests/fuzz_compose/main.rs",
        "tests/reduction_instrument/main.rs",
    ] {
        let source = fs::read_to_string(root.join(relative))?;
        assert!(!source.contains("fn trial_binding()"), "{relative}");
        assert!(
            !source.contains("fn invocation() -> Invocation"),
            "{relative}"
        );
    }
    let work_family = ["trait Work", "Family"].concat();
    assert_occurrences(
        root,
        &work_family,
        &["tests/recipe_economics/bench_driver.rs"],
    )?;
    assert_occurrences(
        root,
        &["fn preflight_call<F: Work", "Family>"].concat(),
        &["tests/recipe_economics/bench_driver.rs"],
    )?;
    for relative in [
        "tests/recipe_economics/main.rs",
        "tests/recipe_economics/breadth_bench.rs",
    ] {
        let source = fs::read_to_string(root.join(relative))?;
        assert!(!source.contains("enum Control"), "{relative}");
        assert!(!source.contains("fn trial_binding"), "{relative}");
        assert!(!source.contains("fn observations()"), "{relative}");
    }
    assert!(
        !fs::read_to_string(root.join("tests/recipe_economics/breadth.rs"))?.contains("fn debug"),
        "tests/recipe_economics/breadth.rs"
    );
    let trust_support = ["tests/trust_opening_evidence/", "support.rs"].concat();
    for owner in [
        ["fn ", "witness() -> Result<MutationWitness"].concat(),
        ["fn qualified_", "no_mutation<"].concat(),
        ["fn qualification_", "of<"].concat(),
        ["fn standard_", "projection<"].concat(),
        ["fn opened_", "trust<"].concat(),
        ["InterpreterAvailability::NoConformingSurface", " => Err("].concat(),
    ] {
        assert_occurrences(root, &owner, &[trust_support.as_str()])?;
    }
    for former_copy in [
        ["InterpreterAvailability::Available(trust)", " => trust,"].concat(),
        [
            ".qualification()\n            .ok_or(MutationRoadFailure::",
            "MissingQualification(",
        ]
        .concat(),
    ] {
        assert_occurrences(root, &former_copy, &[])?;
    }
    Ok(())
}

#[test]
fn named_harness_shape_debt_does_not_expand() -> Result<(), std::io::Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let sources = &root.join("src");
    assert_single_owner_stamps(sources)?;
    assert_closed_shape_debt(sources)?;
    assert_content_address_denominator(sources)?;
    assert_wrap_custody(root)?;
    assert_test_support(root)
}
