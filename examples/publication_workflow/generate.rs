use super::types::{Definition, PublishedValue, Value};
use macroonz::compiler::{
    CrateBinding, Door, GeneratedToken, GeneratedTree, OwnerIdentity, Producer, Request,
    TextCapture,
};
use macroonz::native_publication::{
    Publication, PublicationBinding, PublicationLimits, PublicationPath,
};

const DOOR: Door = Door::declared(
    "example",
    "example.value",
    "example::published_value",
    CrateBinding::declared("adopter"),
    Producer {
        namespace: "example",
        name: "published-value",
    },
);
const ADDRESS: OwnerIdentity = OwnerIdentity {
    subject: "example.value",
    bytes: [1; 32],
};

pub(super) fn publication(value: u64) -> Result<Publication<PublishedValue>, String> {
    let capture =
        TextCapture::read(&format!("value({value})")).map_err(|error| error.to_string())?;
    let expansion = Request::<PublishedValue>::over(capture.input().clone(), Value(value), &DOOR)
        .publishing_at(Definition, ADDRESS)
        .render(|plan, output| {
            output.unit(
                Definition,
                GeneratedTree::assembled(vec![
                    GeneratedToken::word("pub"),
                    GeneratedToken::word("const"),
                    GeneratedToken::word("VALUE"),
                    GeneratedToken::alone(':'),
                    GeneratedToken::word("u64"),
                    GeneratedToken::alone('='),
                    GeneratedToken::number(plan.content().0),
                    GeneratedToken::alone(';'),
                ])?,
            )
        })
        .map_err(|error| error.summary().to_owned())?;
    Publication::declared(
        expansion,
        &[PublicationBinding {
            address: ADDRESS,
            path: PublicationPath::informed("value.rs").map_err(|error| error.to_string())?,
        }],
        PublicationLimits {
            files: 1,
            bytes: 65_536,
        },
    )
    .map_err(|error| error.to_string())
}
