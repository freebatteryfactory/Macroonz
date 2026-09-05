//! Rendering the exploration module one concurrency declaration compresses.
//!
//! One generic function per row: the strand set and the contract arrive at the call, the declared facts are spelled once here, and the pair that comes back is the reading beside its concluded verdict.

use super::{ConcurrencyDeclaration, ExplorationRow};
use crate::bounded::Overflow;
use crate::descriptor::DirectBinding;
use crate::descriptor::emitting::{
    derive_attribute, direct_path, doc_attribute, fallible_return, from_impl, owned_direct_path,
};
use crate::descriptor::fault::CONCURRENCY_FAULT_ARMS;
use crate::token::{GeneratedDelimiter, GeneratedToken, absolute_path};

/// The declaration-site tokens one concurrency payload renders to.
///
/// # Errors
///
/// Returns [`Overflow`] where a composed group carries more tokens than the declared magnitude admits.
pub fn rendered(declaration: &ConcurrencyDeclaration) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = Vec::new();
    doc_attribute(
        "The generated explorations: one generic function per declared row, each handing back the reading beside its concluded verdict.",
        &mut tokens,
    )?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(declaration.module()));
    let mut body = Vec::new();
    fault_enum(declaration.harness(), &mut body)?;
    for row in declaration.rows() {
        row_fn(declaration, row, &mut body)?;
    }
    tokens.push(GeneratedToken::group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The generated fault enum and its `From` impls.
fn fault_enum(harness: &DirectBinding, into: &mut Vec<GeneratedToken>) -> Result<(), Overflow> {
    doc_attribute(
        "Everything a generated exploration can refuse, carried as itself.",
        into,
    )?;
    derive_attribute(&["Debug", "Clone", "PartialEq", "Eq"], into)?;
    into.push(GeneratedToken::word("pub"));
    into.push(GeneratedToken::word("enum"));
    into.push(GeneratedToken::word("Fault"));
    let mut arms = Vec::new();
    for (arm, path, documentation) in &CONCURRENCY_FAULT_ARMS {
        doc_attribute(documentation, &mut arms)?;
        arms.push(GeneratedToken::word(arm));
        let mut carried = Vec::new();
        direct_path(harness, path, &mut carried);
        arms.push(GeneratedToken::group(
            GeneratedDelimiter::Parenthesis,
            carried,
        )?);
        arms.push(GeneratedToken::alone(','));
    }
    into.push(GeneratedToken::group(GeneratedDelimiter::Brace, arms)?);
    for (arm, path, _documentation) in &CONCURRENCY_FAULT_ARMS {
        from_impl(owned_direct_path(harness, path), "Fault", arm, into)?;
    }
    Ok(())
}

/// One generated exploration function.
fn row_fn(
    declaration: &ConcurrencyDeclaration,
    row: &ExplorationRow,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    doc_attribute(&format!("The declared `{}` exploration.", row.name()), into)?;
    into.push(GeneratedToken::word("pub"));
    into.push(GeneratedToken::word("fn"));
    into.push(GeneratedToken::word(row.name()));
    generics(into);
    parameters(declaration.harness(), into)?;
    let mut ok_seat = Vec::new();
    let mut pair = Vec::new();
    direct_path(
        declaration.harness(),
        &["interleave", "ExplorationReading"],
        &mut pair,
    );
    pair.push(GeneratedToken::alone(','));
    direct_path(
        declaration.harness(),
        &["report", "TrialConclusion"],
        &mut pair,
    );
    ok_seat.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        pair,
    )?);
    fallible_return(ok_seat, "Fault", into);
    let mut body = Vec::new();
    explored_let(declaration, row, &mut body)?;
    concluded_let(declaration.harness(), &mut body)?;
    body.extend(absolute_path(&["core", "result", "Result", "Ok"]));
    body.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::group(
            GeneratedDelimiter::Parenthesis,
            vec![
                GeneratedToken::word("reading"),
                GeneratedToken::alone(','),
                GeneratedToken::word("conclusion"),
            ],
        )?],
    )?);
    into.push(GeneratedToken::group(GeneratedDelimiter::Brace, body)?);
    Ok(())
}

/// The `<State, Command: ::core::clone::Clone>` generic seat.
fn generics(into: &mut Vec<GeneratedToken>) {
    into.push(GeneratedToken::alone('<'));
    into.push(GeneratedToken::word("State"));
    into.push(GeneratedToken::alone(','));
    into.push(GeneratedToken::word("Command"));
    into.push(GeneratedToken::alone(':'));
    into.extend(absolute_path(&["core", "clone", "Clone"]));
    into.push(GeneratedToken::alone('>'));
}

/// The `(strands: &StrandSet<Command>, contract: &TransitionContract<State, Command>)` parameter seat.
fn parameters(harness: &DirectBinding, into: &mut Vec<GeneratedToken>) -> Result<(), Overflow> {
    let mut listed = vec![GeneratedToken::word("strands"), GeneratedToken::alone(':')];
    listed.push(GeneratedToken::alone('&'));
    direct_path(harness, &["interleave", "StrandSet"], &mut listed);
    listed.push(GeneratedToken::alone('<'));
    listed.push(GeneratedToken::word("Command"));
    listed.push(GeneratedToken::alone('>'));
    listed.push(GeneratedToken::alone(','));
    listed.push(GeneratedToken::word("contract"));
    listed.push(GeneratedToken::alone(':'));
    listed.push(GeneratedToken::alone('&'));
    direct_path(harness, &["properties", "TransitionContract"], &mut listed);
    listed.push(GeneratedToken::alone('<'));
    listed.push(GeneratedToken::word("State"));
    listed.push(GeneratedToken::alone(','));
    listed.push(GeneratedToken::word("Command"));
    listed.push(GeneratedToken::alone('>'));
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        listed,
    )?);
    Ok(())
}

/// The `let reading = …explored(…)?;` statement.
fn explored_let(
    declaration: &ConcurrencyDeclaration,
    row: &ExplorationRow,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    into.push(GeneratedToken::word("let"));
    into.push(GeneratedToken::word("reading"));
    into.push(GeneratedToken::alone('='));
    direct_path(declaration.harness(), &["interleave", "explored"], into);
    let mut arguments = vec![GeneratedToken::word("strands"), GeneratedToken::alone(',')];
    arguments.push(GeneratedToken::word("contract"));
    arguments.push(GeneratedToken::alone(','));
    direct_path(
        declaration.harness(),
        &["interleave", "ExplorationBound", "declared"],
        &mut arguments,
    );
    arguments.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::number(u64::from(row.interleavings())),
            GeneratedToken::alone(','),
            GeneratedToken::number(u64::from(row.samples())),
        ],
    )?);
    arguments.push(GeneratedToken::alone('?'));
    arguments.push(GeneratedToken::alone(','));
    direct_path(
        declaration.harness(),
        &["descriptor", "PopulationRef", "named"],
        &mut arguments,
    );
    arguments.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::text(declaration.namespace()),
            GeneratedToken::alone(','),
            GeneratedToken::text(row.population()),
        ],
    )?);
    arguments.push(GeneratedToken::alone('?'));
    arguments.push(GeneratedToken::alone(','));
    direct_path(
        declaration.harness(),
        &["generate", "RootSeed", "declared"],
        &mut arguments,
    );
    arguments.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::number(row.seed())],
    )?);
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        arguments,
    )?);
    into.push(GeneratedToken::alone('?'));
    into.push(GeneratedToken::alone(';'));
    Ok(())
}

/// The `let conclusion = …concluded(&reading);` statement.
fn concluded_let(harness: &DirectBinding, into: &mut Vec<GeneratedToken>) -> Result<(), Overflow> {
    into.push(GeneratedToken::word("let"));
    into.push(GeneratedToken::word("conclusion"));
    into.push(GeneratedToken::alone('='));
    direct_path(harness, &["interleave", "concluded"], into);
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::alone('&'), GeneratedToken::word("reading")],
    )?);
    into.push(GeneratedToken::alone(';'));
    Ok(())
}
