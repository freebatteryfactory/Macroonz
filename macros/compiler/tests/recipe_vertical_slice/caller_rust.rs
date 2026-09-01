//! Caller-owned ordinary Rust preserved through the recipe boundary.

use macroonz_compiler::recipe::HarnessPosture;
use macroonz_compiler::{CrateBinding, Door, Producer, TextCapture};

const DOOR: Door = Door::declared(
    "recipe-caller-rust-crossing",
    "recipe-caller-rust-crossing.grammar",
    "recipe-caller-rust-crossing::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "recipe-caller-rust-crossing",
        name: "recipe",
    },
);

const UNICODE_RECIPE: &str = r"
pub mod boulangerie {
    pub enum État {
        Fermé,
        Ouvert,
    }

    pub enum Événement {
        Ouvrir,
    }

    bake! {
        vocabularies(État, Événement);
        transitions {
            (Fermé, Ouvrir) => Ouvert with(crate::ouvrir);
        };
        absence(refused);
        projections {
            companions;
            dispatch(appliquer);
        };
    }
}
";

#[test]
fn lawful_unicode_identifiers_survive_recipe_capture_and_projection() -> Result<(), ()> {
    let read = TextCapture::read(UNICODE_RECIPE).map_err(|_| ())?;
    let baked = macroonz_compiler::recipe::bake(read.input(), HarnessPosture::Available, &DOOR)
        .map_err(|_| ())?;
    let emitted = baked
        .emit()
        .tokens()
        .map(macroonz_compiler::GeneratedTree::inspected)
        .ok_or(())?;

    for identifier in [
        "État",
        "Événement",
        "Fermé",
        "Ouvert",
        "Ouvrir",
        "appliquer",
    ] {
        assert!(
            emitted.contains(identifier),
            "{identifier} was not preserved"
        );
    }
    Ok(())
}
