//! Codec declaration grammar lowered directly into the existing codec owner.

use super::{RecipeCodec, RecipeError, RecipeIssue, grammar};
use crate::bounded::Bounded;
use crate::codec::{
    AssemblyPosture, Cardinality, CodecAssembly, CodecContent, CodecDirection, CodecMember,
    CodecMemberShape, CodecPlacement, CodecShape, CodecTypePath, PathRooting,
};
use crate::token::{
    CaptureCursor, CaptureReadRefusal, CapturedDelimiter, CapturedSpacing, SpanHandle,
};

pub(super) fn read_codecs(cursor: &mut CaptureCursor<'_>) -> Result<Vec<RecipeCodec>, RecipeError> {
    if cursor.next_word() != Some("codecs") {
        return Ok(Vec::new());
    }
    cursor.word("codecs").map_err(grammar)?;
    let captured = cursor
        .group(CapturedDelimiter::Brace)
        .map_err(grammar)?
        .trailing_separated::<_, { super::super::CODEC_LIMIT }>(';', read_codec)
        .map_err(grammar)?
        .as_slice()
        .to_vec();
    cursor
        .punctuation(';', CapturedSpacing::Alone)
        .map_err(grammar)?;
    captured.into_iter().map(CapturedCodec::informed).collect()
}

#[derive(Clone)]
struct CapturedCodec {
    name: String,
    owner: String,
    direction: CodecDirection,
    refusal: String,
    assembly: CapturedAssembly,
    members: Vec<CapturedCodecMember>,
    at: SpanHandle,
    refusal_at: SpanHandle,
    direction_at: SpanHandle,
}

#[derive(Clone)]
struct CapturedAssembly {
    road: String,
    posture: CapturedAssemblyPosture,
}

#[derive(Clone)]
enum CapturedAssemblyPosture {
    Total,
    Checked(CapturedPath),
}

#[derive(Clone)]
struct CapturedCodecMember {
    name: String,
    held_as: CapturedPath,
    shape: CodecMemberShape,
    cardinality: Cardinality,
}

#[derive(Clone)]
struct CapturedPath {
    rooting: PathRooting,
    segments: Vec<String>,
}

fn read_codec(cursor: &mut CaptureCursor<'_>) -> Result<CapturedCodec, CaptureReadRefusal> {
    let (name_token, name) = cursor.identifier()?;
    let mut owner = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (_owner_token, owner_name) = owner.identifier()?;
    owner.finish()?;

    let mut body = cursor.group(CapturedDelimiter::Brace)?;
    body.word("direction")?;
    let (direction, direction_at) = read_direction(&mut body)?;
    body.punctuation(';', CapturedSpacing::Alone)?;
    body.word("refusal")?;
    let (refusal, refusal_at) = read_identifier_group(&mut body)?;
    body.punctuation(';', CapturedSpacing::Alone)?;
    body.word("assembly")?;
    let assembly = read_assembly(&mut body)?;
    body.punctuation(';', CapturedSpacing::Alone)?;
    body.word("members")?;
    let members = body
        .group(CapturedDelimiter::Brace)?
        .trailing_separated::<_, { crate::codec::CODEC_MEMBER_LIMIT }>(';', read_member)?
        .as_slice()
        .to_vec();
    body.punctuation(';', CapturedSpacing::Alone)?;
    body.finish()?;
    Ok(CapturedCodec {
        name: name.to_owned(),
        owner: owner_name.to_owned(),
        direction,
        refusal,
        assembly,
        members,
        at: name_token.span(),
        refusal_at,
        direction_at,
    })
}

fn read_direction(
    cursor: &mut CaptureCursor<'_>,
) -> Result<(CodecDirection, SpanHandle), CaptureReadRefusal> {
    let mut group = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (token, spelling) = group.identifier()?;
    let direction = match spelling {
        "encode" => CodecDirection::Encode,
        "decode" => CodecDirection::Decode,
        "round_trip" => CodecDirection::RoundTrip,
        _ => return Err(posture_word(token.span(), "encode, decode, or round_trip")),
    };
    group.finish()?;
    Ok((direction, token.span()))
}

fn read_identifier_group(
    cursor: &mut CaptureCursor<'_>,
) -> Result<(String, SpanHandle), CaptureReadRefusal> {
    let mut group = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (token, spelling) = group.identifier()?;
    group.finish()?;
    Ok((spelling.to_owned(), token.span()))
}

fn read_assembly(cursor: &mut CaptureCursor<'_>) -> Result<CapturedAssembly, CaptureReadRefusal> {
    let mut assembly = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (_road_token, road) = assembly.identifier()?;
    assembly.punctuation(',', CapturedSpacing::Alone)?;
    let (posture_token, posture) = assembly.identifier()?;
    let posture = match posture {
        "total" => CapturedAssemblyPosture::Total,
        "checked" => {
            let path = assembly.group(CapturedDelimiter::Parenthesis)?;
            CapturedAssemblyPosture::Checked(read_path(path)?)
        }
        _ => return Err(posture_word(posture_token.span(), "total or checked")),
    };
    assembly.finish()?;
    Ok(CapturedAssembly {
        road: road.to_owned(),
        posture,
    })
}

fn read_member(cursor: &mut CaptureCursor<'_>) -> Result<CapturedCodecMember, CaptureReadRefusal> {
    let (_name_token, name) = cursor.identifier()?;
    cursor.punctuation(':', CapturedSpacing::Alone)?;
    let held_as = read_path_until_arrow(cursor)?;
    cursor.fat_arrow()?;
    let (shape_token, shape) = cursor.identifier()?;
    let shape = match shape {
        "count" => CodecMemberShape::Count,
        "bytes" => CodecMemberShape::Bytes,
        "text" => CodecMemberShape::Text,
        "closed_choice" => CodecMemberShape::ClosedChoice,
        "nested" => CodecMemberShape::Nested,
        _ => return Err(posture_word(shape_token.span(), "one codec member shape")),
    };
    let mut group = cursor.group(CapturedDelimiter::Parenthesis)?;
    let (cardinality_token, cardinality_name) = group.identifier()?;
    let cardinality = match cardinality_name {
        "required" => Cardinality::Required,
        "optional" => Cardinality::Optional,
        "repeated" => Cardinality::Repeated,
        _ => {
            return Err(posture_word(
                cardinality_token.span(),
                "required, optional, or repeated",
            ));
        }
    };
    group.finish()?;
    Ok(CapturedCodecMember {
        name: name.to_owned(),
        held_as,
        shape,
        cardinality,
    })
}

fn read_path_until_arrow(
    cursor: &mut CaptureCursor<'_>,
) -> Result<CapturedPath, CaptureReadRefusal> {
    let rooting = match cursor.next_word() {
        Some("crate") => {
            cursor.word("crate")?;
            read_separator(cursor)?;
            PathRooting::CrateAbsolute
        }
        Some("self") => {
            cursor.word("self")?;
            read_separator(cursor)?;
            PathRooting::SelfScoped
        }
        Some("super") => {
            cursor.word("super")?;
            read_separator(cursor)?;
            PathRooting::ParentScoped
        }
        Some(_) | None => PathRooting::InScope,
    };
    let mut segments = Vec::new();
    let (_first_token, first) = cursor.identifier()?;
    segments.push(first.to_owned());
    while cursor
        .next_token()
        .is_some_and(|token| token.joint_punct() == Some(':'))
    {
        read_separator(cursor)?;
        let (_segment_token, segment) = cursor.identifier()?;
        segments.push(segment.to_owned());
    }
    Ok(CapturedPath { rooting, segments })
}

fn read_path(mut cursor: CaptureCursor<'_>) -> Result<CapturedPath, CaptureReadRefusal> {
    let path = read_path_until_arrow(&mut cursor)?;
    cursor.finish()?;
    Ok(path)
}

fn read_separator(cursor: &mut CaptureCursor<'_>) -> Result<(), CaptureReadRefusal> {
    cursor.punctuation(':', CapturedSpacing::Joint)?;
    cursor.punctuation(':', CapturedSpacing::Alone)?;
    Ok(())
}

fn posture_word(at: SpanHandle, expected: &str) -> CaptureReadRefusal {
    CaptureReadRefusal::projected(
        crate::token::CaptureReadIssue::Unexpected(crate::token::CaptureExpectation::Word(
            expected.to_owned(),
        )),
        Some(at),
    )
}

impl CapturedCodec {
    fn informed(self) -> Result<RecipeCodec, RecipeError> {
        let Self {
            name,
            owner,
            direction,
            refusal,
            assembly,
            members,
            at,
            refusal_at,
            direction_at,
        } = self;
        let owner = CodecTypePath::spelled(PathRooting::ParentScoped, vec![owner.clone()])
            .map_err(|error| codec_refusal(name.as_str(), at, error.to_string()))?;
        let posture = match assembly.posture {
            CapturedAssemblyPosture::Total => AssemblyPosture::Total,
            CapturedAssemblyPosture::Checked(path) => AssemblyPosture::Checked {
                refusal: path
                    .informed()
                    .map_err(|reason| codec_refusal(name.as_str(), at, reason))?,
            },
        };
        let assembly = CodecAssembly::stated(assembly.road.as_str(), posture)
            .map_err(|error| codec_refusal(name.as_str(), at, error.to_string()))?;
        let members = members
            .into_iter()
            .map(|member| {
                let held_as = member
                    .held_as
                    .informed()
                    .map_err(|reason| codec_refusal(name.as_str(), at, reason))?;
                CodecMember::declared(
                    member.name.as_str(),
                    held_as,
                    member.shape,
                    member.cardinality,
                )
                .map_err(|error| codec_refusal(name.as_str(), at, error.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let shape = CodecShape::declared(owner, refusal.as_str(), assembly, members)
            .map_err(|error| codec_refusal(name.as_str(), at, error.to_string()))?;
        Ok(RecipeCodec::informed(
            name,
            CodecContent {
                shape,
                direction,
                placement: CodecPlacement::AtDeclarationSite,
                schema: None,
                byte_role: None,
                assumptions: Bounded::empty(),
            },
            at,
            refusal_at,
            direction_at,
        ))
    }
}

fn codec_refusal(name: &str, at: SpanHandle, reason: String) -> RecipeError {
    RecipeError::at(
        RecipeIssue::CodecDeclaration {
            name: name.to_owned(),
            reason,
        },
        Some(at),
    )
}

impl CapturedPath {
    fn informed(self) -> Result<CodecTypePath, String> {
        CodecTypePath::spelled(self.rooting, self.segments).map_err(|refusal| refusal.to_string())
    }
}
