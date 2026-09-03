//! Every recipe-owned magnitude is derived where a closed roster exists and crossed at N-1, N, and N+1.

use super::{bake, refusal_summary};
use macroonz_compiler::recipe::{
    CODEC_LIMIT, EVIDENCE_LIMIT, PROJECTION_CLAUSE_LIMIT, RELATION_LIMIT, RELATION_QUESTION_LIMIT,
    RELATION_ROW_LIMIT, RELATION_TABLE_LIMIT, TRANSITION_LIMIT, VOCABULARY_LIMIT,
};

const SEQUENCE_REFUSAL: &str = "captured sequence carries more members than its declared magnitude";

fn assert_boundary(limit: usize, source: impl Fn(usize) -> String) -> Result<(), String> {
    let before = limit
        .checked_sub(1)
        .ok_or_else(|| String::from("a boundary must admit one earlier magnitude"))?;
    let beyond = limit
        .checked_add(1)
        .ok_or_else(|| String::from("a boundary must admit one later magnitude"))?;
    for count in [before, limit] {
        let candidate = source(count);
        let _baked = bake(candidate.as_str()).map_err(|()| {
            let reason = refusal_summary(candidate.as_str())
                .unwrap_or_else(|()| String::from("the refusal was not projected"));
            format!("{count} was refused at limit {limit}: {reason}")
        })?;
    }
    let summary = refusal_summary(source(beyond).as_str())
        .map_err(|()| format!("{beyond} did not produce a recipe refusal"))?;
    assert!(summary.contains(SEQUENCE_REFUSAL), "{summary}");
    assert!(summary.contains(limit.to_string().as_str()), "{summary}");
    Ok(())
}

#[test]
fn vocabulary_members_and_vocabulary_declarations_cross_their_exact_boundaries()
-> Result<(), String> {
    assert_boundary(VOCABULARY_LIMIT, vocabulary_member_source)?;
    assert_boundary(VOCABULARY_LIMIT, vocabulary_source)
}

#[test]
fn relation_rows_relations_and_relation_table_selections_cross_their_exact_boundaries()
-> Result<(), String> {
    assert_boundary(RELATION_ROW_LIMIT, relation_row_source)?;
    assert_boundary(RELATION_LIMIT, relation_source)?;
    assert_boundary(RELATION_TABLE_LIMIT, relation_table_source)
}

#[test]
fn transition_rows_and_codec_declarations_cross_their_exact_boundaries() -> Result<(), String> {
    assert_boundary(TRANSITION_LIMIT, transition_source)?;
    assert_boundary(CODEC_LIMIT, codec_source)
}

#[test]
fn posture_projection_and_evidence_rosters_cross_their_exact_boundaries() -> Result<(), String> {
    assert_boundary(RELATION_QUESTION_LIMIT, posture_source)?;
    assert_boundary(PROJECTION_CLAUSE_LIMIT, projection_source)?;
    assert_boundary(EVIDENCE_LIMIT, evidence_source)
}

fn vocabulary_member_source(count: usize) -> String {
    let variants = comma_joined(count, |index| format!("V{index}"));
    format!(
        "pub mod subject {{ pub enum Stage {{ {variants} }} bake! {{ vocabularies {{ Stage; }}; projections {{ companions; }}; }} }}"
    )
}

fn vocabulary_source(count: usize) -> String {
    let declarations = joined(count, |index| format!("pub enum V{index} {{ A }}"));
    let names = joined(count, |index| format!("V{index};"));
    format!(
        "pub mod subject {{ {declarations} bake! {{ vocabularies {{ {names} }}; projections {{ companions; }}; }} }}"
    )
}

fn relation_row_source(count: usize) -> String {
    let right_count = count.div_ceil(VOCABULARY_LIMIT);
    let left = comma_joined(VOCABULARY_LIMIT, |index| format!("L{index}"));
    let right = comma_joined(right_count, |index| format!("R{index}"));
    let rows = joined(count, |index| {
        format!(
            "(L{}, R{});",
            index % VOCABULARY_LIMIT,
            index.checked_div(VOCABULARY_LIMIT).unwrap_or_default()
        )
    });
    format!(
        "pub mod subject {{ pub enum Left {{ {left} }} pub enum Right {{ {right} }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ links(Left, Right) {{ {rows} }}; }}; projections {{ companions; }}; }} }}"
    )
}

fn relation_source(count: usize) -> String {
    let relations = joined(count, |index| {
        format!("R{index}(Left, Right) {{ (A, B); }};")
    });
    format!(
        "pub mod subject {{ pub enum Left {{ A }} pub enum Right {{ B }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ {relations} }}; projections {{ companions; }}; }} }}"
    )
}

fn relation_table_source(count: usize) -> String {
    let relations = joined(RELATION_LIMIT, |index| {
        format!("R{index}(Left, Right) {{ (A, B); }};")
    });
    let tables = joined(count, |index| format!("R{};", index % RELATION_LIMIT));
    format!(
        "pub mod subject {{ pub enum Left {{ A }} pub enum Right {{ B }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ {relations} }}; projections {{ relation_tables {{ {tables} }}; }}; }} }}"
    )
}

fn transition_source(count: usize) -> String {
    let event_count = count.div_ceil(VOCABULARY_LIMIT);
    let states = comma_joined(VOCABULARY_LIMIT, |index| format!("S{index}"));
    let events = comma_joined(event_count, |index| format!("E{index}"));
    let rows = joined(count, |index| {
        format!(
            "(S{}, E{}) => S0 with(crate::effect);",
            index % VOCABULARY_LIMIT,
            index.checked_div(VOCABULARY_LIMIT).unwrap_or_default()
        )
    });
    format!(
        "pub mod subject {{ pub enum State {{ {states} }} pub enum Event {{ {events} }} bake! {{ vocabularies {{ State; Event; }}; transitions(State, Event) {{ {rows} }}; absence(refused); projections {{ dispatch; }}; }} }}"
    )
}

fn codec_source(count: usize) -> String {
    let records = joined(count, |index| {
        format!("pub struct Record{index} {{ pub value: u16 }}")
    });
    let codecs = joined(count, |index| {
        format!(
            "codec{index}(Record{index}) {{ direction(encode); refusal(Refusal{index}); assembly(assembled, total); members {{ value: u16 => count(required); }}; }};"
        )
    });
    format!(
        "pub mod subject {{ {records} bake! {{ codecs {{ {codecs} }}; projections {{ codec; }}; }} }}"
    )
}

fn posture_source(count: usize) -> String {
    const CLAUSES: &[&str] = &[
        "empty(allowed);",
        "repetition(allowed);",
        "membership(open, open);",
        "completeness(partial, partial);",
        "density(sparse);",
        "absence(allowed);",
        "self_relation(allowed);",
        "cycle(allowed);",
    ];
    let clauses = CLAUSES
        .iter()
        .copied()
        .cycle()
        .take(count)
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "pub mod subject {{ pub enum Node {{ A }} bake! {{ vocabularies {{ Node; }}; relations {{ links(Node, Node) {{ (A, A); }}; }}; postures {{ links {{ {clauses} }}; }}; projections {{ companions; }}; }} }}"
    )
}

fn projection_source(count: usize) -> String {
    const CLAUSES: &[&str] = &[
        "companions;",
        "relation_tables { policy; };",
        "dispatch;",
        "compile_contract;",
        "declaration_conformance;",
        "typestate(State);",
        "codec;",
    ];
    let clauses = CLAUSES
        .iter()
        .copied()
        .cycle()
        .take(count)
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "pub mod subject {{ pub enum State {{ Closed, Open }} pub enum Event {{ Go }} pub enum Right {{ B }} pub struct Ledger {{ pub value: u16 }} bake! {{ vocabularies {{ State; Event; Right; }}; transitions(State, Event) {{ (Closed, Go) => Open with(crate::effect); }}; relations {{ policy(State, Right) {{ (Closed, B); }}; }}; absence(refused); codecs {{ ledger(Ledger) {{ direction(encode); refusal(LedgerError); assembly(assembled, total); members {{ value: u16 => count(required); }}; }}; }}; projections {{ {clauses} }}; support(recipe_support); }} }}"
    )
}

fn evidence_source(count: usize) -> String {
    const CLAUSES: &[&str] = &[
        "trials unavailable;",
        "mutation unavailable;",
        "benchmarks unavailable;",
        "network unavailable;",
        "concurrency unavailable;",
    ];
    let clauses = CLAUSES
        .iter()
        .copied()
        .cycle()
        .take(count)
        .collect::<Vec<_>>()
        .join(" ");
    format!(
        "pub mod subject {{ pub enum Stage {{ A }} bake! {{ vocabularies {{ Stage; }}; projections {{ companions; }}; evidence {{ {clauses} }}; }} }}"
    )
}

pub(super) fn joined(count: usize, row: impl Fn(usize) -> String) -> String {
    (0..count).map(row).collect::<Vec<_>>().join(" ")
}

fn comma_joined(count: usize, row: impl Fn(usize) -> String) -> String {
    (0..count).map(row).collect::<Vec<_>>().join(", ")
}
