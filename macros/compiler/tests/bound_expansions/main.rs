//! Sealed-expansion claims observed from outside: addressed publications, binding diagnostics and bytes, and complete accounting.

use macroonz_compiler::{
    Accounted, BINDING_FACT, BindError, CrateBinding, Destination, Diagnostic, Disposition,
    DispositionRecord, DispositionSet, Door, Expansion, GeneratedToken, GeneratedTree, Kind,
    KindSet, LineBody, NoQuestions, Observed, Overflow, OwnerIdentity, Phase, Placement, Producer,
    RefusalClass, Refused, Request, Role, Site, TextCapture, encode_bytes,
};

/// The expansion shape this lane observes across every delivery posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Product;

impl Kind for Product {
    const NAME: &'static str = "lane.bound-expansion";
    type Content = &'static str;
    type Role = Seat;
    type Question = NoQuestions;
}

/// The seats whose delivery readings this lane distinguishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    /// The declaration-site unit.
    Declaration,
    /// The test-carrier unit.
    Test,
    /// The bench-carrier unit.
    Bench,
    /// The first publication unit in the declared roster.
    FirstPublication,
    /// The second publication unit in the declared roster.
    SecondPublication,
}

impl Role for Seat {
    const ALL: &'static [Self] = &[
        Self::Declaration,
        Self::Test,
        Self::Bench,
        Self::FirstPublication,
        Self::SecondPublication,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Declaration => "declaration",
            Self::Test => "test",
            Self::Bench => "bench",
            Self::FirstPublication => "first-publication",
            Self::SecondPublication => "second-publication",
        }
    }

    fn destination(self) -> Destination {
        match self {
            Self::Declaration => Destination::DeclarationSite,
            Self::Test => Destination::TestCarrier,
            Self::Bench => Destination::BenchCarrier,
            Self::FirstPublication | Self::SecondPublication => Destination::PublicationArtifact,
        }
    }
}

/// The one value that says who is asking.
const DOOR: Door = Door::declared(
    "lane",
    "lane.bound-expansion.grammar",
    "lane::bound_expansion",
    CrateBinding::declared("demo"),
    Producer {
        namespace: "lane",
        name: "bound-expansion",
    },
);

/// The address of the first publication unit.
const FIRST_ADDRESS: OwnerIdentity = OwnerIdentity {
    subject: "lane.publication",
    bytes: [1; 32],
};

/// The address of the second publication unit.
const SECOND_ADDRESS: OwnerIdentity = OwnerIdentity {
    subject: "lane.publication",
    bytes: [2; 32],
};

/// One generated word as a bounded tree.
fn word(spelling: &str) -> Result<GeneratedTree, Overflow> {
    GeneratedTree::assembled(vec![GeneratedToken::word(spelling)])
}

/// One lawful expansion over the declared source.
fn expanded(source: &str) -> Option<Expansion<Product>> {
    let captured = TextCapture::read(source).ok()?;
    Request::<Product>::over(captured.input().clone(), "product", &DOOR)
        .publishing_at(Seat::FirstPublication, FIRST_ADDRESS)
        .publishing_at(Seat::SecondPublication, SECOND_ADDRESS)
        .render(|_plan, out| {
            out.unit(Seat::Declaration, word("declaration")?)?;
            out.unit(Seat::Test, word("test")?)?;
            out.unit(Seat::Bench, word("bench")?)?;
            out.unit(Seat::SecondPublication, word("second")?)?;
            out.unit(Seat::FirstPublication, word("first")?)
        })
        .ok()
}

/// Publication reading preserves roster order and each unit's own address even when rendering order differs.
#[test]
fn publication_reading_preserves_addresses_and_declared_order() -> Result<(), ()> {
    let expansion = expanded("struct First;").ok_or(())?;
    let mut published = expansion.published();
    let first = published.next().ok_or(())?;
    let second = published.next().ok_or(())?;

    assert_eq!(first.role(), Seat::FirstPublication);
    assert_eq!(second.role(), Seat::SecondPublication);
    assert_eq!(first.address(), Some(FIRST_ADDRESS));
    assert_eq!(second.address(), Some(SECOND_ADDRESS));
    assert_eq!(first.tree().inspected().trim(), "first");
    assert_eq!(second.tree().inspected().trim(), "second");
    assert!(published.next().is_none());
    assert!(
        expansion
            .emission()
            .joined(Destination::PublicationArtifact)
            .is_none()
    );
    Ok(())
}

/// Every binding row has one stable byte spelling, while an actual disagreement projects as one cause with no related remainder.
#[test]
fn binding_rows_have_stable_bytes_and_one_diagnostic_contract() -> Result<(), ()> {
    let first = expanded("struct First;").ok_or(())?;
    let second = expanded("struct Second;").ok_or(())?;
    let first_plan = first.plan().identity();
    let second_plan = second.plan().identity();
    let first_closure = first.closure().identity();
    let second_closure = second.closure().identity();

    let actual = Expansion::bound(
        first.plan().clone(),
        second.closure().clone(),
        first.explain().clone(),
    )
    .err()
    .ok_or(())?;
    let rows = [
        (
            BindError::ClosureProvedAgainstAnotherPlan {
                planned: first_plan,
                proved: second_plan,
            },
            0,
            first_plan.as_bytes(),
            second_plan.as_bytes(),
        ),
        (
            BindError::ExplanationAnsweredOverAnotherPlan {
                planned: first_plan,
                answered: second_plan,
            },
            1,
            first_plan.as_bytes(),
            second_plan.as_bytes(),
        ),
        (
            BindError::ExplanationAnsweredOverAnotherClosure {
                proved: first_closure,
                answered: second_closure,
            },
            2,
            first_closure.as_bytes(),
            second_closure.as_bytes(),
        ),
    ];

    for (refusal, slot, bound, carried) in rows {
        let mut expected = vec![slot];
        encode_bytes(bound, &mut expected);
        encode_bytes(carried, &mut expected);
        assert_eq!(refusal.slot(), slot);
        assert_eq!(refusal.canonical_bytes(), expected);
        assert_eq!(expected.len(), 81usize);
    }

    let diagnostic = Diagnostic::refused(&actual, &DOOR, &Placement::WholeDeclaration);
    assert_eq!(Refused::class(&actual), RefusalClass::ExpansionNotBound);
    assert_eq!(Refused::body(&actual), LineBody::SingleCause);
    assert!(Refused::related(&actual).is_empty());
    assert_eq!(diagnostic.phase(), Phase::Binding);
    assert_eq!(diagnostic.observed(), Observed::IdentityDisagreement);
    assert_eq!(diagnostic.site(), Site::WholeDeclaration);
    assert!(diagnostic.related().carried().is_empty());
    assert_eq!(diagnostic.repairs().len(), 1usize);
    assert_eq!(
        diagnostic.repairs().first().ok_or(())?.declared_by,
        BINDING_FACT
    );
    assert_eq!(diagnostic.route().entry(), DOOR.entry());
    assert!(!diagnostic.summary().contains("further established issues"));
    Ok(())
}

/// The consumer-owned record for the one-kind accounting set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProductDisposition {
    product: Disposition,
}

impl DispositionRecord for ProductDisposition {
    fn into_dispositions(self) -> impl Iterator<Item = Disposition> {
        [self.product].into_iter()
    }
}

/// The set whose complete witness this lane seats beside an expansion.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProductSet;

impl KindSet for ProductSet {
    type Dispositions = ProductDisposition;

    const NAMES: &'static [&'static str] = &[Product::NAME];
}

/// Accounting retains the expansion whole beside the already-complete disposition witness.
#[test]
fn accounting_retains_both_complete_values() -> Result<(), ()> {
    let expansion = expanded("struct Accounted;").ok_or(())?;
    let identity = expansion.identity();
    let unit = expansion
        .closure()
        .rendered()
        .under(Seat::Declaration)
        .ok_or(())?
        .semantic_key();
    let dispositions = DispositionSet::<ProductSet>::complete(ProductDisposition {
        product: Disposition::Generated { unit },
    })
    .map_err(|_refusal| ())?;
    let accounted = Accounted::seated(expansion, dispositions);

    assert_eq!(accounted.expansion().identity(), identity);
    assert_eq!(accounted.dispositions().len(), 1usize);
    let mut rows = accounted.dispositions().iter();
    let (name, disposition) = rows.next().ok_or(())?;
    assert_eq!(name, Product::NAME);
    assert_eq!(disposition, &Disposition::Generated { unit });
    assert!(rows.next().is_none());
    Ok(())
}
