//! Executable relation posture choices and their refusals through the callable facade.

use macroonz::compiler::recipe::{HarnessPosture, RecipeBake};
use macroonz::compiler::{CrateBinding, Door, Producer, TextCapture};

const DOOR: Door = Door::declared(
    "recipe-postures",
    "recipe-postures.recipe",
    "recipe-postures::bake",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "recipe-postures",
        name: "compiler",
    },
);

fn bake(rows: &str, posture: &str) -> Result<RecipeBake, String> {
    let source = format!(
        "pub mod graph {{
            pub enum Point {{ A, B }}
            bake! {{
                vocabularies {{ Point; }};
                relations {{ edges(Point, Point) {{ {rows} }}; }};
                postures {{ edges {{ {posture} }}; }};
                projections {{ companions; typestate; }};
            }}
        }}"
    );
    let captured = TextCapture::read(&source).map_err(|error| error.to_string())?;
    macroonz::compiler::recipe::bake(captured.input(), HarnessPosture::Unavailable, &DOOR)
        .map_err(|diagnostic| diagnostic.summary().to_owned())
}

fn admitted_choices() -> Result<(), String> {
    for (rows, posture) in [
        ("", "empty(allowed);"),
        ("(A, B);", "empty(refused);"),
        ("(A, B); (A, B);", "repetition(allowed);"),
        ("(A, B);", "repetition(refused);"),
        ("(A, B);", "membership(open, open);"),
        ("(A, B);", "membership(closed, closed);"),
        ("(A, B);", "membership(open, closed);"),
        ("(A, B);", "membership(closed, open);"),
        ("(A, B);", "completeness(partial, partial);"),
        ("(A, B); (B, A);", "completeness(total, total);"),
        ("(A, A); (B, A);", "completeness(total, partial);"),
        ("(A, A); (A, B);", "completeness(partial, total);"),
        ("(A, B);", "density(sparse);"),
        ("(A, A); (A, B); (B, A); (B, B);", "density(dense);"),
        ("(A, B);", "absence(allowed);"),
        ("(A, B);", "absence(refused);"),
        ("(A, A);", "self_relation(allowed);"),
        ("(A, B);", "self_relation(refused);"),
        ("(A, B); (B, A);", "cycle(allowed);"),
        ("(A, B);", "cycle(refused);"),
    ] {
        let baked = bake(rows, posture)?;
        assert!(baked.emit().tokens().is_some());
    }
    let repeated = bake("(B, A); (A, B); (B, A);", "repetition(allowed);")?;
    let edges = repeated
        .projection()
        .plan()
        .content()
        .relation("edges")
        .ok_or("missing edges")?;
    assert_eq!(
        edges
            .rows()
            .map(|row| (row.left(), row.right()))
            .collect::<Vec<_>>(),
        [("B", "A"), ("A", "B"), ("B", "A")],
    );
    Ok(())
}

fn refused_choices() -> Result<(), String> {
    for (rows, posture, expected) in [
        ("", "empty(refused);", "requires empty"),
        ("(A, B); (A, B);", "repetition(refused);", "more than once"),
        (
            "(A, B);",
            "completeness(total, partial);",
            "left completeness",
        ),
        (
            "(A, B);",
            "completeness(partial, total);",
            "right completeness",
        ),
        ("(A, B);", "density(dense);", "requires density"),
        ("(A, A);", "self_relation(refused);", "self relation"),
        ("(A, B); (B, A);", "cycle(refused);", "requires cycle"),
        (
            "(Outside, B);",
            "membership(open, open);",
            "undeclared `Point` member `Outside`",
        ),
        (
            "(A, Outside);",
            "membership(closed, closed);",
            "undeclared `Point` member `Outside`",
        ),
        (
            "(A, B);",
            "empty(allowed); empty(refused);",
            "question `empty` more than once",
        ),
    ] {
        let diagnostic = bake(rows, posture)
            .err()
            .ok_or("a contradictory declaration was admitted")?;
        assert!(diagnostic.contains(expected), "{posture}: {diagnostic}");
    }
    Ok(())
}

fn main() -> Result<(), String> {
    admitted_choices()?;
    refused_choices()
}

#[test]
fn documented_postures_admit_lawful_rows_and_refuse_contradictions() -> Result<(), String> {
    main()
}
