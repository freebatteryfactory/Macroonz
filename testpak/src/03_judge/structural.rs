//! Lane B — the structural read: what the artifact DECLARES, recovered from an
//! independent parse of the rendered text.
//!
//! # The question lane A cannot be asked
//!
//! Lane A's claim is exactly *the rendered text contains this exact declared
//! textual form*. Every structural question is outside it, and no number of
//! anchors moves it inside: whether the artifact declares an implementation at
//! all, what that implementation targets, which contract it realizes, whether
//! the anchored constant is a MEMBER of it or merely bytes sitting nearby,
//! whether an item nobody planned came along, and whether the same item was
//! emitted twice. A byte scan that answered any of those would have to decide
//! what the text MEANS, and deciding that means implementing the reader's own
//! understanding of Rust — at which point the scan stops being dumb, which was
//! the property that made it worth trusting.
//!
//! # So the parse is somebody else's
//!
//! This lane answers those questions by handing the text to `syn` and reading
//! the tree it hands back. `syn` shares nothing with the producer: not the
//! capture, not the plan, not the renderer, not the token type the renderer
//! writes, not the projection that turns that token tree into text. It is a
//! parser written by people who never saw this machine, and it decides where an
//! item begins, what a path is, and which constants are members of which
//! implementation, by Rust's rules rather than by ours.
//!
//! That is what makes this lane's agreement worth something. Two readers written
//! by the same hands against the same document agree because they share the
//! challenged understanding; agreement with a parser nobody here wrote is not
//! correlated with the renderer at all.
//!
//! # Everything inside an implementation is read, and nothing is stepped over
//!
//! A reader that recovers three named constants and ignores whatever else the
//! implementation carries has a blind spot exactly the size of "everything the
//! declaration did not name". A method, an associated type, a macro invocation,
//! a second copy of a constant it already read, a `cfg` deciding whether the
//! implementation exists at all — every one of those changes what the artifact
//! declares, and every one of them used to pass this lane silently. So each is a
//! finding of its own now:
//!
//! - a member that is not one of the expected constants is
//!   [`StructuralDisagreement::UnexpectedImplMember`];
//! - a constant read twice is [`StructuralDisagreement::DuplicateMember`], never
//!   an overwrite of the earlier reading;
//! - the exact path a cause row is CONSTRUCTED through is read and compared, so
//!   rows carrying the declared values through some other constructor are
//!   [`StructuralDisagreement::ConstructorPath`];
//! - how the implementation is written — `unsafe`, negative, `default`, generic
//!   — is [`StructuralDisagreement::ImplPosture`];
//! - any attribute that is not a doc comment is
//!   [`StructuralDisagreement::MeaningBearingAttribute`], because a doc comment
//!   decides nothing and a `cfg` decides everything.
//!
//! # What this lane does NOT claim
//!
//! It reads syntax, and syntax is not meaning. This lane never claims that the
//! artifact TYPECHECKS, that the paths it spells resolve to anything, that the
//! trait it names exists, that the target type exists, that the implementation
//! is coherent, or that any constant evaluates to the value its spelling
//! suggests. `::threadpak::refusal::SomethingElse` is read here as *a different
//! path than the one declared* and never as *no such contract*. Every one of
//! those is lane C's, where `rustc` compiles the artifact and the test reads the
//! constants back AS VALUES.
//!
//! [`StructuralVerdict::Unparsable`] carries the same discipline as lane A's
//! `Unreadable`: it is a failure class, never a skip, and never a softer
//! [`StructuralVerdict::Deviates`]. A caller that folded it into `Conforms`
//! would be asserting over a reading that never happened.

/// One cause row, as the artifact declares it: the two constructor paths it is
/// built through, the stable identity minted for the cause, and the spelling
/// that cause is projected under.
///
/// The constructors are columns of the row and not decoration. A row spelling
/// the declared identity and the declared spelling through some other pair of
/// constructors declares something else entirely, and a reader that kept only
/// the two strings would have called it conforming.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CauseRow {
    /// The path the row itself is constructed through.
    pub row_constructor: String,
    /// The path the row's stable identity is minted through.
    pub identity_constructor: String,
    /// The stable identity the row states.
    pub identity: String,
    /// The spelling the row states.
    pub spelling: String,
}

/// One way an implementation may be WRITTEN beyond the plain form.
///
/// An implementation carries no visibility in Rust — there is no seat for one on
/// the item — so the postures a reader can be lied to about are these four, and
/// each of them changes what the artifact declares. A lawful rendering carries
/// none of them, which is why the declaration states an empty roster and any
/// posture at all is a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImplPosture {
    /// `unsafe impl` — a contract with an obligation attached.
    Unsafely,
    /// `impl !Trait for Type` — the opposite of the contract the declaration
    /// named.
    Negative,
    /// `default impl` — a realization other implementations may replace.
    Defaulted,
    /// Generic parameters or a `where` clause: a family of implementations
    /// rather than the one the declaration named.
    Generic,
}

/// One trait implementation the artifact declares, as this lane read it.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplementationStructure {
    /// The type the implementation targets, spelled as its path.
    pub target: String,
    /// The trait path the implementation realizes, spelled with its leading
    /// `::` when it carries one.
    pub trait_path: String,
    /// The postures the implementation is written under, in roster order.
    pub postures: Vec<ImplPosture>,
    /// The attributes the implementation and its members carry that decide
    /// something — every attribute that is not a doc comment, by path.
    pub meaning_bearing_attributes: Vec<String>,
    /// The body-shape word the member constant `SHAPE` states, where this
    /// implementation states one.
    pub shape: Option<String>,
    /// The spellings the member constant `SELECTION_ORDER` states, in order,
    /// where this implementation states it.
    pub selection_order: Option<Vec<String>>,
    /// The path the member constant `DECLARED_ORDER` is constructed through,
    /// where this implementation states it.
    pub order_constructor: Option<String>,
    /// The cause rows the member constant `DECLARED_ORDER` states, in order,
    /// where this implementation states it.
    pub cause_rows: Option<Vec<CauseRow>>,
    /// The members that are not one of the three expected constants, described
    /// by what each one is.
    pub unexpected_members: Vec<String>,
    /// The expected constants this implementation states more than once, by
    /// name. The second reading is recorded here and never written over the
    /// first.
    pub duplicated_members: Vec<String>,
}

/// Everything this lane recovered from one rendered artifact.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArtifactStructure {
    /// The trait implementations the artifact declares, in the order it
    /// declares them.
    pub implementations: Vec<ImplementationStructure>,
    /// How many items the artifact declares that are not trait implementations
    /// of a named type. Nothing lawful renders one, so any count above zero is
    /// a finding rather than a detail.
    pub other_items: usize,
}

/// What the caller states the artifact should declare, written independently of
/// the thing under judgement.
///
/// Every roster here is authored by the caller beside the declaration it handed
/// to the producer. Nothing in this structure is obtained by asking the producer
/// what it did.
#[derive(Debug, Clone, Copy)]
pub struct DeclaredStructure<'a> {
    /// The one type every declared implementation targets.
    pub target: &'a str,
    /// The trait paths the artifact declares, in the order it declares them.
    pub traits: &'a [&'a str],
    /// The postures every declared implementation is written under.
    pub postures: &'a [ImplPosture],
    /// The attributes the declaration admits on an implementation or on one of
    /// its members, by path. Doc comments are not attributes for this purpose
    /// and never appear here.
    pub attributes: &'a [&'a str],
    /// The body-shape word exactly one implementation states.
    pub shape: &'a str,
    /// The cause spellings, in declared order. Both the selection order and the
    /// cause rows are held to this one roster, because both project the same
    /// declared causes.
    pub spellings: &'a [&'a str],
    /// The stable cause identities, in declared order.
    pub identities: &'a [&'a str],
    /// The path the declared order is constructed through.
    pub order_constructor: &'a str,
    /// The path every cause row is constructed through.
    pub row_constructor: &'a str,
    /// The path every row's stable identity is minted through.
    pub identity_constructor: &'a str,
}

/// Which structural fact the artifact and the declaration disagree about.
///
/// One finding, named. A verdict that only said "no" would leave every caller
/// guessing which of a dozen questions came back wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralDisagreement {
    /// The artifact declares an item that is not a trait implementation of a
    /// named type.
    UnexpectedItem,
    /// One trait-and-target pair is implemented more than once.
    DuplicateImplementation,
    /// The artifact declares a different number of implementations than the
    /// declaration names.
    OutputCardinality,
    /// An implementation targets a type the declaration did not name.
    ImplementationTarget,
    /// An implementation realizes a trait path the declaration did not name, or
    /// names them in another order.
    TraitPath,
    /// An implementation is written `unsafe`, negative, `default`, or generic
    /// where the declaration names none of those.
    ImplPosture,
    /// An implementation or one of its members carries an attribute that
    /// decides something and that the declaration did not name.
    MeaningBearingAttribute,
    /// An implementation carries a member that is not one of the expected
    /// associated constants.
    UnexpectedImplMember,
    /// An implementation states one of the expected constants more than once.
    DuplicateMember,
    /// The stated body-shape word is not the declared one, or is stated by no
    /// implementation or by more than one.
    FamilyShape,
    /// The stated selection order is not the declared roster, in order.
    SelectionOrder,
    /// A declared value is carried through a constructor path the declaration
    /// did not name.
    ConstructorPath,
    /// The stated cause rows are not the declared identities and spellings, in
    /// order.
    CauseRows,
}

/// What one structural reading concluded.
///
/// Three answers, and none of them is silence — see the module documentation for
/// why `Unparsable` is its own failure class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructuralVerdict {
    /// The artifact declares exactly what the caller declared it would.
    Conforms,
    /// The artifact and the declaration disagree, about this.
    Deviates(StructuralDisagreement),
    /// The text is not parseable Rust, so nothing structural was read at all.
    Unparsable,
}

/// Read one rendered artifact's structure, or `None` where the text is not
/// parseable Rust.
///
/// `None` is a real answer and never "nothing was wrong": a caller that treated
/// it as conformance would be reporting a verdict about a tree that was never
/// built.
#[must_use]
pub fn structure_of(rendered: &str) -> Option<ArtifactStructure> {
    let file = syn::parse_file(rendered).ok()?;
    let mut implementations = Vec::new();
    let mut other_items = 0usize;
    for item in &file.items {
        #[expect(
            clippy::wildcard_enum_match_arm,
            reason = "`syn::Item` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every item it catches is one this reading counts rather than reads"
        )]
        match item {
            syn::Item::Impl(declared) => match implementation_structure(declared) {
                Some(structure) => implementations.push(structure),
                None => other_items = other_items.saturating_add(1),
            },
            _ => other_items = other_items.saturating_add(1),
        }
    }
    Some(ArtifactStructure {
        implementations,
        other_items,
    })
}

/// Judge one rendered artifact's structure against an independently declared
/// one.
///
/// **The claim this function supports** is lane B's and only lane B's: the
/// artifact DECLARES these implementations, of these traits, for this target,
/// written this way, carrying these members and no others. It says nothing about
/// whether any of it compiles.
#[must_use]
pub fn judge_structure(rendered: &str, declared: &DeclaredStructure<'_>) -> StructuralVerdict {
    let Some(structure) = structure_of(rendered) else {
        return StructuralVerdict::Unparsable;
    };
    match disagreement(&structure, declared) {
        Some(found) => StructuralVerdict::Deviates(found),
        None => StructuralVerdict::Conforms,
    }
}

/// The first structural disagreement between one reading and one declaration.
///
/// The order is deliberate and coarse-to-fine: an artifact carrying an item
/// nobody planned is reported as that, not as whichever member constant the
/// extra item happened to disturb. Inside an implementation the same principle
/// holds — how the implementation is written, and whether it exists at all under
/// some `cfg`, are read before what its members say, because a member's value is
/// only interesting once the item carrying it is the declared item.
fn disagreement(
    structure: &ArtifactStructure,
    declared: &DeclaredStructure<'_>,
) -> Option<StructuralDisagreement> {
    if structure.other_items > 0 {
        return Some(StructuralDisagreement::UnexpectedItem);
    }
    if duplicated(structure) {
        return Some(StructuralDisagreement::DuplicateImplementation);
    }
    if structure.implementations.len() != declared.traits.len() {
        return Some(StructuralDisagreement::OutputCardinality);
    }
    if structure
        .implementations
        .iter()
        .any(|found| found.target != declared.target)
    {
        return Some(StructuralDisagreement::ImplementationTarget);
    }
    if structure
        .implementations
        .iter()
        .zip(declared.traits.iter())
        .any(|(found, expected)| found.trait_path != *expected)
    {
        return Some(StructuralDisagreement::TraitPath);
    }
    if structure
        .implementations
        .iter()
        .any(|found| found.postures.as_slice() != declared.postures)
    {
        return Some(StructuralDisagreement::ImplPosture);
    }
    if structure.implementations.iter().any(|found| {
        found
            .meaning_bearing_attributes
            .iter()
            .any(|carried| !declared.attributes.contains(&carried.as_str()))
    }) {
        return Some(StructuralDisagreement::MeaningBearingAttribute);
    }
    if structure
        .implementations
        .iter()
        .any(|found| !found.unexpected_members.is_empty())
    {
        return Some(StructuralDisagreement::UnexpectedImplMember);
    }
    if structure
        .implementations
        .iter()
        .any(|found| !found.duplicated_members.is_empty())
    {
        return Some(StructuralDisagreement::DuplicateMember);
    }
    member_disagreement(structure, declared)
}

/// The first disagreement among the members the implementations carry.
fn member_disagreement(
    structure: &ArtifactStructure,
    declared: &DeclaredStructure<'_>,
) -> Option<StructuralDisagreement> {
    let shapes: Vec<&String> = structure
        .implementations
        .iter()
        .filter_map(|found| found.shape.as_ref())
        .collect();
    if sole(&shapes).is_none_or(|shape| shape.as_str() != declared.shape) {
        return Some(StructuralDisagreement::FamilyShape);
    }

    let orders: Vec<&Vec<String>> = structure
        .implementations
        .iter()
        .filter_map(|found| found.selection_order.as_ref())
        .collect();
    if sole(&orders).is_none_or(|order| !same_spellings(order, declared.spellings)) {
        return Some(StructuralDisagreement::SelectionOrder);
    }

    let constructors: Vec<&String> = structure
        .implementations
        .iter()
        .filter_map(|found| found.order_constructor.as_ref())
        .collect();
    if sole(&constructors).is_none_or(|called| called.as_str() != declared.order_constructor) {
        return Some(StructuralDisagreement::ConstructorPath);
    }

    let rosters: Vec<&Vec<CauseRow>> = structure
        .implementations
        .iter()
        .filter_map(|found| found.cause_rows.as_ref())
        .collect();
    let Some(rows) = sole(&rosters) else {
        return Some(StructuralDisagreement::CauseRows);
    };
    if rows.iter().any(|row| {
        row.row_constructor != declared.row_constructor
            || row.identity_constructor != declared.identity_constructor
    }) {
        return Some(StructuralDisagreement::ConstructorPath);
    }
    if !same_rows(rows, declared) {
        return Some(StructuralDisagreement::CauseRows);
    }
    None
}

/// The one member of a one-member roster, or `None` for any other count. Two
/// implementations both stating `SHAPE` is as much a finding as neither doing
/// so, and this is where both become one.
fn sole<'a, T>(found: &'a [&'a T]) -> Option<&'a T> {
    match found.split_first() {
        Some((first, [])) => Some(first),
        _ => None,
    }
}

/// Whether one trait-and-target pair is implemented more than once.
fn duplicated(structure: &ArtifactStructure) -> bool {
    structure
        .implementations
        .iter()
        .enumerate()
        .any(|(position, found)| {
            structure
                .implementations
                .iter()
                .skip(position.saturating_add(1))
                .any(|other| other.target == found.target && other.trait_path == found.trait_path)
        })
}

/// Whether one read roster is the declared roster, spelling for spelling and
/// position for position.
fn same_spellings(read: &[String], declared: &[&str]) -> bool {
    read.len() == declared.len()
        && read
            .iter()
            .zip(declared.iter())
            .all(|(found, expected)| found == expected)
}

/// Whether the read cause rows are the declared identities and spellings, in
/// order. Both columns are held: a roster keeping every spelling and recycling
/// one identity is as wrong as a permuted one.
fn same_rows(read: &[CauseRow], declared: &DeclaredStructure<'_>) -> bool {
    read.len() == declared.identities.len()
        && read.len() == declared.spellings.len()
        && read
            .iter()
            .zip(declared.identities.iter().zip(declared.spellings.iter()))
            .all(|(row, (identity, spelling))| {
                row.identity == *identity && row.spelling == *spelling
            })
}

/// Which expected associated constant one member name spells, where it spells
/// one at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ExpectedConstant {
    /// `SHAPE`.
    Shape,
    /// `SELECTION_ORDER`.
    SelectionOrder,
    /// `DECLARED_ORDER`.
    DeclaredOrder,
}

/// The expected constant one member name spells.
fn expected_constant(name: &str) -> Option<ExpectedConstant> {
    match name {
        "SHAPE" => Some(ExpectedConstant::Shape),
        "SELECTION_ORDER" => Some(ExpectedConstant::SelectionOrder),
        "DECLARED_ORDER" => Some(ExpectedConstant::DeclaredOrder),
        _ => None,
    }
}

/// One trait implementation, read out of the tree, or `None` where the item is
/// an implementation of something this lane cannot name — an inherent impl, or
/// one whose target is not a plain path.
fn implementation_structure(declared: &syn::ItemImpl) -> Option<ImplementationStructure> {
    let (path, _) = declared.trait_.as_ref()?;
    let target = type_path(&declared.self_ty)?;
    let mut shape = None;
    let mut selection_order = None;
    let mut order_constructor = None;
    let mut cause_rows = None;
    let mut read: Vec<String> = Vec::new();
    let mut unexpected_members: Vec<String> = Vec::new();
    let mut duplicated_members: Vec<String> = Vec::new();
    let mut meaning_bearing_attributes = meaning_bearing(&declared.attrs);
    for member in &declared.items {
        let (attributes, reading) = member_reading(member);
        meaning_bearing_attributes.extend(meaning_bearing(attributes));
        let constant = match reading {
            ImplMember::Constant(constant) => constant,
            ImplMember::Other(kind) => {
                unexpected_members.push(kind.to_owned());
                continue;
            }
        };
        let name = constant.ident.to_string();
        match expected_constant(&name) {
            None => unexpected_members.push(format!("the associated constant `{name}`")),
            Some(_) if read.contains(&name) => duplicated_members.push(name),
            Some(ExpectedConstant::Shape) => {
                read.push(name);
                shape = last_segment(&constant.expr);
            }
            Some(ExpectedConstant::SelectionOrder) => {
                read.push(name);
                selection_order = string_list(&constant.expr);
            }
            Some(ExpectedConstant::DeclaredOrder) => {
                read.push(name);
                order_constructor = called_path(&constant.expr);
                cause_rows = cause_row_list(&constant.expr);
            }
        }
    }
    Some(ImplementationStructure {
        target,
        trait_path: path_spelling(path),
        postures: postures_of(declared),
        meaning_bearing_attributes,
        shape,
        selection_order,
        order_constructor,
        cause_rows,
        unexpected_members,
        duplicated_members,
    })
}

/// The postures one implementation is written under, in roster order.
fn postures_of(declared: &syn::ItemImpl) -> Vec<ImplPosture> {
    let mut carried = Vec::new();
    if declared.unsafety.is_some() {
        carried.push(ImplPosture::Unsafely);
    }
    if declared.modifiers.polarity.is_some() {
        carried.push(ImplPosture::Negative);
    }
    if declared.modifiers.defaultness.is_some() {
        carried.push(ImplPosture::Defaulted);
    }
    if !declared.generics.params.is_empty() || declared.generics.where_clause.is_some() {
        carried.push(ImplPosture::Generic);
    }
    carried
}

/// What one implementation member is, for the two questions this lane asks of
/// members.
enum ImplMember<'a> {
    /// An associated constant, whose name and value are both read.
    Constant(&'a syn::ImplItemConst),
    /// Anything else, described by what it is. Nothing lawful renders one, so
    /// the description is what the finding carries.
    Other(&'static str),
}

/// One member's attributes, and what the member is.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::ImplItem` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every member it catches is by definition not one of the expected associated constants"
)]
fn member_reading(member: &syn::ImplItem) -> (&[syn::Attribute], ImplMember<'_>) {
    match member {
        syn::ImplItem::Const(constant) => (&constant.attrs, ImplMember::Constant(constant)),
        syn::ImplItem::Fn(method) => (&method.attrs, ImplMember::Other("an associated function")),
        syn::ImplItem::Type(associated) => {
            (&associated.attrs, ImplMember::Other("an associated type"))
        }
        syn::ImplItem::Macro(invocation) => (
            &invocation.attrs,
            ImplMember::Other("a macro invocation in member position"),
        ),
        _ => (&[], ImplMember::Other("a member this lane cannot name")),
    }
}

/// Every attribute that decides something, by path.
///
/// A doc comment reaches the tree as `#[doc = "…"]` and decides nothing, so it
/// is not carried. Everything else is: `cfg` decides whether the implementation
/// exists at all, and an attribute nobody planned is a finding whatever it does.
fn meaning_bearing(attributes: &[syn::Attribute]) -> Vec<String> {
    attributes
        .iter()
        .map(|attribute| path_spelling(attribute.path()))
        .filter(|spelled| spelled != "doc")
        .collect()
}

/// One path, spelled back with its segments and its leading `::`.
fn path_spelling(path: &syn::Path) -> String {
    let mut spelled = String::new();
    if path.leading_colon.is_some() {
        spelled.push_str("::");
    }
    for (position, segment) in path.segments.iter().enumerate() {
        if position > 0 {
            spelled.push_str("::");
        }
        spelled.push_str(&segment.ident.to_string());
    }
    spelled
}

/// The path one type spells, or `None` where the type is not a plain path.
#[expect(
    clippy::wildcard_enum_match_arm,
    reason = "`syn::Type` is non_exhaustive, so no crate outside syn can enumerate its variants; a wildcard is the only arm that closes this match, and every type it catches is by definition not a plain path"
)]
fn type_path(declared: &syn::Type) -> Option<String> {
    match declared {
        syn::Type::Path(typed) => Some(path_spelling(&typed.path)),
        _ => None,
    }
}

/// The last segment of a path expression — the variant word of a path-spelled
/// constant.
fn last_segment(expression: &syn::Expr) -> Option<String> {
    let syn::Expr::Path(spelled) = expression else {
        return None;
    };
    spelled
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string())
}

/// The path a call expression calls, where the callee is a plain path.
fn called_path(expression: &syn::Expr) -> Option<String> {
    let syn::Expr::Call(call) = expression else {
        return None;
    };
    let syn::Expr::Path(spelled) = call.func.as_ref() else {
        return None;
    };
    Some(path_spelling(&spelled.path))
}

/// The string literals of a `&[…]` expression, in order.
fn string_list(expression: &syn::Expr) -> Option<Vec<String>> {
    array_elements(expression)?.map(string_literal).collect()
}

/// The cause rows of a `Type::declared(&[…])` expression, in order.
fn cause_row_list(expression: &syn::Expr) -> Option<Vec<CauseRow>> {
    let syn::Expr::Call(call) = expression else {
        return None;
    };
    array_elements(call.args.first()?)?.map(cause_row).collect()
}

/// The elements of a `&[…]` expression.
fn array_elements(expression: &syn::Expr) -> Option<syn::punctuated::Iter<'_, syn::Expr>> {
    let syn::Expr::Reference(reference) = expression else {
        return None;
    };
    let syn::Expr::Array(array) = reference.expr.as_ref() else {
        return None;
    };
    Some(array.elems.iter())
}

/// One `Row::declared(Identity::declared("…"), "…")` element, as its four
/// columns: the two constructors it is built through, and the two values it
/// carries.
fn cause_row(expression: &syn::Expr) -> Option<CauseRow> {
    let syn::Expr::Call(call) = expression else {
        return None;
    };
    let row_constructor = called_path(expression)?;
    let mut arguments = call.args.iter();
    let minted = arguments.next()?;
    let spelling = string_literal(arguments.next()?)?;
    let identity_constructor = called_path(minted)?;
    let syn::Expr::Call(identity) = minted else {
        return None;
    };
    Some(CauseRow {
        row_constructor,
        identity_constructor,
        identity: string_literal(identity.args.first()?)?,
        spelling,
    })
}

/// The value of a string-literal expression.
fn string_literal(expression: &syn::Expr) -> Option<String> {
    let syn::Expr::Lit(literal) = expression else {
        return None;
    };
    let syn::Lit::Str(text) = &literal.lit else {
        return None;
    };
    Some(text.value())
}
