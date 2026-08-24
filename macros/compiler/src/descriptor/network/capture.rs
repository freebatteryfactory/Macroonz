//! Reading one authored network declaration out of a typed token tree.
//!
//! # The authored grammar
//!
//! ```text
//! <helper>! {
//!     module = <ident>,
//!     namespace = "<owner>",
//!     nodes = [<ident>, ...],
//!     link <ident> = <node> to <node>,
//!     schedule <ident> = [<fault phrase>, ...],
//! }
//! ```
//!
//! Clause order is free and is read by key; roster order is meaning and is preserved.
//! The reading walks the clauses in passes — the names first, then the links against the nodes, then the schedules against the links — so every clause may stand wherever its author put it.

use super::render::RESERVED;
use super::{
    DisciplineRow, FaultRow, LinkRow, NetworkCaptureError, NetworkDeclaration, ScheduleRow,
};
use crate::descriptor::{CaptureCause, Grammar};
use crate::token::{
    CapturedDelimiter, CapturedInput, CapturedTokenTree, SpanHandle, rendered_identifier,
    rust_keyword,
};

/// Read one network payload out of the declaration's body.
///
/// # Errors
///
/// Returns [`NetworkCaptureError`] where the tokens do not say a network declaration — an unreadable clause, an undeclared key, a doubled name, a separator separating nothing, a link drawn to an undeclared node, a phrase on an undrawn link, a phrase this grammar cannot read, a number past its seat's width, a name the language or the generated module already owns — each at the token it was established at, and an absent required clause at the declaration's opening.
pub fn declared(
    body: &CapturedInput,
    grammar: Grammar,
) -> Result<NetworkDeclaration, NetworkCaptureError> {
    let groups = comma_groups(grammar, body.trees())?;
    let world = world_of(grammar, &groups)?;
    let mut schedules: Vec<ScheduleRow> = Vec::new();
    for group in &groups {
        if head_word(group) == Some("schedule") {
            let schedule = schedule_of(grammar, group, &world)?;
            if schedules.iter().any(|held| held.name() == schedule.name()) {
                return Err(refused(
                    grammar,
                    CaptureCause::ChoiceDoubled,
                    opening(group),
                ));
            }
            schedules.push(schedule);
        }
    }
    Ok(NetworkDeclaration::read(
        world.module,
        world.namespace,
        world.nodes,
        world.links,
        schedules,
    ))
}

/// One established grammar refusal at one token.
const fn refused(grammar: Grammar, cause: CaptureCause, at: SpanHandle) -> NetworkCaptureError {
    NetworkCaptureError::grammar_refused(grammar, cause, at)
}

/// The declaration's world: everything a schedule is read against.
struct World {
    /// The module the builders land in.
    module: String,
    /// The namespace every declared name is owned under.
    namespace: String,
    /// The node spellings, in authored order.
    nodes: Vec<String>,
    /// The links, in authored order.
    links: Vec<LinkRow>,
}

/// Cut one body into its comma-separated groups, refusing a separator that separates nothing.
///
/// A trailing comma after the last group is ordinary Rust and lawful; a leading or doubled comma makes an empty group this reader would otherwise silently drop, so it refuses at the comma's own token.
///
/// # Errors
///
/// Returns [`CaptureCause::SeparatorDangling`] at the comma standing where no clause does.
fn comma_groups(
    grammar: Grammar,
    trees: &[CapturedTokenTree],
) -> Result<Vec<Vec<&CapturedTokenTree>>, NetworkCaptureError> {
    let mut groups: Vec<Vec<&CapturedTokenTree>> = Vec::new();
    let mut group: Vec<&CapturedTokenTree> = Vec::new();
    for tree in trees {
        if tree.punct() == Some(',') {
            if group.is_empty() {
                return Err(refused(
                    grammar,
                    CaptureCause::SeparatorDangling,
                    tree.span(),
                ));
            }
            groups.push(core::mem::take(&mut group));
        } else {
            group.push(tree);
        }
    }
    if !group.is_empty() {
        groups.push(group);
    }
    Ok(groups)
}

/// The word one group opens with, where it opens with one.
fn head_word<'trees>(group: &[&'trees CapturedTokenTree]) -> Option<&'trees str> {
    group.first().and_then(|tree| tree.word())
}

/// The token one group opens at, or the declaration's own opening for an empty one.
fn opening(group: &[&CapturedTokenTree]) -> SpanHandle {
    group.first().map_or(SpanHandle::at(0), |tree| tree.span())
}

/// Read the world out of every non-schedule clause, refusing what these passes can already judge.
fn world_of(
    grammar: Grammar,
    groups: &[Vec<&CapturedTokenTree>],
) -> Result<World, NetworkCaptureError> {
    let mut module: Option<String> = None;
    let mut namespace: Option<String> = None;
    let mut nodes: Option<Vec<String>> = None;
    for group in groups {
        match head_word(group) {
            Some("module") => assigned_once(grammar, group, &mut module, assigned_ident)?,
            Some("namespace") => assigned_once(grammar, group, &mut namespace, assigned_text)?,
            Some("nodes") => read_nodes(grammar, group, &mut nodes)?,
            Some("link" | "schedule") => {}
            Some(_) => {
                return Err(refused(
                    grammar,
                    CaptureCause::ClauseUndeclared,
                    opening(group),
                ));
            }
            None => return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group))),
        }
    }
    let Some(nodes) = nodes else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        ));
    };
    let mut links: Vec<LinkRow> = Vec::new();
    for group in groups {
        if head_word(group) == Some("link") {
            let link = link_of(grammar, group, &nodes)?;
            if links.iter().any(|held| held.name() == link.name()) {
                return Err(refused(
                    grammar,
                    CaptureCause::ChoiceDoubled,
                    opening(group),
                ));
            }
            links.push(link);
        }
    }
    let Some(module) = module else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        ));
    };
    let Some(namespace) = namespace else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        ));
    };
    if links.is_empty() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseAbsent,
            SpanHandle::at(0),
        ));
    }
    Ok(World {
        module,
        namespace,
        nodes,
        links,
    })
}

/// Read one `<key> = <value>` clause into its empty seat, refusing a doubled key.
fn assigned_once(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    seat: &mut Option<String>,
    read: fn(Grammar, &[&CapturedTokenTree]) -> Result<String, NetworkCaptureError>,
) -> Result<(), NetworkCaptureError> {
    if seat.is_some() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseDoubled,
            opening(group),
        ));
    }
    *seat = Some(read(grammar, group)?);
    Ok(())
}

/// The one identifier a `<key> = <ident>` clause assigns, refused where the language already owns the spelling.
///
/// The value becomes a rendered item's own name, so a Rust keyword here would compile into a collision inside the adopter's build — where the expansion's lints are silenced — instead of refusing at the authored token.
fn assigned_ident(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
) -> Result<String, NetworkCaptureError> {
    let [value] = value_of(group) else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    };
    let word = value
        .word()
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, value.span()))?;
    if !rendered_identifier(word) {
        return Err(refused(grammar, CaptureCause::ClauseUnread, value.span()));
    }
    if rust_keyword(word) {
        return Err(refused(grammar, CaptureCause::NameReserved, value.span()));
    }
    Ok(word.to_owned())
}

/// The one text literal a `<key> = "<text>"` clause assigns.
fn assigned_text(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
) -> Result<String, NetworkCaptureError> {
    let [value] = value_of(group) else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    };
    value
        .text()
        .map(str::to_owned)
        .ok_or_else(|| refused(grammar, CaptureCause::ClauseUnread, value.span()))
}

/// The value trees past one group's `<key> =` opening, or nothing where no `=` stands second.
fn value_of<'group, 'trees>(
    group: &'group [&'trees CapturedTokenTree],
) -> &'group [&'trees CapturedTokenTree] {
    match group {
        [_key, assigned_by, value @ ..] if assigned_by.punct() == Some('=') => value,
        _malformed => &[],
    }
}

/// Read the node roster, refusing a repeated spelling at its own token.
///
/// An authored `nodes = []` refuses at its own bracket as choosing nothing, so an empty first statement never stands to vanish under a second — and a doubled clause refuses at its own opening, marked by the seat a lawful first clause filled.
/// The commas are grammar rather than noise: two names with no separator between them are one phrase this roster does not read, refused at the second name.
fn read_nodes(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    nodes: &mut Option<Vec<String>>,
) -> Result<(), NetworkCaptureError> {
    if nodes.is_some() {
        return Err(refused(
            grammar,
            CaptureCause::ClauseDoubled,
            opening(group),
        ));
    }
    let [roster] = value_of(group) else {
        return Err(refused(grammar, CaptureCause::RosterUnread, opening(group)));
    };
    let Some((CapturedDelimiter::Bracket, members)) = roster.group() else {
        return Err(refused(grammar, CaptureCause::RosterUnread, roster.span()));
    };
    let mut declared: Vec<String> = Vec::new();
    let mut separated = true;
    for member in members {
        if separated {
            let Some(word) = member.word() else {
                return Err(refused(grammar, CaptureCause::ChoiceUnread, member.span()));
            };
            if declared.iter().any(|held| held == word) {
                return Err(refused(grammar, CaptureCause::ChoiceDoubled, member.span()));
            }
            declared.push(word.to_owned());
            separated = false;
        } else {
            if member.punct() != Some(',') {
                return Err(refused(grammar, CaptureCause::ChoiceUnread, member.span()));
            }
            separated = true;
        }
    }
    if declared.is_empty() {
        return Err(refused(grammar, CaptureCause::NothingChosen, roster.span()));
    }
    *nodes = Some(declared);
    Ok(())
}

/// Read one `link <name> = <from> to <to>` clause against the declared nodes.
fn link_of(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    nodes: &[String],
) -> Result<LinkRow, NetworkCaptureError> {
    let [_link, name_tree, assigned_by, from_tree, to_word, to_tree] = group else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    };
    if assigned_by.punct() != Some('=') || to_word.word() != Some("to") {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    }
    let (Some(name), Some(from), Some(to)) = (name_tree.word(), from_tree.word(), to_tree.word())
    else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    };
    if !nodes.iter().any(|held| held == from) {
        return Err(refused(
            grammar,
            CaptureCause::EndpointUnknown,
            from_tree.span(),
        ));
    }
    if !nodes.iter().any(|held| held == to) {
        return Err(refused(
            grammar,
            CaptureCause::EndpointUnknown,
            to_tree.span(),
        ));
    }
    Ok(LinkRow::drawn(
        name.to_owned(),
        from.to_owned(),
        to.to_owned(),
    ))
}

/// Read one `schedule <name> = [<phrases>]` clause against the drawn links.
fn schedule_of(
    grammar: Grammar,
    group: &[&CapturedTokenTree],
    world: &World,
) -> Result<ScheduleRow, NetworkCaptureError> {
    let [_schedule, name_tree, assigned_by, roster] = group else {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    };
    let Some(name) = name_tree.word() else {
        return Err(refused(
            grammar,
            CaptureCause::ClauseUnread,
            name_tree.span(),
        ));
    };
    if !rendered_identifier(name) {
        return Err(refused(
            grammar,
            CaptureCause::ClauseUnread,
            name_tree.span(),
        ));
    }
    if rust_keyword(name) || RESERVED.contains(&name) {
        return Err(refused(
            grammar,
            CaptureCause::NameReserved,
            name_tree.span(),
        ));
    }
    if assigned_by.punct() != Some('=') {
        return Err(refused(grammar, CaptureCause::ClauseUnread, opening(group)));
    }
    let Some((CapturedDelimiter::Bracket, members)) = roster.group() else {
        return Err(refused(grammar, CaptureCause::RosterUnread, roster.span()));
    };
    let mut disciplines: Vec<DisciplineRow> = Vec::new();
    for phrase in &comma_groups(grammar, members)? {
        let (link, fault) = phrase_of(grammar, phrase, world)?;
        match disciplines
            .iter_mut()
            .find(|held| held.link().name() == link.name())
        {
            Some(discipline) => discipline.push(fault),
            None => disciplines.push(DisciplineRow::gathered(link, vec![fault])),
        }
    }
    Ok(ScheduleRow::declared(name.to_owned(), disciplines))
}

/// Read one fault phrase against the drawn links, handing back the resolved link beside the fault.
fn phrase_of(
    grammar: Grammar,
    phrase: &[&CapturedTokenTree],
    world: &World,
) -> Result<(LinkRow, FaultRow), NetworkCaptureError> {
    let (link_tree, fault) = match phrase {
        [verb, link, at_word, at]
            if verb.word() == Some("drop") && at_word.word() == Some("at") =>
        {
            (
                link,
                FaultRow::Drop {
                    at: ordinal_of(grammar, at)?,
                },
            )
        }
        [verb, link, at_word, at]
            if verb.word() == Some("duplicate") && at_word.word() == Some("at") =>
        {
            (
                link,
                FaultRow::Duplicate {
                    at: ordinal_of(grammar, at)?,
                },
            )
        }
        [verb, link, at_word, at, by_word, by]
            if verb.word() == Some("delay")
                && at_word.word() == Some("at")
                && by_word.word() == Some("by") =>
        {
            (
                link,
                FaultRow::Delay {
                    at: ordinal_of(grammar, at)?,
                    by: ordinal_of(grammar, by)?,
                },
            )
        }
        [verb, link, from_word, from, until_word, until]
            if verb.word() == Some("partition")
                && from_word.word() == Some("from")
                && until_word.word() == Some("until") =>
        {
            (
                link,
                FaultRow::Partition {
                    from: number_of(grammar, from)?,
                    until: number_of(grammar, until)?,
                },
            )
        }
        _unread => {
            return Err(refused(
                grammar,
                CaptureCause::PhraseUnread,
                opening(phrase),
            ));
        }
    };
    let Some(link) = link_tree.word() else {
        return Err(refused(
            grammar,
            CaptureCause::PhraseUnread,
            link_tree.span(),
        ));
    };
    world
        .links
        .iter()
        .find(|held| held.name() == link)
        .cloned()
        .map(|resolved| (resolved, fault))
        .ok_or_else(|| refused(grammar, CaptureCause::EndpointUnknown, link_tree.span()))
}

/// The one unsigned number a tick seat states, at the tick's sixty-four-bit width.
fn number_of(grammar: Grammar, tree: &CapturedTokenTree) -> Result<u64, NetworkCaptureError> {
    let digits = tree
        .number()
        .ok_or_else(|| refused(grammar, CaptureCause::PhraseUnread, tree.span()))?;
    digits
        .parse::<u64>()
        .map_err(|_beyond| refused(grammar, CaptureCause::NumberBeyondSeat, tree.span()))
}

/// The one unsigned number an ordinal or span seat states, at those seats' thirty-two-bit width.
///
/// Parsed at exactly the seat's width so a number past it refuses HERE, at the authored token: generated code cannot outsource the range to rustc, whose overflowing-literal diagnostic is suppressed inside a foreign macro expansion and whose out-of-range literal wraps silently.
fn ordinal_of(grammar: Grammar, tree: &CapturedTokenTree) -> Result<u32, NetworkCaptureError> {
    let digits = tree
        .number()
        .ok_or_else(|| refused(grammar, CaptureCause::PhraseUnread, tree.span()))?;
    digits
        .parse::<u32>()
        .map_err(|_beyond| refused(grammar, CaptureCause::NumberBeyondSeat, tree.span()))
}
