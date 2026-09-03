//! Recipe refusals preserve exact issue, repair, site, and precedence across remaining public families.

use super::DOOR;
use macroonz_compiler::recipe::HarnessPosture;
use macroonz_compiler::{
    CapturedTokenTree, Diagnostic, Observed, Phase, RefusalClass, SpanHandle, TextCapture,
};

const DUPLICATE_REPAIR: &str = "state each authored member, relation endpoint pair, transition seat, projection role, and generated name once";
const EXACT_DISPATCH_REPAIR: &str = "remove the exact function body and leave the semicolon-terminated signature for the standard dispatch projector to fill";
const EXACT_RELATION_TABLE_REPAIR: &str = "remove the exact function body and leave the semicolon-terminated signature for the standard relation-table projector to fill";
const HARNESS_REPAIR: &str =
    "enable the facade harness feature or remove the harness-owned projection from this recipe";
const EMPTY_VOCABULARY_REPAIR: &str =
    "state at least one unit variant in every selected vocabulary";
const SEQUENCE_LIMIT_REPAIR: &str =
    "keep each captured sequence at or below its declared magnitude";

fn refusal(source: &str, harness: HarnessPosture) -> Result<Diagnostic, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), harness, &DOOR)
        .err()
        .ok_or(())
}

fn admitted(source: &str) -> Result<(), ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map(|_| ())
        .map_err(|_| ())
}

fn assert_refusal(
    refused: &Diagnostic,
    summary: &str,
    class: RefusalClass,
    observed: Observed,
    at: SpanHandle,
    repair: &str,
) -> Result<(), ()> {
    assert_eq!(refused.phase(), Phase::Capture);
    assert!(
        refused.summary().contains(class.described()),
        "{}",
        refused.summary()
    );
    assert_eq!(refused.observed(), observed);
    assert!(refused.summary().contains(summary), "{}", refused.summary());
    assert_eq!(refused.site().token(), Some(at));
    assert!(refused.site().coordinate().is_some());
    let [actual] = refused.repairs() else {
        return Err(());
    };
    assert_eq!(actual.description.shown(), repair);
    Ok(())
}

fn flattened(source: &str) -> Result<Vec<CapturedTokenTree>, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    let mut tokens = Vec::new();
    collect(read.input().trees(), &mut tokens);
    Ok(tokens)
}

fn collect(trees: &[CapturedTokenTree], into: &mut Vec<CapturedTokenTree>) {
    for tree in trees {
        into.push(tree.clone());
        if let Some((_delimiter, children)) = tree.group() {
            collect(children, into);
        }
    }
}

fn last_word(source: &str, word: &str) -> Result<SpanHandle, ()> {
    flattened(source)?
        .into_iter()
        .rfind(|tree| tree.word() == Some(word))
        .map(|tree| tree.span())
        .ok_or(())
}

fn first_word(source: &str, word: &str) -> Result<SpanHandle, ()> {
    flattened(source)?
        .into_iter()
        .find(|tree| tree.word() == Some(word))
        .map(|tree| tree.span())
        .ok_or(())
}

fn word_occurrence(source: &str, word: &str, occurrence: usize) -> Result<SpanHandle, ()> {
    flattened(source)?
        .into_iter()
        .filter(|tree| tree.word() == Some(word))
        .nth(occurrence)
        .map(|tree| tree.span())
        .ok_or(())
}

fn narrow_group_containing(source: &str, word: &str) -> Result<SpanHandle, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    find_group(read.input().trees(), word).ok_or(())
}

fn last_group_directly_containing(source: &str, word: &str) -> Result<SpanHandle, ()> {
    flattened(source)?
        .into_iter()
        .rfind(|tree| {
            tree.group().is_some_and(|(_delimiter, children)| {
                children.iter().any(|child| child.word() == Some(word))
            })
        })
        .map(|tree| tree.span())
        .ok_or(())
}

fn find_group(trees: &[CapturedTokenTree], word: &str) -> Option<SpanHandle> {
    for tree in trees {
        let Some((_delimiter, children)) = tree.group() else {
            continue;
        };
        if let Some(found) = find_group(children, word) {
            return Some(found);
        }
        if children.iter().any(|child| child.word() == Some(word)) {
            return Some(tree.span());
        }
    }
    None
}

#[test]
fn duplicate_members_and_projections_point_at_the_second_declaration() -> Result<(), ()> {
    let member = r"
pub mod duplicate_member {
    pub enum Stage { Draft, Draft }
    bake! {
        vocabularies { Stage; };
        projections { companions; };
    }
}
";
    assert_refusal(
        &refusal(member, HarnessPosture::Available)?,
        "states member `Draft` more than once",
        RefusalClass::DeclarationNotRead,
        Observed::IdentityDisagreement,
        last_word(member, "Draft")?,
        DUPLICATE_REPAIR,
    )?;

    let projection = r"
pub mod duplicate_projection {
    pub enum Stage { Draft }
    bake! {
        vocabularies { Stage; };
        projections { companions; companions; };
    }
}
";
    assert_refusal(
        &refusal(projection, HarnessPosture::Available)?,
        "projection `companions` is requested more than once",
        RefusalClass::DeclarationNotRead,
        Observed::IdentityDisagreement,
        last_word(projection, "companions")?,
        DUPLICATE_REPAIR,
    )
}

#[test]
fn harness_unavailability_points_at_the_requested_role_before_support_checks() -> Result<(), ()> {
    let source = r"
pub mod unavailable {
    pub enum State { Closed, Open }
    pub enum Event { OpenDoor }
    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::open);
        };
        absence(refused);
        projections { compile_contract; };
        support(unavailable_support);
    }
}
";
    assert_refusal(
        &refusal(source, HarnessPosture::Unavailable)?,
        "projection `compile-contract` requires the facade harness feature",
        RefusalClass::DeclarationNotRead,
        Observed::ProfileDisagreement,
        last_word(source, "compile_contract")?,
        HARNESS_REPAIR,
    )
}

#[test]
fn exact_dispatch_body_refusal_points_at_the_body_before_row_projection() -> Result<(), ()> {
    let source = r"
pub mod exact_body {
    pub enum State { Closed, Open }
    pub enum Event { OpenDoor }
    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::open);
        };
        absence(refused);
        projections {
            dispatch {
                pub fn apply(state: State, event: Event) -> Result<State, TransitionRefusal> {
                    caller_body
                }
            };
        };
    }
}
";
    assert_refusal(
        &refusal(source, HarnessPosture::Available)?,
        "exact dispatch cannot carry a caller-authored body",
        RefusalClass::DeclarationNotRead,
        Observed::ContractDisagreement,
        narrow_group_containing(source, "caller_body")?,
        EXACT_DISPATCH_REPAIR,
    )
}

#[test]
fn exact_relation_table_body_refusal_keeps_the_shared_issue_contract() -> Result<(), ()> {
    let source = r"
pub mod exact_table_body {
    pub enum Left { A }
    pub enum Right { B }
    bake! {
        vocabularies { Left; Right; };
        relations {
            policy(Left, Right) {
                (A, B) with(crate::allow);
            };
        };
        projections {
            relation_tables {
                policy {
                    pub fn lookup(left: Left, right: Right) -> Option<bool> {
                        caller_body
                    }
                };
            };
        };
    }
}
";
    assert_refusal(
        &refusal(source, HarnessPosture::Available)?,
        "an exact relation table cannot carry a caller-authored body",
        RefusalClass::DeclarationNotRead,
        Observed::ContractDisagreement,
        narrow_group_containing(source, "caller_body")?,
        EXACT_RELATION_TABLE_REPAIR,
    )
}

#[test]
fn vocabulary_magnitude_refusals_name_the_empty_and_first_excess_seats() -> Result<(), ()> {
    let empty = r"
pub mod empty_vocabulary {
    pub enum Empty {}
    bake! {
        vocabularies { Empty; };
        projections { companions; };
    }
}
";
    assert_refusal(
        &refusal(empty, HarnessPosture::Available)?,
        "authored enum `Empty` states no variants",
        RefusalClass::DeclarationNotRead,
        Observed::SeatAbsent,
        last_word(empty, "Empty")?,
        EMPTY_VOCABULARY_REPAIR,
    )?;

    let variants = (0_usize..=64)
        .map(|index| format!("V{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    let members = format!(
        "pub mod member_limit {{ pub enum Stage {{ {variants} }} bake! {{ vocabularies {{ Stage; }}; projections {{ companions; }}; }} }}"
    );
    assert_refusal(
        &refusal(members.as_str(), HarnessPosture::Available)?,
        "captured sequence carries more members than its declared magnitude of 64",
        RefusalClass::DeclarationNotRead,
        Observed::BoundExceeded,
        last_word(members.as_str(), "V64")?,
        SEQUENCE_LIMIT_REPAIR,
    )?;

    let enums = (0_usize..=64)
        .map(|index| format!("pub enum V{index} {{ Only }}"))
        .collect::<Vec<_>>()
        .join(" ");
    let names = (0_usize..=64)
        .map(|index| format!("V{index};"))
        .collect::<Vec<_>>()
        .join(" ");
    let vocabularies = format!(
        "pub mod vocabulary_limit {{ {enums} bake! {{ vocabularies {{ {names} }}; projections {{ companions; }}; }} }}"
    );
    assert_refusal(
        &refusal(vocabularies.as_str(), HarnessPosture::Available)?,
        "captured sequence carries more members than its declared magnitude of 64",
        RefusalClass::DeclarationNotRead,
        Observed::BoundExceeded,
        last_word(vocabularies.as_str(), "V64")?,
        SEQUENCE_LIMIT_REPAIR,
    )
}

#[test]
fn relation_magnitude_refusals_point_at_the_first_excess_row_and_relation() -> Result<(), ()> {
    let rows = (0_usize..=128)
        .map(|_| "(A, B);")
        .collect::<Vec<_>>()
        .join(" ");
    let row_limit = format!(
        "pub mod row_limit {{ pub enum Left {{ A }} pub enum Right {{ B }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ links(Left, Right) {{ {rows} }}; }}; projections {{ companions; }}; }} }}"
    );
    assert_refusal(
        &refusal(row_limit.as_str(), HarnessPosture::Available)?,
        "captured sequence carries more members than its declared magnitude of 128",
        RefusalClass::DeclarationNotRead,
        Observed::BoundExceeded,
        last_group_directly_containing(row_limit.as_str(), "A")?,
        SEQUENCE_LIMIT_REPAIR,
    )?;

    let relations = (0_usize..=64)
        .map(|index| format!("R{index}(Left, Right) {{ (A, B); }};"))
        .collect::<Vec<_>>()
        .join(" ");
    let relation_limit = format!(
        "pub mod relation_limit {{ pub enum Left {{ A }} pub enum Right {{ B }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ {relations} }}; projections {{ companions; }}; }} }}"
    );
    assert_refusal(
        &refusal(relation_limit.as_str(), HarnessPosture::Available)?,
        "captured sequence carries more members than its declared magnitude of 64",
        RefusalClass::DeclarationNotRead,
        Observed::BoundExceeded,
        last_word(relation_limit.as_str(), "R64")?,
        SEQUENCE_LIMIT_REPAIR,
    )
}

#[test]
fn lawful_recipe_collection_maxima_are_admitted() -> Result<(), ()> {
    let enums = (0_usize..64)
        .map(|index| format!("pub enum V{index} {{ Only }}"))
        .collect::<Vec<_>>()
        .join(" ");
    let names = (0_usize..64)
        .map(|index| format!("V{index};"))
        .collect::<Vec<_>>()
        .join(" ");
    admitted(
        format!(
            "pub mod vocabulary_maximum {{ {enums} bake! {{ vocabularies {{ {names} }}; projections {{ companions; }}; }} }}"
        )
        .as_str(),
    )?;

    let relations = (0_usize..64)
        .map(|index| format!("R{index}(Left, Right) {{ (A, B); }};"))
        .collect::<Vec<_>>()
        .join(" ");
    admitted(
        format!(
            "pub mod relation_maximum {{ pub enum Left {{ A }} pub enum Right {{ B }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ {relations} }}; projections {{ companions; }}; }} }}"
        )
        .as_str(),
    )?;

    let left_variants = (0_usize..64)
        .map(|index| format!("L{index}"))
        .collect::<Vec<_>>()
        .join(", ");
    let rows = (0_usize..64)
        .flat_map(|left_index| {
            (0_usize..2).map(move |right_index| format!("(L{left_index}, R{right_index});"))
        })
        .collect::<Vec<_>>()
        .join(" ");
    admitted(
        format!(
            "pub mod row_maximum {{ pub enum Left {{ {left_variants} }} pub enum Right {{ R0, R1 }} bake! {{ vocabularies {{ Left; Right; }}; relations {{ links(Left, Right) {{ {rows} }}; }}; projections {{ companions; }}; }} }}"
        )
        .as_str(),
    )?;

    let records = (0_usize..16)
        .map(|index| format!("pub struct Record{index} {{ pub value: u16 }}"))
        .collect::<Vec<_>>()
        .join(" ");
    let codecs = codec_rows(16);
    admitted(
        format!(
            "pub mod codec_maximum {{ {records} bake! {{ codecs {{ {codecs} }}; projections {{ codec; }}; }} }}"
        )
        .as_str(),
    )
}

#[test]
fn codec_magnitude_refusal_points_at_the_first_excess_declaration() -> Result<(), ()> {
    let records = (0_usize..17)
        .map(|index| format!("pub struct Record{index} {{ pub value: u16 }}"))
        .collect::<Vec<_>>()
        .join(" ");
    let codecs = codec_rows(17);
    let source = format!(
        "pub mod codec_limit {{ {records} bake! {{ codecs {{ {codecs} }}; projections {{ codec; }}; }} }}"
    );
    assert_refusal(
        &refusal(source.as_str(), HarnessPosture::Available)?,
        "captured sequence carries more members than its declared magnitude of 16",
        RefusalClass::DeclarationNotRead,
        Observed::BoundExceeded,
        last_word(source.as_str(), "codec16")?,
        SEQUENCE_LIMIT_REPAIR,
    )
}

fn codec_rows(count: usize) -> String {
    (0_usize..count)
        .map(|index| {
            format!(
                "codec{index}(Record{index}) {{ direction(encode); refusal(Refusal{index}); assembly(assembled, total); members {{ value: u16 => count(required); }}; }};"
            )
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn fixed_typestate_and_configured_dispatch_names_collide_without_other_generated_names()
-> Result<(), ()> {
    let source = r"
pub mod collision {
    pub enum State { Closed, Open }
    pub enum Event { OpenDoor }
    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::open);
        };
        absence(refused);
        projections {
            dispatch(typestate);
            typestate(State);
        };
    }
}
";
    assert_refusal(
        &refusal(source, HarnessPosture::Available)?,
        "generated recipe name `typestate` is already occupied",
        RefusalClass::DeclarationNotRead,
        Observed::IdentityDisagreement,
        first_word(source, "typestate")?,
        DUPLICATE_REPAIR,
    )
}

#[test]
fn companion_and_codec_collisions_point_at_the_declaration_that_reuses_the_name() -> Result<(), ()>
{
    let companion = r"
pub mod companion_collision {
    pub enum State { Closed, Open }
    pub enum Event { OpenDoor }
    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Closed, OpenDoor) => Open with(crate::open);
        };
        absence(refused);
        projections {
            companions;
            dispatch(STATE_VARIANTS);
        };
    }
}
";
    assert_refusal(
        &refusal(companion, HarnessPosture::Available)?,
        "generated recipe name `STATE_VARIANTS` is already occupied",
        RefusalClass::DeclarationNotRead,
        Observed::IdentityDisagreement,
        last_word(companion, "STATE_VARIANTS")?,
        DUPLICATE_REPAIR,
    )?;

    let codec = r"
pub mod codec_collision {
    pub struct Ledger { pub count: u16 }
    pub struct Journal { pub count: u16 }
    bake! {
        codecs {
            ledger(Ledger) {
                direction(decode);
                refusal(SharedDecodeError);
                assembly(assembled, total);
                members { count: u16 => count(required); };
            };
            journal(Journal) {
                direction(decode);
                refusal(SharedDecodeError);
                assembly(assembled, total);
                members { count: u16 => count(required); };
            };
        };
        projections { codec; };
    }
}
";
    assert_refusal(
        &refusal(codec, HarnessPosture::Available)?,
        "generated recipe name `SharedDecodeError` is already occupied",
        RefusalClass::DeclarationNotRead,
        Observed::IdentityDisagreement,
        last_word(codec, "SharedDecodeError")?,
        DUPLICATE_REPAIR,
    )
}

#[test]
fn cross_family_type_collisions_point_at_the_second_generated_owner() -> Result<(), ()> {
    let codec = r"
pub mod combined {
    pub enum Left { A }
    pub enum Right { B }
    pub struct Ledger { pub count: u16 }
    bake! {
        vocabularies { Left; Right; };
        relations { policy(Left, Right) { (A, B); }; };
        codecs {
            ledger(Ledger) {
                direction(decode);
                refusal(policy);
                assembly(assembled, total);
                members { count: u16 => count(required); };
            };
        };
        projections { relation_tables { policy; }; codec; };
    }
}
";
    assert_refusal(
        &refusal(codec, HarnessPosture::Available)?,
        "generated recipe name `policy` is already occupied",
        RefusalClass::DeclarationNotRead,
        Observed::IdentityDisagreement,
        word_occurrence(codec, "policy", 1)?,
        DUPLICATE_REPAIR,
    )?;

    let network = r#"
pub mod combined {
    pub enum Left { A }
    pub enum Right { B }
    bake! {
        vocabularies { Left; Right; };
        relations { policy(Left, Right) { (A, B); }; };
        projections { relation_tables { policy; }; };
        evidence {
            network {
                harness = macroonz::harness,
                module = policy,
                namespace = "collision",
                nodes = [left, right],
                link forward = left to right,
                schedule quiet = [],
            };
        };
    }
}
"#;
    assert_refusal(
        &refusal(network, HarnessPosture::Available)?,
        "generated recipe name `policy` is already occupied",
        RefusalClass::DeclarationNotRead,
        Observed::IdentityDisagreement,
        last_word(network, "network")?,
        DUPLICATE_REPAIR,
    )
}
