//! The two declaration roads, exercised from outside: a lawful body in, the builder or exploration module out, and every malformed clause refused at capture.
//!
//! The positive lanes hold the emitted text to the shapes the grammars promise; the refusal lanes reverse one clause each — an undeclared key, a doubled name, a foreign endpoint, an unreadable phrase, a missing fact, an empty declaration, a separator separating nothing.

use macroonz::descriptor::Grammar;
use macroonz::descriptor::concurrency::ConcurrencyModule;
use macroonz::descriptor::door;
use macroonz::descriptor::network::NetworkModule;
use macroonz::kind::Kind;
use macroonz::{CrateBinding, Diagnostic, Door, Expansion, Phase, Producer, TextCapture};

/// The one value that says who is asking.
const DOOR: Door = Door::declared(
    "lane",
    "lane.declared.grammar",
    "lane::declared",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "declared",
    },
);

/// The network grammar this lane registers.
const NETWORK: Grammar = Grammar {
    attribute: "network",
};

/// The concurrency grammar this lane registers.
const CONCURRENCY: Grammar = Grammar {
    attribute: "concurrency",
};

/// One lawful network declaration body.
const NETWORK_BODY: &str = r#"
    module = net,
    namespace = "lane",
    nodes = [client, server],
    link forward = client to server,
    link back = server to client,
    schedule quiet = [],
    schedule outage = [
        drop forward at 0,
        delay forward at 1 by 2,
        duplicate back at 0,
        partition forward from 0 until 3,
    ],
"#;

/// One lawful concurrency declaration body.
const CONCURRENCY_BODY: &str = r#"
    module = explorations,
    namespace = "lane",
    transfers_hold {
        population = "transfer-orders",
        interleavings = 16,
        samples = 32,
        seed = 11,
    },
"#;

/// The network road walked over one source, or nothing where the lane's own source did not capture.
fn networked(source: &str) -> Option<Result<Expansion<NetworkModule>, Diagnostic>> {
    let read = TextCapture::read(source).ok()?;
    Some(door::network(read.input().clone(), NETWORK, &DOOR))
}

/// The concurrency road walked over one source, on the same terms.
fn concurrent(source: &str) -> Option<Result<Expansion<ConcurrencyModule>, Diagnostic>> {
    let read = TextCapture::read(source).ok()?;
    Some(door::concurrency(read.input().clone(), CONCURRENCY, &DOOR))
}

/// The declaration-site text one expansion emits.
fn emitted<K: Kind>(expansion: &Expansion<K>) -> Option<String> {
    expansion
        .emit()
        .tokens()
        .map(macroonz::GeneratedTree::inspected)
}

/// A lawful network declaration becomes one builder module: the fault enum, the topology, and one function per schedule.
#[test]
fn a_network_declaration_becomes_its_builder_module() -> Result<(), ()> {
    let expansion = networked(NETWORK_BODY).ok_or(())?.ok().ok_or(())?;
    let text = emitted(&expansion).ok_or(())?;
    for spelled in [
        "pub mod net",
        "pub enum Fault",
        "pub fn topology",
        "pub fn quiet",
        "pub fn outage",
        "Topology",
        "NetworkSchedule",
        "LinkDiscipline",
        "DropAt",
        "DelayAt",
        "DuplicateAt",
        "Partition",
        "TickSpan",
    ] {
        assert!(
            text.contains(spelled),
            "the module does not spell {spelled}"
        );
    }
    assert_eq!(text.matches("pub fn").count(), 3usize);
    Ok(())
}

/// A lawful concurrency declaration becomes one exploration module: the fault enum and one generic function per row.
#[test]
fn a_concurrency_declaration_becomes_its_exploration_module() -> Result<(), ()> {
    let expansion = concurrent(CONCURRENCY_BODY).ok_or(())?.ok().ok_or(())?;
    let text = emitted(&expansion).ok_or(())?;
    for spelled in [
        "pub mod explorations",
        "pub enum Fault",
        "pub fn transfers_hold",
        "StrandSet",
        "TransitionContract",
        "ExplorationBound",
        "PopulationRef",
        "RootSeed",
        "concluded",
    ] {
        assert!(
            text.contains(spelled),
            "the module does not spell {spelled}"
        );
    }
    assert_eq!(text.matches("pub fn").count(), 1usize);
    Ok(())
}

/// A malformed network declaration refuses at capture, clause by clause.
#[test]
fn a_malformed_network_declaration_refuses_at_capture() -> Result<(), ()> {
    let missing_module = r#"
        namespace = "lane",
        nodes = [client, server],
        link forward = client to server,
    "#;
    let doubled_node = r#"
        module = net,
        namespace = "lane",
        nodes = [client, client],
        link forward = client to client,
    "#;
    let foreign_endpoint = r#"
        module = net,
        namespace = "lane",
        nodes = [client, server],
        link forward = client to stranger,
    "#;
    let undrawn_link = r#"
        module = net,
        namespace = "lane",
        nodes = [client, server],
        link forward = client to server,
        schedule outage = [drop sideways at 0],
    "#;
    let unread_phrase = r#"
        module = net,
        namespace = "lane",
        nodes = [client, server],
        link forward = client to server,
        schedule outage = [scramble forward at 0],
    "#;
    let undeclared_key = r#"
        module = net,
        namespace = "lane",
        nodes = [client, server],
        link forward = client to server,
        latency = 3,
    "#;
    let empty_then_stated_nodes = r#"
        module = net,
        namespace = "lane",
        nodes = [],
        nodes = [client, server],
        link forward = client to server,
    "#;
    let unseparated_nodes = r#"
        module = net,
        namespace = "lane",
        nodes = [client server],
        link forward = client to server,
    "#;
    let reserved_schedule = r#"
        module = net,
        namespace = "lane",
        nodes = [client, server],
        link forward = client to server,
        schedule topology = [],
    "#;
    let keyword_module = r#"
        module = type,
        namespace = "lane",
        nodes = [client, server],
        link forward = client to server,
    "#;
    let oversized_position = r#"
        module = net,
        namespace = "lane",
        nodes = [client, server],
        link forward = client to server,
        schedule outage = [drop forward at 4294967296],
    "#;
    for source in [
        missing_module,
        doubled_node,
        foreign_endpoint,
        undrawn_link,
        unread_phrase,
        undeclared_key,
        empty_then_stated_nodes,
        unseparated_nodes,
        reserved_schedule,
        keyword_module,
        oversized_position,
    ] {
        let refusal = networked(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
    }
    Ok(())
}

/// A separator separating nothing refuses at its own comma — leading or doubled — while a trailing comma stays ordinary Rust.
#[test]
fn a_network_separator_separating_nothing_refuses() -> Result<(), ()> {
    let leading_separator = r#"
        , module = net,
        namespace = "lane",
        nodes = [client, server],
        link forward = client to server,
    "#;
    let doubled_separator = r#"
        module = net,,
        namespace = "lane",
        nodes = [client, server],
        link forward = client to server,
    "#;
    for source in [leading_separator, doubled_separator] {
        let refusal = networked(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
        assert!(
            refusal
                .summary()
                .contains("a separator stands where no clause does"),
            "{source} does not name the dangling separator"
        );
        assert!(
            refusal.summary().contains("(at "),
            "{source} carries no coordinate"
        );
    }
    Ok(())
}

/// A concurrency separator separating nothing refuses at its own comma, named as such — at the declaration level and inside a row body.
#[test]
fn a_concurrency_separator_separating_nothing_refuses() -> Result<(), ()> {
    let doubled_separator = r#"
        module = explorations,,
        namespace = "lane",
        transfers_hold { population = "a", interleavings = 1, samples = 1, seed = 1 },
    "#;
    let dangling_row_separator = r#"
        module = explorations,
        namespace = "lane",
        transfers_hold { population = "a", interleavings = 1,, samples = 1, seed = 1 },
    "#;
    for source in [doubled_separator, dangling_row_separator] {
        let refusal = concurrent(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
        assert!(
            refusal
                .summary()
                .contains("a separator stands where no clause does"),
            "{source} does not name the dangling separator"
        );
        assert!(
            refusal.summary().contains("(at "),
            "{source} carries no coordinate"
        );
    }
    Ok(())
}

/// The three typed rootings render the language's own qualifiers — the caller's crate, the landing module, and its parent — never the extern prelude.
#[test]
fn a_codec_path_renders_under_its_typed_rooting() -> Result<(), ()> {
    use macroonz::codec::{
        AssemblyPosture, Cardinality, CodecAssembly, CodecContent, CodecDirection, CodecMember,
        CodecMemberShape, CodecPlacement, CodecShape, CodecTypePath, PathRooting, codec_surface,
    };
    let owner = CodecTypePath::spelled(PathRooting::CrateAbsolute, vec!["Demo".to_owned()])
        .map_err(|_| ())?;
    let held = CodecTypePath::spelled(PathRooting::ParentScoped, vec!["Held".to_owned()])
        .map_err(|_| ())?;
    let near =
        CodecTypePath::spelled(PathRooting::SelfScoped, vec!["Near".to_owned()]).map_err(|_| ())?;
    let members = vec![
        CodecMember::declared(
            "held",
            held,
            CodecMemberShape::Nested,
            Cardinality::Required,
        )
        .map_err(|_| ())?,
        CodecMember::declared(
            "near",
            near,
            CodecMemberShape::Nested,
            Cardinality::Required,
        )
        .map_err(|_| ())?,
    ];
    let assembly = CodecAssembly::stated("assembled", AssemblyPosture::Total).map_err(|_| ())?;
    let shape = CodecShape::declared(owner, "DemoRefusal", assembly, members).map_err(|_| ())?;
    let content = CodecContent {
        shape,
        direction: CodecDirection::RoundTrip,
        placement: CodecPlacement::AtDeclarationSite,
        schema: None,
        byte_role: None,
        assumptions: macroonz::Bounded::empty(),
    };
    let text = codec_surface(&content).map_err(|_| ())?.inspected();
    for spelled in ["crate :: Demo", "super :: Held", "self :: Near"] {
        assert!(
            text.contains(spelled),
            "the surface does not spell {spelled}"
        );
    }
    assert!(!text.contains(":: crate"), "the extern prelude leaked in");
    Ok(())
}

/// A keyword cannot name a rendered item: every programmatic name constructor reads the composed law, not the alphabet alone.
///
/// Paths read position-aware the way the language does: the stamp site root admits `crate`, `self`, or a leading run of `super` at the root, the codec path types its qualifier as the rooting — so every later segment is an item name the keyword roster refuses.
#[test]
fn a_keyword_cannot_name_a_rendered_item() {
    assert!(macroonz::descriptor::ModuleName::declared("type").is_err());
    assert!(macroonz::descriptor::SupportName::declared("mod").is_err());
    assert!(macroonz::descriptor::TypeName::declared("gen").is_err());
    assert!(macroonz::descriptor::FunctionName::declared("fn").is_err());
    assert!(macroonz::descriptor::ModuleName::declared("lawful_name").is_ok());
    assert!(macroonz::stamp::StampName::declared("type").is_err());
    assert!(macroonz::support::SupportName::declared("loop").is_err());
    assert!(macroonz::codec::ModuleSpelling::spelled("type").is_err());
    assert!(macroonz::stamp::SiteRoot::spelled(vec!["crate".to_owned()]).is_ok());
    assert!(
        macroonz::stamp::SiteRoot::spelled(vec!["crate".to_owned(), "type".to_owned()]).is_err()
    );
    assert!(
        macroonz::stamp::SiteRoot::spelled(vec![
            "super".to_owned(),
            "super".to_owned(),
            "stamps".to_owned(),
        ])
        .is_ok()
    );
    assert!(
        macroonz::stamp::SiteRoot::spelled(vec!["stamps".to_owned(), "self".to_owned()]).is_err()
    );
    assert!(
        macroonz::codec::CodecTypePath::spelled(
            macroonz::codec::PathRooting::ParentScoped,
            vec!["Thing".to_owned()],
        )
        .is_ok()
    );
    assert!(
        macroonz::codec::CodecTypePath::spelled(
            macroonz::codec::PathRooting::InScope,
            vec!["self".to_owned(), "Thing".to_owned()],
        )
        .is_err()
    );
}

/// A malformed concurrency declaration refuses at capture, clause by clause.
#[test]
fn a_malformed_concurrency_declaration_refuses_at_capture() -> Result<(), ()> {
    let missing_fact = r#"
        module = explorations,
        namespace = "lane",
        transfers_hold {
            population = "transfer-orders",
            interleavings = 16,
            samples = 32,
        },
    "#;
    let doubled_row = r#"
        module = explorations,
        namespace = "lane",
        transfers_hold { population = "a", interleavings = 1, samples = 1, seed = 1 },
        transfers_hold { population = "b", interleavings = 1, samples = 1, seed = 1 },
    "#;
    let no_rows = r#"
        module = explorations,
        namespace = "lane",
    "#;
    let unread_number = r#"
        module = explorations,
        namespace = "lane",
        transfers_hold { population = "a", interleavings = many, samples = 1, seed = 1 },
    "#;
    let oversized_number = r#"
        module = explorations,
        namespace = "lane",
        transfers_hold { population = "a", interleavings = 4294967296, samples = 1, seed = 1 },
    "#;
    let keyword_module = r#"
        module = type,
        namespace = "lane",
        transfers_hold { population = "a", interleavings = 1, samples = 1, seed = 1 },
    "#;
    let keyword_row = r#"
        module = explorations,
        namespace = "lane",
        type { population = "a", interleavings = 1, samples = 1, seed = 1 },
    "#;
    for source in [
        missing_fact,
        doubled_row,
        no_rows,
        unread_number,
        oversized_number,
        keyword_module,
        keyword_row,
    ] {
        let refusal = concurrent(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
    }
    Ok(())
}
