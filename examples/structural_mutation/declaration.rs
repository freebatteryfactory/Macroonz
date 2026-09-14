//! Explicit caller-owned names, fact mapping and permission for one structural site.

use macroonz::compiler::descriptor::mutation::{
    Address, Declaration, FactMapping, FamilySlug, Permission, Policy,
};
use macroonz::compiler::descriptor::{ModuleName, Name, TypeName};

pub(super) fn declared(family: &str) -> Result<Declaration, String> {
    let name = |stem| Name::named("structural-example", stem).map_err(|error| error.to_string());
    let fact = name("selected-structure")?;
    let claim = name("independent-observation")?;
    let permission = Permission::permitted(
        claim.clone(),
        vec![FamilySlug::declared(family).map_err(|error| error.to_string())?],
    )
    .map_err(|error| error.to_string())?;
    let policy = Policy::declared(
        name("declared-subject")?,
        vec![FactMapping {
            fact: fact.clone(),
            claim,
        }],
        vec![permission],
    )
    .map_err(|error| error.to_string())?;
    Ok(Declaration::captured(
        Address {
            module: ModuleName::declared("pressure").map_err(|error| error.to_string())?,
            refusal: TypeName::declared("PressureRefusal").map_err(|error| error.to_string())?,
            support: None,
        },
        policy,
        name("selected-site")?,
        fact,
    ))
}
