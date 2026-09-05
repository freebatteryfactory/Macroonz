//! Generic recipe accounts observed independently of the transition lowering.

use super::support::{bake, refusal_summary};
use macroonz_compiler::CanonicalContent;
use macroonz_compiler::recipe::{RecipeRelationPayload, RecipeRelationPayloadKind};

const GENERIC_RECIPE: &str = r#"
pub mod protocol {
    pub enum Stage {
        Draft,
        Published,
    }

    pub enum Capability {
        Read,
        Write,
    }

    pub enum Marker {
        Stable,
    }

    bake! {
        vocabularies {
            Stage;
            Capability;
            Marker;
        };
        relations {
            evolution(Stage, Stage) {
                (Draft, Published);
            };
            policy(Stage, Capability) {
                (Draft, Read) with(crate::policy::allow);
            };
            labels(Stage, Capability) {
                (Published, Write) with { crate::Decision::Audit };
            };
            vacant(Stage, Capability) {
            };
        };
        postures {
            evolution {
                empty(refused);
                repetition(refused);
                membership(closed, closed);
                completeness(partial, partial);
                density(sparse);
                absence(allowed);
                self_relation(allowed);
                cycle(allowed);
            };
            policy {
                repetition(refused);
            };
            labels {
                repetition(refused);
            };
            vacant {
                empty(allowed);
            };
        };
        projections {
            companions;
            typestate(Stage);
            relation_tables {
                evolution;
                policy {
                    pub fn lookup_policy(
                        stage: &Stage,
                        capability: &Capability,
                    ) -> Option<crate::Decision>;
                };
                labels {
                    pub fn lookup_label(
                        stage: &Stage,
                        capability: &Capability,
                    ) -> Option<crate::Decision>;
                };
            };
        };
        evidence {
            mutation(Capability) {
                module = recipe_mutations,
                refusal = RecipeMutationRefusal,
                support = recipe_mutation_support,
                family = named("recipe", "refusals"),
                point = named("recipe", "capability-order"),
                fact = named("recipe", "capability-order"),
                map named("recipe", "capability-order") = named("recipe", "order-held"),
                permit named("recipe", "order-held") = ["declared-order-permutation"],
            };
        };
    }
}
"#;

const MIXED_TRANSITION_RECIPE: &str = r"
pub mod workflow {
    pub enum State { Draft, Published }
    pub enum Event { Publish }
    pub enum Capability { Read }

    bake! {
        vocabularies { State; Event; Capability; };
        transitions(State, Event) {
            (Draft, Publish) => Published with(crate::publish);
        };
        relations {
            policy(State, Capability) {
                (Draft, Read);
                (Published, Read);
            };
        };
        absence(refused);
        postures {
            policy { repetition(refused); };
        };
        projections {
            companions;
            dispatch(apply);
            typestate(State);
        };
    }
}
";

#[test]
fn generic_vocabularies_relations_payloads_and_postures_share_one_account() -> Result<(), String> {
    let baked = bake(GENERIC_RECIPE).map_err(|()| {
        refusal_summary(GENERIC_RECIPE)
            .unwrap_or_else(|()| "the generic recipe refused without a summary".to_owned())
    })?;
    let recipe = baked.projection().plan().content();
    assert_eq!(recipe.module_name(), "protocol");
    assert_eq!(recipe.vocabularies().count(), 3);
    assert_eq!(recipe.relations().count(), 4);
    assert!(recipe.transition_relation().is_none());
    let lowering = recipe
        .effective(macroonz_compiler::recipe::RecipeRole::RelationTables)
        .ok_or_else(|| "the relation-table lowering is absent".to_owned())?
        .relation_tables()
        .collect::<Vec<_>>();
    let [evolution_lowering, policy_lowering, labels_lowering] = lowering.as_slice() else {
        return Err("the relation-table lowering did not retain all three rows".to_owned());
    };
    assert_eq!(evolution_lowering.relation(), "evolution");
    assert_eq!(evolution_lowering.function(), "contains");
    assert_eq!(policy_lowering.function(), "lookup_policy");
    assert_eq!(labels_lowering.function(), "lookup_label");

    let policy = recipe
        .relation("policy")
        .ok_or_else(|| "the policy relation is absent".to_owned())?;
    assert_eq!(policy.left_vocabulary(), "Stage");
    assert_eq!(policy.right_vocabulary(), "Capability");
    assert_eq!(policy.payload_kind(), RecipeRelationPayloadKind::Path);
    assert_eq!(policy.row_count(), 1);
    let Some(row) = policy.rows().next() else {
        return Err("the policy row is absent".to_owned());
    };
    assert_eq!(row.left(), "Draft");
    assert_eq!(row.right(), "Read");
    let RecipeRelationPayload::Path(path) = row.payload() else {
        return Err("the policy row lost its path payload".to_owned());
    };
    assert_eq!(path.inspected().trim(), "crate :: policy :: allow");

    let labels = recipe
        .relation("labels")
        .ok_or_else(|| "the labels relation is absent".to_owned())?;
    assert_eq!(labels.payload_kind(), RecipeRelationPayloadKind::ExactRust);
    let vacant = recipe
        .relation("vacant")
        .ok_or_else(|| "the vacant relation is absent".to_owned())?;
    assert_eq!(vacant.row_count(), 0);
    assert_eq!(
        vacant.requirements().empty(),
        Some(macroonz_compiler::EmptyPosture::Allowed)
    );

    let emitted = baked
        .emit()
        .tokens()
        .ok_or_else(|| "the generic companions were not emitted".to_owned())?
        .inspected();
    for expected in [
        "STAGE_VARIANTS",
        "CAPABILITY_VARIANTS",
        "MARKER_VARIANTS",
        "EVOLUTION_ROWS",
        "POLICY_ROWS",
        "POLICY_PAYLOADS",
        "LABELS_ROWS",
        "LABELS_PAYLOADS",
        "VACANT_ROWS",
        "mod evolution",
        "contains",
        "mod policy",
        "lookup_policy",
        "mod labels",
        "lookup_label",
        "typestate",
        "recipe_mutations",
    ] {
        assert!(emitted.contains(expected), "missing `{expected}`");
    }
    assert!(emitted.contains("match ( stage , capability )"));
    assert!(emitted.contains("use super :: super :: Stage"));
    assert!(emitted.contains("use super :: super :: Capability"));
    Ok(())
}

#[test]
fn transition_sugar_and_generic_relations_share_one_grammar_and_account() -> Result<(), String> {
    let baked = bake(MIXED_TRANSITION_RECIPE).map_err(|()| {
        refusal_summary(MIXED_TRANSITION_RECIPE)
            .unwrap_or_else(|()| "the mixed recipe refused without a summary".to_owned())
    })?;
    let recipe = baked.projection().plan().content();
    let transition = recipe
        .transition_relation()
        .ok_or_else(|| "the explicit transition lowering is absent".to_owned())?;
    assert_eq!(transition.left_vocabulary(), "State");
    assert_eq!(transition.right_vocabulary(), "Event");
    assert!(recipe.relation("policy").is_some());

    let ambiguous = MIXED_TRANSITION_RECIPE.replace("typestate(State)", "typestate");
    let refusal = refusal_summary(ambiguous.as_str())
        .map_err(|()| "transition position supplied an implicit typestate subject".to_owned())?;
    assert!(refusal.contains("requires one named vocabulary"));
    Ok(())
}

#[test]
fn generic_projection_subjects_and_generated_names_refuse_before_rendering() -> Result<(), String> {
    let missing = GENERIC_RECIPE.replace("typestate(Stage)", "typestate(Missing)");
    let missing = refusal_summary(missing.as_str())
        .map_err(|()| "the missing typestate subject was accepted".to_owned())?;
    assert!(missing.contains("names no authored enum `Missing`"));

    let implicit = GENERIC_RECIPE.replace("typestate(Stage)", "typestate");
    let implicit = refusal_summary(implicit.as_str())
        .map_err(|()| "the ambiguous typestate subject was accepted".to_owned())?;
    assert!(implicit.contains("requires one named vocabulary"));

    let dispatch = GENERIC_RECIPE.replace("typestate(Stage);", "typestate(Stage); dispatch;");
    let dispatch = refusal_summary(dispatch.as_str())
        .map_err(|()| "generic dispatch without a transition lowering was accepted".to_owned())?;
    assert!(dispatch.contains("requires one typed transition lowering"));

    let evidence = GENERIC_RECIPE.replace("mutation(Capability)", "mutation(Missing)");
    let evidence = refusal_summary(evidence.as_str())
        .map_err(|()| "evidence targeting an absent vocabulary was accepted".to_owned())?;
    assert!(evidence.contains("names no authored enum `Missing`"));

    let collision = r"
pub mod collision {
    pub enum HttpState { Ready }
    pub enum Http_State { Waiting }

    bake! {
        vocabularies {
            HttpState;
            Http_State;
        };
        relations {
        };
        projections {
            companions;
        };
    }
}
";
    let collision = refusal_summary(collision)
        .map_err(|()| "two generated companion constants collided silently".to_owned())?;
    assert!(collision.contains("generated recipe name `HTTP_STATE_VARIANTS` is already occupied"));

    let relation_module_collision = r"
pub mod collision {
    pub enum Stage { Draft, Published }

    bake! {
        vocabularies { Stage; };
        relations {
            typestate(Stage, Stage) {
                (Draft, Published);
            };
        };
        projections {
            typestate(Stage);
            relation_tables { typestate; };
        };
    }
}
";
    let relation_module_collision = refusal_summary(relation_module_collision)
        .map_err(|()| "two generated modules occupied `typestate`".to_owned())?;
    assert!(relation_module_collision.contains("generated recipe name `typestate`"));
    Ok(())
}

#[test]
fn typed_relation_tables_refuse_ambiguous_or_authority_bypassing_requests() -> Result<(), String> {
    let empty = GENERIC_RECIPE.replace(
        "            relation_tables {\n                evolution;\n                policy {\n                    pub fn lookup_policy(\n                        stage: &Stage,\n                        capability: &Capability,\n                    ) -> Option<crate::Decision>;\n                };\n                labels {\n                    pub fn lookup_label(\n                        stage: &Stage,\n                        capability: &Capability,\n                    ) -> Option<crate::Decision>;\n                };\n            };",
        "            relation_tables { };",
    );
    let empty = refusal_summary(empty.as_str())
        .map_err(|()| "an empty relation-table family was accepted".to_owned())?;
    assert!(empty.contains("requires at least one caller-named relation"));

    let duplicate = GENERIC_RECIPE.replace(
        "                evolution;",
        "                evolution;\n                evolution;",
    );
    let duplicate = refusal_summary(duplicate.as_str())
        .map_err(|()| "one relation table was requested twice".to_owned())?;
    assert!(duplicate.contains("relation table `evolution` is requested more than once"));

    let missing = GENERIC_RECIPE.replace("                evolution;", "                missing;");
    let missing = refusal_summary(missing.as_str())
        .map_err(|()| "a relation table named an absent relation".to_owned())?;
    assert!(missing.contains("names no relation `missing`"));

    let payload_without_type = GENERIC_RECIPE.replace(
        "                policy {\n                    pub fn lookup_policy(\n                        stage: &Stage,\n                        capability: &Capability,\n                    ) -> Option<crate::Decision>;\n                };",
        "                policy;",
    );
    let payload_without_type = refusal_summary(payload_without_type.as_str())
        .map_err(|()| "a payload table inferred its caller-owned result".to_owned())?;
    assert!(payload_without_type.contains("requires one exact Rust function signature"));

    let transition = MIXED_TRANSITION_RECIPE.replace(
        "            dispatch(apply);",
        "            dispatch(apply);\n            relation_tables { transitions; };",
    );
    let transition = refusal_summary(transition.as_str())
        .map_err(|()| "the relation table consumed transition payloads".to_owned())?;
    assert!(transition.contains("transition payloads owned by the dispatch projection"));

    let body = GENERIC_RECIPE.replace(
        ") -> Option<crate::Decision>;",
        ") -> Option<crate::Decision> { None }",
    );
    let body = refusal_summary(body.as_str())
        .map_err(|()| "the standard table accepted a caller-authored body".to_owned())?;
    assert!(body.contains("cannot carry a caller-authored body"));

    let count = GENERIC_RECIPE.replace(
        "                        stage: &Stage,\n                        capability: &Capability,",
        "                        stage: &Stage,",
    );
    let count = refusal_summary(count.as_str())
        .map_err(|()| "the exact table accepted one parameter".to_owned())?;
    assert!(count.contains("requires two parameters but the signature states 1"));

    let binding = GENERIC_RECIPE.replace(
        "                        stage: &Stage,",
        "                        mut stage: &Stage,",
    );
    let binding = refusal_summary(binding.as_str())
        .map_err(|()| "the exact table accepted a non-identifier binding".to_owned())?;
    assert!(binding.contains("parameter 1 must use one simple identifier binding"));
    Ok(())
}

#[test]
fn exact_same_roster_table_preserves_bindings_and_imports_once() -> Result<(), String> {
    let source = r"
pub mod graph {
    pub enum Stage { Draft, Published }

    bake! {
        vocabularies { Stage; };
        relations {
            labels(Stage, Stage) {
                (Draft, Published) with { crate::Decision::Audit };
            };
        };
        projections {
            relation_tables {
                labels {
                    pub fn lookup(
                        origin: &Stage,
                        destination: &Stage,
                    ) -> Option<crate::Decision>;
                };
            };
        };
    }
}
";
    let baked = bake(source).map_err(|()| {
        refusal_summary(source)
            .unwrap_or_else(|()| "the exact same-roster table refused".to_owned())
    })?;
    let effective = baked
        .projection()
        .plan()
        .content()
        .effective(macroonz_compiler::recipe::RecipeRole::RelationTables)
        .ok_or_else(|| "the exact table lowering is absent".to_owned())?;
    let tables = effective.relation_tables().collect::<Vec<_>>();
    let [table] = tables.as_slice() else {
        return Err("the exact table lowering changed cardinality".to_owned());
    };
    assert_eq!(table.function(), "lookup");

    let emitted = baked
        .emit()
        .tokens()
        .ok_or_else(|| "the exact table emitted no declaration-site material".to_owned())?
        .inspected();
    assert_eq!(emitted.matches("use super :: super :: Stage").count(), 1);
    assert!(emitted.contains("match ( origin , destination )"));
    Ok(())
}

#[test]
fn transition_absence_is_explicit_and_allowed_is_read_back() -> Result<(), String> {
    let missing = MIXED_TRANSITION_RECIPE.replace("        absence(refused);\n", "");
    let missing = refusal_summary(missing.as_str())
        .map_err(|()| "a transition without an absence answer was accepted".to_owned())?;
    assert!(
        missing.contains("not the ordinary word `absence`"),
        "unexpected refusal: {missing}"
    );

    let allowed = MIXED_TRANSITION_RECIPE
        .replace("absence(refused)", "absence(allowed)")
        .replace("            dispatch(apply);\n", "");
    let allowed = bake(allowed.as_str())
        .map_err(|()| "the explicit allowed-absence recipe refused".to_owned())?;
    let transition = allowed
        .projection()
        .plan()
        .content()
        .transition_relation()
        .ok_or_else(|| "the transition lowering is absent".to_owned())?;
    assert_eq!(
        transition.requirements().absence(),
        Some(macroonz_compiler::AbsencePosture::Allowed)
    );
    Ok(())
}

#[test]
fn attributed_discriminants_and_generic_record_boundaries_remain_rust_shaped() -> Result<(), String>
{
    let variants = r#"
pub mod vocabulary {
    pub enum Stage {
        #[doc = "draft"]
        Draft = 0,
        Published = 1,
    }

    bake! {
        vocabularies { Stage; };
        projections { companions; };
    }
}
"#;
    let variants = bake(variants)
        .map_err(|()| "attributed explicit-discriminant variants refused".to_owned())?;
    let vocabulary = variants
        .projection()
        .plan()
        .content()
        .vocabulary("Stage")
        .ok_or_else(|| "the attributed vocabulary is absent".to_owned())?;
    assert_eq!(
        vocabulary
            .members()
            .members()
            .map(macroonz_compiler::recipe::RecipeMember::spelling)
            .collect::<Vec<_>>(),
        ["Draft", "Published"]
    );

    let generic_record = r"
pub mod codec_record {
    pub struct Ledger<T = u16> { pub count: T }
    bake! {
        codecs {
            ledger(Ledger) {
                direction(encode);
                refusal(LedgerDecodeError);
                assembly(assembled, total);
                members { count: T => count(required); };
            };
        };
        projections { codec; };
    }
}
";
    let _generic_record = bake(generic_record)
        .map_err(|()| "a generic record-shaped codec owner refused".to_owned())?;

    let generic_unit = generic_record.replace(
        "pub struct Ledger<T = u16> { pub count: T }",
        "pub struct Ledger<const N: usize = { 1 }> ;",
    );
    let generic_unit = refusal_summary(generic_unit.as_str())
        .map_err(|()| "a generic unit struct was accepted as a record".to_owned())?;
    assert!(generic_unit.contains("owner `Ledger` is not an authored record struct"));
    Ok(())
}

#[test]
fn relation_payload_and_posture_movements_are_identity_material() -> Result<(), String> {
    let baseline = bake(GENERIC_RECIPE).map_err(|()| {
        refusal_summary(GENERIC_RECIPE)
            .unwrap_or_else(|()| "the baseline refused without a summary".to_owned())
    })?;
    let payload_moved = bake(&GENERIC_RECIPE.replace("Decision::Audit", "Decision::Deny"))
        .map_err(|()| "the payload movement refused".to_owned())?;
    let posture_moved = bake(&GENERIC_RECIPE.replace("empty(allowed)", "empty(refused)"))
        .map_err(|()| "the posture movement refused".to_owned());
    let table_surface_moved = bake(&GENERIC_RECIPE.replace("lookup_policy", "query_policy"))
        .map_err(|()| "the relation-table signature movement refused".to_owned())?;
    assert_ne!(
        baseline
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        payload_moved
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );
    assert!(posture_moved.is_err());
    assert_ne!(
        baseline
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        table_surface_moved
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );
    Ok(())
}

#[test]
fn mixed_payload_contracts_and_false_structural_answers_refuse() -> Result<(), String> {
    let mixed = GENERIC_RECIPE.replace(
        "(Draft, Read) with(crate::policy::allow);",
        "(Draft, Read) with(crate::policy::allow);\n                (Published, Write) with { crate::Decision::Audit };",
    );
    let mixed = refusal_summary(mixed.as_str())
        .map_err(|()| "the mixed payload relation was accepted".to_owned())?;
    assert!(
        mixed.contains("mixes `path` and `exact-rust`"),
        "unexpected refusal: {mixed}"
    );

    let dense = GENERIC_RECIPE.replace("density(sparse)", "density(dense)");
    let dense = refusal_summary(dense.as_str())
        .map_err(|()| "the false dense posture was accepted".to_owned())?;
    assert!(dense.contains("requires density `dense`"));
    assert!(dense.contains("compute `sparse`"));
    Ok(())
}
