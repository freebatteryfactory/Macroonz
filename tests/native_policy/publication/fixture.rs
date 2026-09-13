use super::types::{Example, Seat, Values};
use macroonz::compiler::stamp::{
    Fragment, Part, Pattern, PublicationGround, PublishedStamp, Seat as PatternSeat, Seating, Site,
    SiteRoot, Stamp, StampName, Visibility, planned,
};
use macroonz::compiler::{
    CrateBinding, Door, Expansion, GeneratedToken, GeneratedTree, OwnerIdentity, Producer,
    RenderError, Request, TextCapture, metavariable,
};
use macroonz::native_publication::{
    LandingBinding, PublicationBinding, PublicationLimits, PublicationPath,
};

pub(super) const FIRST: OwnerIdentity = OwnerIdentity {
    subject: "fixture.first",
    bytes: [1; 32],
};
pub(super) const SECOND: OwnerIdentity = OwnerIdentity {
    subject: "fixture.second",
    bytes: [2; 32],
};
pub(super) const LIMITS: PublicationLimits = PublicationLimits {
    files: 8,
    bytes: 65536,
};

const DOOR: Door = Door::declared(
    "fixture",
    "fixture.values",
    "fixture::values",
    CrateBinding::declared("adopter"),
    Producer {
        namespace: "fixture",
        name: "publication",
    },
);

pub(super) fn path(spelling: &str) -> Result<PublicationPath, String> {
    PublicationPath::informed(spelling).map_err(|error| format!("{error:?}"))
}

pub(super) fn inline_only() -> Result<Expansion<super::types::InlineExample>, String> {
    let capture = TextCapture::read("inline declaration").map_err(|error| error.to_string())?;
    Request::over(capture.input().clone(), Values([7, 9, 42]), &DOOR)
        .render(|_plan, out| {
            out.unit(
                super::types::InlineSeat,
                GeneratedTree::assembled(vec![GeneratedToken::alone(';')])?,
            )
        })
        .map_err(|error| error.summary().to_owned())
}

pub(super) fn bindings(first: &str, second: &str) -> Result<Vec<PublicationBinding>, String> {
    Ok(vec![
        PublicationBinding {
            address: SECOND,
            path: path(second)?,
        },
        PublicationBinding {
            address: FIRST,
            path: path(first)?,
        },
    ])
}

pub(super) fn landings() -> Result<Vec<LandingBinding>, String> {
    Ok(vec![
        LandingBinding {
            site: "beta".to_owned(),
            path: path("generated/beta.rs")?,
        },
        LandingBinding {
            site: "alpha".to_owned(),
            path: path("generated/alpha.rs")?,
        },
    ])
}

pub(super) fn stamp(name: &str, values: [u8; 2]) -> Result<Stamp, String> {
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
        "Declares one independently supplied value.",
        vec![Part::Seat(
            PatternSeat::declared("value", Seating::One(Fragment::Literal))
                .map_err(|error| error.to_string())?,
        )],
        GeneratedTree::assembled(body).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let sites = ["alpha", "beta"]
        .into_iter()
        .zip(values)
        .map(|(site, value)| {
            Site::declared(
                site,
                SiteRoot::spelled(vec!["crate".to_owned()])?,
                Visibility::Private,
                vec![GeneratedTree::assembled(vec![GeneratedToken::number(
                    u64::from(value),
                )])?],
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    Stamp::declared(
        StampName::declared(name).map_err(|error| error.to_string())?,
        pattern,
        sites,
    )
    .map_err(|error| error.to_string())
}

pub(super) fn expanded(values: [u8; 3]) -> Result<(Expansion<Example>, PublishedStamp), String> {
    expanded_with_stamp(values, &stamp("declared_value", [values[0], values[1]])?)
}

pub(super) fn expanded_with_stamp(
    values: [u8; 3],
    declaration: &Stamp,
) -> Result<(Expansion<Example>, PublishedStamp), String> {
    let capture = TextCapture::read("publication values").map_err(|error| error.to_string())?;
    let [_, _, other] = values;
    let mut publication = None;
    let mut refusal = None;
    let expansion = Request::<Example>::over(capture.input().clone(), Values(values), &DOOR)
        .publishing_at(Seat::Definition, FIRST)
        .publishing_at(Seat::Other, SECOND)
        .render(|plan, out| {
            let rendered = planned(plan, Seat::Definition).and_then(|decision| {
                PublishedStamp::rendered(
                    &decision,
                    declaration,
                    PublicationGround::CrossFileArtifact,
                )
            });
            let artifact = match rendered {
                Ok(artifact) => artifact,
                Err(error) => {
                    refusal = Some(error);
                    return Err(RenderError::NothingRendered);
                }
            };
            out.unit(Seat::Definition, artifact.definition().clone())?;
            out.unit(
                Seat::Other,
                GeneratedTree::assembled(vec![
                    GeneratedToken::word("pub"),
                    GeneratedToken::word("const"),
                    GeneratedToken::word("OTHER"),
                    GeneratedToken::alone(':'),
                    GeneratedToken::word("u8"),
                    GeneratedToken::alone('='),
                    GeneratedToken::number(u64::from(other)),
                    GeneratedToken::alone(';'),
                ])?,
            )?;
            publication = Some(artifact);
            Ok(())
        });
    if let Some(error) = refusal {
        return Err(error.to_string());
    }
    Ok((
        expansion.map_err(|error| error.summary().to_owned())?,
        publication.ok_or("stamp absent")?,
    ))
}
