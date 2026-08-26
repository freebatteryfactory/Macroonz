use super::support::{emitted, shadowed, shadowed_raw};

/// A lawful choice becomes both faces of each chosen name, in authored order, and nothing else.
#[test]
fn a_choice_becomes_both_faces_of_each_name() -> Result<(), ()> {
    let expansion = shadowed("Arc, Mutex").ok_or(())?.ok().ok_or(())?;
    let text = emitted(&expansion).ok_or(())?;
    assert_eq!(text.matches("pub use").count(), 4usize);
    assert_eq!(text.matches("cfg").count(), 4usize);
    assert_eq!(text.matches("not").count(), 2usize);
    assert_eq!(text.matches("Arc").count(), 2usize);
    assert_eq!(text.matches("Mutex").count(), 2usize);
    assert_eq!(text.matches("std").count(), 2usize);
    assert_eq!(text.matches("loom").count(), 6usize);
    assert!(text.contains(":: renamed_facade :: loom"));
    let arc = text.find("Arc").ok_or(())?;
    let mutex = text.find("Mutex").ok_or(())?;
    assert!(arc < mutex, "the rendering lost authored choice order");
    Ok(())
}

/// Trailing separators are lawful at both grammar levels, and clause order carries no authority.
#[test]
fn trailing_separators_and_reversed_clauses_are_lawful() -> Result<(), ()> {
    let expansion =
        shadowed_raw("names = [thread,], loom = one::two::three::four::five::six::seven::eight,")
            .ok_or(())?
            .ok()
            .ok_or(())?;
    let text = emitted(&expansion).ok_or(())?;
    assert_eq!(text.matches("pub use").count(), 2usize);
    assert!(text.contains(":: one :: two :: three :: four :: five :: six :: seven :: eight"));
    assert_eq!(text.matches("thread").count(), 2usize);
    Ok(())
}
