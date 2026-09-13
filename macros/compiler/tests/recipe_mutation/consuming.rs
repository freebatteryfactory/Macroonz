//! Contextual effect refusal and invariant re-admission through actual consuming source.

use super::fixture::{DOOR, captured, declaration, ordinary, source};
use super::support::observe_rustc;
use macroonz_compiler::descriptor::mutation::{
    EFFECT_BINDING_FAMILY, RecipeMutationError, completed_from_effect_bindings,
};
use macroonz_compiler::recipe::{HarnessPosture, RecipeEditError};

const SUBJECT: &str = r"
pub mod parcel {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Phase { Ready, Sent }
    pub enum Event { Send, Mark }
    bake! {
        vocabularies { Phase; Event; };
        transitions(Phase, Event) {
            (Ready, Send) => Sent with(destination) {
                let _: u8 = item;
                resource.phase = destination;
                Ok::<(), crate::Error>(())
            };
            (Sent, Mark) => Ready with(result) {
                let _: u16 = item;
                resource.phase = result;
                Ok::<(), crate::Error>(())
            };
        };
        absence(refused);
        projections {
            typestate(Phase) {
                wrapper(Package);
                resource(resource: crate::Value);
                runtime(engine: &mut ());
                refusal(crate::Error);
                validate(crate::inspect);
                methods { Send => send(item: u8); Mark => mark(item: u16); };
            };
        };
    }
}
";

const OBSERVER: &str = r#"
#[derive(Debug)]
pub struct Value { phase: parcel::Phase }
#[derive(Debug)]
pub struct Error;
pub fn inspect(_: &mut (), value: &Value, phase: parcel::Phase) -> Result<(), Error> {
    if value.phase == phase { Ok(()) } else { Err(Error) }
}
fn main() {
    use parcel::baked::typestate::{Package, Ready};
    let mut engine = ();
    let ready = Package::<Ready>::restore(&mut engine, Value { phase: parcel::Phase::Ready })
        .expect("valid initial phase");
    let sent = ready.send(&mut engine, 1).expect("first consuming step");
    assert_eq!(sent.resource().phase, parcel::Phase::Sent);
    let ready = sent.mark(&mut engine, 257).expect("second consuming step");
    assert_eq!(ready.into_resource().phase, parcel::Phase::Ready);
}
"#;

#[test]
fn an_effect_with_another_payload_type_is_unviable_instead_of_behaviorally_killed()
-> Result<(), String> {
    let read = captured(SUBJECT)?;
    let surface = completed_from_effect_bindings(
        declaration(EFFECT_BINDING_FAMILY)?,
        read.input(),
        HarnessPosture::Unavailable,
        0,
        &DOOR,
    )
    .map_err(|refusal| refusal.to_string())?;
    let baseline = format!("{}\n{OBSERVER}", source(surface.site().production())?);
    let compiled = observe_rustc("consuming_effect_baseline", &baseline, &[])?;
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let [alternative] = surface.site().alternatives() else {
        return Err("the declared sibling effect must remain materialized".to_owned());
    };
    let changed = format!("{}\n{OBSERVER}", source(alternative.meaning())?);
    let refused = observe_rustc("consuming_effect_unviable", &changed, &[])?;
    assert!(!refused.status.success());
    let diagnostic = String::from_utf8_lossy(&refused.stderr);
    assert!(diagnostic.contains("E0308"), "{diagnostic}");
    Ok(())
}

#[test]
fn a_substitution_cannot_skip_the_existing_consuming_binding_guard() -> Result<(), String> {
    let lawful = SUBJECT
        .replace("Mark => mark(item: u16)", "Mark => mark(other: u16)")
        .replace("let _: u16 = item", "let _: u16 = other")
        .replace("with(result)", "with(item)")
        .replace("resource.phase = result", "resource.phase = item");
    let ordinary = ordinary(&lawful)?;
    let rendered = ordinary
        .emit()
        .tokens()
        .ok_or("ordinary source required")?
        .inspected();
    let compiled = observe_rustc(
        "consuming_binding_baseline",
        &format!("{rendered}\n{OBSERVER}"),
        &[],
    )?;
    assert!(
        compiled.status.success(),
        "{}",
        String::from_utf8_lossy(&compiled.stderr)
    );
    let read = captured(&lawful)?;
    let refused = completed_from_effect_bindings(
        declaration(EFFECT_BINDING_FAMILY)?,
        read.input(),
        HarnessPosture::Unavailable,
        0,
        &DOOR,
    );
    let Err(RecipeMutationError::Edit(RecipeEditError::Compiler(refusal))) = refused else {
        return Err("the changed target binding must meet the existing guard".to_owned());
    };
    assert!(
        refusal
            .summary()
            .contains("target binding distinct from runtime, resource and payload")
    );
    Ok(())
}
