use super::types::Values;
use macroonz::compiler::stamp::{
    Fragment, Part, Pattern, Seat, Seating, Site, SiteRoot, Stamp, StampError, StampName,
    Visibility,
};
use macroonz::compiler::{GeneratedToken, GeneratedTree, metavariable};

pub(super) fn stamp(values: Values) -> Result<Stamp, StampError> {
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
