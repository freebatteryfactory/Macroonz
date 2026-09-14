use super::stamp::stamp;
use super::types::{PublicationSeat, PublishedValues, Values};
use macroonz::compiler::stamp::{PublicationGround, PublishedStamp, planned};
use macroonz::compiler::{
    CrateBinding, Door, OwnerIdentity, Producer, RenderError, Request, TextCapture,
};
use macroonz::native_publication::{
    LandingBinding, Publication, PublicationBinding, PublicationLimits, PublicationPath,
};

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
const ADDRESS: OwnerIdentity = OwnerIdentity {
    subject: "example.values.address",
    bytes: [1; 32],
};

pub(super) const LIMITS: PublicationLimits = PublicationLimits {
    files: 3,
    bytes: 65_536,
};

pub(super) fn publication(values: [u8; 2]) -> Result<Publication<PublishedValues>, String> {
    let [first, second] = values;
    let capture = TextCapture::read(&format!("values({first}, {second})"))
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
    let landings = artifact
        .landings()
        .iter()
        .map(|landing| {
            let site = landing.site();
            Ok(LandingBinding {
                site: site.to_owned(),
                path: path(&format!("{site}.rs"))?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Publication::declared(
        expansion,
        &[PublicationBinding {
            address: ADDRESS,
            path: path("definition.rs")?,
        }],
        LIMITS,
    )
    .and_then(|inventory| inventory.with_stamp(artifact, &landings))
    .map_err(|error| error.to_string())
}

fn path(spelling: &str) -> Result<PublicationPath, String> {
    PublicationPath::informed(spelling).map_err(|error| error.to_string())
}
