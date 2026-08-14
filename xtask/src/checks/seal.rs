//! The seal: a stamped scope guard has one road in and no road out.
//!
//! Band 02's `scope_guard_version!` stamp writes a Class-C guard as a private
//! seat holding one `AuthorityPosition`, one road in — `positioned` — and one
//! comparison that reads the seat from inside. The asymmetry IS the law the
//! stamp exists to carry: a position that can leave a role can be re-entered
//! under a different role, and a role a representation can leave has stopped
//! being a wall.
//!
//! # Why a reversal cannot state this claim, and why this law can
//!
//! `testpak/tests/compile-fail/a-production-scope-guard-cannot-be-laundered.rs`
//! attempts the two roads a laundering caller has today: reading the seat as
//! `version.0`, and re-entering it as `FrameVersion(position)`. Both refuse, and
//! both keep refusing after a public `position()` or `into_position()` is added
//! to the guard — the field stays private and the tuple constructor stays
//! unreachable, so the recorded diagnostic does not move by one byte while the
//! sealed value walks out through a road with a name. A fixture can only attempt
//! roads somebody thought of; the absence of EVERY road out is not a sentence
//! Rust can be asked to refuse.
//!
//! So the absence is established here instead, by reading what the tree
//! declares. The population is DERIVED — every type the stamp is invoked for,
//! read off the sources — and so is the seat: `AuthorityPosition` is not written
//! down in this file, it is read out of the stamp's own transcriber, so a stamp
//! reseated over a different inner type is judged over the type it actually
//! seals.
//!
//! # The two places a road out can be written, and both are read
//!
//! One is the stamp itself. An accessor added to the transcriber arrives on
//! every guard the machine stamps at once, which is the worst version of this
//! defect and the cheapest to write. The other is a hand-written implementation
//! beside a guard, which reaches the private seat because a `macro_rules!`
//! expansion is expanded IN the invoking module and its field is private to that
//! module. Both are read.
//!
//! A transcriber is not Rust until its metavariables have values, so it is given
//! values — `$vis` becomes `pub`, `$crate` becomes `crate`, every other
//! metavariable becomes an ordinary identifier, and a repetition contributes its
//! body once — and the result is parsed as Rust. A transcriber that will not
//! parse under that substitution is an offence rather than a skip: a stamp this
//! law cannot read is a stamp this law is not guarding.
//!
//! # What a road out is
//!
//! A public associated function or method of a guarded type whose RETURN
//! mentions every sealed seat. Named and positional seats are read alike, a
//! reference is a road out exactly as an owned value is — `AuthorityPosition`
//! is `Clone`, so a borrow of it is one clone away from a re-wrappable value —
//! and an associated TYPE is read the same way, because `Deref` hands the seat
//! out through `Target` and never through a signature that names it. Every
//! member of a trait implementation is public: a trait's own visibility decides
//! that, and no `pub` appears on the member.
//!
//! The road out under another name is read too: an implementation that takes a
//! guarded type in its trait arguments and stands FOR the sealed seat —
//! `impl From<FrameVersion> for AuthorityPosition<…>` — hands the seat back out
//! without declaring a single member on the guard.
//!
//! # What this reader does not resolve
//!
//! It compiles nothing. A path is read by its LAST SEGMENT, so
//! `AuthorityPosition<S>` and `identity::AuthorityPosition<S>` are one seat here
//! and a type alias to either is neither. It does not evaluate `cfg`: a member
//! written under one is read as declared. It reads roads whose OWNER is the
//! guarded type, so a road declared on some third type that takes a guard by
//! reference and returns something read off it is outside this law — that shape
//! is a reification with its own claim to make, not a representation walking
//! out. And a stamped guard reached only through a macro that composes the
//! stamp's name from fragments is outside it as well.

use proc_macro2::{Delimiter, Group, Ident, Span, TokenStream, TokenTree};

use crate::repository::snapshot::{MACHINE_DIRECTORY, RepositorySnapshot, TOOLING_DIRECTORY};
use crate::repository::types::CanonicalPath;

/// The proof surfaces, excluded from the population by name.
///
/// Both crates' `laws.rs` stamp demonstration guards so the guard law has
/// something to prove itself against. A fixture in a denominator about the
/// machine would inflate the count with roles nothing ships.
const PROOF_SURFACES: [&str; 2] = ["src/laws.rs", "macros/macroc/src/laws.rs"];

/// The stamp whose invocation declares a sealed representation.
const SEAL_STAMP: &str = "scope_guard_version";

/// The type-level marker seat, which seals no value.
///
/// A `PhantomData` field carries nothing a caller could take away, so a road
/// returning one hands out nothing. Counting it as a sealed seat would make
/// every guard whose only private field is a marker read as sealed by this law
/// while the law had said nothing at all.
const MARKER_SEAT: &str = "PhantomData";

/// Every stamped scope guard in the machine seals its position: no public road
/// of a guard hands the position back out, and neither does the stamp that
/// writes them.
///
/// # Errors
///
/// Returns the offences one line at a time, and returns a read failure as
/// itself: a gate that cannot read its subject says so rather than reporting an
/// empty population.
pub(crate) fn check_stamped_guards_seal_their_position(
    snapshot: &RepositorySnapshot,
) -> Result<(), String> {
    let sources = seal_sources(snapshot)?;
    let verdict = seal_verdict(&sources);

    // The denominator is DERIVED and printed on every run, because a population
    // that quietly shrank would otherwise keep this check passing while it
    // guarded less.
    println!(
        "stamped scope guards: {} sealed / {} stamped",
        verdict.sealed, verdict.stamped
    );
    if verdict.stamped == 0 {
        return Err(String::from(
            "no stamped scope guard was found: this denominator cannot be empty while the guards \
             exist, so the reader is looking at the wrong tree",
        ));
    }
    if verdict.offenders.is_empty() {
        Ok(())
    } else {
        Err(verdict.offenders.join("; "))
    }
}

/// What the seal leg counted, and what it refuses.
#[derive(Debug)]
struct SealVerdict {
    /// Types the stamp is invoked for.
    stamped: usize,
    /// Those of them no road hands a position out of.
    sealed: usize,
    /// Every offence, one line each.
    offenders: Vec<String>,
}

/// One type the stamp is invoked for.
struct StampedGuard {
    /// The repository-relative path that stamps it.
    path: String,
    /// The type the invocation names.
    name: String,
}

/// One implementation, as this law reads one.
struct Implementation {
    /// The repository-relative path that declares it.
    path: String,
    /// The head of the type it is written for.
    owner: Option<String>,
    /// Every path head the implemented-for type mentions.
    stands_for: Vec<String>,
    /// Every path head the implemented trait's own arguments mention, which is
    /// where a conversion names the type it converts FROM.
    takes: Vec<String>,
    /// Every member it declares.
    members: Vec<Member>,
}

/// One member of an implementation, and what it hands its caller.
struct Member {
    /// Its declared name.
    name: String,
    /// How far a caller can reach it from.
    reach: Reach,
    /// Every path head the member's return type — or, for an associated type,
    /// its value — mentions.
    hands_out: Vec<String>,
}

/// How far one member of an implementation reaches.
///
/// Every member of a trait implementation is `Outside`: the trait's own
/// visibility decides that, and no `pub` is written on the member.
#[derive(PartialEq, Eq)]
enum Reach {
    /// Reachable from outside the crate that declares it.
    Outside,
    /// Reachable only from inside.
    Inside,
}

/// Everything one pass over the sources read.
struct Reading {
    /// Every struct declared, by name, with the seats it seals.
    seats: Vec<(String, Vec<String>)>,
    /// Every implementation, in every module the pass entered.
    implementations: Vec<Implementation>,
    /// Every type the stamp is invoked for.
    stamped: Vec<StampedGuard>,
    /// Every declaration of the stamp itself, as the transcriber tokens it
    /// carries.
    stamps: Vec<(String, TokenStream)>,
    /// Sources that are not parseable Rust, one offence each. Never a skip: a
    /// file this reader could not read is a hole in the population.
    unparsable: Vec<String>,
}

/// Reads the stamp, its invocations, and every implementation out of parsed
/// trees, and judges each stamped guard.
///
/// Pure over its inputs — `(canonical path, parsed tree)` pairs handed over by
/// the snapshot — so the reversals below are planted in memory and the law that
/// guards the tree is never proven by editing one. A source that did not parse
/// never reaches here: the snapshot carries it as unread, and the caller refuses
/// the whole reading rather than deriving a population one file short.
fn seal_verdict(sources: &[(&CanonicalPath, &syn::File)]) -> SealVerdict {
    let reading = read_sources(sources);
    let mut verdict = SealVerdict {
        stamped: reading.stamped.len(),
        sealed: 0,
        offenders: reading.unparsable.clone(),
    };
    let opened = verdict.offenders.len();
    let seats = match reading.stamps.len() {
        1 => stamp_seats(&reading, &mut verdict.offenders),
        0 => {
            verdict.offenders.push(format!(
                "no `{SEAL_STAMP}!` declaration was found: the seat this law judges guards against \
                 is read off the stamp, so a tree without one is a tree this law is not reading"
            ));
            Vec::new()
        }
        _ => {
            verdict.offenders.push(format!(
                "two `{SEAL_STAMP}!` declarations stand in the tree, so which shape a guard is \
                 stamped in is a traversal order rather than a fact"
            ));
            Vec::new()
        }
    };
    // A road the stamp itself emits arrives on every guard at once, so no guard
    // is sealed while one stands. The offence is reported once, at the stamp,
    // and the numerator says what it costs.
    let stamp_holds = verdict.offenders.len() == opened;
    if seats.is_empty() {
        return verdict;
    }
    for guard in &reading.stamped {
        let before = verdict.offenders.len();
        judge(
            &reading,
            &guard.name,
            Some(&guard.path),
            &seats,
            &mut verdict.offenders,
        );
        if stamp_holds && verdict.offenders.len() == before {
            verdict.sealed = verdict.sealed.saturating_add(1);
        }
    }
    verdict
}

/// The seats the stamp seals, read out of the one transcriber it carries — and
/// the stamp's own emitted roads, judged against them here, because a road the
/// stamp writes arrives on every guard at once.
fn stamp_seats(reading: &Reading, offenders: &mut Vec<String>) -> Vec<String> {
    let Some((path, tokens)) = reading.stamps.first() else {
        return Vec::new();
    };
    // The transcriber's own coordinate, so an offence at the stamp cannot read
    // as an offence in the file's ordinary items.
    let coordinate = format!("{path} (the `{SEAL_STAMP}!` transcriber)");
    let mut emitted = Vec::new();
    for transcriber in transcribers(tokens.clone()) {
        match syn::parse2::<syn::File>(substituted(transcriber)) {
            Ok(file) => {
                let mut shape = empty_reading();
                read_module(&coordinate, &file.items, &mut shape);
                emitted.push(shape);
            }
            Err(error) => offenders.push(format!(
                "{path}: the `{SEAL_STAMP}!` transcriber does not parse as Rust once its \
                 metavariables are given values, so what the stamp emits is unknown rather than \
                 empty: {error}"
            )),
        }
    }
    let mut seats = Vec::new();
    for shape in &emitted {
        emitted_seats(shape, &mut seats, offenders);
    }
    if seats.is_empty() {
        offenders.push(format!(
            "{path}: the `{SEAL_STAMP}!` transcriber declares no private seat, so there is no \
             sealed position for this law to be about"
        ));
    }
    seats
}

/// Every seat one emitted shape seals, collected once each, with the shape's
/// own roads judged against the seats they belong to.
fn emitted_seats(shape: &Reading, seats: &mut Vec<String>, offenders: &mut Vec<String>) {
    for (name, sealed) in &shape.seats {
        if sealed.is_empty() {
            continue;
        }
        judge(shape, name, None, sealed, offenders);
        for seat in sealed {
            if !seats.contains(seat) {
                seats.push(seat.clone());
            }
        }
    }
}

/// Judges one guarded name against every implementation one reading holds,
/// pushing one offence per road out.
///
/// `stamped_at` is where the invocation that declares the guard stands, and it
/// is named in every offence because the road out and the declaration it
/// unseals are routinely in two different files.
fn judge(
    reading: &Reading,
    guard: &str,
    stamped_at: Option<&str>,
    seats: &[String],
    offenders: &mut Vec<String>,
) {
    let declared_at = match stamped_at {
        Some(where_stamped) => format!(", on the guard stamped at {where_stamped}"),
        None => String::new(),
    };
    for declared in &reading.implementations {
        judge_one(declared, guard, &declared_at, seats, offenders);
    }
}

/// Judges one implementation against one guarded name.
fn judge_one(
    declared: &Implementation,
    guard: &str,
    declared_at: &str,
    seats: &[String],
    offenders: &mut Vec<String>,
) {
    let path = &declared.path;
    if declared.owner.as_deref() == Some(guard) {
        let roads = declared
            .members
            .iter()
            .filter(|member| member.reach == Reach::Outside)
            .filter(|member| hands_back(&member.hands_out, seats));
        for road in roads {
            offenders.push(format!(
                "{path}: `{guard}::{}` hands the sealed position back out{declared_at}; the stamp \
                 emits one road in and none out, and a position that can leave its role can be \
                 re-entered under another one",
                road.name
            ));
        }
    }
    if declared.takes.iter().any(|taken| taken == guard) && hands_back(&declared.stands_for, seats)
    {
        offenders.push(format!(
            "{path}: an implementation takes `{guard}` and stands for its sealed \
             position{declared_at}, which is the road out written as a conversion"
        ));
    }
}

/// Whether one return hands every sealed seat back.
///
/// An empty seat set never satisfies this: a guard sealing nothing is a guard
/// this law has no claim about, and reading it as satisfied would turn silence
/// into coverage.
fn hands_back(hands_out: &[String], seats: &[String]) -> bool {
    !seats.is_empty() && seats.iter().all(|seat| hands_out.contains(seat))
}

/// An empty reading, so the pass and the transcriber's own shape are built the
/// same way.
fn empty_reading() -> Reading {
    Reading {
        seats: Vec::new(),
        implementations: Vec::new(),
        stamped: Vec::new(),
        stamps: Vec::new(),
        unparsable: Vec::new(),
    }
}

/// Reads the stamp, its invocations, and the implementations out of the parsed
/// trees.
fn read_sources(sources: &[(&CanonicalPath, &syn::File)]) -> Reading {
    let mut reading = empty_reading();
    for (path, file) in sources {
        read_module(path.as_str(), &file.items, &mut reading);
    }
    reading
}

/// Reads one module's items, then every inline module inside it.
fn read_module(path: &str, items: &[syn::Item], reading: &mut Reading) {
    for item in items {
        if let syn::Item::Struct(declared) = item {
            reading
                .seats
                .push((declared.ident.to_string(), sealed_seats(&declared.fields)));
        } else if let syn::Item::Impl(declared) = item {
            reading.implementations.push(implementation(path, declared));
        } else if let syn::Item::Macro(declared) = item {
            read_macro(path, declared, reading);
        } else if let syn::Item::Mod(module) = item
            && let Some((_, inner)) = &module.content
        {
            read_module(path, inner, reading);
        }
    }
}

/// Reads one macro item: the stamp's own declaration, or one invocation of it.
fn read_macro(path: &str, declared: &syn::ItemMacro, reading: &mut Reading) {
    if let Some(name) = &declared.ident {
        if name == SEAL_STAMP {
            reading
                .stamps
                .push((path.to_string(), declared.mac.tokens.clone()));
        }
        return;
    }
    if last_segment(&declared.mac.path).is_none_or(|last| last != SEAL_STAMP) {
        return;
    }
    match stamped_name(declared.mac.tokens.clone()) {
        Some(name) => reading.stamped.push(StampedGuard {
            path: path.to_string(),
            name,
        }),
        None => reading.unparsable.push(format!(
            "{path}: a `{SEAL_STAMP}!` invocation names no stamped struct, so the guard it declares \
             cannot be judged"
        )),
    }
}

/// The seats one body seals: the declared type heads of its PRIVATE fields,
/// each once, without the marker seat.
///
/// A public field seals nothing — it is already the caller's — so it is not a
/// seat this law is about.
fn sealed_seats(fields: &syn::Fields) -> Vec<String> {
    let declared: Vec<&syn::Field> = match *fields {
        syn::Fields::Named(ref named) => named.named.iter().collect(),
        syn::Fields::Unnamed(ref unnamed) => unnamed.unnamed.iter().collect(),
        syn::Fields::Unit => Vec::new(),
    };
    let mut seats = Vec::new();
    for field in declared {
        if matches!(field.vis, syn::Visibility::Public(_)) {
            continue;
        }
        let Some(head) = head_of(&field.ty) else {
            continue;
        };
        if head == MARKER_SEAT || seats.contains(&head) {
            continue;
        }
        seats.push(head);
    }
    seats
}

/// One implementation, read as this law reads one.
fn implementation(path: &str, declared: &syn::ItemImpl) -> Implementation {
    let mut stands_for = Vec::new();
    mentions(&declared.self_ty, &mut stands_for);
    let mut takes = Vec::new();
    let contract = declared.trait_.as_ref().map(|(named, _)| named);
    if let Some(named) = contract {
        mentions_arguments(named, &mut takes);
    }
    let mut members = Vec::new();
    for member in &declared.items {
        if let syn::ImplItem::Fn(function) = member {
            let mut hands_out = Vec::new();
            if let syn::ReturnType::Type(_, output) = &function.sig.output {
                mentions(output, &mut hands_out);
            }
            members.push(Member {
                name: function.sig.ident.to_string(),
                reach: reach_of(contract, &function.vis),
                hands_out,
            });
        } else if let syn::ImplItem::Type(associated) = member {
            let mut hands_out = Vec::new();
            mentions(&associated.ty, &mut hands_out);
            members.push(Member {
                name: associated.ident.to_string(),
                reach: reach_of(contract, &associated.vis),
                hands_out,
            });
        }
    }
    Implementation {
        path: path.to_string(),
        owner: head_of(&declared.self_ty),
        stands_for,
        takes,
        members,
    }
}

/// How far one member reaches: as far as its own word says inside an inherent
/// implementation, and always outside inside a trait implementation, where the
/// trait's own visibility decides and no word is written on the member.
///
/// The contract arrives as the path it is rather than as a flag, because a flag
/// beside the path it was computed from is a second thing to keep true.
fn reach_of(contract: Option<&syn::Path>, declared: &syn::Visibility) -> Reach {
    if contract.is_some() || matches!(*declared, syn::Visibility::Public(_)) {
        Reach::Outside
    } else {
        Reach::Inside
    }
}

/// Every path head one declared type mentions, at any depth.
///
/// A type this reader does not recognize contributes nothing, which is the
/// conservative direction: it can leave a road unjudged, and it can never
/// invent one.
fn mentions(declared: &syn::Type, into: &mut Vec<String>) {
    if let syn::Type::Path(typed) = declared {
        if let Some(qualified) = &typed.qself {
            mentions(&qualified.ty, into);
        }
        mentions_path(&typed.path, into);
    } else if let syn::Type::Reference(borrowed) = declared {
        mentions(&borrowed.elem, into);
    } else if let syn::Type::Ptr(pointer) = declared {
        mentions(&pointer.elem, into);
    } else if let syn::Type::Tuple(tuple) = declared {
        for element in &tuple.elems {
            mentions(element, into);
        }
    } else if let syn::Type::Slice(sliced) = declared {
        mentions(&sliced.elem, into);
    } else if let syn::Type::Array(array) = declared {
        mentions(&array.elem, into);
    } else if let syn::Type::Paren(parenthesized) = declared {
        mentions(&parenthesized.elem, into);
    } else if let syn::Type::Group(grouped) = declared {
        mentions(&grouped.elem, into);
    } else if let syn::Type::ImplTrait(opaque) = declared {
        mentions_bounds(&opaque.bounds, into);
    } else if let syn::Type::TraitObject(object) = declared {
        mentions_bounds(&object.bounds, into);
    }
}

/// Every path head one path mentions: each segment, and everything inside its
/// arguments.
fn mentions_path(path: &syn::Path, into: &mut Vec<String>) {
    for segment in &path.segments {
        into.push(segment.ident.to_string());
        mentions_segment_arguments(&segment.arguments, into);
    }
}

/// Every path head one path's ARGUMENTS mention, without the path's own
/// segments — which is where a conversion names the type it converts from.
fn mentions_arguments(path: &syn::Path, into: &mut Vec<String>) {
    for segment in &path.segments {
        mentions_segment_arguments(&segment.arguments, into);
    }
}

/// Every path head one segment's arguments mention.
fn mentions_segment_arguments(arguments: &syn::PathArguments, into: &mut Vec<String>) {
    if let syn::PathArguments::AngleBracketed(angled) = arguments {
        for argument in &angled.args {
            if let syn::GenericArgument::Type(inner) = argument {
                mentions(inner, into);
            } else if let syn::GenericArgument::AssocType(associated) = argument {
                mentions(&associated.ty, into);
            }
        }
    } else if let syn::PathArguments::Parenthesized(parenthesized) = arguments {
        for input in &parenthesized.inputs {
            mentions(&input.ty, into);
        }
        if let syn::ReturnType::Type(_, output) = &parenthesized.output {
            mentions(output, into);
        }
    }
}

/// Every path head one bound list mentions.
fn mentions_bounds(
    bounds: &syn::punctuated::Punctuated<syn::TypeParamBound, syn::Token![+]>,
    into: &mut Vec<String>,
) {
    for bound in bounds {
        if let syn::TypeParamBound::Trait(contract) = bound {
            mentions_path(&contract.path, into);
        }
    }
}

/// The last path segment of one type, or `None` where the type is not a plain
/// path.
fn head_of(declared: &syn::Type) -> Option<String> {
    if let syn::Type::Path(typed) = declared {
        last_segment(&typed.path)
    } else {
        None
    }
}

/// The last segment of one path, by name.
fn last_segment(path: &syn::Path) -> Option<String> {
    path.segments.last().map(|last| last.ident.to_string())
}

/// The type one stamp invocation names, read off its tokens: the identifier
/// that follows the `struct` word.
fn stamped_name(tokens: TokenStream) -> Option<String> {
    let mut trees = tokens.into_iter();
    while let Some(tree) = trees.next() {
        if let TokenTree::Ident(word) = tree
            && word == "struct"
        {
            if let Some(TokenTree::Ident(name)) = trees.next() {
                return Some(name.to_string());
            }
            return None;
        }
    }
    None
}

/// Every transcriber one `macro_rules!` body carries: the braced body that
/// follows each `=>`.
fn transcribers(tokens: TokenStream) -> Vec<TokenStream> {
    let mut found = Vec::new();
    let mut arrow = 0_u8;
    for tree in tokens {
        if let TokenTree::Punct(punct) = &tree {
            let character = punct.as_char();
            if character == '=' {
                arrow = 1;
                continue;
            }
            if character == '>' && arrow == 1 {
                arrow = 2;
                continue;
            }
            arrow = 0;
            continue;
        }
        if let TokenTree::Group(group) = &tree
            && arrow == 2
            && group.delimiter() == Delimiter::Brace
        {
            found.push(group.stream());
        }
        arrow = 0;
    }
    found
}

/// One transcriber with its metavariables given values, so `syn` can read it as
/// the Rust it becomes.
///
/// A repetition contributes its body once, which is exactly what an expansion
/// of one invocation does with a single-element repetition and is enough for
/// every question this law asks: what a repeated doc comment repeats is not a
/// road.
fn substituted(tokens: TokenStream) -> TokenStream {
    let mut out: Vec<TokenTree> = Vec::new();
    let mut trees = tokens.into_iter();
    while let Some(tree) = trees.next() {
        if !is_dollar(&tree) {
            if let TokenTree::Group(group) = &tree {
                out.push(TokenTree::Group(Group::new(
                    group.delimiter(),
                    substituted(group.stream()),
                )));
            } else {
                out.push(tree);
            }
            continue;
        }
        let Some(next) = trees.next() else {
            break;
        };
        if let TokenTree::Ident(name) = &next {
            out.push(TokenTree::Ident(Ident::new(
                &stand_in(&name.to_string()),
                Span::call_site(),
            )));
            continue;
        }
        if let TokenTree::Group(group) = &next
            && group.delimiter() == Delimiter::Parenthesis
        {
            out.extend(substituted(group.stream()));
            consume_repetition(&mut trees);
            continue;
        }
        out.push(next);
    }
    out.into_iter().collect()
}

/// Whether one token is the metavariable sigil.
fn is_dollar(tree: &TokenTree) -> bool {
    if let TokenTree::Punct(punct) = tree {
        punct.as_char() == '$'
    } else {
        false
    }
}

/// Consumes a repetition's optional separator and its operator, so neither
/// lands in the substituted text.
fn consume_repetition(trees: &mut impl Iterator<Item = TokenTree>) {
    for tree in trees {
        if let TokenTree::Punct(punct) = &tree {
            let character = punct.as_char();
            if character == '*' || character == '+' || character == '?' {
                return;
            }
        }
    }
}

/// The value one metavariable is given.
///
/// A visibility metavariable becomes `pub`, because the widest reach a caller
/// can ask the stamp for is the reach this law must judge. `$crate` becomes
/// `crate`, which is what it resolves to inside the crate that declares the
/// stamp. Everything else becomes an ordinary identifier, which is a lawful
/// type name, a lawful attribute path, and a lawful item name at once.
fn stand_in(name: &str) -> String {
    if name == "vis" {
        return String::from("pub");
    }
    if name == "crate" {
        return String::from("crate");
    }
    let mut characters = name.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect(),
        None => String::from("Metavariable"),
    }
}

/// Every parsed source the population is derived from: the machine's own
/// sources and the services', minus the two proof surfaces.
///
/// Taken from the one reading. A source the snapshot could not parse refuses
/// the whole law rather than leaving the population one file short.
fn seal_sources(
    snapshot: &RepositorySnapshot,
) -> Result<Vec<(&CanonicalPath, &syn::File)>, String> {
    Ok(snapshot
        .rust()
        .parsed_under(&[MACHINE_DIRECTORY, TOOLING_DIRECTORY])?
        .into_iter()
        .filter(|(path, _)| !PROOF_SURFACES.contains(&path.as_str()))
        .collect())
}

/// Planted reversals for the seal, and the real repository judged by it.
///
/// Every leg is pure over `(path, text)` pairs, so a reversal is a fixture held
/// in memory: the law that guards the machine's guards is never proven by
/// unsealing one. The test that reads the real tree is named `the_real_…` and
/// states what it found rather than what it hoped for.
#[cfg(test)]
mod tests {
    use super::{SealVerdict, seal_sources, seal_verdict as verdict_of_trees};
    use crate::repository::snapshot::repository_snapshot;
    use crate::repository::types::CanonicalPath;

    /// The verdict over fixture source TEXT.
    ///
    /// The law itself is handed trees the snapshot already parsed, so a source
    /// it could not read never reaches it. A fixture is text, so this adapter
    /// parses one and reports a fixture that does not parse exactly as the
    /// reading reports a source it could not read.
    fn seal_verdict(sources: &[(String, String)]) -> SealVerdict {
        let mut parsed = Vec::new();
        let mut unparsable = Vec::new();
        for (path, text) in sources {
            match syn::parse_file(text) {
                Ok(file) => parsed.push((CanonicalPath::spelled(path), file)),
                Err(error) => unparsable.push(format!(
                    "{path}: this file is not parseable Rust, so the population derived from it \
                     is unknown rather than empty: {error}"
                )),
            }
        }
        let trees: Vec<(&CanonicalPath, &syn::File)> =
            parsed.iter().map(|(path, file)| (path, file)).collect();
        let mut verdict = verdict_of_trees(&trees);
        verdict.offenders.splice(0..0, unparsable);
        verdict
    }

    /// The stamp as the machine writes it: one road in, one comparison that
    /// reads the seat from inside, and no road out.
    const LAWFUL_STAMP: &str = "\
macro_rules! scope_guard_version {
    (
        $(#[$note:meta])*
        $vis:vis struct $name:ident over $scope:ty;
    ) => {
        $(#[$note])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        $vis struct $name($crate::identity::AuthorityPosition<$scope>);

        impl $name {
            /// The one road in.
            #[must_use]
            $vis fn positioned(
                position: $crate::identity::AuthorityPosition<$scope>,
            ) -> Self {
                Self(position)
            }

            /// The one lawful comparison.
            $vis fn try_cmp_same_scope(
                &self,
                other: &Self,
            ) -> ::core::result::Result<
                ::core::cmp::Ordering,
                $crate::identity::OrderComparison,
            > {
                self.0.try_cmp_same_scope(&other.0)
            }
        }
    };
}
";

    /// One invocation of the stamp.
    const ONE_INVOCATION: &str = "\
crate::scope_guard_version! {
    /// One version of a reference frame.
    pub struct FrameVersion over ReferenceFrameId;
}
";

    /// The stamp, one invocation, and whatever else a reversal adds.
    fn tree(extra: &str) -> Vec<(String, String)> {
        vec![
            (
                String::from("src/02_identity/mod.rs"),
                String::from(LAWFUL_STAMP),
            ),
            (
                String::from("src/11_navigation/types.rs"),
                format!("{ONE_INVOCATION}{extra}"),
            ),
        ]
    }

    /// Whether some offence says the named thing.
    fn says(verdict: &SealVerdict, fragment: &str) -> bool {
        verdict
            .offenders
            .iter()
            .any(|offence| offence.contains(fragment))
    }

    /// The positive control: the stamp as written, one guard, no road out. A
    /// check that flagged everything would satisfy every reversal below and be
    /// worthless.
    #[test]
    fn a_stamped_guard_with_no_road_out_is_lawful() {
        let verdict = seal_verdict(&tree(""));
        assert_eq!(verdict.stamped, 1);
        assert_eq!(verdict.sealed, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal: the accessor the stamp's own contract forbids, added
    /// to the transcriber. It arrives on every guard the machine stamps at
    /// once, and the compile-refusal fixture beside it goes on refusing for its
    /// original reason.
    #[test]
    fn an_accessor_in_the_stamp_is_a_violation() {
        let unsealed = LAWFUL_STAMP.replace(
            "            /// The one lawful comparison.",
            "            /// The road out.\n\
             \x20           #[must_use]\n\
             \x20           $vis fn position(&self) -> $crate::identity::AuthorityPosition<$scope> {\n\
             \x20               self.0.clone()\n\
             \x20           }\n\
             \n\
             \x20           /// The one lawful comparison.",
        );
        let verdict = seal_verdict(&[
            (String::from("src/02_identity/mod.rs"), unsealed),
            (
                String::from("src/11_navigation/types.rs"),
                String::from(ONE_INVOCATION),
            ),
        ]);
        // One offence, at the stamp, and no guard counted sealed: the accessor
        // is on all of them, and reporting it twelve times would bury it.
        assert_eq!(verdict.stamped, 1);
        assert_eq!(verdict.sealed, 0, "{:?}", verdict.offenders);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(says(&verdict, "position"), "{:?}", verdict.offenders);
        assert!(says(&verdict, "transcriber"), "{:?}", verdict.offenders);
        assert!(
            says(&verdict, "hands the sealed position back out"),
            "{:?}",
            verdict.offenders
        );
    }

    /// Planted reversal: a hand-written accessor beside one guard. The
    /// expansion happened in this module, so its private seat is reachable from
    /// exactly here.
    #[test]
    fn a_hand_written_accessor_is_a_violation() {
        let verdict = seal_verdict(&tree(
            "impl FrameVersion {\n\
             \x20   #[must_use]\n\
             \x20   pub fn into_position(self) -> AuthorityPosition<ReferenceFrameId> {\n\
             \x20       self.0\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.stamped, 1);
        assert_eq!(verdict.sealed, 0);
        assert!(says(&verdict, "into_position"), "{:?}", verdict.offenders);
    }

    /// Planted reversal: the seat handed out through an associated TYPE. A
    /// reader that only looked at return signatures would report this tree
    /// sealed while `*version` yielded the position.
    #[test]
    fn a_deref_target_is_a_violation() {
        let verdict = seal_verdict(&tree(
            "impl core::ops::Deref for FrameVersion {\n\
             \x20   type Target = AuthorityPosition<ReferenceFrameId>;\n\
             \x20   fn deref(&self) -> &Self::Target {\n\
             \x20       &self.0\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.sealed, 0);
        assert!(says(&verdict, "Target"), "{:?}", verdict.offenders);
    }

    /// Planted reversal: a trait implementation's member carries no `pub`, and
    /// is public anyway. A reader that asked for the keyword would let every
    /// conversion trait through.
    #[test]
    fn a_trait_member_is_public_without_the_word() {
        let verdict = seal_verdict(&tree(
            "impl FrameVersion {\n\
             \x20   fn hidden(self) -> AuthorityPosition<ReferenceFrameId> {\n\
             \x20       self.0\n\
             \x20   }\n\
             }\n\
             \n\
             impl AsRef<AuthorityPosition<ReferenceFrameId>> for FrameVersion {\n\
             \x20   fn as_ref(&self) -> &AuthorityPosition<ReferenceFrameId> {\n\
             \x20       &self.0\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.sealed, 0);
        assert!(says(&verdict, "as_ref"), "{:?}", verdict.offenders);
        assert!(!says(&verdict, "hidden"), "{:?}", verdict.offenders);
    }

    /// Planted reversal: the road out written as a conversion, declaring no
    /// member on the guard at all.
    #[test]
    fn a_conversion_into_the_seat_is_a_violation() {
        let verdict = seal_verdict(&tree(
            "impl From<FrameVersion> for AuthorityPosition<ReferenceFrameId> {\n\
             \x20   fn from(version: FrameVersion) -> Self {\n\
             \x20       version.0\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.sealed, 0);
        assert!(
            says(&verdict, "written as a conversion"),
            "{:?}",
            verdict.offenders
        );
    }

    /// The reader's narrowness, stated as a test: a road returning something
    /// else is not a road out, a private road is not a public one, and a road
    /// on a type nobody stamped is nobody's laundering.
    #[test]
    fn the_reader_counts_roads_out_and_nothing_else() {
        let verdict = seal_verdict(&tree(
            "impl FrameVersion {\n\
             \x20   pub fn scope(&self) -> ReferenceFrameId {\n\
             \x20       self.0.scope()\n\
             \x20   }\n\
             \x20   fn seat(&self) -> &AuthorityPosition<ReferenceFrameId> {\n\
             \x20       &self.0\n\
             \x20   }\n\
             }\n\
             \n\
             impl SomethingElse {\n\
             \x20   pub fn position(&self) -> AuthorityPosition<ReferenceFrameId> {\n\
             \x20       self.0.clone()\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.stamped, 1);
        assert_eq!(verdict.sealed, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A guard stamped privately gets private operations, and a private road
    /// out is still a road out inside its own module. The stamp carries the
    /// caller's own visibility, so this law reads the widest reach the stamp
    /// can be asked for rather than the narrowest.
    #[test]
    fn the_transcriber_is_read_at_the_widest_visibility_it_can_carry() {
        let verdict = seal_verdict(&tree(""));
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
        assert_eq!(verdict.sealed, verdict.stamped);
    }

    /// A source this reader cannot parse is a hole in the population, and it is
    /// reported as one.
    #[test]
    fn an_unparsable_source_is_an_offence_rather_than_an_absence() {
        let verdict = seal_verdict(&[(
            String::from("src/11_navigation/types.rs"),
            String::from("impl FrameVersion for {\n"),
        )]);
        assert!(
            says(&verdict, "not parseable Rust"),
            "{:?}",
            verdict.offenders
        );
    }

    /// A transcriber this reader cannot parse is the same hole one level in,
    /// and it is reported rather than skipped: a stamp nobody could read is a
    /// stamp nobody is guarding.
    #[test]
    fn an_unreadable_transcriber_is_an_offence_rather_than_a_skip() {
        let verdict = seal_verdict(&[
            (
                String::from("src/02_identity/mod.rs"),
                String::from("macro_rules! scope_guard_version { () => { struct } ; }"),
            ),
            (
                String::from("src/11_navigation/types.rs"),
                String::from(ONE_INVOCATION),
            ),
        ]);
        assert_eq!(verdict.sealed, 0);
        assert!(
            says(&verdict, "does not parse as Rust"),
            "{:?}",
            verdict.offenders
        );
    }

    /// A tree with invocations and no stamp is a tree this law is not reading,
    /// and it says so rather than reporting every guard sealed.
    #[test]
    fn a_missing_stamp_is_an_offence_rather_than_a_pass() {
        let verdict = seal_verdict(&[(
            String::from("src/11_navigation/types.rs"),
            String::from(ONE_INVOCATION),
        )]);
        assert_eq!(verdict.stamped, 1);
        assert_eq!(verdict.sealed, 0);
        assert!(
            says(&verdict, "declaration was found"),
            "{:?}",
            verdict.offenders
        );
    }

    /// The seat is READ off the transcriber rather than written down here: a
    /// stamp reseated over a different inner type is judged over the type it
    /// actually seals.
    #[test]
    fn the_seat_is_read_off_the_stamp_rather_than_named_here() {
        let reseated = LAWFUL_STAMP.replace("AuthorityPosition", "SomeOtherSeat");
        let verdict = seal_verdict(&[
            (String::from("src/02_identity/mod.rs"), reseated),
            (
                String::from("src/11_navigation/types.rs"),
                format!(
                    "{ONE_INVOCATION}impl FrameVersion {{\n\
                     \x20   pub fn out(self) -> SomeOtherSeat<ReferenceFrameId> {{\n\
                     \x20       self.0\n\
                     \x20   }}\n\
                     }}\n"
                ),
            ),
        ]);
        assert_eq!(verdict.sealed, 0);
        assert!(
            says(&verdict, "`FrameVersion::out`"),
            "{:?}",
            verdict.offenders
        );
    }

    /// The real repository holds: every guard the stamp writes seals its
    /// position, and the derived population is real rather than empty.
    ///
    /// A gate that cannot READ its subject says it could not read its subject.
    #[test]
    fn the_real_tree_seals_every_stamped_guard() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let sources = seal_sources(snapshot)?;
        let verdict = verdict_of_trees(&sources);
        assert!(verdict.offenders.is_empty(), "{verdict:?}");
        assert!(
            verdict.stamped > 0,
            "no stamped scope guard found in the real tree: {verdict:?}"
        );
        assert_eq!(
            verdict.sealed, verdict.stamped,
            "the real tree stamps a guard whose position has a road out"
        );
        Ok(())
    }
}
