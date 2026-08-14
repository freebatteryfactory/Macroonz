//! The mint join: a refusal body whose every seat is private is handed back by
//! no road reachable from outside the crate that raises it.
//!
//! A private seat closes the LITERAL. It does not close the ROAD. A function
//! that takes an issue and hands back a refusal is the loading dock behind that
//! fence: any holder of an issue mints a body no pass established, and any
//! holder of a borrowed body clones the issues out and reseats them through the
//! same road. The record either produces is indistinguishable from one a seam
//! returned, which is the whole defect the private seat was supposed to end.
//!
//! The compile-time half of that claim is
//! `testpak/tests/compile-fail/a-services-refusal-minted-outside-its-plane.rs`,
//! which names each road and refuses it with `E0624`. This law is the half a
//! fixture cannot state: a fixture names the roads that EXIST, so a family added
//! tomorrow with a public mint leaves it compiling and passing, and the sentence
//! above it goes false with nothing failing. A universal claim needs a universal
//! fixture or a derived population, and an enumeration is neither. Here the
//! population is DERIVED — every closed record the subsystem declares that some
//! road refuses with, and every road that hands one back, read off the sources —
//! so a family added without a crate-internal mint is caught by the derivation
//! rather than by anybody remembering to extend a list.
//!
//! # The two facts, and why each is the one it is
//!
//! **A refusal is what a road refuses with.** The reader takes the error
//! position of every `Result` the subsystem returns. That is the subsystem's own
//! statement about which of its types are refusals, written where the refusing
//! happens, and it needs no marker anybody has to remember to apply.
//!
//! **A body is closed when no seat is public.** A record with a public seat can
//! be spelled as a literal from outside whatever its roads say, so the mint
//! question is not the question about it — the seat is, and
//! `macroc.a-refusal-body-seat-cannot-be-written-from-outside` is where that one
//! is asked. This law is about the records where the seat is already shut.
//!
//! # One question, and it is RESOLVED rather than pattern-matched
//!
//! Does this road hand a caller OWNERSHIP of a closed body?
//!
//! Three readers used to answer that question off the surface of the syntax
//! instead of resolving what the surface stood for, and each of them was the
//! same defect: a road with a receiver was dropped before its output was looked
//! at, an answer was unwrapped through two named wrappers and every other shape
//! recorded the wrapper's own name, and a road in a trait implementation was
//! called reachable because it was in a trait implementation. The first two hid
//! real mints — `Issue::into_refusal(self) -> Body`, `-> Box<Body>`, `-> Alias` —
//! and hiding is the dangerous direction, because the check stays green while
//! the obligation is false. The third refused a lawful road under a private
//! trait, which is loud, wrong, and stops a build.
//!
//! So the answer is now walked as a TYPE rather than matched as a spelling:
//!
//! * every position that hands ownership across is followed — a path's generic
//!   arguments, a tuple's elements, an array's element, a group's or a
//!   parenthesis's inside, an opaque or dynamic type's written bindings, a
//!   function pointer's own output, a raw pointer's target — so `Box<Body>`,
//!   `Vec<Body>`, `(Token, Body)` and `impl Iterator<Item = Body>` are each a
//!   road that hands a body over;
//! * a type ALIAS is resolved to what it stands for, transitively, so a name
//!   that is a spelling of the body is the body;
//! * a BORROW is not ownership. `&Body` and `&mut Body` hand out access to a
//!   body that already exists, which is what a reader is; nothing new is minted
//!   through one, and this is why the seven bodies' own `body()` readers are not
//!   roads;
//! * the ERROR position of a `Result` is deliberately not ownership of a new
//!   body. That is the caller receiving the refusal a seam raised, which is what
//!   the type is FOR, and it is the one exclusion in this walk that is a
//!   decision rather than a fact about the shape;
//! * a shape this reader has no reading for — a type a macro produces, a
//!   qualified path, a bound it cannot open, a nesting past its stated depth —
//!   is an OFFENCE naming the road. Whether such a road hands a body over is
//!   unknown, and unknown must not read as no.
//!
//! And a receiver no longer ends the question. A road is a copy rather than a
//! mint only where its RECEIVER IS THE BODY it hands back: that road is handed a
//! body and gives one back, so nothing exists after it that did not exist
//! before. A receiver of any other type is a producer — an issue, a draft, a
//! seam — and it mints.
//!
//! # The population is read from a parse, not from lines
//!
//! The reader is `syn`, for the reason the coupling law's is. What the parse
//! establishes that a scan cannot: that an item IS a record declaration, that a
//! FIELD is public rather than a `pub` somewhere on the line, that a function's
//! declared visibility is `pub` rather than `pub(crate)`, that a return type's
//! error position is the second type argument of a `Result` rather than the
//! second word after a comma, and that a receiver is a receiver. Those are
//! questions about items, members and paths, and answering them off text means
//! writing a Rust parser by hand inside a check.
//!
//! # How far a road reaches, and where that is read from
//!
//! From the declaration that states it, and from nothing else.
//!
//! An INHERENT road states its own visibility, so `pub` is the reading. That is
//! exact whenever the type is reachable — an inherent implementation is not
//! name-resolved through modules, so a `pub` road on a public type is reachable
//! however private the module that writes it — and strict otherwise.
//!
//! A road in a TRAIT implementation states no visibility of its own: it is
//! exactly as reachable as the trait it implements. So the trait is what this
//! reader resolves. A contract the subsystem declares is read at its own
//! declared visibility, which is why a `trait` or a `pub(crate) trait` closes
//! every road under it; a contract the subsystem does not declare is foreign —
//! `From`, `Default`, band 00's own contracts — and a downstream crate can name
//! it, so it is reachable.
//!
//! **The stated ceiling: a module chain is not consulted, and that is a
//! direction rather than an oversight.** A `pub` item inside a module nobody
//! re-exports is not in fact reachable, so reading `pub` as reachable can refuse
//! a road that no outside caller could ever spell. That is the loud direction,
//! and the repair is one word on the declaration. Consulting the chain could
//! only move verdicts the other way — toward "unreachable", toward passing —
//! and this subsystem publishes almost everything through `pub use`
//! re-exports out of private modules, so a chain-walking reader that missed one
//! re-export would call a genuinely reachable mint closed and say nothing at
//! all. Between a refusal somebody can argue with and a silence nobody can see,
//! this reader takes the refusal.
//!
//! # What this reader does not resolve
//!
//! Its subject is the metaprogramming subsystem and not the machine. Band 00's
//! own report package carries private seats and a PUBLIC mint on purpose — that
//! mint is the road the services reach for — so a law reading the machine on
//! these terms would refuse the thing it depends on. The machine's bodies are
//! guarded by their own laws and this one says nothing about them.
//!
//! A record is resolved by its declared NAME across the whole subsystem rather
//! than inside a home. That is the strict direction and it is deliberate: a road
//! anywhere in the subsystem that hands back a body is a road, wherever the body
//! was declared, and scoping the join to a home would leave exactly the
//! cross-home mint this law exists to refuse outside it. Two homes declaring one
//! name are read as one subject, so a road for either is judged against both —
//! the offence names both paths, and the repair is a rename rather than a looser
//! reading.
//!
//! It does not compile anything. A path is read by its LAST SEGMENT, so
//! `ProjectionPlanning` and `refusal::ProjectionPlanning` are one name here. It
//! does not evaluate `cfg`: a member written under one is read as declared. It
//! does not expand macros, so a record or a road assembled by one is outside
//! this law — and a return type a macro produces is refused rather than passed
//! over, because that one this reader can see.

use crate::repository::snapshot::{RepositorySnapshot, TOOLING_DIRECTORY};
use crate::repository::types::CanonicalPath;

/// The proof surface, excluded from the population by name.
///
/// The services' `laws.rs` declares demonstration families whose whole content
/// is a pair of constants — they exist so the admission algebra has something to
/// refuse, and counting them would put a fixture in a denominator about the
/// subsystem. Excluded for the reason and by the name the coupling law excludes
/// it.
const PROOF_SURFACE: &str = "macros/macroc/src/laws.rs";

/// The return a road refuses through, whose SECOND type argument is the refusal
/// rather than the answer.
const REFUSING_RETURN: &str = "Result";

/// The spelling a road inside an implementation names its own type by.
const OWN_TYPE: &str = "Self";

/// How deep this reader follows a return type through wrappers, aliases,
/// generic arguments and bindings.
///
/// Stated rather than unbounded, and a return type nested past it is an offence
/// rather than a shrug: the depth is what makes the ceiling a number somebody
/// can raise instead of a silence nobody can see.
const RESOLUTION_DEPTH: usize = 16;

/// Every refusal body the metaprogramming subsystem declares with every seat
/// private is handed back by no road reachable from outside the crate.
///
/// # Errors
///
/// Returns the offences one line at a time, and returns a read failure as
/// itself: a gate that cannot read its subject says so rather than reporting an
/// empty population.
pub(crate) fn check_refusal_mints_are_inside_the_plane(
    snapshot: &RepositorySnapshot,
) -> Result<(), String> {
    let sources = services_sources(snapshot)?;
    let verdict = mint_verdict(&sources);

    // The denominators are DERIVED and printed on every run, because a
    // population that quietly shrank would otherwise keep this check passing
    // while it guarded less.
    println!(
        "refusal mints: {} roads / {} closed refusal bodies",
        verdict.roads, verdict.bodies
    );
    if verdict.bodies == 0 {
        return Err(String::from(
            "no closed refusal body was found in the metaprogramming subsystem: this denominator \
             cannot be empty while the services refuse, so the reader is looking at the wrong tree",
        ));
    }
    if verdict.offenders.is_empty() {
        Ok(())
    } else {
        Err(verdict.offenders.join("; "))
    }
}

/// What the mint leg counted, and what it refuses.
#[derive(Debug)]
struct MintVerdict {
    /// Closed records some road refuses with: the refusal bodies.
    bodies: usize,
    /// Roads that hand one of those bodies over.
    roads: usize,
    /// Roads reachable from outside the crate, bodies no road produces, shapes
    /// the reader could not resolve, and sources it could not parse — one
    /// offence each.
    offenders: Vec<String>,
}

/// One record the subsystem declares whose every seat is private.
struct ClosedRecord {
    /// The repository-relative path that declares it.
    path: String,
    /// Its declared name.
    name: String,
}

/// One road that hands ownership of something across.
struct Road {
    /// The repository-relative path that declares it.
    path: String,
    /// Its declared name.
    name: String,
    /// Every name it hands a caller ownership of, by last path segment, with
    /// `Self` resolved to the type the enclosing implementation is for and
    /// aliases resolved to what they stand for.
    owned: Vec<String>,
    /// The type of its receiver, where it has one. A road whose receiver IS the
    /// body it hands back is a copy of a body that already existed; a road with
    /// a receiver of any other type, or none, mints.
    receiver: Option<String>,
    /// How far it reaches.
    reach: Reach,
}

/// How far one road reaches — a named pair rather than a flag, because "true"
/// at a call site says nothing about which direction it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reach {
    /// Out of the crate: the declaration that states this road's visibility
    /// says `pub`, or the contract it implements is one this subsystem does not
    /// declare and a downstream crate can name.
    OutsideTheCrate,
    /// No further than the crate: `pub(crate)`, `pub(in …)`, `pub(super)`, the
    /// absence of any spelling at all, or a contract declared here at any of
    /// those.
    InsideTheCrate,
}

/// Which side of a road's answer one position stands on.
#[derive(Debug, Clone, Copy)]
enum Slot {
    /// What the road hands back as its answer.
    Answer,
    /// What the road refuses with.
    Refusal,
}

/// Whether the arguments being read belong to the refusing return, whose SECOND
/// type argument stands on the other side from the first.
#[derive(Debug, Clone, Copy)]
enum Refusing {
    /// They do.
    Yes,
    /// They do not, so every argument stands where its parent stood.
    No,
}

/// Everything one pass over the parsed trees establishes before any road is
/// judged.
///
/// Read first and whole, because a road in one file is resolved against an alias
/// or a contract declared in another: a reader that answered as it walked would
/// answer differently depending on which file it reached first.
struct Declarations<'a> {
    /// Every record declared with no public seat.
    closed: Vec<ClosedRecord>,
    /// Every type alias the subsystem declares: the name, and the type it
    /// stands for.
    aliases: Vec<(String, &'a syn::Type)>,
    /// Every contract the subsystem declares, and how far its own declaration
    /// reaches.
    contracts: Vec<(String, Reach)>,
}

/// Everything one pass over the roads read.
struct Reading {
    /// Every type name a road in the subsystem refuses with.
    refused: Vec<String>,
    /// Every road that hands ownership across.
    roads: Vec<Road>,
    /// Return shapes this reader has no reading for, one offence each. Never a
    /// skip: whether such a road hands a body over is unknown rather than false.
    unresolvable: Vec<String>,
}

/// Reads the records, the refusals and the roads out of parsed trees and judges
/// each body.
///
/// Pure over its inputs — `(canonical path, parsed tree)` pairs handed over by
/// the snapshot — so the reversals below are planted in memory and the law that
/// guards the tree is never proven by opening a seat in one. A source that did
/// not parse never reaches here: the snapshot carries it as unread, and the
/// caller refuses the whole reading rather than deriving a population one file
/// short.
fn mint_verdict(parsed: &[(&CanonicalPath, &syn::File)]) -> MintVerdict {
    let mut offenders = Vec::new();
    let declarations = read_declarations(parsed);
    let Reading {
        refused,
        roads,
        unresolvable,
    } = read_roads(parsed, &declarations);
    offenders.extend(unresolvable);
    let mut verdict = MintVerdict {
        bodies: 0,
        roads: 0,
        offenders,
    };
    judge(&declarations, &refused, &roads, &mut verdict);
    verdict
}

/// Judges every closed record some road refuses with.
fn judge(
    declarations: &Declarations<'_>,
    refused_with: &[String],
    declared_roads: &[Road],
    verdict: &mut MintVerdict,
) {
    for record in &declarations.closed {
        let name = &record.name;
        if !refused_with.iter().any(|refused| refused == name) {
            continue;
        }
        verdict.bodies = verdict.bodies.saturating_add(1);
        let declared = &record.path;
        let roads: Vec<&Road> = declared_roads
            .iter()
            .filter(|road| road.mints(name))
            .collect();
        if roads.is_empty() {
            verdict.offenders.push(format!(
                "{declared}: {name} is refused with and every seat it declares is private, and no \
                 road in the subsystem hands one back; this law's numerator over it is empty, \
                 which guards nothing while reading as coverage"
            ));
            continue;
        }
        for road in roads {
            verdict.roads = verdict.roads.saturating_add(1);
            if road.reach == Reach::OutsideTheCrate {
                let at = &road.path;
                let spelled = &road.name;
                verdict.offenders.push(format!(
                    "{at}: {name}::{spelled} hands back {name}, whose every seat {declared} \
                     declares is private, and it is reachable from outside the crate; a private \
                     seat closes the literal and a public road is the loading dock behind it, \
                     because any holder of an issue mints a body no pass established"
                ));
            }
        }
    }
}

impl Road {
    /// Whether this road MINTS the named body rather than copying one.
    ///
    /// It hands the body's ownership across, and its receiver is not that same
    /// body. A road whose receiver IS the body is handed one and gives one back,
    /// so nothing exists after it that did not exist before; a road with a
    /// receiver of any other type is holding the parts rather than the whole,
    /// which is exactly the loading dock this law is about.
    fn mints(&self, body: &str) -> bool {
        self.owned.iter().any(|owned| owned == body) && self.receiver.as_deref() != Some(body)
    }
}

/// Every declaration the subsystem makes that a road is resolved against.
fn read_declarations<'a>(parsed: &[(&CanonicalPath, &'a syn::File)]) -> Declarations<'a> {
    let mut declarations = Declarations {
        closed: Vec::new(),
        aliases: Vec::new(),
        contracts: Vec::new(),
    };
    for (path, file) in parsed {
        read_declared_items(path.as_str(), &file.items, &mut declarations);
    }
    declarations
}

/// Reads one module's declarations, then every inline module inside it.
///
/// Written as an `if let` chain rather than a match because `syn::Item` is
/// `non_exhaustive`: the items this reading has a question about are named, and
/// every other item is passed over without a wildcard arm standing in for a set
/// no crate outside `syn` can enumerate.
fn read_declared_items<'a>(
    path: &str,
    items: &'a [syn::Item],
    declarations: &mut Declarations<'a>,
) {
    for item in items {
        if let syn::Item::Struct(declared) = item {
            if is_closed(declared) {
                declarations.closed.push(ClosedRecord {
                    path: path.to_string(),
                    name: declared.ident.to_string(),
                });
            }
        } else if let syn::Item::Type(declared) = item {
            declarations
                .aliases
                .push((declared.ident.to_string(), &declared.ty));
        } else if let syn::Item::Trait(declared) = item {
            declarations
                .contracts
                .push((declared.ident.to_string(), reach_of(&declared.vis)));
        } else if let syn::Item::Mod(module) = item
            && let Some((_, inner)) = &module.content
        {
            read_declared_items(path, inner, declarations);
        }
    }
}

/// Every road the subsystem declares, judged against what it declares.
fn read_roads<'a>(
    parsed: &[(&CanonicalPath, &'a syn::File)],
    declarations: &Declarations<'a>,
) -> Reading {
    let mut reading = Reading {
        refused: Vec::new(),
        roads: Vec::new(),
        unresolvable: Vec::new(),
    };
    for (path, file) in parsed {
        read_module(path.as_str(), &file.items, declarations, &mut reading);
    }
    reading
}

/// Reads one module's roads, then every inline module inside it.
fn read_module<'a>(
    path: &str,
    items: &'a [syn::Item],
    declarations: &Declarations<'a>,
    reading: &mut Reading,
) {
    for item in items {
        if let syn::Item::Fn(declared) = item {
            let road = Standing {
                reach: reach_of(&declared.vis),
                own: None,
            };
            read_signature(path, &declared.sig, &road, declarations, reading);
        } else if let syn::Item::Impl(declared) = item {
            read_implementation(path, declared, declarations, reading);
        } else if let syn::Item::Trait(declared) = item {
            read_contract(path, declared, declarations, reading);
        } else if let syn::Item::Mod(module) = item
            && let Some((_, inner)) = &module.content
        {
            read_module(path, inner, declarations, reading);
        }
    }
}

/// Where one road stands: how far it reaches, and what `Self` means inside it.
struct Standing {
    /// How far the declaration that states this road's visibility reaches.
    reach: Reach,
    /// The type the enclosing implementation is for, where there is one.
    own: Option<String>,
}

/// Reads every road one implementation declares.
///
/// A road in a trait implementation states no visibility of its own, so the
/// trait it implements is what decides how far it reaches — resolved against
/// what the subsystem declares rather than assumed from the fact that a trait is
/// there at all. That assumption refused a lawful road under a private trait,
/// which is why the resolution is here.
fn read_implementation<'a>(
    path: &str,
    declared: &'a syn::ItemImpl,
    declarations: &Declarations<'a>,
    reading: &mut Reading,
) {
    let own = head_of(&declared.self_ty);
    let contract = declared
        .trait_
        .as_ref()
        .and_then(|(named, _)| last_segment(named));
    for member in &declared.items {
        if let syn::ImplItem::Fn(road) = member {
            let reach = match contract.as_deref() {
                Some(named) => contract_reach(named, declarations),
                None => reach_of(&road.vis),
            };
            let standing = Standing {
                reach,
                own: own.clone(),
            };
            read_signature(path, &road.sig, &standing, declarations, reading);
        }
    }
}

/// Reads every road one contract declares.
///
/// A contract's road is as reachable as the contract, and the contract states
/// that itself.
fn read_contract<'a>(
    path: &str,
    declared: &'a syn::ItemTrait,
    declarations: &Declarations<'a>,
    reading: &mut Reading,
) {
    let standing = Standing {
        reach: reach_of(&declared.vis),
        own: None,
    };
    for member in &declared.items {
        if let syn::TraitItem::Fn(road) = member {
            read_signature(path, &road.sig, &standing, declarations, reading);
        }
    }
}

/// How far a road under one named contract reaches.
///
/// A contract this subsystem DECLARES answers at its own declared visibility, so
/// a `trait` or a `pub(crate) trait` closes every road under it. A contract it
/// does not declare is foreign — `From`, `Default`, band 00's own contracts —
/// and a downstream crate can name it, so a road under one is reachable.
fn contract_reach(named: &str, declarations: &Declarations<'_>) -> Reach {
    declarations
        .contracts
        .iter()
        .find(|(declared, _)| declared == named)
        .map_or(Reach::OutsideTheCrate, |&(_, reach)| reach)
}

/// How far one declared visibility reaches.
///
/// `pub` and nothing else. `pub(crate)`, `pub(in …)` and `pub(super)` arrive as
/// `syn::Visibility::Restricted` and reach no further than the crate, which is
/// exactly the distinction this law turns on.
fn reach_of(declared: &syn::Visibility) -> Reach {
    if matches!(*declared, syn::Visibility::Public(_)) {
        Reach::OutsideTheCrate
    } else {
        Reach::InsideTheCrate
    }
}

/// Reads both legs off one signature: what it refuses with, and what it hands a
/// caller ownership of.
fn read_signature<'a>(
    path: &str,
    sig: &'a syn::Signature,
    standing: &Standing,
    declarations: &Declarations<'a>,
    reading: &mut Reading,
) {
    let mut walk = Walk {
        aliases: &declarations.aliases,
        own: standing.own.clone(),
        owned: Vec::new(),
        refused: Vec::new(),
        unresolvable: Vec::new(),
    };
    walk.read_output(&sig.output, Slot::Answer, RESOLUTION_DEPTH);
    let name = sig.ident.to_string();
    reading.refused.extend(walk.refused);
    for unreadable in walk.unresolvable {
        reading.unresolvable.push(format!(
            "{path}: `{name}` returns {unreadable}, so whether it hands a caller a refusal body is \
             unknown rather than false; a shape this reader cannot resolve is refused rather than \
             passed over"
        ));
    }
    if !walk.owned.is_empty() {
        reading.roads.push(Road {
            path: path.to_string(),
            name,
            owned: walk.owned,
            receiver: receiver_of(sig, standing.own.as_deref()),
            reach: standing.reach,
        });
    }
}

/// The type of one signature's receiver, where it has one.
///
/// A receiver is the enclosing implementation's own type, which is what makes
/// "is the receiver the body it hands back" an answerable question rather than a
/// flag.
fn receiver_of(sig: &syn::Signature, own: Option<&str>) -> Option<String> {
    let takes_one = sig
        .inputs
        .iter()
        .any(|input| matches!(*input, syn::FnArg::Receiver(_)));
    if takes_one {
        own.map(str::to_string)
    } else {
        None
    }
}

/// One road's return type, walked.
///
/// Everything it hands ownership of, everything it refuses with, and every shape
/// it could not resolve — gathered in one pass over one type, so no two readings
/// of the same signature can disagree.
struct Walk<'d, 'a> {
    /// The aliases the subsystem declares, for resolving a name to what it
    /// stands for.
    aliases: &'d [(String, &'a syn::Type)],
    /// The type the enclosing implementation is for, which is what `Self` means
    /// inside it.
    own: Option<String>,
    /// Every name the road hands a caller ownership of.
    owned: Vec<String>,
    /// Every name the road refuses with.
    refused: Vec<String>,
    /// Every shape this reader has no reading for, described.
    unresolvable: Vec<String>,
}

impl<'a> Walk<'_, 'a> {
    /// Reads one return type, or nothing where the road returns nothing.
    fn read_output(&mut self, output: &'a syn::ReturnType, slot: Slot, depth: usize) {
        if let syn::ReturnType::Type(_, declared) = output {
            self.read(declared, slot, depth);
        }
    }

    /// Reads one type: every position that hands ownership across, followed.
    ///
    /// A borrow stops the walk — access to a body that already exists is what a
    /// reader is, and nothing new comes through one. Everything this reader has
    /// no reading for lands in `unresolvable`, which is the whole difference
    /// between a ceiling and a hole.
    fn read(&mut self, declared: &'a syn::Type, slot: Slot, depth: usize) {
        let Some(inside) = depth.checked_sub(1) else {
            self.unresolvable.push(format!(
                "a type nested deeper than the {RESOLUTION_DEPTH} levels this reader follows"
            ));
            return;
        };
        if let syn::Type::Path(typed) = declared {
            self.read_path(typed, slot, inside);
        } else if let syn::Type::Tuple(tuple) = declared {
            for element in &tuple.elems {
                self.read(element, slot, inside);
            }
        } else if let syn::Type::Array(array) = declared {
            self.read(&array.elem, slot, inside);
        } else if let syn::Type::Slice(slice) = declared {
            self.read(&slice.elem, slot, inside);
        } else if let syn::Type::Group(group) = declared {
            self.read(&group.elem, slot, inside);
        } else if let syn::Type::Paren(paren) = declared {
            self.read(&paren.elem, slot, inside);
        } else if let syn::Type::Ptr(pointer) = declared {
            self.read(&pointer.elem, slot, inside);
        } else if let syn::Type::ImplTrait(opaque) = declared {
            self.read_bounds(&opaque.bounds, slot, inside);
        } else if let syn::Type::TraitObject(dynamic) = declared {
            self.read_bounds(&dynamic.bounds, slot, inside);
        } else if let syn::Type::FnPtr(pointer) = declared {
            self.read_output(&pointer.output, slot, inside);
        } else if !matches!(
            *declared,
            syn::Type::Reference(_) | syn::Type::Never(_) | syn::Type::Infer(_)
        ) {
            self.unresolvable
                .push(String::from("a type shape this reader has no reading for"));
        }
    }

    /// Reads one path type: the name it stands for, and everything its
    /// arguments hand across.
    fn read_path(&mut self, typed: &'a syn::TypePath, slot: Slot, depth: usize) {
        if typed.qself.is_some() {
            self.unresolvable.push(String::from(
                "a qualified path, whose meaning is decided by an implementation this reader does \
                 not resolve",
            ));
            return;
        }
        let Some(head) = last_segment(&typed.path) else {
            self.unresolvable
                .push(String::from("a path with no segment at all"));
            return;
        };
        let refusing = if head == REFUSING_RETURN {
            Refusing::Yes
        } else {
            Refusing::No
        };
        // `Self` inside an implementation is the type that implementation is
        // for, and a name the subsystem declares an alias for is the type that
        // alias stands for. Neither is a different type from the one it spells.
        let named = if head == OWN_TYPE {
            self.own.clone().unwrap_or(head)
        } else {
            head
        };
        let stands_for = self
            .aliases
            .iter()
            .find(|(declared, _)| *declared == named)
            .map(|&(_, target)| target);
        if let Some(target) = stands_for {
            self.read(target, slot, depth);
            return;
        }
        match slot {
            Slot::Answer => self.owned.push(named),
            Slot::Refusal => self.refused.push(named),
        }
        if let Some(last) = typed.path.segments.last() {
            self.read_arguments(&last.arguments, slot, refusing, depth);
        }
    }

    /// Reads one path segment's arguments.
    ///
    /// The refusing return's SECOND type argument is the one position in this
    /// walk that changes sides, and it changes sides by decision: a seam handing
    /// a caller the refusal it raised is what the type is for.
    fn read_arguments(
        &mut self,
        arguments: &'a syn::PathArguments,
        slot: Slot,
        refusing: Refusing,
        depth: usize,
    ) {
        if let syn::PathArguments::AngleBracketed(bracketed) = arguments {
            self.read_angle_bracketed(&bracketed.args, slot, refusing, depth);
        } else if let syn::PathArguments::Parenthesized(spelled) = arguments {
            self.read_output(&spelled.output, slot, depth);
        }
    }

    /// Reads one angle-bracketed argument list, counting TYPE positions only.
    ///
    /// The position is what the refusing return's second slot is identified by,
    /// and a lifetime standing ahead of a type does not move it: `Result<'a, T,
    /// E>` is not a thing, but a reader that counted every argument would be one
    /// lifetime away from calling the answer a refusal.
    fn read_angle_bracketed(
        &mut self,
        arguments: &'a syn::punctuated::Punctuated<syn::GenericArgument, syn::token::Comma>,
        slot: Slot,
        refusing: Refusing,
        depth: usize,
    ) {
        let mut position = 0usize;
        for argument in arguments {
            self.read_argument(argument, slot, refusing, position, depth);
            if matches!(*argument, syn::GenericArgument::Type(_)) {
                position = position.saturating_add(1);
            }
        }
    }

    /// Reads one generic argument, standing where its position puts it.
    fn read_argument(
        &mut self,
        argument: &'a syn::GenericArgument,
        slot: Slot,
        refusing: Refusing,
        position: usize,
        depth: usize,
    ) {
        if let syn::GenericArgument::Type(inner) = argument {
            let stands = match (refusing, position) {
                (Refusing::Yes, 1) => Slot::Refusal,
                (Refusing::Yes | Refusing::No, _) => slot,
            };
            self.read(inner, stands, depth);
        } else if let syn::GenericArgument::AssocType(bound) = argument {
            self.read(&bound.ty, slot, depth);
        } else if let syn::GenericArgument::Constraint(constrained) = argument {
            self.read_bounds(&constrained.bounds, slot, depth);
        } else if !matches!(
            *argument,
            syn::GenericArgument::Lifetime(_)
                | syn::GenericArgument::Const(_)
                | syn::GenericArgument::AssocConst(_)
        ) {
            self.unresolvable.push(String::from(
                "a generic argument this reader has no reading for",
            ));
        }
    }

    /// Reads the bounds of an opaque or dynamic type: what a caller holding one
    /// can get out of it is whatever its written bindings say.
    fn read_bounds(
        &mut self,
        bounds: &'a syn::punctuated::Punctuated<syn::TypeParamBound, syn::token::Plus>,
        slot: Slot,
        depth: usize,
    ) {
        for bound in bounds {
            self.read_bound(bound, slot, depth);
        }
    }

    /// Reads one bound: a contract's written bindings are what a caller holding
    /// the opaque value can get out of it.
    fn read_bound(&mut self, bound: &'a syn::TypeParamBound, slot: Slot, depth: usize) {
        if let syn::TypeParamBound::Trait(contract) = bound {
            if let Some(last) = contract.path.segments.last() {
                self.read_arguments(&last.arguments, slot, Refusing::No, depth);
            }
        } else if !matches!(
            *bound,
            syn::TypeParamBound::Lifetime(_) | syn::TypeParamBound::PreciseCapture(_)
        ) {
            self.unresolvable.push(String::from(
                "a bound this reader cannot open, so what the opaque type yields is unknown",
            ));
        }
    }
}

/// One record declared with at least one seat and no public seat.
///
/// A record with no seat at all is not a body: there is nothing a literal could
/// write and nothing a mint could fill, so the question this law asks is not
/// about it.
fn is_closed(declared: &syn::ItemStruct) -> bool {
    if !matches!(declared.vis, syn::Visibility::Public(_)) {
        return false;
    }
    let mut seats = declared.fields.iter();
    let Some(first) = seats.next() else {
        return false;
    };
    [first]
        .into_iter()
        .chain(seats)
        .all(|seat| !matches!(seat.vis, syn::Visibility::Public(_)))
}

/// The last path segment of one type, or `None` where the type is not a plain
/// path.
///
/// Generic arguments are not part of the head, so `ProjectionClosureRefusal<R>`
/// and `ProjectionClosureRefusal` name one type here, exactly as they name one
/// body.
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

/// Every parsed source the population is derived from: the metaprogramming
/// subsystem's own sources, minus the proof surface.
///
/// Taken from the one reading. A subsystem with no sources at all is an empty
/// population and says so downstream, and a source the snapshot could not parse
/// refuses the whole law rather than leaving the population one file short.
fn services_sources(
    snapshot: &RepositorySnapshot,
) -> Result<Vec<(&CanonicalPath, &syn::File)>, String> {
    let sources: Vec<(&CanonicalPath, &syn::File)> = snapshot
        .rust()
        .parsed_under(&[TOOLING_DIRECTORY])?
        .into_iter()
        .filter(|(path, _)| path.as_str() != PROOF_SURFACE)
        .collect();
    if sources.is_empty() {
        return Err(format!(
            "{TOOLING_DIRECTORY}/ carries no Rust source: the subsystem this law is about cannot \
             be read, which is not the same as its having no refusal bodies"
        ));
    }
    Ok(sources)
}

/// Planted reversals for the join, and the real subsystem judged by it.
///
/// The leg is pure over `(path, text)` pairs, so a reversal is a fixture held in
/// memory: the law that guards the services' mints is never proven by opening
/// one. The test that reads the real tree is named `the_real_…` and states what
/// it found rather than what it hoped for.
#[cfg(test)]
mod tests {
    use super::{MintVerdict, mint_verdict as verdict_of_trees, services_sources};
    use crate::repository::snapshot::repository_snapshot;
    use crate::repository::types::CanonicalPath;

    /// The verdict over fixture source TEXT.
    ///
    /// The law itself is handed trees the snapshot already parsed, so a source
    /// it could not read never reaches it. A fixture is text, so this adapter
    /// parses one and reports a fixture that does not parse exactly as the
    /// reading reports a source it could not read.
    fn mint_verdict(sources: &[(String, String)]) -> MintVerdict {
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

    /// The seam that puts a name into the refused population, which is what
    /// makes a closed record a refusal body.
    const SEAM: &str = "impl DemoSeam {\n\
         \x20   pub fn checked(&self) -> Result<Self, DemoRefusal> {\n\
         \x20       Ok(Self)\n\
         \x20   }\n\
         }\n";

    /// The closed record every fixture below is about.
    const BODY: &str = "pub struct DemoRefusal {\n\
         \x20   body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
         }\n";

    /// One synthetic source file: the body, the seam that refuses with it, and
    /// whatever roads the case is about.
    fn source(roads: &str) -> Vec<(String, String)> {
        vec![(
            String::from("macros/macroc/src/home/types.rs"),
            format!("{BODY}\n{roads}\n{SEAM}"),
        )]
    }

    /// One synthetic source file at a named path.
    fn source_at(path: &str, text: &str) -> (String, String) {
        (path.to_string(), text.to_string())
    }

    /// The positive control: a closed body, refused with, whose one road is
    /// crate-internal. A check that flagged everything would satisfy every
    /// reversal below and be worthless.
    #[test]
    fn a_crate_internal_mint_is_lawful() {
        let verdict = source(
            "impl DemoRefusal {\n\
             \x20   pub(crate) fn established(issue: DemoIssue) -> Self {\n\
             \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
             \x20   }\n\
             }\n",
        );
        let verdict = mint_verdict(&verdict);
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal: the fence with a loading dock behind it. The seat is
    /// private, the record cannot be written as a literal from outside, and one
    /// `pub` road hands the whole body back to any holder of an issue.
    #[test]
    fn a_public_mint_on_a_closed_body_is_a_violation() {
        let verdict = mint_verdict(&source(
            "impl DemoRefusal {\n\
             \x20   pub fn established(issue: DemoIssue) -> Self {\n\
             \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("DemoRefusal::established"))
        );
    }

    /// Planted reversal for the receiver blind spot: a road on ANOTHER type that
    /// consumes an issue and hands back the body. A reader that dropped every
    /// road with a receiver before looking at its output let this through in
    /// silence while the crate-internal constructors kept the numerator full.
    #[test]
    fn a_receiver_of_another_type_still_mints() {
        let verdict = mint_verdict(&source(
            "impl DemoIssue {\n\
             \x20   pub fn into_refusal(self) -> DemoRefusal {\n\
             \x20       DemoRefusal { body: AdmittedPrefix::carrying_one(self) }\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("DemoRefusal::into_refusal"))
        );
    }

    /// A road whose receiver IS the body is a copy of one that already existed,
    /// and it is not a mint. Reading it as one would refuse a hand-written
    /// `Clone` and admit the derived one, which is a verdict decided by how a
    /// road was spelled.
    #[test]
    fn a_receiver_of_the_body_is_a_copy_road() {
        let verdict = mint_verdict(&source(
            "impl DemoRefusal {\n\
             \x20   pub fn duplicated(&self) -> Self {\n\
             \x20       Self { body: self.body.clone() }\n\
             \x20   }\n\
             \x20   pub(crate) fn established(issue: DemoIssue) -> Self {\n\
             \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal for the wrapper blind spot: the body handed back inside
    /// a `Box`. A reader unwrapping two named wrappers recorded `Box` and
    /// counted no mint at all — the same evasion this repository has already
    /// repaired once, in another law, through the same wrapper.
    #[test]
    fn a_boxed_answer_still_mints() {
        let verdict = mint_verdict(&source(
            "pub fn boxed(issue: DemoIssue) -> Box<DemoRefusal> {\n\
             \x20   Box::new(DemoRefusal { body: AdmittedPrefix::carrying_one(issue) })\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("DemoRefusal::boxed"))
        );
    }

    /// Planted reversal for the alias blind spot: the body handed back under a
    /// name that stands for it. A name is not a different type.
    #[test]
    fn an_alias_for_the_body_still_mints() {
        let verdict = mint_verdict(&source(
            "pub type DemoOutcome = DemoRefusal;\n\
             \n\
             pub fn aliased(issue: DemoIssue) -> DemoOutcome {\n\
             \x20   DemoRefusal { body: AdmittedPrefix::carrying_one(issue) }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("DemoRefusal::aliased"))
        );
    }

    /// The same evasion through every other ownership position: a tuple, a
    /// collection, and an opaque iterator each hand a body across.
    #[test]
    fn every_ownership_position_still_mints() {
        for spelled in [
            "pub fn paired(issue: DemoIssue) -> (DemoToken, DemoRefusal) { todo() }\n",
            "pub fn collected(issue: DemoIssue) -> Vec<DemoRefusal> { todo() }\n",
            "pub fn streamed(issue: DemoIssue) -> impl Iterator<Item = DemoRefusal> { todo() }\n",
            "pub fn optional(issue: DemoIssue) -> Option<DemoRefusal> { todo() }\n",
        ] {
            let verdict = mint_verdict(&source(spelled));
            assert_eq!(verdict.bodies, 1, "{spelled}");
            assert_eq!(verdict.roads, 1, "{spelled}");
            assert_eq!(
                verdict.offenders.len(),
                1,
                "{spelled}: {:?}",
                verdict.offenders
            );
        }
    }

    /// A borrow is not ownership. The seven bodies' own readers hand back
    /// `&AdmittedPrefix<…>` and an iterator of borrows, and a reader that
    /// counted those would refuse every public reader in the subsystem.
    #[test]
    fn a_borrowed_answer_is_not_a_mint() {
        let verdict = mint_verdict(&source(
            "impl DemoRefusal {\n\
             \x20   pub const fn body(&self) -> &AdmittedPrefix<DemoIssue, DemoIssueLimit> {\n\
             \x20       &self.body\n\
             \x20   }\n\
             }\n\
             \n\
             pub fn borrowed(held: &DemoRefusal) -> &DemoRefusal {\n\
             \x20   held\n\
             }\n\
             \n\
             impl DemoRefusal {\n\
             \x20   pub(crate) fn established(issue: DemoIssue) -> Self {\n\
             \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A public seam that REFUSES with the body is not a mint. This is the
    /// distinction the whole leg turns on: a caller receiving the refusal a seam
    /// raised is what the type exists for, and a reader that could not tell the
    /// error position from the success one would refuse every refusing road in
    /// the subsystem.
    #[test]
    fn a_public_refusing_seam_is_not_a_mint() {
        let verdict = mint_verdict(&source(
            "impl DemoRefusal {\n\
             \x20   pub(crate) fn established(issue: DemoIssue) -> Self {\n\
             \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.roads, 1);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// Planted reversal for the trait-implementation flag, in the direction it
    /// failed: a PRIVATE contract, implemented for a producer, whose road hands
    /// a closed body back. No caller outside the crate can name the contract, so
    /// no caller outside the crate can reach the road — and a reader that called
    /// every trait road reachable refused this one, which is a build stopped
    /// over a road that does not exist from outside.
    #[test]
    fn a_road_under_a_private_contract_is_not_reachable() {
        let verdict = mint_verdict(&source(
            "trait DemoRaises {\n\
             \x20   fn raised(&self) -> DemoRefusal;\n\
             }\n\
             \n\
             impl DemoRaises for DemoIssue {\n\
             \x20   fn raised(&self) -> DemoRefusal {\n\
             \x20       DemoRefusal { body: AdmittedPrefix::carrying_one(*self) }\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 2);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// The other direction of the same resolution: a PUBLIC contract is one a
    /// downstream crate can name, so the road under it is reachable and the
    /// mint is refused. A reader that answered "not reachable" for every trait
    /// road would have closed the false refusal by opening a silence.
    #[test]
    fn a_road_under_a_public_contract_is_reachable() {
        let verdict = mint_verdict(&source(
            "pub trait DemoRaises {\n\
             \x20   fn raised(&self) -> DemoRefusal;\n\
             }\n\
             \n\
             impl DemoRaises for DemoIssue {\n\
             \x20   fn raised(&self) -> DemoRefusal {\n\
             \x20       DemoRefusal { body: AdmittedPrefix::carrying_one(*self) }\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 2);
        assert_eq!(verdict.offenders.len(), 2, "{:?}", verdict.offenders);
    }

    /// A contract this subsystem does not declare is one a downstream crate
    /// already has: `impl From<DemoIssue> for DemoRefusal` is a public mint
    /// spelled as a conversion, and it is refused as one.
    #[test]
    fn a_road_under_a_foreign_contract_is_reachable() {
        let verdict = mint_verdict(&source(
            "impl From<DemoIssue> for DemoRefusal {\n\
             \x20   fn from(issue: DemoIssue) -> Self {\n\
             \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
             \x20   }\n\
             }\n",
        ));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 1);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("DemoRefusal::from"))
        );
    }

    /// A body added later with a public mint is refused by the same derivation,
    /// with nothing about it written down anywhere: the population is derived
    /// rather than named, which is the whole of what this law adds to the
    /// fixture beside it.
    #[test]
    fn a_family_added_later_is_already_in_the_population() {
        let lawful = format!(
            "{BODY}\nimpl DemoRefusal {{\n\
             \x20   pub(crate) fn established(issue: DemoIssue) -> Self {{\n\
             \x20       Self {{ body: AdmittedPrefix::carrying_one(issue) }}\n\
             \x20   }}\n\
             }}\n\n{SEAM}"
        );
        let verdict = mint_verdict(&[
            source_at("macros/macroc/src/home/types.rs", &lawful),
            source_at(
                "macros/macroc/src/later/types.rs",
                "pub struct LaterRefusal {\n\
                 \x20   body: AdmittedPrefix<LaterIssue, LaterIssueLimit>,\n\
                 }\n\
                 \n\
                 impl LaterRefusal {\n\
                 \x20   pub fn established(issue: LaterIssue) -> Self {\n\
                 \x20       Self { body: AdmittedPrefix::carrying_one(issue) }\n\
                 \x20   }\n\
                 }\n\
                 \n\
                 impl LaterSeam {\n\
                 \x20   pub fn checked(&self) -> Result<Self, LaterRefusal> {\n\
                 \x20       Ok(Self)\n\
                 \x20   }\n\
                 }\n",
            ),
        ]);
        assert_eq!(verdict.bodies, 2);
        assert_eq!(verdict.roads, 2);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("LaterRefusal::established"))
        );
    }

    /// A record with a public seat is not this law's subject, whatever its roads
    /// say. The literal is already writable from outside, so the question about
    /// it is the SEAT's, and answering it here would report the wrong defect.
    #[test]
    fn a_record_with_a_public_seat_is_not_in_the_population() {
        let verdict = mint_verdict(&[source_at(
            "macros/macroc/src/home/types.rs",
            &format!(
                "pub struct DemoRefusal {{\n\
                 \x20   pub body: AdmittedPrefix<DemoIssue, DemoIssueLimit>,\n\
                 }}\n\n\
                 impl DemoRefusal {{\n\
                 \x20   pub fn established(issue: DemoIssue) -> Self {{\n\
                 \x20       Self {{ body: AdmittedPrefix::carrying_one(issue) }}\n\
                 \x20   }}\n\
                 }}\n\n{SEAM}"
            ),
        )]);
        assert_eq!(verdict.bodies, 0);
        assert_eq!(verdict.roads, 0);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A closed record nobody refuses with is an ordinary guarded type, and a
    /// public constructor on one is ordinary. A reader that took every closed
    /// record for a refusal body would refuse half the subsystem.
    #[test]
    fn a_closed_record_nobody_refuses_with_is_not_a_body() {
        let verdict = mint_verdict(&[source_at(
            "macros/macroc/src/home/types.rs",
            "pub struct SpanTable {\n\
             \x20   positions: Vec<u32>,\n\
             }\n\
             \n\
             impl SpanTable {\n\
             \x20   pub fn issued(positions: Vec<u32>) -> Self {\n\
             \x20       Self { positions }\n\
             \x20   }\n\
             }\n",
        )]);
        assert_eq!(verdict.bodies, 0);
        assert!(verdict.offenders.is_empty(), "{:?}", verdict.offenders);
    }

    /// A refusal body no road produces is an offence rather than a quiet pass.
    /// The law's numerator over it is empty, so it guards nothing while the
    /// printed denominator counts it as covered — which is the one failure a
    /// derived population exists to prevent.
    #[test]
    fn a_body_no_road_produces_is_a_violation() {
        let verdict = mint_verdict(&source(""));
        assert_eq!(verdict.bodies, 1);
        assert_eq!(verdict.roads, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("no road in the subsystem hands one back"))
        );
    }

    /// Planted reversal for the reader's own ceiling: a return type a macro
    /// produces. Whether it hands a body across is UNKNOWN, and unknown is
    /// reported rather than read as no — which is the difference between a
    /// stated ceiling and a hole.
    #[test]
    fn an_unresolvable_return_shape_is_an_offence_rather_than_a_silence() {
        let verdict = mint_verdict(&source("pub fn produced() -> answer!() { todo() }\n"));
        assert_eq!(
            verdict.offenders.len(),
            2,
            "the macro road and the body no road produces: {:?}",
            verdict.offenders
        );
        assert!(
            verdict
                .offenders
                .iter()
                .any(|offence| offence.contains("unknown rather than false"))
        );
    }

    /// A source this reader cannot parse is a hole in the population, and it is
    /// reported as one. Silently reading it as "no bodies here" is the exact
    /// failure the derived denominator exists to prevent.
    #[test]
    fn an_unparsable_source_is_an_offence_rather_than_an_absence() {
        let verdict = mint_verdict(&[source_at(
            "macros/macroc/src/home/types.rs",
            "pub struct DemoRefusal {\n",
        )]);
        assert_eq!(verdict.bodies, 0);
        assert_eq!(verdict.offenders.len(), 1, "{:?}", verdict.offenders);
        assert!(
            verdict
                .offenders
                .first()
                .is_some_and(|offence| offence.contains("not parseable Rust"))
        );
    }

    /// The real subsystem holds: every closed refusal body it declares is minted
    /// only from inside the crate, and the derived population is real rather
    /// than empty.
    ///
    /// The counts are asserted as RELATIONS and never as a number. A test
    /// naming seven would be the hand-maintained inventory this law was written
    /// to retire, moved one file over; the run prints the numbers, and the
    /// relation is what has to hold.
    #[test]
    fn the_real_subsystem_mints_every_body_from_inside() -> Result<(), String> {
        let snapshot = repository_snapshot()?;
        let sources = services_sources(snapshot)?;
        let verdict = verdict_of_trees(&sources);
        assert!(verdict.offenders.is_empty(), "{verdict:?}");
        assert!(
            verdict.bodies > 0,
            "no closed refusal body found in the real subsystem: {verdict:?}"
        );
        assert!(
            verdict.roads >= verdict.bodies,
            "a closed refusal body in the real subsystem is produced by no road at all"
        );
        Ok(())
    }
}
