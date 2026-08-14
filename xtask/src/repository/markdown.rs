//! Markdown structure, and the data blocks this repository writes inside it.
//!
//! Two authorities, and the line between them is the point of this module.
//!
//! **What a document's STRUCTURE is** — where a fenced block begins, where it
//! ends, what its info string says, and what is prose rather than data — is a
//! question about Markdown, and `pulldown-cmark` answers it. Nothing here counts
//! fences, tracks indentation to find a block, or matches a line against a
//! backtick.
//!
//! **What one data block DECLARES** is this repository's own schema, and this
//! module owns it, because the blocks are yaml-SHAPED and are not YAML
//! documents. Measured: `macros/macroc/README.md` and `testpak/README.md` write
//! their tooling ledger as a scalar mapping value followed by more-indented
//! keys, which is a YAML error rather than a YAML document, so a YAML decoder
//! handed those blocks refuses them. And no YAML decoder can be admitted here
//! anyway: `deny.toml` sets `multiple-versions = "deny"`, and measured against
//! the committed lock, `yaml-rust2` resolves `hashbrown` 0.16 beside the
//! `hashbrown` 0.17 this graph already holds, while `saphyr` and
//! `saphyr-parser` reach `thiserror`, which requires `syn` 2 beside the `syn` 3
//! this workspace pins. Admitting either would break a supply-chain law to
//! satisfy a reading law.
//!
//! So the schema is read HERE, and the reading is narrowed twice so that what it
//! is narrowed to is small enough to be total:
//!
//! 1. **A row exists only inside a block the parser found**, and only inside one
//!    whose declared SCHEMA the reading is about. A `green:` written in prose, in
//!    a worked example, in a `text` fence, or in a block declaring a different
//!    schema reaches nothing. The whole-file scan that could not tell those apart
//!    is gone.
//! 2. **A record is delimited by the sequence item that opens it**, never by how
//!    deep its fields are indented. The indentation grammar is gone with it: no
//!    reader here compares one line's leading whitespace to another's.
//!
//! # The ceiling, and which way it falls
//!
//! What this establishes is that a block declares the schema's own shape — a
//! document key, a sequence item, a field, a continuation. A block written in
//! YAML's flow style, carrying an anchor, an alias, a block scalar, or a quoted
//! key, is read as the lines it is written on rather than as the YAML it would
//! decode to. That direction fails CLOSED: such a block's fields are not
//! recognized, so its records carry no rows, and a record carrying no rows is
//! refused by the join rather than qualifying quietly. What it costs is an
//! author who wanted an exotic spelling, who is told exactly which line was not
//! read.
//!
//! It opens when a YAML mechanism can be admitted without breaking
//! `multiple-versions = "deny"` — at which point this schema reading is deleted
//! rather than taught, because a decoder and a reader agreeing about a document
//! is two authorities over one fact.

use std::collections::BTreeMap;
use std::path::Path;

use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

use crate::repository::snapshot::CanonicalFileMap;
use crate::repository::types::{
    AbsenceReason, CanonicalPath, GreenRow, ObligationRecord, Read, ReadFailure,
};

/// The info string a data block declares itself under.
const DATA_LANGUAGE: &str = "yaml";

/// Every Markdown document in the tree, parsed once.
pub(crate) struct MarkdownSnapshot {
    /// Keyed by canonical path, so no reader spells a document twice.
    documents: BTreeMap<CanonicalPath, Read<MarkdownDocument>>,
}

impl MarkdownSnapshot {
    /// Parses every `.md` file the file map carries.
    pub(crate) fn read(files: &CanonicalFileMap) -> Self {
        let mut documents = BTreeMap::new();
        for (path, fact) in files.iter() {
            if !path.extension_is("md") {
                continue;
            }
            let parsed = match *fact.text() {
                Read::Known(ref text) => Read::Known(MarkdownDocument::parse(text)),
                Read::DeclaredAbsent(reason) => Read::DeclaredAbsent(reason),
                Read::Unreadable(ref failure) => Read::Unreadable(failure.clone()),
            };
            documents.insert(path.clone(), parsed);
        }
        Self { documents }
    }

    /// One parsed document, or the declared absence of the file carrying it.
    pub(crate) fn document(&self, path: &CanonicalPath) -> Read<&MarkdownDocument> {
        match self.documents.get(path) {
            Some(Read::Known(document)) => Read::Known(document),
            Some(Read::DeclaredAbsent(reason)) => Read::DeclaredAbsent(*reason),
            Some(Read::Unreadable(failure)) => Read::Unreadable(failure.clone()),
            None => Read::DeclaredAbsent(AbsenceReason::NoSuchPath),
        }
    }
}

/// One Markdown document, reduced to the data blocks it declares.
///
/// Prose is not carried. What a law asks of a document is which data it
/// declares, and a reading that also carried the prose would be a reading two
/// laws could disagree about.
pub(crate) struct MarkdownDocument {
    /// Every fenced block, in document order, each carrying its declared
    /// schema.
    blocks: Vec<DataBlock>,
}

impl MarkdownDocument {
    /// Every fenced block one document declares, in document order.
    ///
    /// The parser decides what a block IS. This walks its events and keeps the
    /// fenced ones, which is the whole of the structure this module reads.
    pub(crate) fn parse(text: &str) -> Self {
        let mut blocks = Vec::new();
        let mut open: Option<String> = None;
        let mut body = String::new();
        for event in Parser::new(text) {
            match event {
                Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                    open = Some(info.into_string());
                    body.clear();
                }
                Event::Text(written) if open.is_some() => body.push_str(&written),
                Event::End(TagEnd::CodeBlock) => close(&mut open, &mut body, &mut blocks),
                Event::Start(_)
                | Event::End(_)
                | Event::Text(_)
                | Event::Code(_)
                | Event::InlineMath(_)
                | Event::DisplayMath(_)
                | Event::Html(_)
                | Event::InlineHtml(_)
                | Event::FootnoteReference(_)
                | Event::SoftBreak
                | Event::HardBreak
                | Event::Rule
                | Event::TaskListMarker(_) => (),
            }
        }
        Self { blocks }
    }

    /// The one block declaring a named schema, or the declared absence of one.
    ///
    /// A document declaring the schema TWICE is a failure rather than a choice:
    /// two blocks answering one reading is the duplicate authority this whole
    /// model exists to remove, and picking one of them by position is exactly the
    /// first-fence rule this replaced.
    pub(crate) fn block(&self, schema: BlockSchema) -> Read<&DataBlock> {
        let mut declaring = self.blocks.iter().filter(|block| block.schema == schema);
        let Some(found) = declaring.next() else {
            return Read::DeclaredAbsent(AbsenceReason::NoBlockDeclaresThisSchema);
        };
        if declaring.next().is_some() {
            return Read::Unreadable(ReadFailure::new(
                schema.spelling(),
                "two data blocks in one document declare this schema, so which one a reading is \
                 about is decided by position rather than by the document",
            ));
        }
        Read::Known(found)
    }

    /// Every data-language block whose schema this repository does not
    /// recognize.
    ///
    /// Reported rather than skipped. A block written in the data language that
    /// declares no schema is a block no reading is about, and a ledger that
    /// silently stopped being read is the exact failure this model was built to
    /// end.
    pub(crate) fn unrecognized_data_blocks(&self) -> usize {
        self.blocks
            .iter()
            .filter(|block| block.schema == BlockSchema::UnrecognizedData)
            .count()
    }
}

/// Closes the block the parser just ended, carrying its declared schema with
/// it.
fn close(open: &mut Option<String>, body: &mut String, into: &mut Vec<DataBlock>) {
    let Some(language) = open.take() else {
        return;
    };
    let carried = std::mem::take(body);
    into.push(DataBlock {
        schema: BlockSchema::declared_by(&language, &carried),
        body: carried,
    });
}

/// Which schema one fenced block declares.
///
/// Identity is taken from the keys the block writes at its own document level,
/// never from where the block sits among the fences. That is the whole repair:
/// the reading this replaced took the FIRST fenced block carrying the data
/// language and called it the one it wanted, which was a fact about the current
/// order of one file rather than about the block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlockSchema {
    /// `phase`, `toolchain`, `workspace_members`: what the repository builds on.
    PhaseDeclaration,
    /// `home` and `obligations`: one home's obligation ledger.
    ObligationLedger,
    /// `tooling-obligation`: a tooling home's qualification ledger.
    ToolingObligationLedger,
    /// `seat` and `state`: a reserved architectural coordinate.
    SeatReservation,
    /// A data-language block declaring no schema this repository reads.
    UnrecognizedData,
    /// A fenced block that is not written in the data language at all — a
    /// diagram, a shell transcript, a Rust example.
    NotData,
}

impl BlockSchema {
    /// The schema a block declares, read off the keys it writes at its own
    /// document level.
    fn declared_by(language: &str, body: &str) -> Self {
        if language != DATA_LANGUAGE {
            return BlockSchema::NotData;
        }
        let keys = document_keys(body);
        let declares = |key: &str| keys.iter().any(|written| written == key);
        if declares("obligations") {
            BlockSchema::ObligationLedger
        } else if declares("tooling-obligation") {
            BlockSchema::ToolingObligationLedger
        } else if declares("phase") {
            BlockSchema::PhaseDeclaration
        } else if declares("seat") && declares("state") {
            BlockSchema::SeatReservation
        } else {
            BlockSchema::UnrecognizedData
        }
    }

    /// How the schema is named in a refusal.
    const fn spelling(self) -> &'static str {
        match self {
            BlockSchema::PhaseDeclaration => "the phase declaration block",
            BlockSchema::ObligationLedger => "the obligation ledger block",
            BlockSchema::ToolingObligationLedger => "the tooling obligation ledger block",
            BlockSchema::SeatReservation => "the seat reservation block",
            BlockSchema::UnrecognizedData => "a data block declaring no known schema",
            BlockSchema::NotData => "a block that is not data",
        }
    }
}

/// The keys one block writes at its own document level, in the order written.
///
/// A document key is a field written flush against the block's own left edge.
/// That is not an indentation grammar: nothing is compared to anything, and no
/// depth decides what a line BELONGS to — the question here is only which keys
/// the document itself states, which is what a schema identity is made of.
fn document_keys(body: &str) -> Vec<String> {
    body.lines()
        .filter(|line| !line.starts_with(char::is_whitespace))
        .filter_map(|line| field_key(line).map(str::to_owned))
        .collect()
}

/// One data block: the schema it declares and the text it carries.
pub(crate) struct DataBlock {
    /// What the block declares itself to be.
    schema: BlockSchema,
    /// The block's own text, as the parser handed it back.
    body: String,
}

/// One line of a data block, classified by the schema grammar this repository
/// writes.
///
/// Total over its input: every line is one of these four, so there is no line a
/// reading walks past without having said what it is.
#[derive(Debug, PartialEq, Eq)]
enum BlockLine<'block> {
    /// A line stating nothing.
    Blank,
    /// A sequence ITEM, which is what opens a record: `- key: value`, or `-
    /// value` with no key.
    Item {
        /// The field the item opens with, where it opens with one.
        key: Option<&'block str>,
        /// The value the item states.
        value: &'block str,
    },
    /// A field: `key: value`, or `key:` opening a list.
    Field {
        /// The field's name.
        key: &'block str,
        /// The value it states, empty where the field opens a list.
        value: &'block str,
    },
    /// Anything else: the wrapped remainder of the value above it.
    Continuation,
}

/// How one line of a data block reads.
fn classify_line(line: &str) -> BlockLine<'_> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return BlockLine::Blank;
    }
    if let Some(item) = trimmed.strip_prefix("- ") {
        return match field_of(item) {
            Some((key, value)) => BlockLine::Item {
                key: Some(key),
                value,
            },
            None => BlockLine::Item {
                key: None,
                value: item.trim(),
            },
        };
    }
    match field_of(trimmed) {
        Some((key, value)) => BlockLine::Field { key, value },
        None => BlockLine::Continuation,
    }
}

/// The `key: value` one line states, where it states one.
///
/// A key is an identifier written before a colon that is followed by a space or
/// by the end of the line. Prose wrapping onto a following line does not open a
/// field, which is what keeps a wrapped account from being read as a row.
fn field_of(line: &str) -> Option<(&str, &str)> {
    let key = field_key(line)?;
    let rest = line.get(key.len().saturating_add(1)..)?;
    Some((key, rest.trim()))
}

/// The field key one line opens with, where it opens with one.
///
/// A key is an IDENTIFIER written before a colon, and what follows the colon is
/// the value however it is spaced — `green: x`, `green:x`, `green:` with nothing
/// after it, and `green:` followed by a tab are one field written four ways. The
/// reader this replaced matched a literal `"green: "` in one place and a bare
/// `"green:"` in another, and the row a keystroke of whitespace dropped from the
/// strict side was seated by the loose one and claimed by neither.
///
/// The identifier restriction is what keeps prose from opening a field: a
/// wrapped account carrying `e.g.:` or a URL's `https://` states no key, because
/// neither head is written in a key's characters.
fn field_key(line: &str) -> Option<&str> {
    let (key, _) = line.split_once(':')?;
    if key.is_empty() || !key.chars().all(is_key_character) {
        return None;
    }
    Some(key)
}

/// The characters a schema key is written with.
fn is_key_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_' || character == '-'
}

/// What the phase declaration block states.
pub(crate) struct PhaseDeclaration {
    /// The toolchain the repository declares it builds on.
    toolchain: String,
    /// The workspace members the repository declares.
    members: Vec<String>,
}

impl PhaseDeclaration {
    /// The toolchain the block declares.
    pub(crate) fn toolchain(&self) -> &str {
        &self.toolchain
    }

    /// The workspace members the block declares, in the order written.
    pub(crate) fn members(&self) -> &[String] {
        &self.members
    }
}

/// The phase declaration one document states.
pub(crate) fn phase_declaration(document: &MarkdownDocument) -> Read<PhaseDeclaration> {
    let block = match document.block(BlockSchema::PhaseDeclaration) {
        Read::Known(block) => block,
        Read::DeclaredAbsent(reason) => return Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => return Read::Unreadable(failure),
    };
    let mut toolchain: Option<String> = None;
    let mut members: Vec<String> = Vec::new();
    let mut listing = false;
    for line in block.body.lines() {
        match classify_line(line) {
            BlockLine::Field { key, value } => {
                listing = value.is_empty() && key == "workspace_members";
                if key == "toolchain" {
                    toolchain = Some(unquoted(value).to_owned());
                }
            }
            BlockLine::Item { key: None, value } => {
                if listing {
                    members.push(unquoted(value).to_owned());
                }
            }
            BlockLine::Item { key: Some(_), .. } | BlockLine::Blank | BlockLine::Continuation => (),
        }
    }
    match toolchain {
        Some(toolchain) => Read::Known(PhaseDeclaration { toolchain, members }),
        None => Read::Unreadable(ReadFailure::new(
            "the phase declaration block",
            "states no `toolchain:` field",
        )),
    }
}

/// One scalar with its surrounding quotes removed, under either quote.
fn unquoted(value: &str) -> &str {
    for quote in ['"', '\''] {
        if let Some(inner) = value
            .strip_prefix(quote)
            .and_then(|rest| rest.strip_suffix(quote))
        {
            return inner;
        }
    }
    value
}

/// The obligation records one home declares, and what the reading refused.
pub(crate) struct ObligationLedger {
    /// Every record the block declared, in the order written.
    pub(crate) records: Vec<ObligationRecord>,
    /// What the reading itself refused: a row no record owns, an item opening
    /// with no identity.
    pub(crate) offences: Vec<String>,
}

impl ObligationLedger {
    /// Every record the block declared.
    pub(crate) fn records(&self) -> &[ObligationRecord] {
        &self.records
    }

    /// What the reading refused.
    pub(crate) fn offences(&self) -> &[String] {
        &self.offences
    }
}

/// The field a record opens with.
const RECORD_IDENTITY: &str = "id";

/// The field a record states its positive control in.
const GREEN_FIELD: &str = "green";

/// The field a record states its reversal in.
const RED_FIELD: &str = "red";

/// The field a tooling obligation states its reversal in.
const TOOLING_RED_FIELD: &str = "tooling-red";

/// The obligation ledger one document declares.
///
/// A record opens at the sequence ITEM that states its identity, and every
/// field written after that item and before the next one belongs to it. Nothing
/// here looks at how deep a line is written: the item marker is the delimiter,
/// which is what a sequence item IS.
pub(crate) fn obligation_ledger(document: &MarkdownDocument, home: &str) -> Read<ObligationLedger> {
    let block = match document.block(BlockSchema::ObligationLedger) {
        Read::Known(block) => block,
        Read::DeclaredAbsent(reason) => return Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => return Read::Unreadable(failure),
    };
    let mut records: Vec<ObligationRecord> = Vec::new();
    let mut offences: Vec<String> = Vec::new();
    for line in block.body.lines() {
        match classify_line(line) {
            BlockLine::Item { key, value } => match key {
                Some(RECORD_IDENTITY) => records.push(ObligationRecord {
                    id: value.to_owned(),
                    green: Vec::new(),
                    red: Vec::new(),
                }),
                Some(other) => offences.push(format!(
                    "{home}: an obligation record opens with `{other}:` rather than with \
                     `{RECORD_IDENTITY}:`, so the rows written beneath it belong to no obligation \
                     anything can name"
                )),
                None => offences.push(format!(
                    "{home}: an obligation record opens with `{value}`, which states no field at \
                     all, so the rows written beneath it belong to no obligation anything can name"
                )),
            },
            BlockLine::Field { key, value } => {
                let row = match key {
                    GREEN_FIELD | RED_FIELD => key,
                    _ => continue,
                };
                let Some(record) = records.last_mut() else {
                    offences.push(format!(
                        "{home}: a `{row}:` row stands outside every obligation record. This join \
                         reads rows through the record that declared them, so a row no record owns \
                         is joined by nothing and counted by nothing — the repair is to write it \
                         inside the record it belongs to, beneath that record's own `- id:` item"
                    ));
                    continue;
                };
                if row == GREEN_FIELD {
                    record.green.push(classify_green_row(value));
                } else {
                    record.red.push(value.to_owned());
                }
            }
            BlockLine::Blank | BlockLine::Continuation => (),
        }
    }
    Read::Known(ObligationLedger { records, offences })
}

/// Every `tooling-red:` row one tooling document declares, in the order
/// written.
///
/// Read exactly like a core `red:` row — same field grammar, same emptied row
/// still read, same name-then-prose value — and counted on its own denominator.
///
/// Several rows state what their reversal does after naming it, and one names
/// sibling fixtures in that prose. The ledger resolves the first token and reads
/// none of the rest, which is a stated ceiling rather than a convention: a row
/// carrying more than one fixture reference has exactly one of them joined.
/// Closing that takes typed fixture references in the row itself.
pub(crate) fn tooling_reversal_rows(document: &MarkdownDocument) -> Read<Vec<String>> {
    let block = match document.block(BlockSchema::ToolingObligationLedger) {
        Read::Known(block) => block,
        Read::DeclaredAbsent(reason) => return Read::DeclaredAbsent(reason),
        Read::Unreadable(failure) => return Read::Unreadable(failure),
    };
    let mut rows = Vec::new();
    for line in block.body.lines() {
        if let BlockLine::Field { key, value } = classify_line(line)
            && key == TOOLING_RED_FIELD
        {
            rows.push(value.to_owned());
        }
    }
    Read::Known(rows)
}

/// The spelling a green row opens with when its positive control is a
/// compile-time seat.
const COMPILE_TIME_SEAT: &str = "laws.rs";

/// The separator a `none` or `owed` disposition states its account after, as
/// every such row in this repository is written.
const DISPOSITION_DASH: char = '—';

/// The opener a `structural` disposition states its account inside.
const DISPOSITION_PAREN: char = '(';

/// The grammar one green row is written in, chosen by the word the row opens
/// with.
///
/// Three grammars, and being none of them is the absence of one. This exists so
/// that the ceiling on an account — what a row may state, and where it must stop
/// — is asked of the ROW's grammar in one place, instead of being remembered
/// inside whichever branch happens to build the row. Three branches each
/// carrying their own ceiling is three chances to forget one, and the history of
/// this reading is exactly that: the rule was written for the seat branch, the
/// route branch went on taking its first token and discarding the rest, and the
/// defect regrew one branch to the right. A rule applied per site regrows one
/// site over; a rule applied to the class does not.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Grammar {
    /// `laws.rs module::law`: the account is the target, and a target is one
    /// token.
    Seat,
    /// `path/to/file.rs`: the opening word IS the claim, so the account is
    /// silent — nothing at all follows the path.
    Route,
    /// `none — …`, `owed — …`, `structural (…)`: the account is a SENTENCE,
    /// opened by this character, and it runs for as long as the sentence takes.
    Disposition(char),
}

/// One green row's value, classified. Nothing is dropped: a value no lawful
/// spelling reads comes back as [`GreenRow::Unreadable`] carrying itself.
///
/// The row's grammar is decided first, its account is held to that grammar's
/// ceiling second, and only then is a row built. The ceiling is applied HERE, to
/// every grammar at once, rather than inside the arms below: an arm can only
/// interpret an account this function has already agreed is the whole of what
/// the row stated, so there is no arm left that could read past its own account.
pub(crate) fn classify_green_row(value: &str) -> GreenRow {
    let value = value.trim();
    let Some(opening) = value.split_whitespace().next() else {
        return GreenRow::Unreadable(value.to_owned());
    };
    let account = value.get(opening.len()..).map_or("", str::trim);
    let read = green_grammar(opening)
        .filter(|&grammar| states_only_its_account(grammar, account))
        .and_then(|grammar| match grammar {
            Grammar::Seat => seat_target(account).map(|(module, law)| GreenRow::CompileTimeSeat {
                module: module.to_owned(),
                law: law.to_owned(),
            }),
            Grammar::Route => Some(GreenRow::Route(opening.to_owned())),
            Grammar::Disposition(opener) => {
                accounts_after(account, opener).then_some(GreenRow::Disposition)
            }
        });
    match read {
        Some(row) => row,
        None => GreenRow::Unreadable(value.to_owned()),
    }
}

/// Which grammar a green row's opening word puts it in, or none where the word
/// opens no grammar this repository reads.
///
/// The seat is asked about BEFORE the route, and the order is load-bearing:
/// `laws.rs` is itself a path to a Rust file, so a reader that asked the route
/// question first would read every one of this repository's seat rows as a route
/// naming a file at the repository root and demand a test binary of it. The
/// opening word alone decides the grammar; nothing after it is looked at here,
/// because what a row may say after its opening word is the next question and
/// has one answer for the whole class.
fn green_grammar(opening: &str) -> Option<Grammar> {
    if opening == COMPILE_TIME_SEAT {
        Some(Grammar::Seat)
    } else if opening == "none" || opening == "owed" {
        Some(Grammar::Disposition(DISPOSITION_DASH))
    } else if opening == "structural" {
        Some(Grammar::Disposition(DISPOSITION_PAREN))
    } else if is_rust_route(opening) {
        Some(Grammar::Route)
    } else {
        None
    }
}

/// Whether a green row states the account its grammar defines and NOTHING after
/// it.
///
/// THE ceiling on a green account, and the only one. Whatever kind of green row
/// it is, the account is exactly the tokens that row's grammar defines, and a
/// token past them makes the row [`GreenRow::Unreadable`]. One statement, over
/// the class — because it has been stated per branch, and per branch it only
/// ever held for the branch written last.
///
/// # The disposition grammar has no ceiling, and that is a decision
///
/// A `none`, `owed`, or `structural` row accounts for why NO file holds a
/// positive control, and an account of that kind is prose. Prose legitimately
/// has many tokens, so there is no number to hold it to and this rule admits it
/// without one — deliberately, and stated here rather than left as the case
/// nobody got to.
///
/// It is the same asymmetry the `red:` rows are read under, for the same reason.
/// A seat target and a route path are JOIN KEYS: they are resolved against
/// `laws.rs` and against testpak, and a key that names two things resolves
/// neither. A disposition's account joins nothing and is read by a person.
fn states_only_its_account(grammar: Grammar, account: &str) -> bool {
    let stated: usize = match grammar {
        // The file IS the claim: the row names it and stops.
        Grammar::Route => 0,
        // The target IS the claim, and a target is one token.
        Grammar::Seat => 1,
        // A sentence runs as long as it takes; see above.
        Grammar::Disposition(_) => return true,
    };
    account.split_whitespace().count() <= stated
}

/// The `module::name` target a `laws.rs` row states, split where it splits.
///
/// # A target is EXACTLY `module::law`
///
/// One separator, and neither half empty. `root::a_law::extra` is not a deeper
/// target, it is two separators; `::a_law` names no module and `root::` names no
/// law, and each of those halves is a name the join resolves against. Split
/// looser — on the FIRST `::`, with nothing said about the rest — all three
/// became seats, and the empty-module one is a name `laws.rs` can actually
/// produce, because that file is read by tracking the module last opened at the
/// crate root and it starts as no module at all.
fn seat_target(account: &str) -> Option<(&str, &str)> {
    let mut halves = account.split("::");
    let module = halves.next()?;
    let law = halves.next()?;
    if halves.next().is_some() || module.is_empty() || law.is_empty() {
        return None;
    }
    Some((module, law))
}

/// Whether a disposition opens its account with `opener` and states something
/// after it.
///
/// Something means a word: an opener followed by nothing, by its own closing
/// bracket, or by punctuation states the absence and accounts for none of it,
/// which is the half of the form that carries the whole meaning.
fn accounts_after(account: &str, opener: char) -> bool {
    account
        .strip_prefix(opener)
        .is_some_and(|why| why.chars().any(char::is_alphanumeric))
}

/// Whether one green row's first word is a path to a Rust file.
///
/// Read through `Path` rather than off the end of the string: a row states a
/// repository-relative path with forward slashes, and asking the path type for
/// its extension is the reading that stays right on either platform.
fn is_rust_route(named: &str) -> bool {
    Path::new(named)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("rs"))
}

/// Planted reversals for the block reading and the row grammars.
///
/// Every case is a fixture document held in memory: the reading that decides
/// which rows this repository publishes is never proven by editing a README the
/// repository stands on.
#[cfg(test)]
mod tests {
    use super::{
        BlockSchema, MarkdownDocument, classify_green_row, obligation_ledger, phase_declaration,
        tooling_reversal_rows,
    };
    use crate::repository::types::{GreenRow, Read};

    /// One seat carrying the target it named.
    fn seat(module: &str, law: &str) -> GreenRow {
        GreenRow::CompileTimeSeat {
            module: String::from(module),
            law: String::from(law),
        }
    }

    /// A document whose obligation ledger is written SECOND, after a phase
    /// block, and whose prose carries a worked example of a row.
    const TWO_BLOCKS_AND_A_WORKED_EXAMPLE: &str = "# Home\n\n\
        A row is written like this:\n\n\
        green: laws.rs bounds::a_row_nobody_declared\n\n\
        ```text\n\
        - id: bounds.an-example-nobody-declared\n\
        \x20 green: laws.rs bounds::also_nobody_declared\n\
        ```\n\n\
        ```yaml\n\
        phase: architecture-closure\n\
        toolchain: \"1.97.1\"\n\
        workspace_members:\n\
        \x20 - macros/macroc\n\
        \x20 - testpak\n\
        ```\n\n\
        ```yaml\n\
        home: bounds\n\
        obligations:\n\
        \x20 - id: bounds.classes-are-closed\n\
        \x20   challenge_kind: compile-law\n\
        \x20   green: laws.rs bounds::classes_are_closed\n\
        \x20   red: owed-to-testpak\n\
        \x20 - id: bounds.a-stamped-roster\n\
        \x20   challenge_kind: compile-refusal\n\
        \x20   green: testpak/tests/stamp_row_ceiling.rs\n\
        \x20   red: testpak/tests/compile-fail/a-roster.rs\n\
        ```\n";

    /// The block a reading is about is the one declaring its SCHEMA, wherever
    /// it sits among the fences.
    ///
    /// Planted reversal for the first-fence rule. The phase block here is
    /// written second among the yaml blocks in the file's own order and the
    /// ledger third; a reading that took the first fenced yaml block would have
    /// joined a ledger against the manifest as though it were a phase
    /// declaration, and the day somebody inserted a block above would have been
    /// the day it started doing that silently.
    #[test]
    fn a_block_is_chosen_by_its_schema_and_never_by_its_position() -> Result<(), String> {
        let document = MarkdownDocument::parse(TWO_BLOCKS_AND_A_WORKED_EXAMPLE);
        let phase = phase_declaration(&document).taken("the phase block")?;
        assert_eq!(phase.toolchain(), "1.97.1");
        assert_eq!(
            phase.members(),
            [String::from("macros/macroc"), String::from("testpak")]
        );
        let ledger =
            obligation_ledger(&document, "home/README.md").taken("the obligation ledger")?;
        assert_eq!(ledger.records().len(), 2, "{:?}", ledger.offences());
        Ok(())
    }

    /// Planted reversal: a row written in ordinary prose, and a record written
    /// inside a fence that is not the data language.
    ///
    /// Both used to enter the published ledger, because the reading was a scan
    /// of the WHOLE file that looked at no structure at all. The parser decides
    /// what a block is, and the schema decides which block a reading is about,
    /// so neither of these is reachable — and a writer who wants to DESCRIBE a
    /// row can now do it.
    #[test]
    fn a_row_written_in_prose_reaches_nothing() -> Result<(), String> {
        let document = MarkdownDocument::parse(TWO_BLOCKS_AND_A_WORKED_EXAMPLE);
        let ledger =
            obligation_ledger(&document, "home/README.md").taken("the obligation ledger")?;
        assert!(ledger.offences().is_empty(), "{:?}", ledger.offences());
        let ids: Vec<&str> = ledger
            .records()
            .iter()
            .map(|record| record.id.as_str())
            .collect();
        assert_eq!(
            ids,
            vec!["bounds.classes-are-closed", "bounds.a-stamped-roster"],
            "a record nobody declared entered the ledger"
        );
        assert!(
            ledger
                .records()
                .iter()
                .all(|record| record.green.len() == 1 && record.red.len() == 1),
            "a record lost or gained a row"
        );
        Ok(())
    }

    /// Every row lands in the record whose own item wrote it, and a record that
    /// lost a row carries none.
    ///
    /// Planted reversal for the two independent whole-file scans this replaced:
    /// with its `green:` line deleted an obligation named no positive control,
    /// so none was resolved and it qualified on its red row alone; with its
    /// `red:` line deleted the published denominator shrank by one with nothing
    /// saying so.
    #[test]
    fn a_record_that_lost_a_row_carries_none() -> Result<(), String> {
        let document = MarkdownDocument::parse(
            "```yaml\n\
             home: bounds\n\
             obligations:\n\
             \x20 - id: bounds.no-route-at-all\n\
             \x20   red: owed-to-testpak\n\
             \x20 - id: bounds.no-reversal-at-all\n\
             \x20   green: laws.rs bounds::budget_is_affine\n\
             ```\n",
        );
        let ledger =
            obligation_ledger(&document, "home/README.md").taken("the obligation ledger")?;
        assert_eq!(ledger.records().len(), 2);
        assert!(
            ledger
                .records()
                .first()
                .is_some_and(|record| record.green.is_empty() && record.red.len() == 1),
            "the record that lost its green row is not the record carrying none"
        );
        assert!(
            ledger
                .records()
                .last()
                .is_some_and(|record| record.green.len() == 1 && record.red.is_empty()),
            "the record that lost its red row is not the record carrying none"
        );
        Ok(())
    }

    /// Planted reversal: rows written where no record owns them, and an item
    /// that opens with something other than an identity.
    ///
    /// The reading's own failure mode, refused rather than trusted. A row no
    /// record carries is a row the join never joins, so losing one quietly would
    /// be the silence this whole model was built to end.
    #[test]
    fn a_row_no_record_owns_is_refused() -> Result<(), String> {
        let document = MarkdownDocument::parse(
            "```yaml\n\
             home: bounds\n\
             obligations:\n\
             \x20 green: laws.rs bounds::a_row_above_every_record\n\
             \x20 - challenge_kind: compile-law\n\
             \x20   green: laws.rs bounds::a_row_under_a_nameless_item\n\
             ```\n",
        );
        let ledger =
            obligation_ledger(&document, "home/README.md").taken("the obligation ledger")?;
        // Three: the row above every item, the item that opens with no
        // identity, and the row written beneath that item — which no record owns
        // either, because the item that would have owned it opened nothing.
        assert_eq!(ledger.offences().len(), 3, "{:?}", ledger.offences());
        assert!(
            ledger
                .offences()
                .iter()
                .any(|offence| offence.contains("stands outside every obligation record")),
            "{:?}",
            ledger.offences()
        );
        assert!(
            ledger
                .offences()
                .iter()
                .any(|offence| offence.contains("opens with `challenge_kind:`")),
            "{:?}",
            ledger.offences()
        );
        Ok(())
    }

    /// A document declaring one schema TWICE is refused rather than resolved by
    /// position.
    #[test]
    fn two_blocks_declaring_one_schema_are_refused() {
        let document = MarkdownDocument::parse(
            "```yaml\nhome: a\nobligations:\n```\n\n```yaml\nhome: b\nobligations:\n```\n",
        );
        let found = obligation_ledger(&document, "home/README.md");
        assert!(
            matches!(found, Read::Unreadable(_)),
            "two ledgers in one document resolved to one"
        );
    }

    /// A data block declaring no schema this repository reads is COUNTED rather
    /// than skipped, so a ledger that quietly stopped being recognized is
    /// something a law can refuse.
    #[test]
    fn a_data_block_declaring_no_schema_is_counted() {
        let document = MarkdownDocument::parse("```yaml\nsomething: else\nentirely: true\n```\n");
        assert_eq!(document.unrecognized_data_blocks(), 1);
        assert!(matches!(
            document.block(BlockSchema::ObligationLedger),
            Read::DeclaredAbsent(_)
        ));
    }

    /// The tooling ledger's rows are read out of the block that declares them,
    /// and a `tooling-red:` written in that document's prose is not one.
    #[test]
    fn a_tooling_row_is_read_out_of_its_own_block() -> Result<(), String> {
        let document = MarkdownDocument::parse(
            "prose mentioning tooling-red: not-a-row.rs\n\n\
             ```yaml\n\
             tooling-obligation: macroc.one\n\
             \x20 claim: >\n\
             \x20   a sentence\n\
             \x20 tooling-red: testpak/tests/planted_defect.rs\n\
             \n\
             tooling-obligation: macroc.two\n\
             \x20 tooling-red: owed-to-testpak — a renderer hardcoding the binding\n\
             ```\n",
        );
        let rows = tooling_reversal_rows(&document).taken("the tooling ledger")?;
        assert_eq!(
            rows,
            vec![
                String::from("testpak/tests/planted_defect.rs"),
                String::from("owed-to-testpak — a renderer hardcoding the binding"),
            ]
        );
        Ok(())
    }

    /// The positive control: every spelling this repository writes is read as
    /// the spelling it is.
    #[test]
    fn every_lawful_green_spelling_is_read_as_itself() {
        assert_eq!(
            classify_green_row("laws.rs root::a_seat_that_exists"),
            seat("root", "a_seat_that_exists")
        );
        assert_eq!(
            classify_green_row("none — the type's nonexistence is what refuses"),
            GreenRow::Disposition
        );
        assert_eq!(
            classify_green_row("owed — executable when the roster lands"),
            GreenRow::Disposition
        );
        assert_eq!(
            classify_green_row("structural (a phantom makes the handle !Send)"),
            GreenRow::Disposition
        );
        assert_eq!(
            classify_green_row("testpak/tests/stamp_row_ceiling.rs"),
            GreenRow::Route(String::from("testpak/tests/stamp_row_ceiling.rs"))
        );
    }

    /// Planted reversal: a row carrying a token AFTER its account, on both join
    /// keys.
    ///
    /// The row resolves a real law or a real seat, so every join leg downstream
    /// says yes and the obligation qualifies. What it says beyond its account
    /// was simply thrown away — a second target somebody meant to add, a stray
    /// note, half of a finished rename. A green account is exactly what its
    /// grammar defines, and a row that says more says something this repository
    /// does not read.
    #[test]
    fn a_row_carrying_more_than_its_account_is_unreadable() {
        for value in [
            "laws.rs root::reading_is_not_gaining extra",
            "laws.rs root::reading_is_not_gaining\troot::closure_bar_is_implementable",
            "testpak/tests/stamp_row_ceiling.rs missing-control.rs",
            "testpak/tests/stamp_row_ceiling.rs testpak/tests/stamp_row_ceiling.rs",
        ] {
            assert!(
                matches!(classify_green_row(value), GreenRow::Unreadable(_)),
                "`{value}` was truncated into a claim its author did not make"
            );
        }
    }

    /// Planted reversal: a target that is not exactly `module::law`, in all four
    /// malformed spellings, and a disposition that states the absence and
    /// withholds the account of it.
    #[test]
    fn a_malformed_account_is_unreadable() {
        for value in [
            "laws.rs root::reading_is_not_gaining::extra",
            "laws.rs ::reading_is_not_gaining",
            "laws.rs root::",
            "laws.rs root",
            "none",
            "owed",
            "none - a hyphen is not the declared separator",
            "structural ()",
            "owed —",
            "",
            "laws.rs",
            "testpak/tests/stamp_row_ceiling.r",
        ] {
            assert!(
                matches!(classify_green_row(value), GreenRow::Unreadable(_)),
                "`{value}` was read as a claim"
            );
        }
    }

    /// The disposition account is PROSE and is held to no token ceiling, and
    /// this is the control that says the asymmetry is a decision rather than the
    /// case a pass over the class forgot.
    #[test]
    fn a_disposition_account_carries_no_token_ceiling() {
        assert_eq!(
            classify_green_row(
                "none — no family payload can carry a spelling, skeleton, or scalar"
            ),
            GreenRow::Disposition
        );
        assert_eq!(
            classify_green_row("structural (raw-pointer phantom makes the handle !Send and !Sync)"),
            GreenRow::Disposition
        );
    }
}
