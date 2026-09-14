//! A caller-owned kind renders an addressed definition and the sites that adopt it.

use macroonz_compiler::stamp::{
    Fragment, Part, Pattern, PublicationGround, PublishedStamp, Seat, Seating, Site, SiteRoot,
    Stamp, StampError, StampName, Visibility, planned,
};
use macroonz_compiler::{
    CanonicalContent, CrateBinding, Destination, Door, GeneratedToken, GeneratedTree, Kind,
    NoQuestions, OwnerIdentity, Producer, RenderError, Request, Role, TextCapture, metavariable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Values([u8; 2]);

impl CanonicalContent for Values {
    fn encode_content_into(&self, into: &mut Vec<u8>) {
        into.extend_from_slice(&self.0);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublicationSeat {
    Definition,
}

impl Role for PublicationSeat {
    const ALL: &'static [Self] = &[Self::Definition];

    fn name(self) -> &'static str {
        "definition"
    }

    fn destination(self) -> Destination {
        Destination::PublicationArtifact
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PublishedValues;

impl Kind for PublishedValues {
    const NAME: &'static str = "example.published-values";
    type Content = Values;
    type Role = PublicationSeat;
    type Question = NoQuestions;
}

const DOOR: Door = Door::declared(
    "example",
    "example.values",
    "example::published_values",
    CrateBinding::declared("consumer"),
    Producer {
        namespace: "example",
        name: "published-values",
    },
);

// The caller's logical address is an input, not a filesystem path or a write receipt.
const ADDRESS: OwnerIdentity = OwnerIdentity {
    subject: "example.values.address",
    bytes: [1; 32],
};

fn stamp(values: Values) -> Result<Stamp, StampError> {
    let mut body = vec![
        GeneratedToken::word("pub"),
        GeneratedToken::word("const"),
        GeneratedToken::word("VALUE"),
        GeneratedToken::alone(':'),
        GeneratedToken::word("u8"),
        GeneratedToken::alone('='),
    ];
    body.extend(metavariable("value"));
    body.push(GeneratedToken::alone(';'));
    let pattern = Pattern::declared(
        "Declares the value supplied by this site.",
        vec![Part::Seat(Seat::declared(
            "value",
            Seating::One(Fragment::Literal),
        )?)],
        GeneratedTree::assembled(body)?,
    )?;
    let root = SiteRoot::spelled(vec!["crate".to_owned()])?;
    let sites = ["first", "second"]
        .into_iter()
        .zip(values.0)
        .map(|(site, value)| {
            Site::declared(
                site,
                root.clone(),
                Visibility::Private,
                vec![GeneratedTree::assembled(vec![GeneratedToken::number(
                    u64::from(value),
                )])?],
            )
        })
        .collect::<Result<Vec<_>, StampError>>()?;
    Stamp::declared(StampName::declared("declared_value")?, pattern, sites)
}

/// Renders a sealed definition and its explicitly named adoption sites as source material.
pub(crate) fn generated_source(values: [u8; 2]) -> Result<String, String> {
    let capture = TextCapture::read(&format!("values({}, {})", values[0], values[1]))
        .map_err(|error| error.to_string())?;
    let mut publication = None;
    let mut stamp_refusal = None;
    let expansion =
        Request::<PublishedValues>::over(capture.input().clone(), Values(values), &DOOR)
            .publishing_at(PublicationSeat::Definition, ADDRESS)
            .render(|plan, out| {
                let rendered = planned(plan, PublicationSeat::Definition).and_then(|decision| {
                    PublishedStamp::rendered(
                        &decision,
                        &stamp(*plan.content())?,
                        PublicationGround::CrossFileArtifact,
                    )
                });
                let artifact = match rendered {
                    Ok(artifact) => artifact,
                    Err(refusal) => {
                        // Keep the actual stamp cause while the request refuses absent output.
                        stamp_refusal = Some(refusal);
                        return Err(RenderError::NothingRendered);
                    }
                };
                out.unit(PublicationSeat::Definition, artifact.definition().clone())?;
                publication = Some(artifact);
                Ok(())
            });
    if let Some(refusal) = stamp_refusal {
        return Err(refusal.to_string());
    }
    let expansion = expansion.map_err(|diagnostic| diagnostic.summary().to_owned())?;
    let artifact = publication.ok_or("the renderer returned no publication material")?;
    let mut units = expansion.published();
    let definition = units.next().ok_or("the definition was not delivered")?;
    assert!(units.next().is_none());
    assert_eq!(definition.address(), Some(ADDRESS));
    assert_eq!(definition.semantic_key(), artifact.record().unit());
    assert_eq!(definition.bytes(), artifact.definition().canonical_bytes());
    assert_eq!(
        definition.digest(),
        definition.digest_under(artifact.record().staged())
    );
    let mut source = definition.tree().inspected();
    for landing in artifact.landings() {
        source.push_str("\nmod ");
        source.push_str(landing.site());
        source.push_str(" { ");
        source.push_str(&landing.invocation().inspected());
        source.push_str(" }\n");
    }
    Ok(source)
}

pub(crate) fn main() -> Result<(), String> {
    let _source_for_the_publication_actor = generated_source([7, 9])?;
    Ok(())
}
