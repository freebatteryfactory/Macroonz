//! Structural mutation producers delivered through the public compiler door.

use macroonz_compiler::codec::{
    AssemblyPosture, Cardinality, CodecAssembly, CodecContent, CodecDirection, CodecMember,
    CodecMemberShape, CodecPlacement, CodecShape, CodecTypePath, PathRooting,
};
use macroonz_compiler::descriptor::mutation::{self, Declaration, Surface};
use macroonz_compiler::descriptor::{Grammar, door};
use macroonz_compiler::recipe::HarnessPosture;
use macroonz_compiler::{Bounded, CrateBinding, Door, Producer, SpanHandle, TextCapture};

const DOOR: Door = Door::declared(
    "structural-crossing",
    "structural-crossing.grammar",
    "structural_crossing::mutation",
    CrateBinding::declared("bakery"),
    Producer {
        namespace: "structural-crossing",
        name: "mutation",
    },
);
const GRAMMAR: Grammar = Grammar {
    attribute: "mutations",
};

const RECIPE: &str = r"
pub mod machine {
    pub enum State { Shut, Open, Locked }
    pub enum Event { OpenDoor, ShutDoor }
    bake! {
        vocabularies { State; Event; };
        transitions(State, Event) {
            (Shut, OpenDoor) => Open with(result) { *log += 3; Ok(result) };
            (Open, ShutDoor) => Shut with(result) { *log += 7; Ok(result) };
        };
        absence(refused);
        projections {
            companions;
            dispatch(current, event) {
                pub fn apply(log: &mut u8, current: State, event: Event)
                    -> Result<State, TransitionRefusal>;
            };
        };
    }
}
";

pub(super) fn producer() -> Result<String, String> {
    let item = TextCapture::read(RECIPE).map_err(debug)?;
    let mut source = String::from("#![forbid(unsafe_code)]\n#![deny(warnings)]\n");
    for (module, permit) in [
        ("targets", mutation::TRANSITION_TARGET_FAMILY),
        ("effects", mutation::EFFECT_BINDING_FAMILY),
        ("denied", mutation::DECLARED_ORDER_FAMILY),
    ] {
        let body = body(module, permit)?;
        let declaration = declaration(&body)?;
        let surface = if module == "effects" {
            mutation::completed_from_effect_bindings(
                declaration,
                item.input(),
                HarnessPosture::Unavailable,
                0,
                &DOOR,
            )
        } else {
            mutation::completed_from_transition_targets(
                declaration,
                item.input(),
                HarnessPosture::Unavailable,
                0,
                &DOOR,
            )
        }
        .map_err(debug)?;
        source.push_str(&carrier(&body, &item, surface)?);
    }
    let body = body("codec", mutation::DECLARED_ORDER_FAMILY)?;
    let codec_item =
        TextCapture::read("pub struct Record { first: u8, second: u8 }").map_err(debug)?;
    let surface =
        mutation::completed_from_codec_order(declaration(&body)?, &codec()?).map_err(debug)?;
    source.push_str(&carrier(&body, &codec_item, surface)?);
    Ok(source)
}

fn body(module: &str, permit: &str) -> Result<TextCapture, String> {
    TextCapture::read(&format!(
        r#"
        module = {module}, refusal = SurfaceRefusal, support = {module}_support,
        family = named("crossing", "{module}"),
        point = named("crossing", "selected"), fact = named("crossing", "fact"),
        map named("crossing", "fact") = named("crossing", "claim"),
        permit named("crossing", "claim") = ["{permit}"],
    "#
    ))
    .map_err(debug)
}

fn declaration(body: &TextCapture) -> Result<Declaration, String> {
    let trees = body.input().trees().iter().collect::<Vec<_>>();
    mutation::captured(&trees, SpanHandle::at(0), GRAMMAR).map_err(debug)
}

fn carrier(body: &TextCapture, item: &TextCapture, surface: Surface) -> Result<String, String> {
    let delivered =
        door::mutations_from_surface(body.input(), item.input(), surface, GRAMMAR, &DOOR)
            .map_err(debug)?;
    delivered
        .emit()
        .tokens()
        .map(macroonz_compiler::GeneratedTree::inspected)
        .ok_or_else(|| "the carrier emitted no declaration-site source".to_owned())
}

fn codec() -> Result<CodecContent, String> {
    let path = |name: &str| {
        CodecTypePath::spelled(PathRooting::InScope, vec![name.to_owned()]).map_err(debug)
    };
    let members = ["first", "second"]
        .into_iter()
        .map(|name| {
            CodecMember::declared(
                name,
                path("u8")?,
                CodecMemberShape::Count,
                Cardinality::Required,
            )
            .map_err(debug)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(CodecContent {
        shape: CodecShape::declared(
            path("Record")?,
            "DecodeRefusal",
            CodecAssembly::stated("assembled", AssemblyPosture::Total).map_err(debug)?,
            members,
        )
        .map_err(debug)?,
        direction: CodecDirection::RoundTrip,
        placement: CodecPlacement::AtDeclarationSite,
        schema: None,
        byte_role: None,
        assumptions: Bounded::empty(),
    })
}

fn debug(cause: impl core::fmt::Debug) -> String {
    format!("{cause:?}")
}
