//! Rendering the builder module one network declaration compresses.
//!
//! The module holds one generated fault enum, `topology()`, and one function per declared schedule — exactly what an author would have written by hand against the harness's network home, with every refusal traveling as itself.

use super::{DisciplineRow, FaultRow, LinkRow, NetworkDeclaration, ScheduleRow};
use crate::bounded::Overflow;
use crate::descriptor::DirectBinding;
use crate::descriptor::emitting::{
    derive_attribute, direct_path, doc_attribute, fallible_return, from_impl,
};
use crate::token::{GeneratedDelimiter, GeneratedToken, absolute_path};

/// The names the generated module writes beside the authored ones, which no schedule may take.
///
/// One seat for both halves: the emission below spells these constants, and the capture refuses an authored schedule that spells one of them — so the roster cannot drift from the items it reserves.
pub(super) const RESERVED: [&str; 2] = [TOPOLOGY_ROAD, FAULT_ENUM];

/// The generated topology function's name.
const TOPOLOGY_ROAD: &str = "topology";

/// The generated fault enum's name.
const FAULT_ENUM: &str = "Fault";

/// The fault enum's arms: the arm, the refusal it carries, and its stated doc.
const FAULT_ARMS: [(&str, [&str; 2], &str); 4] = [
    (
        "Name",
        ["descriptor", "NameRefusal"],
        "A declared name was refused by the name vocabulary.",
    ),
    (
        "Topology",
        ["network", "TopologyRefusal"],
        "The declared topology was refused by its own guard.",
    ),
    (
        "Span",
        ["network", "TickSpanRefusal"],
        "A declared delay span was refused by its own guard.",
    ),
    (
        "Schedule",
        ["network", "NetworkScheduleRefusal"],
        "A declared schedule was refused by its own guard.",
    ),
];

/// The declaration-site tokens one network payload renders to.
///
/// # Errors
///
/// Returns [`Overflow`] where a composed group carries more tokens than the declared magnitude admits.
pub fn rendered(declaration: &NetworkDeclaration) -> Result<Vec<GeneratedToken>, Overflow> {
    let mut tokens = Vec::new();
    doc_attribute(
        "The generated network builders: the declared topology, and one function per declared schedule.",
        &mut tokens,
    )?;
    tokens.push(GeneratedToken::word("pub"));
    tokens.push(GeneratedToken::word("mod"));
    tokens.push(GeneratedToken::word(declaration.module()));
    let mut body = Vec::new();
    fault_enum(declaration.harness(), &mut body)?;
    topology_fn(declaration, &mut body)?;
    for schedule in declaration.schedules() {
        schedule_fn(declaration, schedule, &mut body)?;
    }
    tokens.push(GeneratedToken::group(GeneratedDelimiter::Brace, body)?);
    Ok(tokens)
}

/// The generated fault enum and its `From` impls.
fn fault_enum(harness: &DirectBinding, into: &mut Vec<GeneratedToken>) -> Result<(), Overflow> {
    doc_attribute(
        "Everything a generated builder can refuse, carried as itself.",
        into,
    )?;
    derive_attribute(&["Debug", "Clone", "PartialEq", "Eq"], into)?;
    into.push(GeneratedToken::word("pub"));
    into.push(GeneratedToken::word("enum"));
    into.push(GeneratedToken::word(FAULT_ENUM));
    let mut arms = Vec::new();
    for (arm, path, doc) in &FAULT_ARMS {
        doc_attribute(doc, &mut arms)?;
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
    for (arm, path, _doc) in &FAULT_ARMS {
        from_impl(harness_path(harness, path), FAULT_ENUM, arm, into)?;
    }
    Ok(())
}

/// One `<harness>::network::NodeRef::declared(<harness>::descriptor::NamespacedName::named("<ns>", "<node>")?)` expression.
fn node_expr(
    harness: &DirectBinding,
    namespace: &str,
    node: &str,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    direct_path(harness, &["network", "NodeRef", "declared"], into);
    let mut inner = Vec::new();
    direct_path(
        harness,
        &["descriptor", "NamespacedName", "named"],
        &mut inner,
    );
    inner.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::text(namespace),
            GeneratedToken::alone(','),
            GeneratedToken::text(node),
        ],
    )?);
    inner.push(GeneratedToken::alone('?'));
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        inner,
    )?);
    Ok(())
}

/// One `<harness>::network::Link::between(<from>, <to>)` expression.
fn link_expr(
    harness: &DirectBinding,
    namespace: &str,
    link: &LinkRow,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    direct_path(harness, &["network", "Link", "between"], into);
    let mut inner = Vec::new();
    node_expr(harness, namespace, link.from(), &mut inner)?;
    inner.push(GeneratedToken::alone(','));
    node_expr(harness, namespace, link.to(), &mut inner)?;
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        inner,
    )?);
    Ok(())
}

/// One `::std::vec::Vec::from([<members>])` expression, or `::std::vec::Vec::new()` where nothing is listed.
fn vec_expr(
    members: Vec<Vec<GeneratedToken>>,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    if members.is_empty() {
        into.extend(absolute_path(&["std", "vec", "Vec", "new"]));
        into.push(GeneratedToken::group(
            GeneratedDelimiter::Parenthesis,
            Vec::new(),
        )?);
        return Ok(());
    }
    into.extend(absolute_path(&["std", "vec", "Vec", "from"]));
    let mut listed = Vec::new();
    for (position, member) in members.into_iter().enumerate() {
        if position > 0 {
            listed.push(GeneratedToken::alone(','));
        }
        listed.extend(member);
    }
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::group(GeneratedDelimiter::Bracket, listed)?],
    )?);
    Ok(())
}

/// The generated `topology()` function.
fn topology_fn(
    declaration: &NetworkDeclaration,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    doc_attribute("The declared topology.", into)?;
    into.push(GeneratedToken::word("pub"));
    into.push(GeneratedToken::word("fn"));
    into.push(GeneratedToken::word(TOPOLOGY_ROAD));
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        Vec::new(),
    )?);
    let mut ok_seat = Vec::new();
    direct_path(
        declaration.harness(),
        &["network", "Topology"],
        &mut ok_seat,
    );
    fallible_return(ok_seat, FAULT_ENUM, into);
    let mut body = Vec::new();
    body.extend(absolute_path(&["core", "result", "Result", "Ok"]));
    let mut inner = Vec::new();
    direct_path(
        declaration.harness(),
        &["network", "Topology", "declared"],
        &mut inner,
    );
    let mut nodes = Vec::new();
    for node in declaration.nodes() {
        let mut expression = Vec::new();
        node_expr(
            declaration.harness(),
            declaration.namespace(),
            node,
            &mut expression,
        )?;
        nodes.push(expression);
    }
    let mut links = Vec::new();
    for link in declaration.links() {
        let mut expression = Vec::new();
        link_expr(
            declaration.harness(),
            declaration.namespace(),
            link,
            &mut expression,
        )?;
        links.push(expression);
    }
    let mut seats = Vec::new();
    vec_expr(nodes, &mut seats)?;
    seats.push(GeneratedToken::alone(','));
    vec_expr(links, &mut seats)?;
    inner.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        seats,
    )?);
    inner.push(GeneratedToken::alone('?'));
    body.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        inner,
    )?);
    into.push(GeneratedToken::group(GeneratedDelimiter::Brace, body)?);
    Ok(())
}

/// One generated schedule function.
fn schedule_fn(
    declaration: &NetworkDeclaration,
    schedule: &ScheduleRow,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    doc_attribute(
        &format!("The declared `{}` schedule.", schedule.name()),
        into,
    )?;
    into.push(GeneratedToken::word("pub"));
    into.push(GeneratedToken::word("fn"));
    into.push(GeneratedToken::word(schedule.name()));
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        Vec::new(),
    )?);
    let mut ok_seat = Vec::new();
    direct_path(
        declaration.harness(),
        &["network", "NetworkSchedule"],
        &mut ok_seat,
    );
    fallible_return(ok_seat, FAULT_ENUM, into);
    let mut body = Vec::new();
    body.extend(absolute_path(&["core", "result", "Result", "Ok"]));
    let mut inner = Vec::new();
    direct_path(
        declaration.harness(),
        &["network", "NetworkSchedule", "declared"],
        &mut inner,
    );
    let mut seats = Vec::new();
    direct_path(
        declaration.harness(),
        &["descriptor", "NamespacedName", "named"],
        &mut seats,
    );
    seats.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![
            GeneratedToken::text(declaration.namespace()),
            GeneratedToken::alone(','),
            GeneratedToken::text(schedule.name()),
        ],
    )?);
    seats.push(GeneratedToken::alone('?'));
    seats.push(GeneratedToken::alone(','));
    let mut disciplines = Vec::new();
    for discipline in schedule.disciplines() {
        let mut expression = Vec::new();
        discipline_expr(
            declaration.harness(),
            declaration.namespace(),
            discipline,
            &mut expression,
        )?;
        disciplines.push(expression);
    }
    vec_expr(disciplines, &mut seats)?;
    inner.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        seats,
    )?);
    inner.push(GeneratedToken::alone('?'));
    body.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        inner,
    )?);
    into.push(GeneratedToken::group(GeneratedDelimiter::Brace, body)?);
    Ok(())
}

/// One `<harness>::network::LinkDiscipline::declared(<link>, <faults>)` expression.
fn discipline_expr(
    harness: &DirectBinding,
    namespace: &str,
    discipline: &DisciplineRow,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    direct_path(harness, &["network", "LinkDiscipline", "declared"], into);
    let mut seats = Vec::new();
    link_expr(harness, namespace, discipline.link(), &mut seats)?;
    seats.push(GeneratedToken::alone(','));
    let mut faults = Vec::new();
    for fault in discipline.faults() {
        let mut expression = Vec::new();
        fault_expr(harness, fault, &mut expression)?;
        faults.push(expression);
    }
    vec_expr(faults, &mut seats)?;
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        seats,
    )?);
    Ok(())
}

/// One `<harness>::network::LinkFault::…` expression.
fn fault_expr(
    harness: &DirectBinding,
    fault: &FaultRow,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    match *fault {
        FaultRow::Drop { at } => positional_fault(harness, "DropAt", at, Vec::new(), into),
        FaultRow::Duplicate { at } => {
            positional_fault(harness, "DuplicateAt", at, Vec::new(), into)
        }
        FaultRow::Delay { at, by } => {
            let mut ticks = vec![
                GeneratedToken::alone(','),
                GeneratedToken::word("ticks"),
                GeneratedToken::alone(':'),
            ];
            direct_path(harness, &["network", "TickSpan", "declared"], &mut ticks);
            ticks.push(GeneratedToken::group(
                GeneratedDelimiter::Parenthesis,
                vec![GeneratedToken::number(u64::from(by))],
            )?);
            ticks.push(GeneratedToken::alone('?'));
            positional_fault(harness, "DelayAt", at, ticks, into)
        }
        FaultRow::Partition { from, until } => {
            direct_path(harness, &["network", "LinkFault", "Partition"], into);
            let mut fields = vec![GeneratedToken::word("opens"), GeneratedToken::alone(':')];
            tick_expr(harness, from, &mut fields)?;
            fields.push(GeneratedToken::alone(','));
            fields.push(GeneratedToken::word("heals"));
            fields.push(GeneratedToken::alone(':'));
            tick_expr(harness, until, &mut fields)?;
            into.push(GeneratedToken::group(GeneratedDelimiter::Brace, fields)?);
            Ok(())
        }
    }
}

/// One positional fault arm: `LinkFault::<arm> { position: SendOrdinal::at(<n>)<extra fields> }`.
fn positional_fault(
    harness: &DirectBinding,
    arm: &str,
    at: u32,
    extra_fields: Vec<GeneratedToken>,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    direct_path(harness, &["network", "LinkFault", arm], into);
    let mut fields = vec![GeneratedToken::word("position"), GeneratedToken::alone(':')];
    direct_path(harness, &["network", "SendOrdinal", "at"], &mut fields);
    fields.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::number(u64::from(at))],
    )?);
    fields.extend(extra_fields);
    into.push(GeneratedToken::group(GeneratedDelimiter::Brace, fields)?);
    Ok(())
}

/// One `<harness>::network::Tick::at(<n>)` expression.
fn tick_expr(
    harness: &DirectBinding,
    ordinal: u64,
    into: &mut Vec<GeneratedToken>,
) -> Result<(), Overflow> {
    direct_path(harness, &["network", "Tick", "at"], into);
    into.push(GeneratedToken::group(
        GeneratedDelimiter::Parenthesis,
        vec![GeneratedToken::number(ordinal)],
    )?);
    Ok(())
}

/// One owned generated path at the direct harness binding.
fn harness_path(harness: &DirectBinding, destination: &[&str]) -> Vec<GeneratedToken> {
    let mut tokens = Vec::new();
    direct_path(harness, destination, &mut tokens);
    tokens
}
