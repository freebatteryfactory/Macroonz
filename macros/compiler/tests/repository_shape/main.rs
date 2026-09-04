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
    assert_occurrences(root, "fn harness_path(", &["recipe/render/evidence.rs"])?;
    Ok(())
}

fn assert_declared_name_grammar_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "pub(crate) const fn name_is_grammatical",
        &["identity/grammar.rs"],
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

fn assert_owned_vector_spelling_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "pub(crate) fn vector(",
        &["token/generation/compose.rs"],
    )?;
    assert_occurrences(root, "fn vec_expr(", &[])?;
    let network = fs::read_to_string(root.join("descriptor/network/render.rs"))?;
    let codec = fs::read_to_string(root.join("codec/spell.rs"))?;
    assert!(!network.contains("Vec\", \"new"));
    assert!(!network.contains("Vec\", \"from"));
    assert!(!codec.contains("tokens.extend(associated(\"new\"))"));
    Ok(())
}

fn assert_token_generation_spelling_seats_are_distinct(root: &Path) -> Result<(), std::io::Error> {
    assert_occurrences(
        root,
        "pub fn rendered_identifier(spelling: &str) -> bool",
        &["token/generation/spelling.rs"],
    )?;
    assert_occurrences(
        root,
        "pub fn rendered_name(spelling: &str) -> bool",
        &["token/generation/spelling.rs"],
    )?;
    assert_occurrences(root, "fn absolute_path(", &["token/generation/compose.rs"])?;
    assert_occurrences(
        root,
        "pub fn documentation(sentence: &str)",
        &["token/generation/compose.rs"],
    )?;
    Ok(())
}

fn assert_direct_clause_mechanics_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    for operation in [
        "pub(crate) fn comma_groups",
        "pub(crate) fn opening(",
        "pub(crate) fn value_of",
        "pub(crate) fn fill_once",
        "pub(crate) fn binding_once",
        "pub(crate) fn assigned_identifier",
        "pub(crate) fn assigned_text",
        "pub(crate) fn assigned_number",
    ] {
        assert_occurrences(root, operation, &["descriptor/clause/direct.rs"])?;
    }
    for obsolete in [
        "fn assigned_once(",
        "fn assigned_ident(",
        "fn read_binding(",
        "fn number_once",
    ] {
        assert_occurrences(root, obsolete, &[])?;
    }
    Ok(())
}

fn assert_attribute_clause_mechanics_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    for operation in [
        "pub(crate) fn declaration_clauses",
        "pub(crate) fn assignment_clauses",
        "pub(crate) fn assigned<",
        "pub(crate) fn identifier<",
        "pub(crate) fn named_reference<",
        "pub(crate) fn named_value<",
        "pub(crate) fn number<",
    ] {
        assert_occurrences(root, operation, &["descriptor/clause/capture.rs"])?;
    }
    for obsolete in [
        "enum Clause<'trees>",
        "fn declaration_clauses<'trees>",
        "fn row_clauses<'trees>",
        "fn distinct(grammar: Grammar, clauses:",
    ] {
        assert_occurrences(root, obsolete, &[])?;
    }
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
    assert_occurrences(
        root,
        "pub(super) fn slot_in<T: Copy + Eq>",
        &["kind/type_contract.rs"],
    )?;
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

fn assert_test_framing_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    let framed = ["fn frame", "d(material: &[u8], into: &mut Vec<u8>)"].concat();
    assert_occurrences(
        root,
        &framed,
        &[
            "diagnostic_related_sets/main.rs",
            "independent_identity_transcripts/main.rs",
            "recorded_origins/main.rs",
        ],
    )?;
    let frame = ["fn fra", "me(material: &[u8]"].concat();
    assert_occurrences(root, &frame, &[])?;
    let slotted = ["fn frame", "d(slot: u8"].concat();
    assert_occurrences(root, &slotted, &[])
}

fn assert_recipe_observation_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    let walker = ["fn ", "flattened(trees: &[CapturedTokenTree])"].concat();
    assert_occurrences(root, &walker, &["support/captured_tokens.rs"])?;
    for former_walk in [
        ["fn collect_", "routes("].concat(),
        ["fn collect_", "tokens("].concat(),
        ["fn collect(trees: &[", "CapturedTokenTree]"].concat(),
    ] {
        assert_occurrences(root, &former_walk, &[])?;
    }
    let finders = "recipe_vertical_slice/support/tokens.rs";
    for finder in [
        ["enum ", "Occurrence"].concat(),
        ["fn word_", "handle("].concat(),
        ["fn group_after_", "word("].concat(),
        ["fn narrow_group_", "containing("].concat(),
        ["fn last_group_directly_", "containing("].concat(),
    ] {
        assert_occurrences(root, &finder, &[finders])?;
    }
    let driver = "recipe_vertical_slice/support/observe.rs";
    for road in [
        ["pub(crate) fn ", "refusal(source: &str)"].concat(),
        ["fn refusal_", "under("].concat(),
        ["fn bake_", "under("].concat(),
        ["fn bake_", "with("].concat(),
        ["fn bake_with_", "refusal("].concat(),
    ] {
        assert_occurrences(root, &road, &[driver])?;
    }
    assert_occurrences(
        root,
        &["macroonz_compiler::recipe::", "bake_with("].concat(),
        &[driver, driver],
    )?;
    assert_occurrences(
        root,
        &["macroonz_compiler::recipe::", "bake(read"].concat(),
        &[driver, driver],
    )?;
    assert_occurrences(
        root,
        &["macroonz_compiler::recipe::", "bake("].concat(),
        &[
            "recipe_vertical_slice/host_parity.rs",
            "recipe_vertical_slice/maximal_recipe.rs",
            driver,
            driver,
        ],
    )?;
    let specimens = "support/attribute_specimens.rs";
    for body in [
        ["const TRIAL_", "BODY"].concat(),
        ["const MUTATION_", "BODY"].concat(),
        ["const MUTATION_", "ITEM"].concat(),
        ["const BENCH_", "BODY"].concat(),
    ] {
        assert_occurrences(root, &body, &[specimens])?;
    }
    Ok(())
}

fn assert_compiler_test_support_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    let specimen_path = ["fn specimen_", "path("].concat();
    let rustup = ["Command::new(\"", "rustup\")"].concat();
    let ambient_temp = ["std::env::temp_", "dir()"].concat();
    let support_module = ["#[path = \"../support/", "mod.rs\"]"].concat();
    let observed = ["use crate::support::observe_", "rustc;"].concat();
    let discarded_cleanup = ["drop(std::fs::remove_", "file"].concat();
    assert_occurrences(root, &specimen_path, &["support/rustc_specimen.rs"])?;
    assert_occurrences(
        root,
        &rustup,
        &["bounded_collections/main.rs", "support/rustc_specimen.rs"],
    )?;
    assert_occurrences(root, &ambient_temp, &[])?;
    assert_occurrences(
        root,
        &support_module,
        &[
            "declared_modules/main.rs",
            "published_stamps/main.rs",
            "structural_token_projection/main.rs",
        ],
    )?;
    assert_occurrences(
        root,
        &observed,
        &[
            "declared_modules/codec_generated_behavior.rs",
            "published_stamps/main.rs",
            "structural_token_projection/behavior.rs",
            "structural_token_projection/items.rs",
            "structural_token_projection/traits.rs",
        ],
    )?;
    assert_occurrences(root, &discarded_cleanup, &[])
}

fn assert_structural_projection_debt_is_closed(root: &Path) -> Result<(), std::io::Error> {
    let member_limit = ["const MEMBER_", "LIMIT: usize = 4;"].concat();
    assert_occurrences(
        root,
        &member_limit,
        &["structural_token_projection/main.rs"],
    )
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
    assert_harness_path_debt_is_closed(&root)?;
    assert_declared_name_grammar_debt_is_closed(&root)?;
    assert_role_join_debt_is_closed(&root)?;
    assert_diagnostic_projection_debt_is_closed(&root)?;
    assert_owned_vector_spelling_debt_is_closed(&root)?;
    assert_token_generation_spelling_seats_are_distinct(&root)?;
    assert_direct_clause_mechanics_debt_is_closed(&root)?;
    assert_attribute_clause_mechanics_debt_is_closed(&root)?;
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
    )?;
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    assert_test_framing_debt_is_closed(&tests)?;
    assert_compiler_test_support_debt_is_closed(&tests)?;
    assert_structural_projection_debt_is_closed(&tests)?;
    assert_recipe_observation_debt_is_closed(&tests)
}
