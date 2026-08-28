//! Codec claims observed from outside: the public bill, canonical preimages, and generated Rust compiled and executed through the pinned toolchain.

use macroonz_compiler::codec::{
    AssemblyPosture, Cardinality, CodecAssembly, CodecContent, CodecDirection, CodecIssue,
    CodecMember, CodecMemberShape, CodecPlacement, CodecShape, CodecTypePath, MEMBER_CONTRACT,
    MemberContract, ModuleSpelling, PathRooting, codec_surface,
};
use macroonz_compiler::{Bounded, CanonicalContent};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};

static SPECIMEN_ORDINAL: AtomicU32 = AtomicU32::new(0);

fn in_scope(spelling: &str) -> Result<CodecTypePath, String> {
    CodecTypePath::spelled(PathRooting::InScope, vec![spelling.to_owned()])
        .map_err(|refusal| refusal.to_string())
}

fn declared_member(
    spelling: &str,
    held_as: &str,
    shape: CodecMemberShape,
    cardinality: Cardinality,
) -> Result<CodecMember, String> {
    CodecMember::declared(spelling, in_scope(held_as)?, shape, cardinality)
        .map_err(|refusal| refusal.to_string())
}

fn codec_content(direction: CodecDirection) -> Result<CodecContent, String> {
    let assembly = CodecAssembly::stated(
        "assembled",
        AssemblyPosture::Checked {
            refusal: in_scope("AssemblyRefusal")?,
        },
    )
    .map_err(|refusal| refusal.to_string())?;
    let members = vec![
        declared_member(
            "count",
            "u16",
            CodecMemberShape::Count,
            Cardinality::Required,
        )?,
        declared_member(
            "payload",
            "EvenBytes",
            CodecMemberShape::Bytes,
            Cardinality::Required,
        )?,
        declared_member(
            "label",
            "String",
            CodecMemberShape::Text,
            Cardinality::Optional,
        )?,
        declared_member(
            "modes",
            "Choice",
            CodecMemberShape::ClosedChoice,
            Cardinality::Repeated,
        )?,
        declared_member(
            "child",
            "Nested",
            CodecMemberShape::Nested,
            Cardinality::Required,
        )?,
    ];
    let shape = CodecShape::declared(in_scope("Demo")?, "DemoRefusal", assembly, members)
        .map_err(|refusal| refusal.to_string())?;
    Ok(CodecContent {
        shape,
        direction,
        placement: CodecPlacement::AtDeclarationSite,
        schema: None,
        byte_role: None,
        assumptions: Bounded::empty(),
    })
}

fn frame(material: &[u8], into: &mut Vec<u8>) {
    let length = u64::try_from(material.len()).unwrap_or(u64::MAX);
    into.extend_from_slice(&length.to_be_bytes());
    into.extend_from_slice(material);
}

fn independent_path(rooting: &str, segments: &[&str], into: &mut Vec<u8>) {
    frame(rooting.as_bytes(), into);
    let count = u64::try_from(segments.len()).unwrap_or(u64::MAX);
    into.extend_from_slice(&count.to_be_bytes());
    for segment in segments {
        frame(segment.as_bytes(), into);
    }
}

fn independent_member(
    spelling: &str,
    held_as: &str,
    shape: &str,
    cardinality: &str,
    into: &mut Vec<u8>,
) {
    let mut member = Vec::new();
    frame(spelling.as_bytes(), &mut member);
    independent_path("in-scope", &[held_as], &mut member);
    frame(shape.as_bytes(), &mut member);
    frame(cardinality.as_bytes(), &mut member);
    frame(&member, into);
}

fn independent_content_bytes() -> Vec<u8> {
    let mut bytes = Vec::new();
    independent_path("in-scope", &["Demo"], &mut bytes);
    frame(b"DemoRefusal", &mut bytes);
    frame(b"assembled", &mut bytes);
    bytes.push(1);
    independent_path("in-scope", &["AssemblyRefusal"], &mut bytes);
    bytes.extend_from_slice(&5_u64.to_be_bytes());
    independent_member("count", "u16", "count", "required", &mut bytes);
    independent_member("payload", "EvenBytes", "bytes", "required", &mut bytes);
    independent_member("label", "String", "text", "optional", &mut bytes);
    independent_member("modes", "Choice", "closed-choice", "repeated", &mut bytes);
    independent_member("child", "Nested", "nested", "required", &mut bytes);
    frame(b"round-trip", &mut bytes);
    bytes.push(0);
    bytes.push(0);
    bytes.push(0);
    bytes.extend_from_slice(&0_u64.to_be_bytes());
    bytes
}

/// Claim: the public five-row bill and the operations emitted for all five shapes remain one observable contract.
///
/// Population: every `CodecMemberShape` row in one round-trip surface.
/// Hostile control: the expected bill is independently restated at the public boundary, so a changed road or row order disagrees before generated compilation can mask it.
/// Evidence ceiling: this establishes the public bill and emitted callable spellings, while the compiled specimen below establishes that the calls type-check and execute for one representative owner.
#[test]
fn every_member_contract_row_reaches_the_generated_surface() -> Result<(), String> {
    let expected = [
        MemberContract {
            shape: CodecMemberShape::Count,
            encode_road: "u64::from",
            decode_road: "<T as ::core::convert::TryFrom<u64>>::try_from",
        },
        MemberContract {
            shape: CodecMemberShape::Bytes,
            encode_road: "<T as ::core::convert::AsRef<[u8]>>::as_ref",
            decode_road: "<T as ::core::convert::TryFrom<::std::vec::Vec<u8>>>::try_from",
        },
        MemberContract {
            shape: CodecMemberShape::Text,
            encode_road: "<T as ::core::convert::AsRef<str>>::as_ref",
            decode_road: "<T as ::core::convert::TryFrom<::std::string::String>>::try_from",
        },
        MemberContract {
            shape: CodecMemberShape::ClosedChoice,
            encode_road: "slot",
            decode_road: "ALL",
        },
        MemberContract {
            shape: CodecMemberShape::Nested,
            encode_road: "encode_canonical",
            decode_road: "decode_canonical",
        },
    ];
    assert_eq!(MEMBER_CONTRACT, expected);

    let surface = codec_surface(&codec_content(CodecDirection::RoundTrip)?)
        .map_err(|refusal| refusal.to_string())?
        .inspected();
    for contract in MEMBER_CONTRACT {
        let encode = contract
            .encode_road
            .rsplit("::")
            .next()
            .unwrap_or(contract.encode_road);
        let decode = contract
            .decode_road
            .rsplit("::")
            .next()
            .unwrap_or(contract.decode_road);
        assert!(surface.contains(encode), "the surface omits {encode}");
        assert!(surface.contains(decode), "the surface omits {decode}");
    }
    Ok(())
}

/// Claim: the codec content preimage is complete and stable at the public canonical-content boundary.
///
/// Population: one content value carrying all five wire shapes, all three cardinalities, checked assembly, and both roads.
/// Reversal: changing only the direction changes the preimage.
/// Denominator: every field of `CodecContent`, its shape, its assembly, and every member row is re-encoded without calling the compiler's framing helpers.
/// Evidence ceiling: this fixes the preimage bytes for this representative content and does not claim collision resistance or every possible owner identity and assumption roster.
#[test]
fn codec_content_bytes_match_an_independent_preimage() -> Result<(), String> {
    let content = codec_content(CodecDirection::RoundTrip)?;
    let mut actual = Vec::new();
    content.encode_content_into(&mut actual);
    assert_eq!(actual, independent_content_bytes());

    let mut encode_only = Vec::new();
    codec_content(CodecDirection::Encode)?.encode_content_into(&mut encode_only);
    assert_ne!(actual, encode_only);
    Ok(())
}

fn issue_material(spelling: &str) -> Vec<u8> {
    let mut material = Vec::new();
    frame(spelling.as_bytes(), &mut material);
    material
}

fn issue_bytes(slot: u8, material: &[u8]) -> Vec<u8> {
    let mut bytes = vec![slot];
    frame(material, &mut bytes);
    bytes
}

/// Claim: every diagnostic issue row commits to its stable slot and complete typed payload.
///
/// Population: all 13 `CodecIssue` rows.
/// Hostile controls: two spelling-bearing rows carrying the same spelling remain separated by their slots, and changing one spelling changes its bytes.
/// Denominator: the expected bytes use an independent u64 big-endian framing helper and no codec issue encoder.
/// Evidence ceiling: this establishes codec issue material, not the diagnostic home's later family and subject derivation.
#[test]
fn every_codec_issue_matches_its_independent_bytes() {
    let mut shadowed = issue_material("material");
    frame(b"material", &mut shadowed);
    let mut path_bound = Vec::new();
    path_bound.extend_from_slice(&8_u64.to_be_bytes());
    path_bound.extend_from_slice(&9_u64.to_be_bytes());
    let mut member_bound = Vec::new();
    member_bound.extend_from_slice(&64_u64.to_be_bytes());
    member_bound.extend_from_slice(&65_u64.to_be_bytes());
    let cases = vec![
        (CodecIssue::PathSegmentsAbsent, issue_bytes(0, &[])),
        (
            CodecIssue::SegmentNotAnIdentifier {
                segment: "bad!".to_owned(),
            },
            issue_bytes(1, &issue_material("bad!")),
        ),
        (
            CodecIssue::PathSegmentsUnbounded {
                bound: 8,
                observed: 9,
            },
            issue_bytes(2, &path_bound),
        ),
        (CodecIssue::MemberSpellingAbsent, issue_bytes(3, &[])),
        (
            CodecIssue::MemberSpellingNotAnIdentifier {
                spelling: "bad!".to_owned(),
            },
            issue_bytes(4, &issue_material("bad!")),
        ),
        (
            CodecIssue::MemberSpellingDoubled {
                spelling: "same".to_owned(),
            },
            issue_bytes(5, &issue_material("same")),
        ),
        (
            CodecIssue::MemberShadowsBinding {
                spelling: "material".to_owned(),
                binding: "material",
            },
            issue_bytes(6, &shadowed),
        ),
        (CodecIssue::AssemblyRoadAbsent, issue_bytes(7, &[])),
        (
            CodecIssue::AssemblyRoadNotAnIdentifier {
                spelling: "bad!".to_owned(),
            },
            issue_bytes(8, &issue_material("bad!")),
        ),
        (
            CodecIssue::RefusalSpellingNotAnIdentifier {
                spelling: "bad!".to_owned(),
            },
            issue_bytes(9, &issue_material("bad!")),
        ),
        (
            CodecIssue::ModuleSpellingNotAnIdentifier {
                spelling: "bad!".to_owned(),
            },
            issue_bytes(10, &issue_material("bad!")),
        ),
        (CodecIssue::MembersAbsent, issue_bytes(11, &[])),
        (
            CodecIssue::MembersUnbounded {
                bound: 64,
                observed: 65,
            },
            issue_bytes(12, &member_bound),
        ),
    ];
    for (issue, expected) in cases {
        assert_eq!(issue.canonical_bytes(), expected);
    }

    let same = CodecIssue::MemberSpellingDoubled {
        spelling: "same".to_owned(),
    };
    let another_row = CodecIssue::AssemblyRoadNotAnIdentifier {
        spelling: "same".to_owned(),
    };
    let another_spelling = CodecIssue::MemberSpellingDoubled {
        spelling: "other".to_owned(),
    };
    assert_ne!(same.canonical_bytes(), another_row.canonical_bytes());
    assert_ne!(same.canonical_bytes(), another_spelling.canonical_bytes());
}

/// The three typed rootings render the language's own qualifiers — the caller's crate, the landing module, and its parent — never the extern prelude.
#[test]
fn a_codec_path_renders_under_its_typed_rooting() -> Result<(), String> {
    let owner = CodecTypePath::spelled(PathRooting::CrateAbsolute, vec!["Demo".to_owned()])
        .map_err(|refusal| refusal.to_string())?;
    let held = CodecTypePath::spelled(PathRooting::ParentScoped, vec!["Held".to_owned()])
        .map_err(|refusal| refusal.to_string())?;
    let near = CodecTypePath::spelled(PathRooting::SelfScoped, vec!["Near".to_owned()])
        .map_err(|refusal| refusal.to_string())?;
    let members = vec![
        CodecMember::declared(
            "held",
            held,
            CodecMemberShape::Nested,
            Cardinality::Required,
        )
        .map_err(|refusal| refusal.to_string())?,
        CodecMember::declared(
            "near",
            near,
            CodecMemberShape::Nested,
            Cardinality::Required,
        )
        .map_err(|refusal| refusal.to_string())?,
    ];
    let assembly = CodecAssembly::stated("assembled", AssemblyPosture::Total)
        .map_err(|refusal| refusal.to_string())?;
    let shape = CodecShape::declared(owner, "DemoRefusal", assembly, members)
        .map_err(|refusal| refusal.to_string())?;
    let content = CodecContent {
        shape,
        direction: CodecDirection::RoundTrip,
        placement: CodecPlacement::AtDeclarationSite,
        schema: None,
        byte_role: None,
        assumptions: Bounded::empty(),
    };
    let text = codec_surface(&content)
        .map_err(|refusal| refusal.to_string())?
        .inspected();
    for spelled in ["crate :: Demo", "super :: Held", "self :: Near"] {
        assert!(
            text.contains(spelled),
            "the surface does not spell {spelled}"
        );
    }
    assert!(!text.contains(":: crate"), "the extern prelude leaked in");
    Ok(())
}

fn specimen_path(extension: &str) -> PathBuf {
    let ordinal = SPECIMEN_ORDINAL.fetch_add(1, Ordering::SeqCst);
    std::env::temp_dir().join(format!(
        "macroonz_codec_specimen_{}_{ordinal}{extension}",
        std::process::id()
    ))
}

fn compile_and_run(source: &str) -> Result<(), String> {
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
    if !compiled.status.success() {
        return Err(String::from_utf8_lossy(&compiled.stderr).into_owned());
    }
    let executed = Command::new(&executable)
        .output()
        .map_err(|error| error.to_string())?;
    drop(std::fs::remove_file(&executable));
    if !executed.status.success() {
        return Err(String::from_utf8_lossy(&executed.stderr).into_owned());
    }
    Ok(())
}

const SPECIMEN_DECLARATIONS: &str = r"
#[derive(Debug, Clone, PartialEq, Eq)]
struct EvenBytes(Vec<u8>);

impl AsRef<[u8]> for EvenBytes {
    fn as_ref(&self) -> &[u8] { &self.0 }
}

impl TryFrom<Vec<u8>> for EvenBytes {
    type Error = ();

    fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.first() == Some(&0xff) { Err(()) } else { Ok(Self(bytes)) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Choice { First, Second }

impl Choice {
    const ALL: [Self; 2] = [Self::First, Self::Second];

    const fn slot(self) -> u8 {
        match self { Self::First => 0, Self::Second => 1 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Nested(u8);

#[derive(Debug, Clone, PartialEq, Eq)]
struct NestedRefusal;

impl Nested {
    fn encode_canonical(&self, into: &mut Vec<u8>) { into.push(self.0); }

    fn decode_canonical(material: &[u8]) -> Result<Self, NestedRefusal> {
        match material { [value] if *value != 0 => Ok(Self(*value)), _ => Err(NestedRefusal) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssemblyRefusal;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Demo {
    count: u16,
    payload: EvenBytes,
    label: Option<String>,
    modes: Vec<Choice>,
    child: Nested,
}

impl Demo {
    fn assembled(
        count: u16,
        payload: EvenBytes,
        label: Option<String>,
        modes: Vec<Choice>,
        child: Nested,
    ) -> Result<Self, AssemblyRefusal> {
        if count == 0 {
            Err(AssemblyRefusal)
        } else {
            Ok(Self { count, payload, label, modes, child })
        }
    }
}
";

const SPECIMEN_ASSERTIONS: &str = r#"
fn framed(material: &[u8], into: &mut Vec<u8>) {
    into.extend_from_slice(&u64::try_from(material.len()).unwrap_or(u64::MAX).to_be_bytes());
    into.extend_from_slice(material);
}

fn main() {
    let value = Demo {
        count: 513,
        payload: EvenBytes(vec![3, 4]),
        label: Some(String::from("hi")),
        modes: vec![Choice::First, Choice::Second],
        child: Nested(7),
    };
    let mut encoded = Vec::new();
    value.encode_canonical(&mut encoded);
    let mut expected = Vec::new();
    expected.extend_from_slice(&513_u64.to_be_bytes());
    framed(&[3, 4], &mut expected);
    expected.push(u8::from(true));
    framed(b"hi", &mut expected);
    expected.extend_from_slice(&2_u64.to_be_bytes());
    expected.extend_from_slice(&[0, 1]);
    framed(&[7], &mut expected);
    assert_eq!(encoded, expected);
    assert_eq!(Demo::decode_canonical(&encoded), Ok(value.clone()));

    let mut trailing = encoded.clone();
    trailing.push(9);
    assert_eq!(Demo::decode_canonical(&trailing), Err(DemoRefusal::TrailingBytes));

    let mut bad_presence = encoded.clone();
    bad_presence[18] = 2;
    assert_eq!(
        Demo::decode_canonical(&bad_presence),
        Err(DemoRefusal::PresenceNotAdmitted { member: "label" }),
    );

    let mut bad_slot = encoded.clone();
    bad_slot[37] = 9;
    assert_eq!(
        Demo::decode_canonical(&bad_slot),
        Err(DemoRefusal::SlotNotAdmitted { member: "modes" }),
    );

    let mut bad_nested = encoded.clone();
    bad_nested[47] = 0;
    assert_eq!(
        Demo::decode_canonical(&bad_nested),
        Err(DemoRefusal::NestedMemberRefused { member: "child" }),
    );

    let mut refused_member = encoded.clone();
    refused_member[16] = 0xff;
    assert_eq!(
        Demo::decode_canonical(&refused_member),
        Err(DemoRefusal::MemberNotAdmitted { member: "payload" }),
    );

    let mut bad_text = encoded.clone();
    bad_text[27] = 0xff;
    assert_eq!(
        Demo::decode_canonical(&bad_text),
        Err(DemoRefusal::TextNotUtf8 { member: "label" }),
    );

    let mut wide_count = encoded.clone();
    wide_count[..8].copy_from_slice(&u64::MAX.to_be_bytes());
    assert_eq!(
        Demo::decode_canonical(&wide_count),
        Err(DemoRefusal::CountPastDeclaredWidth { member: "count" }),
    );

    let mut long_payload = encoded.clone();
    long_payload[8..16].copy_from_slice(&99_u64.to_be_bytes());
    assert_eq!(
        Demo::decode_canonical(&long_payload),
        Err(DemoRefusal::LengthPastRemaining { member: "payload" }),
    );

    assert_eq!(
        Demo::decode_canonical(&[0, 1, 2]),
        Err(DemoRefusal::Truncated { member: "count" }),
    );

    let refused = Demo {
        count: 0,
        payload: EvenBytes(vec![3, 4]),
        label: None,
        modes: Vec::new(),
        child: Nested(7),
    };
    let mut refused_bytes = Vec::new();
    refused.encode_canonical(&mut refused_bytes);
    assert_eq!(
        Demo::decode_canonical(&refused_bytes),
        Err(DemoRefusal::NotAssembled(AssemblyRefusal)),
    );
}
"#;

/// Claim: generated codec Rust for every wire shape and cardinality compiles and executes its public round-trip and refusal behavior.
///
/// Population: one checked-assembly surface covering five shapes, three cardinalities, and every generated refusal arm that can be reached by bounded hostile material.
/// Hostile controls: trailing, malformed presence, foreign slot, nested refusal, member refusal, invalid UTF-8, count overflow, overlong frame, truncation, and checked-assembly refusal.
/// Denominator: the generated source is compiled by Rust 1.98 and its standalone executable must pass every assertion.
/// Evidence ceiling: this is one representative type roster on the local Windows host, not arbitrary downstream types, Wasm, Linux, packaging, or performance.
#[test]
fn generated_codec_rust_compiles_executes_and_refuses_hostile_bytes() -> Result<(), String> {
    let surface = codec_surface(&codec_content(CodecDirection::RoundTrip)?)
        .map_err(|refusal| refusal.to_string())?
        .inspected();
    let mut source = String::from(SPECIMEN_DECLARATIONS);
    source.push_str(&surface);
    source.push_str(SPECIMEN_ASSERTIONS);
    compile_and_run(&source)
}

/// Claim: published-module placement wraps the complete codec surface in one public module that imports its parent scope and remains executable.
///
/// Population: the same representative checked-assembly surface used by the generated-behavior crossing, moved from the declaration site into one named module.
/// Hostile control: the source assertions stand outside the generated module, so compilation or execution fails if the wrapper drops parent-scope access, module visibility, the generated refusal, or either codec road.
/// Denominator: the public `CodecPlacement::PublishedModule` route through `codec_surface` and Rust 1.98 compilation.
/// Evidence ceiling: this establishes one module spelling and one representative owner, not arbitrary surrounding imports or nested landing modules.
#[test]
fn published_module_placement_compiles_and_executes_from_its_parent_scope() -> Result<(), String> {
    let mut content = codec_content(CodecDirection::RoundTrip)?;
    content.placement = CodecPlacement::PublishedModule {
        spelling: ModuleSpelling::spelled("demo_codec").map_err(|refusal| refusal.to_string())?,
    };
    let surface = codec_surface(&content)
        .map_err(|refusal| refusal.to_string())?
        .inspected();
    for spelling in [
        "pub mod demo_codec",
        "use super :: *",
        "The canonical encode and decode roads",
        "pub enum DemoRefusal",
        "impl Demo",
    ] {
        assert!(surface.contains(spelling), "the wrapper omits {spelling}");
    }

    let mut source = String::from(SPECIMEN_DECLARATIONS);
    source.push_str(&surface);
    source.push_str("use demo_codec::DemoRefusal;");
    source.push_str(SPECIMEN_ASSERTIONS);
    compile_and_run(&source)
}
