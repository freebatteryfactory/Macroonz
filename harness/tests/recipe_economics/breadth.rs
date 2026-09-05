//! Deterministic work movement across the remaining recipe axes.

use super::DOOR;
use macroonz_compiler::recipe::{HarnessPosture, RecipeBake, RecipeRole};
use macroonz_compiler::{CanonicalContent, TextCapture};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BakeMetrics {
    source_bytes: usize,
    vocabularies: usize,
    variants: usize,
    relations: usize,
    rows: usize,
    codecs: usize,
    codec_members: usize,
    selected_roles: usize,
    planned_units: usize,
    rendered_units: usize,
    explanation_answers: usize,
    generated_bytes: usize,
    recipe_bytes: usize,
    delivered_bytes: usize,
    test_carrier_bytes: usize,
}

impl BakeMetrics {
    const fn counts(self) -> [usize; 15] {
        [
            self.source_bytes,
            self.vocabularies,
            self.variants,
            self.relations,
            self.rows,
            self.codecs,
            self.codec_members,
            self.selected_roles,
            self.planned_units,
            self.rendered_units,
            self.explanation_answers,
            self.generated_bytes,
            self.recipe_bytes,
            self.delivered_bytes,
            self.test_carrier_bytes,
        ]
    }
}

#[test]
fn every_remaining_recipe_axis_moves_repeatable_declared_work() -> Result<(), String> {
    verify_breadth()
}

pub(super) fn verify_breadth() -> Result<(), String> {
    let postures = observe_curve(&[1, 2, 4, 8], posture_source)?;
    assert_axis(&postures, |metrics| metrics.source_bytes)?;
    assert_axis(&postures, |metrics| metrics.recipe_bytes)?;
    assert_constant(&postures, |metrics| metrics.vocabularies, 1)?;
    assert_constant(&postures, |metrics| metrics.variants, 2)?;
    assert_constant(&postures, |metrics| metrics.relations, 1)?;
    assert_constant(&postures, |metrics| metrics.rows, 4)?;
    assert!(
        postures
            .iter()
            .all(|(_, metrics)| metrics.explanation_answers > 0)
    );

    let fields = observe_curve(&[1, 2, 4, 8], codec_source)?;
    assert_exact_axis(&fields, |metrics| metrics.codec_members);
    assert_constant(&fields, |metrics| metrics.codecs, 1)?;
    assert_axis(&fields, |metrics| metrics.generated_bytes)?;
    assert_axis(&fields, |metrics| metrics.delivered_bytes)?;

    let paths = observe_curve(&[1, 2, 4, 8], payload_path_source)?;
    assert_axis(&paths, |metrics| metrics.source_bytes)?;
    assert_axis(&paths, |metrics| metrics.recipe_bytes)?;
    assert_axis(&paths, |metrics| metrics.generated_bytes)?;

    let projections = observe_curve(&[1, 2, 3, 4, 5], projection_catalog_source)?;
    assert_exact_axis(&projections, |metrics| metrics.selected_roles);
    assert_exact_axis(&projections, |metrics| metrics.planned_units);
    assert_exact_axis(&projections, |metrics| metrics.rendered_units);
    assert_axis(&projections, |metrics| metrics.generated_bytes)?;

    let complete_catalog_source = String::from(COMPLETE_CATALOG_SOURCE);
    let complete_catalog = observe(complete_catalog_source.as_str())?;
    assert_eq!(complete_catalog, observe(complete_catalog_source.as_str())?);
    assert_eq!(complete_catalog.selected_roles, RecipeRole::ALL.len());
    assert_eq!(complete_catalog.planned_units, RecipeRole::ALL.len());
    assert_eq!(complete_catalog.rendered_units, RecipeRole::ALL.len());
    assert!(complete_catalog.test_carrier_bytes > 0);

    let carriers = observe_curve(&[1, 2], carrier_source)?;
    for (axis, metrics) in &carriers {
        let expected_roles = axis
            .checked_add(2)
            .ok_or_else(|| String::from("carrier role count overflow"))?;
        assert_eq!(metrics.selected_roles, expected_roles);
        assert!(metrics.test_carrier_bytes > 0);
    }
    assert_axis(&carriers, |metrics| metrics.test_carrier_bytes)?;
    Ok(())
}

pub(super) fn observe_counts(source: &str) -> Result<[u64; 16], String> {
    let [
        source_bytes,
        vocabularies,
        variants,
        relations,
        rows,
        codecs,
        codec_members,
        selected_roles,
        planned_units,
        rendered_units,
        explanation_answers,
        generated_bytes,
        recipe_bytes,
        delivered_bytes,
        test_carrier_bytes,
    ] = observe(source)?.counts();
    Ok([
        u64::try_from(source_bytes).map_err(super::debug)?,
        u64::try_from(vocabularies).map_err(super::debug)?,
        u64::try_from(variants).map_err(super::debug)?,
        u64::try_from(relations).map_err(super::debug)?,
        u64::try_from(rows).map_err(super::debug)?,
        u64::try_from(codecs).map_err(super::debug)?,
        u64::try_from(codec_members).map_err(super::debug)?,
        u64::try_from(selected_roles).map_err(super::debug)?,
        u64::try_from(planned_units).map_err(super::debug)?,
        u64::try_from(rendered_units).map_err(super::debug)?,
        u64::try_from(explanation_answers).map_err(super::debug)?,
        u64::try_from(generated_bytes).map_err(super::debug)?,
        u64::try_from(recipe_bytes).map_err(super::debug)?,
        u64::try_from(delivered_bytes).map_err(super::debug)?,
        u64::try_from(test_carrier_bytes).map_err(super::debug)?,
        0,
    ])
}

fn observe_curve(
    axes: &[usize],
    source: fn(usize) -> String,
) -> Result<Vec<(usize, BakeMetrics)>, String> {
    axes.iter()
        .copied()
        .map(|axis| {
            let source = source(axis);
            let first = observe(source.as_str())?;
            let repeated = observe(source.as_str())?;
            if first != repeated {
                return Err(format!(
                    "axis {axis} produced two different work readings: {first:?} and {repeated:?}"
                ));
            }
            Ok((axis, first))
        })
        .collect()
}

fn assert_axis(
    curve: &[(usize, BakeMetrics)],
    read: fn(&BakeMetrics) -> usize,
) -> Result<(), String> {
    for pair in curve.windows(2) {
        let [(left_axis, left), (right_axis, right)] = pair else {
            return Err(String::from("an economics pair changed cardinality"));
        };
        if read(left) >= read(right) {
            return Err(format!(
                "work did not increase from axis {left_axis} to {right_axis}: {left:?} -> {right:?}"
            ));
        }
    }
    Ok(())
}

fn assert_exact_axis(curve: &[(usize, BakeMetrics)], read: fn(&BakeMetrics) -> usize) {
    for (axis, metrics) in curve {
        assert_eq!(read(metrics), *axis);
    }
}

fn assert_constant(
    curve: &[(usize, BakeMetrics)],
    read: fn(&BakeMetrics) -> usize,
    expected: usize,
) -> Result<(), String> {
    for (axis, metrics) in curve {
        if read(metrics) != expected {
            return Err(format!(
                "axis {axis} moved a fixed work seat away from {expected}: {metrics:?}"
            ));
        }
    }
    Ok(())
}

fn observe(source: &str) -> Result<BakeMetrics, String> {
    let capture = TextCapture::read(source).map_err(super::debug)?;
    let bake = macroonz_compiler::recipe::bake(capture.input(), HarnessPosture::Available, &DOOR)
        .map_err(|refusal| refusal.summary().to_owned())?;
    metrics(source, &bake)
}

fn metrics(source: &str, bake: &RecipeBake) -> Result<BakeMetrics, String> {
    let expansion = bake.projection();
    let recipe = expansion.plan().content();
    let variants = recipe
        .vocabularies()
        .try_fold(0usize, |count, vocabulary| {
            count
                .checked_add(vocabulary.members().count())
                .ok_or_else(|| String::from("variant count overflow"))
        })?;
    let rows = recipe.relations().try_fold(0usize, |count, relation| {
        count
            .checked_add(relation.row_count())
            .ok_or_else(|| String::from("relation-row count overflow"))
    })?;
    let codec_members = recipe.codecs().try_fold(0usize, |count, codec| {
        count
            .checked_add(codec.content().shape.count())
            .ok_or_else(|| String::from("codec-member count overflow"))
    })?;
    let selected_roles = RecipeRole::ALL
        .iter()
        .copied()
        .filter(|role| recipe.effective(*role).is_some())
        .count();
    let generated_bytes = expansion
        .closure()
        .rendered()
        .units()
        .iter()
        .map(|unit| unit.tree().canonical_bytes().len())
        .sum();
    let explanation_answers = expansion
        .explain()
        .universal()
        .len()
        .checked_add(expansion.explain().declared().len())
        .ok_or_else(|| String::from("explanation-answer count overflow"))?;
    Ok(BakeMetrics {
        source_bytes: source.len(),
        vocabularies: recipe.vocabularies().count(),
        variants,
        relations: recipe.relations().count(),
        rows,
        codecs: recipe.codecs().count(),
        codec_members,
        selected_roles,
        planned_units: expansion.plan().membership().count(),
        rendered_units: expansion.closure().rendered().count(),
        explanation_answers,
        generated_bytes,
        recipe_bytes: recipe.canonical_content_bytes().len(),
        delivered_bytes: cargo_bytes(bake.emit()),
        test_carrier_bytes: cargo_bytes(expansion.test_carrier()),
    })
}

fn cargo_bytes(cargo: &macroonz_compiler::PartitionCargo) -> usize {
    cargo
        .tokens()
        .map_or(0, |tokens| tokens.canonical_bytes().len())
}

pub(super) fn posture_source(question_count: usize) -> String {
    let questions = [
        "empty(refused);",
        "repetition(allowed);",
        "membership(open, open);",
        "completeness(total, total);",
        "density(dense);",
        "absence(refused);",
        "self_relation(allowed);",
        "cycle(allowed);",
    ];
    let mut source = String::from(
        "pub mod structure { pub enum Node { A, B } bake! { vocabularies { Node; }; relations { links(Node, Node) { (A, A); (A, B); (B, A); (B, B); }; }; postures { links {",
    );
    for question in questions.iter().take(question_count) {
        source.push_str(question);
    }
    source.push_str("}; }; projections { relation_tables { links; }; }; } }");
    source
}

pub(super) fn codec_source(field_count: usize) -> String {
    let mut source = String::from("pub mod record { pub struct Ledger {");
    for field in 0..field_count {
        source.push_str("pub field");
        source.push_str(&field.to_string());
        source.push_str(": u16,");
    }
    source.push_str("} bake! { codecs { ledger(Ledger) { direction(encode); refusal(LedgerError); assembly(assembled, total); members {");
    for field in 0..field_count {
        source.push_str("field");
        source.push_str(&field.to_string());
        source.push_str(": u16 => count(required);");
    }
    source.push_str("}; }; }; projections { codec; }; } }");
    source
}

pub(super) fn payload_path_source(depth: usize) -> String {
    let mut path = String::from("crate");
    for segment in 0..depth {
        path.push_str("::segment");
        path.push_str(&segment.to_string());
    }
    path.push_str("::effect");
    format!(
        "pub mod paths {{ pub enum Left {{ A }} pub enum Right {{ B }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ links(Left, Right) {{ (A, B) with({path}); }}; }}; projections {{ companions; }}; }} }}"
    )
}

pub(super) fn projection_catalog_source(count: usize) -> String {
    let projections = [
        "companions;",
        "dispatch(apply);",
        "typestate(State);",
        "relation_tables { policy; };",
        "codec;",
    ];
    let mut source = String::from(
        "pub mod catalog { pub enum State { Closed, Open } pub enum Event { OpenDoor } pub enum Capability { Read } pub struct Ledger { pub value: u16 } bake! { vocabularies { State; Event; Capability; }; transitions(State, Event) { (Closed, OpenDoor) => Open with(crate::open); }; relations { policy(State, Capability) { (Closed, Read); }; }; absence(refused); codecs { ledger(Ledger) { direction(encode); refusal(LedgerError); assembly(assembled, total); members { value: u16 => count(required); }; }; }; projections {",
    );
    for projection in projections.iter().take(count) {
        source.push_str(projection);
    }
    source.push_str("}; } }");
    source
}

pub(super) fn carrier_source(count: usize) -> String {
    let carriers = ["compile_contract;", "declaration_conformance;"];
    let mut source = String::from(
        "pub mod carrier { pub enum State { Closed, Open } pub enum Event { OpenDoor } bake! { vocabularies { State; Event; }; transitions(State, Event) { (Closed, OpenDoor) => Open with(crate::open); }; absence(refused); projections { companions; dispatch(apply);",
    );
    for carrier in carriers.iter().take(count) {
        source.push_str(carrier);
    }
    source.push_str("}; support(recipe_support); } }");
    source
}

const COMPLETE_CATALOG_SOURCE: &str = r#"
pub mod complete_catalog {
    pub enum State { Closed, Open, Locked }
    pub enum Event { OpenDoor, CloseDoor }
    pub enum Capability { Read, Write }
    pub struct Ledger { pub count: u16 }

    bake! {
        vocabularies { State; Event; Capability; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::open);
            (Open, CloseDoor) => Closed with(crate::close);
        };
        relations {
            policy(State, Capability) {
                (Closed, Read);
                (Open, Write);
            };
        };
        absence(refused);
        codecs {
            ledger(Ledger) {
                direction(encode);
                refusal(LedgerError);
                assembly(assembled, total);
                members { count: u16 => count(required); };
            };
        };
        projections {
            companions;
            relation_tables { policy; };
            dispatch(apply);
            compile_contract;
            declaration_conformance;
            typestate(State);
            codec;
        };
        evidence {
            trials {
                support = recipe_trials_support,
                module = recipe_trials,
                table = named("economics", "trials"),
                suite checks = named("economics", "unit") {
                    transition_answers {
                        claim = named("economics", "transition-answers"),
                        subject = named("economics", "dispatch"),
                        check = named("economics", "exact"),
                        population = named("economics", "declared-rows"),
                    },
                },
            };
            mutation(State) {
                module = recipe_mutations,
                refusal = RecipeMutationRefusal,
                support = recipe_mutation_support,
                family = named("economics", "mutation"),
                point = named("economics", "state-order"),
                fact = named("economics", "state-order"),
                map named("economics", "state-order") = named("economics", "order-held"),
                permit named("economics", "order-held") = ["declared-order-permutation"],
            };
            benchmarks {
                support = recipe_bench_support,
                table_function = recipe_bench_table,
                table = named("economics", "bench-table"),
                reporter = recipe_bench_reporter,
                dispatch_pace {
                    workload = named("economics", "dispatch"),
                    preflight = named("economics", "dispatch-correct"),
                    planted_worse = named("economics", "dispatch-worse"),
                    complexity = named("economics", "linear"),
                    axis = [2, 4, 8],
                    samples = 16,
                    warmups = 4,
                    ratio_numerator = 3,
                    ratio_denominator = 1,
                    observe = [named("economics", "rows-touched")],
                },
            };
            network {
                harness = macroonz::harness,
                module = recipe_network,
                namespace = "economics",
                nodes = [client, server],
                link forward = client to server,
                schedule quiet = [],
            };
            concurrency {
                harness = macroonz::harness,
                module = recipe_concurrency,
                namespace = "economics",
                transitions_hold {
                    population = "transition-orders",
                    interleavings = 16,
                    samples = 32,
                    seed = 11,
                },
            };
        };
        support(recipe_support);
    }
}
"#;
