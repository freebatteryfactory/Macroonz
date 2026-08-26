use super::support::{emitted, shadowed, shadowed_raw};
use macroonz_compiler::Phase;

/// A name outside the roster, a doubled name, a choice that is not a name, and an empty declaration each refuse at capture.
#[test]
fn a_malformed_choice_refuses_at_capture() -> Result<(), ()> {
    for source in ["Telepathy", "Arc, Arc", "5", "Arc Mutex", ""] {
        let refusal = shadowed(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
    }
    Ok(())
}

/// Each owned grammar disagreement retains its typed cause in the diagnostic projection and its token coordinate.
#[test]
fn grammar_refusals_name_the_established_cause() -> Result<(), ()> {
    let cases = [
        (
            ", loom = loom, names = [Arc]",
            "a separator stands where no clause does",
        ),
        (
            "loom = loom,, names = [Arc]",
            "a separator stands where no clause does",
        ),
        (
            "loom = loom, names = [Arc], mystery = value",
            "a clause is not one the grammar declares",
        ),
        (
            "loom = loom, names = []",
            "the declaration chooses no name at all",
        ),
        (
            "loom = loom, names = [Arc, Arc]",
            "one name is chosen twice",
        ),
    ];
    for (source, cause) in cases {
        let refusal = shadowed_raw(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
        assert!(refusal.summary().contains(cause), "{source} lost {cause}");
        assert!(
            refusal.summary().contains("(at "),
            "{source} carries no coordinate"
        );
    }
    Ok(())
}

/// A direct shadow binding is required, singular, bounded, and a fully consumed Rust path.
#[test]
fn a_direct_shadow_binding_refuses_every_unwritable_shape() -> Result<(), ()> {
    let lawful = shadowed_raw("loom = a::b::c::d::e::f::g::h, names = [Arc]")
        .ok_or(())?
        .ok()
        .ok_or(())?;
    assert!(
        emitted(&lawful)
            .ok_or(())?
            .contains(":: a :: b :: c :: d :: e :: f :: g :: h")
    );

    let malformed = [
        "names = [Arc]",
        "loom = , names = [Arc]",
        "loom = type, names = [Arc]",
        "loom = renamed:loom, names = [Arc]",
        "loom = renamed::, names = [Arc]",
        "loom = one, loom = two, names = [Arc]",
        "loom = a::b::c::d::e::f::g::h::i, names = [Arc]",
    ];
    for source in malformed {
        let refusal = shadowed_raw(source).ok_or(())?.err().ok_or(())?;
        assert_eq!(refusal.phase(), Phase::Capture, "{source} did not refuse");
        assert!(
            refusal.summary().contains("(at "),
            "{source} carries no coordinate"
        );
    }
    Ok(())
}
