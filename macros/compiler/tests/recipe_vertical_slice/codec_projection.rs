//! The codec projection observed through the recipe road: one codec declaration reuses the codec owner and moves recipe identity, and generated codec surfaces refuse collisions before rendering.

use super::support::{CODEC_RECIPE, bake, refusal_summary};
use macroonz_compiler::CanonicalContent;
use macroonz_compiler::codec::{Cardinality, CodecMemberShape};

#[test]
fn codec_projection_reuses_the_codec_owner_and_moves_recipe_identity() -> Result<(), String> {
    let baseline = bake(CODEC_RECIPE).map_err(|()| {
        refusal_summary(CODEC_RECIPE)
            .unwrap_or_else(|()| "the codec recipe refused without a summary".to_owned())
    })?;
    let recipe = baseline.projection().plan().content();
    let codec = recipe
        .codec("ledger")
        .ok_or_else(|| "the ledger codec is absent".to_owned())?;
    assert_eq!(codec.name(), "ledger");
    assert_eq!(recipe.codecs().count(), 1);
    let member_contracts = codec
        .content()
        .shape
        .members()
        .map(|member| (member.shape(), member.cardinality()))
        .collect::<Vec<_>>();
    assert_eq!(
        member_contracts,
        [
            (CodecMemberShape::Count, Cardinality::Required),
            (CodecMemberShape::Bytes, Cardinality::Required),
            (CodecMemberShape::Text, Cardinality::Optional),
            (CodecMemberShape::ClosedChoice, Cardinality::Repeated),
            (CodecMemberShape::Nested, Cardinality::Required),
        ]
    );
    let emitted = baseline
        .emit()
        .tokens()
        .ok_or_else(|| "the codec recipe emitted no tokens".to_owned())?
        .inspected();
    assert!(emitted.contains("encode_canonical"));
    assert!(emitted.contains("decode_canonical"));

    let encode_only = bake(&CODEC_RECIPE.replace("round_trip", "encode"))
        .map_err(|()| "the encode-only codec refused".to_owned())?;
    assert_ne!(
        recipe.canonical_content_bytes(),
        encode_only
            .projection()
            .plan()
            .content()
            .canonical_content_bytes()
    );

    let empty = CODEC_RECIPE
        .replace("count: u16 => count(required);", "")
        .replace("payload: Payload => bytes(required);", "")
        .replace("label: String => text(optional);", "")
        .replace("modes: Choice => closed_choice(repeated);", "")
        .replace("child: Nested => nested(required);", "");
    let empty = refusal_summary(empty.as_str())
        .map_err(|()| "a codec without members was accepted".to_owned())?;
    assert!(empty.contains("codec `ledger` was refused"));
    assert!(empty.contains("no member"));

    let absent = CODEC_RECIPE.replace(
        "        codecs {\n            ledger(Ledger) {\n                direction(round_trip);\n                refusal(LedgerDecodeError);\n                assembly(assembled, total);\n                members {\n                    count: u16 => count(required);\n                    payload: Payload => bytes(required);\n                    label: String => text(optional);\n                    modes: Choice => closed_choice(repeated);\n                    child: Nested => nested(required);\n                };\n            };\n        };\n",
        "",
    );
    let absent = refusal_summary(absent.as_str())
        .map_err(|()| "the codec projection without codec content was accepted".to_owned())?;
    assert!(absent.contains("requires at least one existing-owner codec declaration"));

    let missing_owner = CODEC_RECIPE.replace("pub struct Ledger", "pub struct Journal");
    let missing_owner = refusal_summary(missing_owner.as_str())
        .map_err(|()| "a codec targeting an absent record was accepted".to_owned())?;
    assert!(missing_owner.contains("owner `Ledger` is not an authored record struct"));

    let non_record = CODEC_RECIPE.replace(
        "    pub struct Ledger {\n        pub count: u16,\n        pub payload: Payload,\n        pub label: Option<String>,\n        pub modes: Vec<Choice>,\n        pub child: Nested,\n    }",
        "    pub enum Ledger { Empty }",
    );
    let non_record = refusal_summary(non_record.as_str())
        .map_err(|()| "a codec targeting a non-record item was accepted".to_owned())?;
    assert!(non_record.contains("owner `Ledger` is not an authored record struct"));
    Ok(())
}

#[test]
fn codec_surfaces_refuse_generated_type_and_method_collisions_before_rendering()
-> Result<(), String> {
    let repeated_refusal = CODEC_RECIPE.replace(
        "    impl Ledger {",
        "    pub struct Journal { pub count: u16 }\n\n    impl Journal {\n        pub const fn assembled(count: u16) -> Self { Self { count } }\n    }\n\n    impl Ledger {",
    ).replace(
        "        codecs {",
        "        codecs {\n            journal(Journal) {\n                direction(decode);\n                refusal(LedgerDecodeError);\n                assembly(assembled, total);\n                members {\n                    count: u16 => count(required);\n                };\n            };",
    );
    let repeated_refusal = refusal_summary(repeated_refusal.as_str())
        .map_err(|()| "two codec surfaces emitted one refusal name".to_owned())?;
    assert!(repeated_refusal.contains("generated recipe name `LedgerDecodeError`"));

    let repeated_encode = CODEC_RECIPE.replace(
        "        codecs {",
        "        codecs {\n            alternate(Ledger) {\n                direction(encode);\n                refusal(AlternateDecodeError);\n                assembly(assembled, total);\n                members {\n                    count: u16 => count(required);\n                };\n            };",
    );
    let repeated_encode = refusal_summary(repeated_encode.as_str())
        .map_err(|()| "two codec surfaces emitted one owner encode road".to_owned())?;
    assert!(repeated_encode.contains("generated recipe name `encode_canonical`"));
    Ok(())
}
