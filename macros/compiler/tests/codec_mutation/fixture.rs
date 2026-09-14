//! Caller declarations and source extraction for the codec-pressure witness.

use macroonz_compiler::codec::{
    AssemblyPosture, Cardinality, CodecAssembly, CodecContent, CodecDirection, CodecMember,
    CodecMemberShape, CodecPlacement, CodecShape, CodecTypePath, PathRooting,
};
use macroonz_compiler::descriptor::mutation::{
    Address, Declaration, FactMapping, FamilySlug, Permission, Policy,
};
use macroonz_compiler::descriptor::{ModuleName, Name, TypeName};
use macroonz_compiler::{Bounded, GeneratedToken};

pub(super) fn declaration() -> Result<Declaration, String> {
    let name = |stem| Name::named("codec-pressure", stem).map_err(|refusal| refusal.to_string());
    let fact = name("wire-members")?;
    let claim = name("wire-order")?;
    let permission = Permission::permitted(
        claim.clone(),
        vec![
            FamilySlug::declared("declared-order-permutation")
                .map_err(|refusal| refusal.to_string())?,
        ],
    )
    .map_err(|refusal| refusal.to_string())?;
    let policy = Policy::declared(
        name("record")?,
        vec![FactMapping {
            fact: fact.clone(),
            claim,
        }],
        vec![permission],
    )
    .map_err(|refusal| refusal.to_string())?;
    Ok(Declaration::captured(
        Address {
            module: ModuleName::declared("pressure").map_err(|refusal| refusal.to_string())?,
            refusal: TypeName::declared("PressureRefusal")
                .map_err(|refusal| refusal.to_string())?,
            support: None,
        },
        policy,
        name("members")?,
        fact,
    ))
}

pub(super) fn content(members: &[(&str, &str)]) -> Result<CodecContent, String> {
    let path = |name: &str| {
        CodecTypePath::spelled(PathRooting::InScope, vec![name.to_owned()])
            .map_err(|refusal| refusal.to_string())
    };
    let members = members
        .iter()
        .map(|(name, kind)| {
            CodecMember::declared(
                name,
                path(kind)?,
                CodecMemberShape::Count,
                Cardinality::Required,
            )
            .map_err(|refusal| refusal.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(CodecContent {
        shape: CodecShape::declared(
            path("Record")?,
            "DecodeRefusal",
            CodecAssembly::stated("assembled", AssemblyPosture::Total)
                .map_err(|refusal| refusal.to_string())?,
            members,
        )
        .map_err(|refusal| refusal.to_string())?,
        direction: CodecDirection::RoundTrip,
        placement: CodecPlacement::AtDeclarationSite,
        schema: None,
        byte_role: None,
        assumptions: Bounded::empty(),
    })
}

pub(super) fn source(tokens: &[GeneratedToken]) -> Result<&str, String> {
    match tokens {
        [GeneratedToken::Text(source)] => Ok(source),
        _ => Err("codec mutation material must be one source literal".to_owned()),
    }
}
