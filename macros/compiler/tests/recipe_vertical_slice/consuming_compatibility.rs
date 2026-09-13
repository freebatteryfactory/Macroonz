//! An independently written canonical specimen for the unconfigured typestate contract.

use super::support::bake;
use macroonz_compiler::CanonicalContent;

const SOURCE: &str = "pub mod sample { pub enum Phase { Ready } bake! { vocabularies { Phase; }; projections { typestate(Phase); }; } }";

#[test]
fn unconfigured_typestate_keeps_its_independently_written_canonical_account() -> Result<(), ()> {
    let standard = bake(SOURCE)?;
    assert_eq!(
        standard
            .projection()
            .plan()
            .content()
            .canonical_content_bytes(),
        expected()?
    );
    Ok(())
}

fn expected() -> Result<Vec<u8>, ()> {
    let mut bytes = Vec::new();
    field(b"sample", &mut bytes)?;
    let mut head = Vec::new();
    for word in [b"pub".as_slice(), b"mod", b"sample"] {
        identifier(word, &mut head)?;
    }
    field(&head, &mut bytes)?;
    let mut body = Vec::new();
    for word in [b"pub".as_slice(), b"enum", b"Phase"] {
        identifier(word, &mut body)?;
    }
    body.extend([4, 1]);
    body.extend(1u64.to_be_bytes());
    identifier(b"Ready", &mut body)?;
    field(&body, &mut bytes)?;
    bytes.extend(1u64.to_be_bytes());
    field(b"Phase", &mut bytes)?;
    bytes.extend(1u64.to_be_bytes());
    field(b"Ready", &mut bytes)?;
    bytes.extend(0u64.to_be_bytes());
    bytes.extend(0u64.to_be_bytes());
    bytes.push(0);
    for (name, destination) in [
        ("companions", "declaration-site"),
        ("relation-tables", "declaration-site"),
        ("dispatch", "declaration-site"),
        ("compile-contract", "test-carrier"),
        ("declaration-conformance", "test-carrier"),
        ("typestate", "declaration-site"),
        ("trials", "declaration-site"),
        ("mutation", "declaration-site"),
        ("benchmarks", "declaration-site"),
        ("network", "declaration-site"),
        ("concurrency", "declaration-site"),
        ("codec", "declaration-site"),
    ] {
        field(name.as_bytes(), &mut bytes)?;
        if name == "typestate" {
            bytes.push(1);
            field(b"configuration", &mut bytes)?;
            bytes.extend([0, 1]);
            field(b"Phase", &mut bytes)?;
            bytes.push(0);
            bytes.extend(0u64.to_be_bytes());
        } else {
            bytes.push(0);
        }
        field(destination.as_bytes(), &mut bytes)?;
        bytes.push(0);
    }
    bytes.push(0);
    Ok(bytes)
}

fn field(value: &[u8], into: &mut Vec<u8>) -> Result<(), ()> {
    into.extend(u64::try_from(value.len()).map_err(|_| ())?.to_be_bytes());
    into.extend(value);
    Ok(())
}

fn identifier(value: &[u8], into: &mut Vec<u8>) -> Result<(), ()> {
    into.push(1);
    field(value, into)
}
