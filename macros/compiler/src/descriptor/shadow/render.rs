//! Rendering the two faces of every chosen name.
//!
//! For each row, exactly the pair its author would have written by hand: the ordinary face behind `#[cfg(not(loom))]` over the standard-library path, and the shadowed face behind `#[cfg(loom)]` over the shadow path.
//! Emission order is authored order, and nothing else is written.

use super::Shadows;
use crate::bounded::Overflow;
use crate::token::{GeneratedDelimiter, GeneratedToken};

/// Which of a row's two faces one arm writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Face {
    /// The ordinary build's face, behind `#[cfg(not(loom))]`.
    Ordinary,
    /// The shadowed build's face, behind `#[cfg(loom)]`.
    Shadowed,
}

/// The declaration-site tokens one shadow payload renders to.
///
/// # Errors
///
/// Returns [`Overflow`] where a composed group carries more tokens than the declared magnitude admits.
pub fn faces(shadows: &Shadows) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = Vec::new();
    for row in shadows.chosen() {
        arm(&mut tokens, Face::Ordinary, row.std_path())?;
        arm(&mut tokens, Face::Shadowed, row.loom_path())?;
    }
    Ok(tokens)
}

/// One `#[cfg(…)] pub use <path>;` arm.
fn arm(into: &mut Vec<GeneratedToken>, face: Face, path: &[&'static str]) -> Result<(), Overflow> {
    into.push(GeneratedToken::alone('#'));
    let condition = match face {
        Face::Ordinary => vec![
            GeneratedToken::word("not"),
            GeneratedToken::group(
                GeneratedDelimiter::Parenthesis,
                vec![GeneratedToken::word("loom")],
            )?,
        ],
        Face::Shadowed => vec![GeneratedToken::word("loom")],
    };
    let cfg = vec![
        GeneratedToken::word("cfg"),
        GeneratedToken::group(GeneratedDelimiter::Parenthesis, condition)?,
    ];
    into.push(GeneratedToken::group(GeneratedDelimiter::Bracket, cfg)?);
    into.push(GeneratedToken::word("pub"));
    into.push(GeneratedToken::word("use"));
    for (position, segment) in path.iter().enumerate() {
        if position > 0 {
            into.push(GeneratedToken::joint(':'));
            into.push(GeneratedToken::alone(':'));
        }
        into.push(GeneratedToken::word(segment));
    }
    into.push(GeneratedToken::alone(';'));
    Ok(())
}
