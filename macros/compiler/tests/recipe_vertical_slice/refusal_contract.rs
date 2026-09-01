//! Recipe-envelope refusals observed through the callable text-capture road.

use macroonz_compiler::recipe::HarnessPosture;
use macroonz_compiler::{
    CrateBinding, Diagnostic, Door, Observed, Phase, Producer, Site, TextCapture,
};

const DOOR: Door = Door::declared(
    "recipe-refusal-crossing",
    "recipe-refusal-crossing.grammar",
    "recipe-refusal-crossing::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "recipe-refusal-crossing",
        name: "recipe",
    },
);

const REPAIR: &str = "write one inline module whose final bake declaration names authored enum vocabularies, checked transitions, one absence posture, and every requested projection";

fn refusal(source: &str) -> Result<Diagnostic, ()> {
    let read = TextCapture::read(source).map_err(|_| ())?;
    macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .err()
        .ok_or(())
}

fn assert_envelope_refusal(
    source: &str,
    issue: &str,
    expected_site: fn(Site) -> bool,
) -> Result<(), ()> {
    let refused = refusal(source)?;
    assert_eq!(refused.phase(), Phase::Capture);
    assert_eq!(refused.observed(), Observed::SeatAbsent);
    assert!(refused.summary().contains(issue), "{}", refused.summary());
    assert!(expected_site(refused.site()), "{:?}", refused.site());
    let [repair] = refused.repairs() else {
        return Err(());
    };
    assert_eq!(repair.description.shown(), REPAIR);
    Ok(())
}

fn whole_declaration(site: Site) -> bool {
    site == Site::WholeDeclaration
}

fn captured_token(site: Site) -> bool {
    site.token().is_some() && site.coordinate().is_some()
}

#[test]
fn an_empty_recipe_refuses_as_one_missing_inline_module() -> Result<(), ()> {
    assert_envelope_refusal(
        "",
        "a recipe must contain exactly one inline Rust module",
        whole_declaration,
    )
}

#[test]
fn a_non_module_recipe_refuses_at_its_captured_token() -> Result<(), ()> {
    assert_envelope_refusal(
        "pub struct NotARecipe;",
        "a recipe must contain exactly one inline Rust module",
        captured_token,
    )
}

#[test]
fn an_inline_module_without_bake_refuses_at_its_last_authored_token() -> Result<(), ()> {
    assert_envelope_refusal(
        "pub mod door { pub enum State { Closed } }",
        "the recipe module must end with exactly one `bake!` declaration",
        captured_token,
    )
}

#[test]
fn two_bake_declarations_refuse_at_the_nonfinal_suffix() -> Result<(), ()> {
    assert_envelope_refusal(
        "pub mod door { bake! {}; bake! {}; }",
        "the recipe module must end with exactly one `bake!` declaration",
        captured_token,
    )
}

#[test]
fn an_authored_item_after_bake_refuses_at_that_item() -> Result<(), ()> {
    assert_envelope_refusal(
        "pub mod door { bake! {}; pub struct After; }",
        "the recipe module must end with exactly one `bake!` declaration",
        captured_token,
    )
}
