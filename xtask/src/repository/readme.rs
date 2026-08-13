//! Reading a home README.
//!
//! A home README is markdown prose plus fenced yaml blocks and obligation rows
//! that tooling parses. These readers turn those blocks and rows into values and
//! stop there — the joins that decide whether the values agree with the tree
//! live in `crate::checks::obligations` and `crate::checks::toolchain`.

use std::fs;
use std::path::{Path, PathBuf};

use crate::repository::types::GreenRow;

/// The lines inside the README's fenced yaml block.
///
/// # It is the FIRST such block, and that is a named ceiling
///
/// The block is chosen by POSITION: this opens at the first fence line carrying
/// the yaml language tag and closes at the next bare fence. The root README
/// writes two yaml blocks — the phase and workspace declaration this reader
/// wants, and the root calculus's own obligation rows — and this reader gets the
/// right one because the right one happens to be written first. That is a fact
/// about the file's current order rather than about the reader, so it fails OPEN
/// the day the order changes: a block inserted above would be read as the
/// toolchain and member declaration and joined against the manifest as though it
/// were one.
///
/// It is the same missing reading [`classify_green_rows`] states its own ceiling
/// on, and it opens on the same condition — a Markdown parser and typed fenced
/// blocks selected by SCHEMA rather than by position. Teaching this one to count
/// fences would close the position case and leave the schema case standing, in a
/// second reader that would then have to agree with the first about what a block
/// is.
pub(crate) fn readme_yaml_block(root: &Path) -> Result<Vec<String>, String> {
    let readme =
        fs::read_to_string(root.join("README.md")).map_err(|e| format!("README.md: {e}"))?;
    let mut lines = Vec::new();
    let mut inside = false;
    for line in readme.lines() {
        if inside {
            if line.trim() == "```" {
                return Ok(lines);
            }
            lines.push(line.to_string());
        } else if line.trim() == "```yaml" {
            inside = true;
        }
    }
    Err(String::from("README.md has no fenced yaml block"))
}

/// Every home README the join reads: the root one, and one per numbered band.
pub(crate) fn home_readmes(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut readmes = vec![root.join("README.md")];
    let src = root.join("src");
    let entries = fs::read_dir(&src).map_err(|e| format!("{}: {e}", src.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", src.display()))?;
        let candidate = entry.path().join("README.md");
        if candidate.is_file() {
            readmes.push(candidate);
        }
    }
    Ok(readmes)
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
/// this reader is exactly that: the rule was written for the seat branch, the
/// route branch went on taking its first token and discarding the rest, and the
/// defect regrew one branch to the right. A rule applied per site regrows one
/// site over; a rule applied to the class does not.
///
/// Private to this reader. It is not vocabulary two families share — nothing
/// outside these few functions has an opinion about how a green row is spelled —
/// so it is not in `types.rs`.
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

/// Every `green:` obligation row in one README, classified, in file order.
///
/// THE prefix discipline for green rows, and the only one: this reader is the
/// whole population, so nothing downstream matches a prefix of its own.
///
/// The prefix is matched WITHOUT its trailing space and the value is trimmed
/// after, so a row whose value was emptied is read as a row with an empty value
/// rather than vanishing from the population — and, for the same reason, a row
/// spelled `green:laws.rs …`, or with a tab, or with two spaces, is read as the
/// row it plainly is. A second reader matching `"green: laws.rs "` dropped every
/// one of those while this one seated them, and a row seated by one reader and
/// claimed by neither is an obligation that qualifies while naming a law nobody
/// wrote. There is no second reader now: the seat carries its target, so the
/// spacing a row happens to be written with cannot decide whether its claim is
/// joined.
///
/// # The scan is the WHOLE file, and that is a named ceiling
///
/// A row is found by reading every line of the README, so a line beginning
/// `green:` outside an obligation block — in prose, in a `text` fence, in a
/// worked example showing what a row looks like — is read as an obligation row
/// and joined like one. Nothing about a fence, a heading, or a block is looked
/// at here, because nothing in this file reads Markdown structure at all.
///
/// It fails LOUDLY rather than open, and that is why it is a ceiling rather than
/// a defect being carried quietly. An invented row is classified like every
/// other and then answered: unreadable where no grammar reads it, resolved
/// against `laws.rs` and testpak where one does, and named against the README
/// that wrote it wherever it does not resolve. Nothing reads as proven that is
/// not. What the ceiling costs is a writer who wanted to DESCRIBE a row without
/// declaring one — and the tree pays that price today rather than suffering
/// from it: no line beginning `green:` is written outside an obligation block
/// anywhere in the homes this reader is given.
///
/// It is not closed here, and the reason is the SHAPE of the repair rather than
/// its size. Restricting the scan to the obligation blocks means knowing where
/// those blocks are, which means reading Markdown structure — and this file
/// already carries that same missing reading a second time, in
/// [`readme_yaml_block`], which takes the first fenced block and calls it the
/// one it wanted. A fence-tracking line scan here would be one more ad-hoc
/// reader in a file whose entire defect history is ad-hoc readers, and it would
/// have to be written twice or shared between two readers that want different
/// blocks. The opening condition is the typed repository model: one Markdown
/// parser, typed fenced data blocks selected by SCHEMA rather than by position,
/// and one versioned obligation schema read out of them. Both readings close
/// there, together, and neither closes here.
///
/// The exposure is stated above and deliberately NOT asserted. A control over it
/// would have to find the obligation blocks to know which rows are outside one,
/// which is exactly the reading this ceiling is waiting for — a test that built
/// the thing it is waiting for would be that thing, arriving through the test
/// file instead of through the reader.
pub(crate) fn classify_green_rows(readme_text: &str) -> Vec<GreenRow> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("green:"))
        .map(|value| classify_green_row(value.trim()))
        .collect()
}

/// One green row's value, classified. Nothing is dropped: a value no lawful
/// spelling reads comes back as [`GreenRow::Unreadable`] carrying itself.
///
/// The row's grammar is decided first, its account is held to that grammar's
/// ceiling second, and only then is a row built. The ceiling is applied HERE, to
/// every grammar at once, rather than inside the arms below: an arm can only
/// interpret an account this function has already agreed is the whole of what
/// the row stated, so there is no arm left that could read past its own account.
fn classify_green_row(value: &str) -> GreenRow {
    let Some(opening) = value.split_whitespace().next() else {
        return GreenRow::Unreadable(value.to_string());
    };
    let account = value.strip_prefix(opening).unwrap_or_default().trim();
    let read = green_grammar(opening)
        .filter(|&grammar| states_only_its_account(grammar, account))
        .and_then(|grammar| match grammar {
            Grammar::Seat => seat_target(account).map(|(module, law)| GreenRow::CompileTimeSeat {
                module: module.to_string(),
                law: law.to_string(),
            }),
            Grammar::Route => Some(GreenRow::Route(opening.to_string())),
            Grammar::Disposition(opener) => {
                accounts_after(account, opener).then_some(GreenRow::Disposition)
            }
        });
    read.unwrap_or_else(|| GreenRow::Unreadable(value.to_string()))
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
/// ever held for the branch written last. A seat row carrying a word after its
/// target was closed inside the seat reader, and a route row carrying a word
/// after its path went on resolving the real file and qualifying while the rest
/// of what it stated was thrown away. The defect did not survive that repair; it
/// moved one branch over.
///
/// What a row said beyond its account is never the point. A second target
/// somebody meant to add, a note, half of a finished rename, a path that was
/// supposed to replace the one in front of it — the row said something this
/// repository does not read, and a reader that truncates it silently converts it
/// into a claim its author did not make. Named against the README that wrote it,
/// the author says what they meant instead.
///
/// # The disposition grammar has no ceiling, and that is a decision
///
/// A `none`, `owed`, or `structural` row accounts for why NO file holds a
/// positive control, and an account of that kind is prose: eight to twelve words
/// in every such row this repository has written, some of them running on across
/// a wrapped line. Prose legitimately has many tokens, so there is no number to
/// hold it to and this rule admits it without one — deliberately, and stated
/// here rather than left as the case nobody got to.
///
/// It is the same asymmetry the `red:` rows are read under, for the same reason.
/// A seat target and a route path are JOIN KEYS: they are resolved against
/// `laws.rs` and against testpak, and a key that names two things resolves
/// neither. A disposition's account joins nothing and is read by a person.
/// Holding the sentences to the keys' rule would unread every disposition row in
/// this tree; holding the keys to the sentences' rule is the defect this
/// function refuses.
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
/// The one place a green target is read. The split happens here rather than
/// downstream so that a seat and its claim are the same act: every row this
/// function splits becomes a seat carrying that exact pair, and every row it
/// cannot split names no target at all and leaves as [`GreenRow::Unreadable`],
/// answered by the leg that names it against the README that wrote it. Neither
/// outcome depends on how the row was spaced, and no later reader gets a second
/// opinion about which characters the target was made of.
///
/// The account arrives already held to one token by
/// [`states_only_its_account`], so a target is never truncated out of a longer
/// account here — this function reads what the row stated, whole.
///
/// # A target is EXACTLY `module::law`
///
/// One separator, and neither half empty. `root::a_law::extra` is not a deeper
/// target, it is two separators; `::a_law` names no module and `root::` names no
/// law, and each of those halves is a name the join resolves against. Split
/// looser — on the FIRST `::`, with nothing said about the rest — all three
/// became seats: `root::a_law::extra` seated the module `root` with a law called
/// `a_law::extra`, and `::a_law` seated a law under a module whose name is the
/// empty string.
///
/// That last one is not merely a wrong-looking pair. `laws.rs` is read by
/// tracking the module last opened at the crate root, which begins as no module
/// at all, so a `#[test]` written above the first `mod` would be declared under
/// exactly that empty name — and a row spelled `laws.rs ::that_law` would then
/// resolve to it and qualify. Today no such law is written and all three
/// spellings are refused downstream instead, by the leg that reports a claim on
/// a law `laws.rs` does not have: a real refusal, on the wrong subject, sending
/// the author to `laws.rs` when the repair is in the README's own row. Read
/// here, the row is named against the README that wrote it, which is how every
/// other unreadable row in this repository is answered, and the door the empty
/// module left open is shut before anything walks through it.
///
/// The RED rows are a DIFFERENT grammar and are deliberately not held to any of
/// this. A `red:` or `tooling-red:` row names its reversal and then continues in
/// prose, across wrapped lines, as this repository's tooling ledgers are
/// written; that convention is documented where those rows are declared and it
/// stays. A green compile-time target is not prose with a path in front of it,
/// and reading the two the same way would either silence this row or break every
/// one of those.
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

/// The value of every `red:` obligation row in one README, in file order.
///
/// The prefix is matched on the TRIMMED line, so a word merely ending in `red`
/// followed by a colon — `unnumbered:`, `authored:`, `Shred:` — is never a row.
/// It is matched WITHOUT its trailing space for the reason the green side is: a
/// row whose value was emptied is still a row, and a reader that stopped seeing
/// it would quietly shrink the denominator this repository publishes.
///
/// THE prefix discipline for red rows, and the only one, on the same rule the
/// green side is held to: one population gets one reader. Two readers over these
/// rows would not merely disagree, they would disagree about a published number,
/// and the row each of them dropped would be the row nobody looked at.
///
/// The whole value is carried, prose and all. A red row NAMES its reversal and
/// then says what the reversal does, wrapped across as many lines as the
/// sentence takes — `owed-to-testpak — cloning a Budget must not compile` is the
/// same grammar as a row naming a fixture and then describing it. The ledger
/// reads the name off the front and the prose is for the reader. That is
/// deliberately NOT the green compile-time grammar, where the target is one
/// token and a second token makes the row unreadable: a green seat is a join key
/// and a red row is a sentence, and holding either to the other's rule would
/// break every ledger row this repository has written.
pub(crate) fn red_twin_rows(readme_text: &str) -> Vec<String> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("red:"))
        .map(|value| value.trim().to_string())
        .collect()
}

/// The value of every `tooling-red:` obligation row in one README, in file
/// order.
///
/// Read exactly like a core `red:` row — same prefix discipline, same emptied
/// row still read, same name-then-prose grammar — and counted on its own ledger.
/// An `owed-to-…` row is a lawful debt; any other row NAMES a reversal that must
/// resolve to a real testpak test or compile-fail fixture, and the check refuses
/// it if it does not.
///
/// Several of the tooling ledgers' rows state what their reversal does after
/// naming it, and one of them names sibling fixtures in that prose. The ledger
/// resolves the first token and reads none of the rest, which is a stated
/// ceiling rather than a convention: a row carrying more than one fixture
/// reference has exactly one of them joined, and the others are prose no check
/// looks at. Closing that takes typed fixture references in the row itself,
/// which the versioned claim and evidence schema opens; it is not closed by
/// splitting a sentence on whitespace.
pub(crate) fn tooling_red_rows(readme_text: &str) -> Vec<String> {
    readme_text
        .lines()
        .filter_map(|line| line.trim().strip_prefix("tooling-red:"))
        .map(|value| value.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{classify_green_rows, red_twin_rows, tooling_red_rows};
    use crate::repository::types::GreenRow;

    /// A row is read off the trimmed line, so an ordinary word ending in `red`
    /// followed by a colon is never mistaken for one.
    #[test]
    fn only_a_red_row_is_a_red_row() {
        let text = "unnumbered: first-class, not a row\n\
                    Shred: the four progress facts, not a row\n\
                    ## The connectives (authored: a heading, not a row)\n\
                    \x20   red: owed-to-testpak\n";
        assert_eq!(red_twin_rows(text), vec![String::from("owed-to-testpak")]);
    }

    /// Planted reversal: a `red:` and a `tooling-red:` row whose value was
    /// emptied. Read on the trailing space, neither line was a row at all, and
    /// the obligation left the denominator without anything refusing.
    ///
    /// The red side's twin of the green side's dropped route, and the more
    /// expensive of the two: the core and tooling red counts are PUBLISHED on
    /// every run, so a row that stops being read shrinks a number the campaign
    /// reports rather than merely going unchecked.
    #[test]
    fn an_emptied_red_row_is_still_a_row() {
        let text = "    red:\n\
                    \x20   tooling-red:\n\
                    \x20   red: owed-to-testpak\n";
        assert_eq!(
            red_twin_rows(text),
            vec![String::new(), String::from("owed-to-testpak")]
        );
        assert_eq!(tooling_red_rows(text), vec![String::new()]);
    }

    /// One seat carrying the target it named.
    fn seat(module: &str, law: &str) -> GreenRow {
        GreenRow::CompileTimeSeat {
            module: String::from(module),
            law: String::from(law),
        }
    }

    /// The positive control: every spelling this repository actually writes is
    /// read as the spelling it is, and the route population is exactly the rows
    /// naming a file.
    ///
    /// A reader that also returned the `laws.rs` rows as routes would demand a
    /// test FILE from a row whose control is a compile-time seat, and one that
    /// swallowed the prose dispositions would demand a file from a row whose
    /// whole content is that no file exists.
    #[test]
    fn every_lawful_green_spelling_is_read_as_itself() {
        let text = "    green: laws.rs root::a_seat_that_exists\n\
                    \x20   green: none — the type's nonexistence is what refuses\n\
                    \x20   green: owed — executable when the roster lands\n\
                    \x20   green: structural (a phantom makes the handle !Send)\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.rs\n";
        assert_eq!(
            classify_green_rows(text),
            vec![
                seat("root", "a_seat_that_exists"),
                GreenRow::Disposition,
                GreenRow::Disposition,
                GreenRow::Disposition,
                GreenRow::Route(String::from("testpak/tests/stamp_row_ceiling.rs")),
            ]
        );
    }

    /// Planted reversal: four seat rows whose only fault is how they were
    /// spaced. A SECOND reader matching the literal `"green: laws.rs "` dropped
    /// every one of them — no space after the colon, two spaces, a tab, a tab
    /// between `laws.rs` and its target — while this reader seated all four.
    ///
    /// That gap is the whole defect, and it was silent by construction: the
    /// dropped rows never reached the join, so the leg that refuses a claim on a
    /// law nobody wrote never saw them, and the count of rows READ still agreed
    /// with the count of rows WRITTEN because the classifier had counted them.
    /// An obligation could therefore qualify while naming a law that does not
    /// exist, on nothing but a keystroke of whitespace. Every one of these now
    /// carries the same target as the row spelled the ordinary way, so the join
    /// resolves them all.
    #[test]
    fn a_seat_row_is_read_however_it_is_spaced() {
        let text = "    green:laws.rs root::a_seat_that_exists\n\
                    \x20   green:  laws.rs root::a_seat_that_exists\n\
                    \x20   green:\tlaws.rs root::a_seat_that_exists\n\
                    \x20   green: laws.rs\troot::a_seat_that_exists\n";
        let read = classify_green_rows(text);
        assert_eq!(
            read,
            vec![
                seat("root", "a_seat_that_exists"),
                seat("root", "a_seat_that_exists"),
                seat("root", "a_seat_that_exists"),
                seat("root", "a_seat_that_exists"),
            ],
            "{read:?}"
        );
    }

    /// Planted reversal: a seat row whose target is not `module::law`, in the
    /// two ways it goes wrong — one colon, and a name with no module in front
    /// of it.
    ///
    /// Both used to be read by a second reader that answered a MALFORMED target
    /// by failing the whole join with a message about the target alone. Read
    /// here, the row is named against the README that wrote it and the rest of
    /// the population is still judged, which is how every other unreadable row
    /// in this repository is answered.
    #[test]
    fn a_seat_without_a_module_law_target_is_unreadable() {
        let text = "    green: laws.rs root:a_seat_that_exists\n\
                    \x20   green: laws.rs a_seat_that_exists\n";
        assert_eq!(
            classify_green_rows(text),
            vec![
                GreenRow::Unreadable(String::from("laws.rs root:a_seat_that_exists")),
                GreenRow::Unreadable(String::from("laws.rs a_seat_that_exists")),
            ]
        );
    }

    /// Planted reversal: a seat row carrying a token AFTER its target.
    ///
    /// The row resolves a real law — `root::reading_is_not_gaining` is written
    /// in the root README and declared in `laws.rs` — so every join leg downstream
    /// says yes and the obligation qualifies. What it says beyond the target was
    /// simply thrown away: the reader took the first token of the account and
    /// dropped the rest, so a second target somebody meant to add, a stray note,
    /// or half of a finished rename all read as the ordinary one-token row. A
    /// green target is ONE token; a row that says more says something this
    /// repository does not read, and it is named against the README that wrote it
    /// rather than truncated into a claim it did not make.
    ///
    /// The rule this now stands on is the class's, not the seat branch's. It is
    /// the same rule the route row two tests below is refused by, which is the
    /// whole point: stated per branch it held here and nowhere else.
    ///
    /// The last two are the same defect where the trailing token is itself
    /// target-shaped, which is the spelling a reader that stopped at the first
    /// `::` would never notice.
    #[test]
    fn a_seat_row_carrying_more_than_its_target_is_unreadable() {
        let text = "    green: laws.rs root::reading_is_not_gaining extra\n\
                    \x20   green: laws.rs root::reading_is_not_gaining — and the note nobody read\n\
                    \x20   green: laws.rs root::reading_is_not_gaining\troot::closure_bar_is_implementable\n\
                    \x20   green: laws.rs root::reading_is_not_gaining root::closure_bar_is_implementable\n";
        let read = classify_green_rows(text);
        assert_eq!(read.len(), 4, "{read:?}");
        assert!(
            read.iter()
                .all(|row| matches!(*row, GreenRow::Unreadable(_))),
            "a trailing token was discarded and the row still seated: {read:?}"
        );
        assert!(
            read.first().is_some_and(|row| matches!(
                *row,
                GreenRow::Unreadable(ref value) if value == "laws.rs root::reading_is_not_gaining extra"
            )),
            "{read:?}"
        );
    }

    /// The positive control for that narrowing: the target this repository
    /// actually writes — one token, however the row is spaced — is still a seat.
    ///
    /// A reader that refused every account carrying whitespace would satisfy the
    /// reversal above and would unseat all 183 seat rows in the tree.
    #[test]
    fn a_seat_row_stating_exactly_its_target_is_still_a_seat() {
        let text = "    green: laws.rs root::reading_is_not_gaining\n\
                    \x20   green:laws.rs root::reading_is_not_gaining\n\
                    \x20   green: laws.rs\troot::reading_is_not_gaining\n\
                    \x20   green:  laws.rs   root::reading_is_not_gaining  \n";
        let read = classify_green_rows(text);
        assert_eq!(
            read,
            vec![
                seat("root", "reading_is_not_gaining"),
                seat("root", "reading_is_not_gaining"),
                seat("root", "reading_is_not_gaining"),
                seat("root", "reading_is_not_gaining"),
            ],
            "{read:?}"
        );
    }

    /// Planted reversal: a route row carrying a token AFTER its path.
    ///
    /// The same defect as the seat row above, one branch to the right, and it
    /// survived the round that closed the seat branch because that round wrote
    /// its rule inside the seat reader. The route branch went on reading the
    /// FIRST token and discarding the account: a row spelled
    /// `testpak/tests/stamp_row_ceiling.rs missing-control.rs` resolved the real
    /// seat, satisfied the phantom-route leg, and qualified while the second path
    /// it stated was never looked for — which is precisely the failure the route
    /// leg exists to refuse, arriving through the reader that feeds it.
    ///
    /// The last row is the spelling that makes the silence worst: the trailing
    /// token is itself a real, executable seat, so the row states two positive
    /// controls and exactly one of them is ever examined.
    #[test]
    fn a_route_row_carrying_more_than_its_path_is_unreadable() {
        let text = "    green: testpak/tests/stamp_row_ceiling.rs missing-control.rs\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.rs — and the note nobody read\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.rs\tmissing-control.rs\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.rs testpak/tests/stamp_row_ceiling.rs\n";
        let read = classify_green_rows(text);
        assert_eq!(read.len(), 4, "{read:?}");
        assert!(
            read.iter()
                .all(|row| matches!(*row, GreenRow::Unreadable(_))),
            "a token after the path was discarded and the row still routed: {read:?}"
        );
        assert!(
            read.first().is_some_and(|row| matches!(
                *row,
                GreenRow::Unreadable(ref value)
                    if value == "testpak/tests/stamp_row_ceiling.rs missing-control.rs"
            )),
            "{read:?}"
        );
    }

    /// The positive control for that narrowing: the one route row this
    /// repository actually writes, in every spacing the reader admits, is still
    /// a route carrying its exact path.
    ///
    /// A reader that refused a route whose value carried any whitespace at all
    /// would satisfy the reversal above and would unroute the only green route in
    /// the tree — and an unrouted row is an unreadable row, so the whole check
    /// would fail loudly rather than silently. Which is why the control names the
    /// real path rather than a fixture one.
    #[test]
    fn a_route_row_stating_exactly_its_path_is_still_a_route() {
        let text = "    green: testpak/tests/stamp_row_ceiling.rs\n\
                    \x20   green:testpak/tests/stamp_row_ceiling.rs\n\
                    \x20   green:  testpak/tests/stamp_row_ceiling.rs  \n\
                    \x20   green:\ttestpak/tests/stamp_row_ceiling.rs\n";
        let read = classify_green_rows(text);
        assert_eq!(
            read,
            vec![
                GreenRow::Route(String::from("testpak/tests/stamp_row_ceiling.rs")),
                GreenRow::Route(String::from("testpak/tests/stamp_row_ceiling.rs")),
                GreenRow::Route(String::from("testpak/tests/stamp_row_ceiling.rs")),
                GreenRow::Route(String::from("testpak/tests/stamp_row_ceiling.rs")),
            ],
            "{read:?}"
        );
    }

    /// Planted reversal: a target that is not exactly `module::law`, in all four
    /// of its malformed spellings.
    ///
    /// Split on the FIRST `::` and asked nothing further, the first three were
    /// seats. `root::reading_is_not_gaining::extra` seated the module `root` with
    /// a law named `reading_is_not_gaining::extra`; `root::` seated a law whose
    /// name is empty; `::reading_is_not_gaining` seated a law under a module
    /// whose name is empty — and the empty module name is one `laws.rs` can
    /// actually produce, because that file is read by tracking the module last
    /// opened at the crate root and it starts as no module at all. A `#[test]`
    /// written above the first `mod` is declared under exactly that name, and
    /// this row would then resolve to it and qualify.
    ///
    /// Today none of the three resolves, so each was refused downstream by the
    /// leg reporting a claim on a law `laws.rs` does not have: the right verdict
    /// with the wrong subject, sending the author to `laws.rs` when the repair is
    /// in the row. The fourth, a bare word with no separator at all, was already
    /// read here. All four are now one answer.
    #[test]
    fn a_seat_target_that_is_not_exactly_module_law_is_unreadable() {
        let text = "    green: laws.rs root::reading_is_not_gaining::extra\n\
                    \x20   green: laws.rs ::reading_is_not_gaining\n\
                    \x20   green: laws.rs root::\n\
                    \x20   green: laws.rs root\n";
        let read = classify_green_rows(text);
        assert_eq!(read.len(), 4, "{read:?}");
        assert!(
            read.iter()
                .all(|row| matches!(*row, GreenRow::Unreadable(_))),
            "a malformed target was seated: {read:?}"
        );
        assert!(
            read.first().is_some_and(|row| matches!(
                *row,
                GreenRow::Unreadable(ref value)
                    if value == "laws.rs root::reading_is_not_gaining::extra"
            )),
            "{read:?}"
        );
    }

    /// The disposition account is PROSE, and it is held to no token ceiling.
    ///
    /// Stated as its own test so the asymmetry is a decision with a control on it
    /// rather than the case a pass over the green class forgot. A disposition
    /// accounts for why no file holds a positive control, and that account is a
    /// sentence a person reads — it joins nothing, so there is no key for a
    /// second token to make ambiguous. Every such row in this tree runs eight to
    /// twelve words, and some run on across a wrapped line; a pass that carried
    /// the seat and route ceiling across to them would unread all eleven at once.
    ///
    /// The reversal that matters here is the opposite one, and it is the test
    /// below: prose having no CEILING is not prose having no FORM. The opener and
    /// a word after it are still required.
    #[test]
    fn a_disposition_account_is_prose_and_carries_no_token_ceiling() {
        let text = "    green: none — no family payload can carry a spelling, skeleton, or scalar\n\
                    \x20   green: owed — the P1–P10 campaigns land with testpak (the heartbeat's\n\
                    \x20   green: structural (raw-pointer phantom makes the handle !Send and !Sync)\n\
                    \x20   green: none — one\n";
        let read = classify_green_rows(text);
        assert_eq!(
            read,
            vec![
                GreenRow::Disposition,
                GreenRow::Disposition,
                GreenRow::Disposition,
                GreenRow::Disposition,
            ],
            "{read:?}"
        );
    }

    /// The red rows are a DIFFERENT grammar, and this is the test that says so.
    ///
    /// A red row names its reversal and then speaks prose about it, which is how
    /// this repository's tooling ledgers are written and how several rows in
    /// `macros/macroc/README.md` are written today. The green side's target is now
    /// exactly one token, and a pass that carried that rule across to the red side
    /// would silently unresolve every one of those rows and shrink a published
    /// denominator. Written here so the asymmetry is a decision with a control on
    /// it rather than an inconsistency somebody tidies up.
    #[test]
    fn a_red_row_names_its_reversal_and_then_speaks_prose() {
        let text = "  tooling-red: testpak/tests/failed_seat_refusals.rs — the plane restores each\n\
                    \x20 red: owed-to-testpak — cloning a Budget must not compile\n";
        assert_eq!(
            tooling_red_rows(text),
            vec![String::from(
                "testpak/tests/failed_seat_refusals.rs — the plane restores each"
            )]
        );
        assert_eq!(
            red_twin_rows(text),
            vec![String::from(
                "owed-to-testpak — cloning a Budget must not compile"
            )]
        );
    }

    /// Planted reversal: five green rows no lawful spelling reads. Under a
    /// reader that FILTERED to the path-shaped rows, every one of them was
    /// dropped — and a dropped row is an obligation that qualifies while the
    /// positive control it names is never looked for.
    #[test]
    fn a_green_row_no_spelling_reads_is_named_not_dropped() {
        let text = "    green: testpak/tests/stamp_row_ceiling.r\n\
                    \x20   green: testpak/tests/stamp_row_ceiling.md\n\
                    \x20   green: structural\n\
                    \x20   green:\n\
                    \x20   green: laws.rs\n";
        assert_eq!(
            classify_green_rows(text),
            vec![
                GreenRow::Unreadable(String::from("testpak/tests/stamp_row_ceiling.r")),
                GreenRow::Unreadable(String::from("testpak/tests/stamp_row_ceiling.md")),
                GreenRow::Unreadable(String::from("structural")),
                GreenRow::Unreadable(String::new()),
                GreenRow::Unreadable(String::from("laws.rs")),
            ]
        );
    }

    /// Planted reversal: a disposition that states the absence and withholds
    /// the account of it, in all three of its words.
    ///
    /// The word alone is the half of the form that says nothing. `none` with no
    /// reason is indistinguishable from a row somebody gave up on, and the
    /// separator is what this repository's rows are actually written with.
    #[test]
    fn a_disposition_without_its_account_is_unreadable() {
        let text = "    green: none\n\
                    \x20   green: owed\n\
                    \x20   green: none - a hyphen is not the declared separator\n\
                    \x20   green: structural ()\n\
                    \x20   green: owed —\n";
        let read = classify_green_rows(text);
        assert!(
            read.iter()
                .all(|row| matches!(*row, GreenRow::Unreadable(_))),
            "{read:?}"
        );
        assert_eq!(read.len(), 5, "{read:?}");
    }
}
