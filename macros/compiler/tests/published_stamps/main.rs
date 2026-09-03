//! Published stamp claims observed from outside: exact reach transport, opaque visibility refusal, standalone generated source, and deterministic output.

use macroonz_compiler::stamp::{
    DECLARED_REACH, Fragment, Landing, OPAQUE_REACH_REFUSAL, Part, Pattern, PublicationGround,
    PublishedStamp, Seat, Seating, Site, SiteRoot, Stamp, StampError, StampName, TRANSPORTED_REACH,
    TransportedReach, Visibility, declared_reach, planned, transported_reach,
};
use macroonz_compiler::{
    CrateBinding, Destination, Door, GeneratedDelimiter, GeneratedToken, GeneratedTree, Kind,
    NoQuestions, OwnerIdentity, Producer, Request, Role, TextCapture, group, metavariable,
};
use std::path::PathBuf;
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

static SPECIMEN_ORDINAL: AtomicU32 = AtomicU32::new(0);

/// The one publication seat the specimen plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PublicationSeat {
    /// A standalone source artifact.
    Artifact,
}

impl Role for PublicationSeat {
    const ALL: &'static [Self] = &[Self::Artifact];

    fn name(self) -> &'static str {
        "artifact"
    }

    fn destination(self) -> Destination {
        Destination::PublicationArtifact
    }
}

/// The kind whose plan supplies the publication decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Publication;

impl Kind for Publication {
    const NAME: &'static str = "lane.published-stamp";
    type Content = ();
    type Role = PublicationSeat;
    type Question = NoQuestions;
}

/// Who asks for the specimen publication.
const DOOR: Door = Door::declared(
    "lane",
    "lane.published-stamp.grammar",
    "lane::published_stamp",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "published-stamp",
    },
);

/// The external address the plan binds to the artifact seat.
const ADDRESS: OwnerIdentity = OwnerIdentity {
    subject: "lane.published-stamp.address",
    bytes: [7; 32],
};

fn specimen_path(extension: &str) -> PathBuf {
    let ordinal = SPECIMEN_ORDINAL.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "macroonz_published_stamp_{}_{ordinal}{extension}",
        std::process::id()
    ))
}

fn compile(source: &str) -> Result<Output, String> {
    let source_path = specimen_path(".rs");
    let executable = specimen_path(std::env::consts::EXE_SUFFIX);
    std::fs::write(&source_path, source).map_err(|error| error.to_string())?;
    let compiled = Command::new("rustup")
        .arg("run")
        .arg("1.98.0")
        .arg("rustc")
        .arg(&source_path)
        .arg("--edition=2024")
        .arg("-o")
        .arg(&executable)
        .output()
        .map_err(|error| error.to_string())?;
    drop(std::fs::remove_file(&source_path));
    if compiled.status.success() {
        let executed = Command::new(&executable)
            .output()
            .map_err(|error| error.to_string())?;
        drop(std::fs::remove_file(&executable));
        if !executed.status.success() {
            return Err(String::from_utf8_lossy(&executed.stderr).into_owned());
        }
    }
    Ok(compiled)
}

fn punctuated_path(root: Vec<GeneratedToken>, tail: &str) -> Vec<GeneratedToken> {
    let mut tokens = root;
    tokens.push(GeneratedToken::joint(':'));
    tokens.push(GeneratedToken::alone(':'));
    tokens.push(GeneratedToken::word(tail));
    tokens
}

fn pattern_body() -> Result<GeneratedTree, String> {
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

    let mut tokens = vec![GeneratedToken::word("mod")];
    tokens.extend(metavariable("name"));
    tokens.push(group(GeneratedDelimiter::Brace, constant).map_err(|refusal| refusal.to_string())?);
    tokens.extend(metavariable(DECLARED_REACH));
    tokens.push(GeneratedToken::word("use"));
    tokens.extend(punctuated_path(metavariable("name"), "VALUE"));
    tokens.push(GeneratedToken::alone(';'));
    GeneratedTree::assembled(tokens).map_err(|refusal| refusal.to_string())
}

fn argument(spelling: &str) -> Result<GeneratedTree, String> {
    GeneratedTree::assembled(vec![GeneratedToken::word(spelling)])
        .map_err(|refusal| refusal.to_string())
}

fn declared_stamp(site_order: &[Visibility]) -> Result<Stamp, String> {
    let name = StampName::declared("published_reach").map_err(|refusal| refusal.to_string())?;
    let seat = Seat::declared("name", Seating::One(Fragment::Identifier))
        .map_err(|refusal| refusal.to_string())?;
    let pattern = Pattern::declared(
        "Seats one value under the reach its adopter declared.",
        vec![Part::Reach, Part::Seat(seat)],
        pattern_body()?,
    )
    .map_err(|refusal| refusal.to_string())?;
    let root =
        SiteRoot::spelled(vec!["crate".to_owned()]).map_err(|refusal| refusal.to_string())?;
    let sites = site_order
        .iter()
        .map(|reach| {
            let (site, module) = match reach {
                Visibility::Private => ("private", "private_value"),
                Visibility::Module => ("module", "module_value"),
                Visibility::Parent => ("parent", "parent_value"),
                Visibility::Crate => ("crate", "crate_value"),
                Visibility::Public => ("public", "public_value"),
            };
            Site::declared(site, root.clone(), *reach, vec![argument(module)?])
                .map_err(|refusal| refusal.to_string())
        })
        .collect::<Result<Vec<_>, String>>()?;
    Stamp::declared(name, pattern, sites).map_err(|refusal| refusal.to_string())
}

/// Pattern and site namespaces refuse the first repeated spelling at its declared position.
#[test]
fn stamp_namespaces_refuse_the_first_repeated_spelling() -> Result<(), String> {
    let first_seat = Seat::declared("name", Seating::One(Fragment::Identifier))
        .map_err(|refusal| refusal.to_string())?;
    let second_seat = Seat::declared("name", Seating::One(Fragment::Identifier))
        .map_err(|refusal| refusal.to_string())?;
    let doubled_pattern = Pattern::declared(
        "Carries two declared seats.",
        vec![Part::Seat(first_seat), Part::Seat(second_seat)],
        pattern_body()?,
    );
    assert_eq!(doubled_pattern, Err(StampError::SeatNameDoubled { at: 1 }));

    let pattern = declared_stamp(&[Visibility::Private])?.pattern().clone();
    let root =
        SiteRoot::spelled(vec!["crate".to_owned()]).map_err(|refusal| refusal.to_string())?;
    let first = Site::declared(
        "same",
        root.clone(),
        Visibility::Private,
        vec![argument("first")?],
    )
    .map_err(|refusal| refusal.to_string())?;
    let second = Site::declared("same", root, Visibility::Public, vec![argument("second")?])
        .map_err(|refusal| refusal.to_string())?;
    let name = StampName::declared("doubled_sites").map_err(|refusal| refusal.to_string())?;
    assert_eq!(
        Stamp::declared(name, pattern, vec![first, second]),
        Err(StampError::SiteNameDoubled { at: 1 })
    );
    Ok(())
}

fn published(site_order: &[Visibility]) -> Result<PublishedStamp, String> {
    let capture =
        TextCapture::read("struct Publication;").map_err(|refusal| refusal.to_string())?;
    let expansion = Request::<Publication>::over(capture.input().clone(), (), &DOOR)
        .publishing_at(PublicationSeat::Artifact, ADDRESS)
        .render(|_plan, out| {
            out.unit(
                PublicationSeat::Artifact,
                GeneratedTree::assembled(vec![GeneratedToken::word("publication")])?,
            )
        })
        .map_err(|refusal| refusal.summary().to_owned())?;
    let decision = planned(expansion.plan(), PublicationSeat::Artifact)
        .map_err(|refusal| refusal.to_string())?;
    PublishedStamp::rendered(
        &decision,
        &declared_stamp(site_order)?,
        PublicationGround::CrossFileArtifact,
    )
    .map_err(|refusal| refusal.to_string())
}

const ALL_REACHES: [Visibility; 5] = [
    Visibility::Private,
    Visibility::Module,
    Visibility::Parent,
    Visibility::Crate,
    Visibility::Public,
];

/// Claim: every closed site reach has exactly one transported reach and both source spellings agree with that table.
///
/// Population: all five `Visibility` rows and all four `TransportedReach` rows.
/// Reversal: the parent-facing row is required to gain one ancestor segment while the two absolute rows remain unchanged.
/// Evidence ceiling: this proves this home's transport table and token projection, while the compiled specimen below proves the rendered scopes are accepted by Rust 1.98.
#[test]
fn every_site_reach_transports_exactly_one_module_inward() -> Result<(), String> {
    let cases = [
        (
            Visibility::Private,
            TransportedReach::Enclosing,
            "",
            "pub ( super ) ",
        ),
        (
            Visibility::Module,
            TransportedReach::Enclosing,
            "pub ( self ) ",
            "pub ( super ) ",
        ),
        (
            Visibility::Parent,
            TransportedReach::Ancestor,
            "pub ( super ) ",
            "pub ( in super :: super ) ",
        ),
        (
            Visibility::Crate,
            TransportedReach::Crate,
            "pub ( crate ) ",
            "pub ( crate ) ",
        ),
        (Visibility::Public, TransportedReach::Public, "pub ", "pub "),
    ];
    for (declared, transported, declared_text, transported_text) in cases {
        assert_eq!(declared.transported(), transported);
        assert_eq!(
            GeneratedTree::assembled(declared_reach(declared).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?
                .inspected(),
            declared_text
        );
        assert_eq!(
            GeneratedTree::assembled(
                transported_reach(transported).map_err(|error| error.to_string())?
            )
            .map_err(|error| error.to_string())?
            .inspected(),
            transported_text
        );
    }
    Ok(())
}

fn compiled_specimen_source(artifact: &PublishedStamp) -> Result<String, String> {
    let mut landings = artifact.landings().iter();
    let private = landings.next().ok_or("private landing absent")?;
    let module = landings.next().ok_or("module landing absent")?;
    let parent = landings.next().ok_or("parent landing absent")?;
    let crate_reach = landings.next().ok_or("crate landing absent")?;
    let public = landings.next().ok_or("public landing absent")?;
    if landings.next().is_some() {
        return Err("an undeclared landing was rendered".to_owned());
    }
    Ok(format!(
        r"
{}

mod private_site {{
    {}
    pub fn read() -> u8 {{ VALUE }}
}}

mod module_site {{
    {}
    pub fn read() -> u8 {{ VALUE }}
}}

mod outer {{
    pub mod parent_site {{
        {}
    }}

    pub fn read() -> u8 {{ parent_site::VALUE }}
}}

mod crate_site {{
    {}
}}

pub mod public_site {{
    {}
}}

fn main() {{
    assert_eq!(private_site::read(), 7);
    assert_eq!(module_site::read(), 7);
    assert_eq!(outer::read(), 7);
    assert_eq!(crate_site::VALUE, 7);
    assert_eq!(public_site::VALUE, 7);
}}
",
        artifact.definition().inspected(),
        private.invocation().inspected(),
        module.invocation().inspected(),
        parent.invocation().inspected(),
        crate_reach.invocation().inspected(),
        public.invocation().inspected(),
    ))
}

/// Claim: a complete published stamp is standalone Rust source whose five reach postures compile and execute without a dependency on this compiler.
///
/// Population: one artifact containing every reach row and one landing per declared site.
/// Hostile control: each assertion reads from the farthest scope that row promises, so a copied or narrowed transport fails compilation, while the independent spelling assertions above catch a widened transport.
/// Evidence ceiling: this is one representative pattern compiled by Rust 1.98 on the local host, not arbitrary caller token material, packaging, Wasm, Linux, or a filesystem publication receipt.
#[test]
fn generated_published_source_compiles_and_executes_in_a_scratch_crate() -> Result<(), String> {
    let artifact = published(&ALL_REACHES)?;
    assert_eq!(artifact.count(), ALL_REACHES.len());
    assert_eq!(
        artifact.record().manifest().collect::<Vec<_>>(),
        vec!["private", "module", "parent", "crate", "public"]
    );
    let compiled = compile(&compiled_specimen_source(&artifact)?)?;
    if !compiled.status.success() {
        return Err(String::from_utf8_lossy(&compiled.stderr).into_owned());
    }
    Ok(())
}

/// Claim: an opaque forwarded visibility refuses at the generated definition's public front door rather than being copied or widened.
///
/// Hostile control: an outer macro captures `pub` as a `vis` fragment before forwarding it, so its token spelling looks lawful while its opaque fragment provenance makes transport impossible.
/// Evidence ceiling: this fixes the generated refusal sentence and compile-time posture for one opaque public reach under Rust 1.98; proc-macro placement belongs to the proc host rather than this source artifact.
#[test]
fn an_opaque_forwarded_visibility_refuses_instead_of_guessing_a_scope() -> Result<(), String> {
    let artifact = published(&ALL_REACHES)?;
    let source = format!(
        r"
{}

macro_rules! opaque_forward {{
    ($reach:vis $name:ident) => {{ $crate::published_reach! {{ $reach $name }} }};
}}

opaque_forward!(pub refused_value);

fn main() {{}}
",
        artifact.definition().inspected()
    );
    let compiled = compile(&source)?;
    assert!(!compiled.status.success(), "the opaque visibility compiled");
    let diagnostic = String::from_utf8_lossy(&compiled.stderr);
    assert!(diagnostic.contains(OPAQUE_REACH_REFUSAL), "{diagnostic}");
    Ok(())
}

/// Claim: one declared stamp and one plan decision render one stable artifact value on every repeat.
///
/// Reversal: reversing the site declaration order changes the landing order and therefore changes the complete value.
/// Evidence ceiling: equality and canonical token bytes establish deterministic construction inside this process; cross-host and packaged reproducibility belong to later repository-wide qualification.
#[test]
fn the_same_declared_inputs_render_the_same_artifact() -> Result<(), String> {
    let first = published(&ALL_REACHES)?;
    let second = published(&ALL_REACHES)?;
    assert_eq!(first, second);
    assert_eq!(
        first.definition().canonical_bytes(),
        second.definition().canonical_bytes()
    );
    let mut reversed = ALL_REACHES;
    reversed.reverse();
    let changed = published(&reversed)?;
    assert_ne!(first, changed);
    assert_ne!(
        first
            .landings()
            .iter()
            .map(Landing::site)
            .collect::<Vec<_>>(),
        changed
            .landings()
            .iter()
            .map(Landing::site)
            .collect::<Vec<_>>()
    );
    Ok(())
}
