//! The recipe subject's independent repeatability and complete-output observation.

use macroonz_compiler::recipe::{HarnessPosture, bake};
use macroonz_compiler::{CrateBinding, Door, Producer, TextCapture};

pub(super) const INPUT_LIMIT: usize = 2_048;

const DOOR: Door = Door::declared(
    "grammar-campaign",
    "grammar-campaign.recipe",
    "grammar-campaign::recipe",
    CrateBinding::declared("macroonz"),
    Producer {
        namespace: "grammar-campaign",
        name: "recipe",
    },
);

#[derive(Debug, PartialEq, Eq)]
pub(super) enum Outcome {
    Baked(Vec<u8>),
    Refused(String),
}

pub(super) fn observe(input: &[u8]) -> Result<Outcome, &'static str> {
    let Ok(source) = core::str::from_utf8(input) else {
        return Ok(Outcome::Refused("non-UTF-8 input".to_owned()));
    };
    let capture = match TextCapture::read(source) {
        Ok(capture) => capture,
        Err(refusal) => return Ok(Outcome::Refused(format!("text: {refusal}"))),
    };
    match bake(capture.input(), HarnessPosture::Available, &DOOR) {
        Ok(expansion) => {
            let tokens = expansion
                .emit()
                .tokens()
                .ok_or("successful recipe has no output")?;
            let bytes = tokens.canonical_bytes();
            if bytes.is_empty() {
                return Err("successful recipe has empty output");
            }
            Ok(Outcome::Baked(bytes))
        }
        Err(refusal) => Ok(Outcome::Refused(format!("recipe: {refusal:?}"))),
    }
}
