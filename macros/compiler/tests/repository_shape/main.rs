//! The compiler package's named shape debt, held at its exact current denominator until each owner closes it.

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

fn assert_occurrences(root: &Path, needle: &str, expected: &[&str]) -> Result<(), std::io::Error> {
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    assert_eq!(occurrence_paths(root, needle)?, expected, "{needle}");
    Ok(())
}

fn assert_visibility_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "pub(crate) fn declared_reach_tokens",
        &["stamp/render.rs"],
    )?;
    assert_occurrences(root, "fn crate_visibility(", &[])?;
    assert_occurrences(root, "fn visibility()", &[])?;
    Ok(())
}

fn assert_harness_path_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "pub(crate) fn owned_direct_path",
        &["descriptor/emitting.rs"],
    )?;
    assert_occurrences(root, "fn harness_path(", &["recipe/render_evidence.rs"])?;
    Ok(())
}

fn assert_declared_name_grammar_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "pub(crate) const fn name_is_grammatical",
        &["identity/type_guard.rs"],
    )?;
    assert_occurrences(root, "fn diagnostic_name_is_kebab_case", &[])?;
    Ok(())
}

fn assert_role_join_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(root, "pub(crate) fn rows_under", &["kind/join.rs"])?;
    assert_occurrences(root, "pub(crate) fn rows_to", &["kind/join.rs"])?;
    assert_occurrences(root, "JoinOrder::Offering,", &["plan/type_guard.rs"])?;
    assert_occurrences(
        root,
        "JoinOrder::Roster(R::ALL),",
        &["render/type_guard.rs"],
    )?;
    Ok(())
}

fn assert_diagnostic_projection_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "pub(super) fn diagnostic<E: Refused>",
        &["diagnostic/project.rs"],
    )?;
    assert_occurrences(
        root,
        "pub(super) fn placement_site",
        &["diagnostic/project.rs"],
    )?;
    assert_occurrences(
        root,
        "fn assemble_diagnostic",
        &["diagnostic/type_guard.rs"],
    )?;
    assert_occurrences(root, "fn compose_diagnostic", &[])?;
    assert_occurrences(root, "fn placement_line_site", &[])?;
    Ok(())
}

fn assert_duplicate_group_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(root, "fn admit_keys<", &["bounded/type_guard.rs"])?;
    assert_occurrences(
        root,
        "pub(crate) fn duplicate_keys",
        &["bounded/type_guard.rs"],
    )?;
    assert_occurrences(
        root,
        "duplicate: DuplicateKey<RelationPair, N>",
        &["relation/types.rs"],
    )?;
    Ok(())
}

fn assert_doubled_set_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "pub(crate) fn first_duplicate_position",
        &["bounded/type_guard.rs"],
    )?;
    for relative in [
        "descriptor/bench/type_guard.rs",
        "descriptor/mutation/type_guard.rs",
        "descriptor/trial/type_guard.rs",
        "stamp/type_guard.rs",
    ] {
        let source = fs::read_to_string(root.join(relative))?;
        assert!(!source.contains("BTreeSet"), "{relative}");
    }
    Ok(())
}

fn assert_helper_refusal_projection_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "macro_rules! impl_helper_capture_contract",
        &["descriptor/type_contract.rs"],
    )?;
    assert_occurrences(
        root,
        "impl_helper_capture_contract!(",
        &[
            "descriptor/bench/type_contract.rs",
            "descriptor/concurrency/type_contract.rs",
            "descriptor/mutation/type_contract.rs",
            "descriptor/network/type_contract.rs",
            "descriptor/shadow/type_contract.rs",
            "descriptor/trial/type_contract.rs",
            "descriptor/type_contract.rs",
        ],
    )?;
    for error in [
        "BenchCaptureError",
        "ConcurrencyCaptureError",
        "MutationCaptureError",
        "NetworkCaptureError",
        "ShadowCaptureError",
        "TrialCaptureError",
    ] {
        assert_occurrences(root, &format!("impl Refused for {error}"), &[])?;
    }
    Ok(())
}

fn assert_roster_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(root, "macro_rules! subjects", &["identity/stamp.rs"])?;
    assert_occurrences(root, "const RUST_KEYWORDS: &[&str]", &["token/bank.rs"])?;
    assert_occurrences(
        root,
        "const RAW_IDENTIFIER_EXCLUSIONS: &[&str]",
        &["token/bank.rs"],
    )?;
    assert_occurrences(
        root,
        "pub fn rust_keyword(spelling: &str) -> bool",
        &["token/bank.rs"],
    )?;
    assert_occurrences(
        root,
        "pub(crate) enum RelationQuestion",
        &["relation/types.rs"],
    )?;
    assert_occurrences(root, "RELATION_QUESTION_NAMES", &[])?;
    assert_occurrences(
        root,
        "pub(crate) fn roster_row<Row: Copy>",
        &["kind/type_contract.rs"],
    )?;
    assert_occurrences(
        root,
        "macro_rules! vocabulary",
        &["descriptor/vocabulary/stamp.rs"],
    )?;
    assert_occurrences(root, "impl HarnessName", &[])?;
    assert_occurrences(root, "impl HarnessWord", &[])?;
    assert_occurrences(root, "impl TextLexicalCause", &[])?;
    assert_occurrences(root, "impl CaptureBound", &[])?;
    assert_occurrences(root, "impl LiteralReadCause", &[])?;
    assert_occurrences(root, "macro_rules! named_vocabulary", &["recipe/stamp.rs"])?;
    assert_occurrences(root, "impl HarnessPosture", &[])?;
    assert_occurrences(root, "impl LoweringSource", &[])?;
    assert_occurrences(root, "impl ProjectionDisposition", &[])?;
    assert_occurrences(root, "impl RecipeRelationPayloadKind", &[])?;
    Ok(())
}

#[test]
fn named_compiler_shape_debt_does_not_expand() -> Result<(), std::io::Error> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    assert_occurrences(
        &root,
        "RENDERED_PATH_SEGMENT_LIMIT: usize = 8;",
        &["token/types.rs"],
    )?;
    assert_occurrences(
        &root,
        "pub const PATH_SEGMENT_LIMIT: usize = crate::token::RENDERED_PATH_SEGMENT_LIMIT;",
        &["descriptor/types.rs", "stamp/types.rs", "support/types.rs"],
    )?;
    assert_occurrences(
        &root,
        "CODEC_PATH_SEGMENT_LIMIT: usize = crate::token::RENDERED_PATH_SEGMENT_LIMIT;",
        &["codec/types.rs"],
    )?;
    assert_occurrences(&root, "fn absolute_path(", &["token/generation/compose.rs"])?;
    assert_harness_path_debt_is_closed(&root)?;
    assert_declared_name_grammar_debt_is_closed(&root)?;
    assert_role_join_debt_is_closed(&root)?;
    assert_diagnostic_projection_debt_is_closed(&root)?;
    assert_duplicate_group_debt_is_closed(&root)?;
    assert_doubled_set_debt_is_closed(&root)?;
    assert_helper_refusal_projection_debt_is_closed(&root)?;
    assert_occurrences(&root, "const FAULT_ARMS:", &[])?;
    assert_occurrences(
        &root,
        "const NAME_REFUSAL: (&str, &[&str], &str)",
        &["descriptor/fault.rs"],
    )?;
    assert_roster_debt_is_closed(&root)?;
    assert_occurrences(
        &root,
        "format!(\"{lens}_{seat}\")",
        &["descriptor/emitting.rs"],
    )?;
    assert_visibility_debt_is_closed(&root)?;
    assert_occurrences(
        &root,
        "the complete relation-question roster guards this match",
        &[],
    )?;
    assert_occurrences(
        &root,
        "pub const CAPTURED_DECLARATION_PROFILE: Profile",
        &["identity/bank.rs"],
    )?;
    assert_occurrences(
        &root,
        "pub const MEMBER_CONTRACT: [MemberContract; 5]",
        &["codec/bank.rs"],
    )?;
    assert_occurrences(
        &root,
        "pub const RESERVED_BINDINGS: [&str; 12]",
        &["codec/bank.rs"],
    )?;
    assert_occurrences(
        &root,
        "recipe issue category must be formatted exactly once",
        &[],
    )
}
