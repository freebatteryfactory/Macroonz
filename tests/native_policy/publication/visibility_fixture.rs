use super::fixture::{LIMITS, bindings, expanded_with_stamp, path};
use super::types::Example;
use macroonz::compiler::stamp::{
    DECLARED_REACH, Fragment, Part, Pattern, Seat, Seating, Site, SiteRoot, Stamp, StampName,
    TRANSPORTED_REACH, Visibility,
};
use macroonz::compiler::{GeneratedDelimiter, GeneratedToken, GeneratedTree, group, metavariable};
use macroonz::native_publication::{LandingBinding, Publication};

const REACHES: [(&str, Visibility); 5] = [
    ("private", Visibility::Private),
    ("module", Visibility::Module),
    ("parent", Visibility::Parent),
    ("crate", Visibility::Crate),
    ("public", Visibility::Public),
];

pub(super) const DRIVER: &str = r#"
#[path = "generated/definition.rs"] mod definition;
#[path = "generated/private.rs"] mod private_site;
#[path = "generated/module.rs"] mod module_site;
mod outer {
    pub mod parent_site { include!("generated/parent.rs"); }
    pub fn read() -> u8 { parent_site::VALUE }
}
#[path = "generated/crate.rs"] mod crate_site;
#[path = "generated/public.rs"] pub mod public_site;
#[path = "generated/other.rs"] mod other;
use std::io::Write;
fn main() -> std::io::Result<()> {
    assert_eq!(private_site::read(), 7);
    assert_eq!(module_site::read(), 7);
    assert_eq!(outer::read(), 7);
    assert_eq!(crate_site::VALUE, 7);
    assert_eq!(public_site::VALUE, 7);
    writeln!(std::io::stdout(), "{}", other::OTHER)
}
"#;

pub(super) fn publication() -> Result<Publication<Example>, String> {
    let pattern = Pattern::declared(
        "Seats a value under exactly the caller's declared reach.",
        vec![
            Part::Reach,
            Part::Seat(
                Seat::declared("name", Seating::One(Fragment::Identifier))
                    .map_err(|error| error.to_string())?,
            ),
        ],
        body()?,
    )
    .map_err(|error| error.to_string())?;
    let sites = REACHES
        .iter()
        .map(|(name, reach)| {
            Site::declared(
                name,
                SiteRoot::spelled(vec!["crate".to_owned()])?,
                *reach,
                vec![GeneratedTree::assembled(vec![GeneratedToken::word(
                    "nested",
                )])?],
            )
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| error.to_string())?;
    let stamp = Stamp::declared(
        StampName::declared("published_reach").map_err(|error| error.to_string())?,
        pattern,
        sites,
    )
    .map_err(|error| error.to_string())?;
    let (expansion, artifact) = expanded_with_stamp([7, 7, 42], &stamp)?;
    let landings = REACHES
        .iter()
        .map(|(name, _)| {
            Ok(LandingBinding {
                site: (*name).to_owned(),
                path: path(&format!("generated/{name}.rs"))?,
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    Publication::declared(
        expansion,
        &bindings("generated/definition.rs", "generated/other.rs")?,
        LIMITS,
    )
    .and_then(|publication| publication.with_stamp(artifact, &landings))
    .map_err(|error| error.to_string())
}

fn body() -> Result<GeneratedTree, String> {
    let mut constant = metavariable(TRANSPORTED_REACH);
    constant.extend([
        GeneratedToken::word("const"),
        GeneratedToken::word("VALUE"),
        GeneratedToken::alone(':'),
        GeneratedToken::word("u8"),
        GeneratedToken::alone('='),
        GeneratedToken::number(7),
        GeneratedToken::alone(';'),
    ]);
    let mut body = vec![GeneratedToken::word("mod")];
    body.extend(metavariable("name"));
    body.push(group(GeneratedDelimiter::Brace, constant).map_err(|error| error.to_string())?);
    body.extend(metavariable(DECLARED_REACH));
    body.push(GeneratedToken::word("use"));
    body.extend(metavariable("name"));
    body.extend([
        GeneratedToken::joint(':'),
        GeneratedToken::alone(':'),
        GeneratedToken::word("VALUE"),
        GeneratedToken::alone(';'),
        GeneratedToken::word("pub"),
        GeneratedToken::word("fn"),
        GeneratedToken::word("read"),
        group(GeneratedDelimiter::Parenthesis, Vec::new()).map_err(|error| error.to_string())?,
        GeneratedToken::joint('-'),
        GeneratedToken::alone('>'),
        GeneratedToken::word("u8"),
        group(
            GeneratedDelimiter::Brace,
            vec![GeneratedToken::word("VALUE")],
        )
        .map_err(|error| error.to_string())?,
    ]);
    GeneratedTree::assembled(body).map_err(|error| error.to_string())
}
