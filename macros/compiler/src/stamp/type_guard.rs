//! The stamp home's invariant nucleus: every road that reaches a private field, and the one road that composes a published artifact.
//!
//! Declared inside `types.rs` as its own child, which is what makes this home's claims structural rather than remembered.
//! A spelling is admitted against the alphabet here, so a name the consumer's compiler would read as something else is not a value anybody can hold.
//! A pattern's seats and a stamp's sites are closed here, so a definition that binds one metavariable twice, or a manifest that names one site twice, is refused before a token exists.
//! And an artifact is composed here, so there is no half-rendered publication unit for a reader to mistake for a whole one.

use super::super::render;
use super::{
    Landing, PART_LIMIT, PATH_SEGMENT_LIMIT, Part, Pattern, PublicationGround, PublicationRecord,
    PublishedStamp, SITE_LIMIT, Seat, Seating, Site, SiteRoot, Stamp, StampError, StampName,
    StampedPlan, Visibility,
};
use crate::bounded::{Bounded, NonEmpty, NonEmptyError};
use crate::identity::{self, Identity};
use crate::plan::DigestContract;
use crate::token::GeneratedTree;
use std::collections::BTreeSet;

impl Seat {
    /// Declare one metavariable seat.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::NotAnIdentifier`] where the name is not one Rust identifier, which is what a matcher needs it to be.
    pub fn declared(name: &str, seating: Seating) -> Result<Self, StampError> {
        if !is_identifier(name) {
            return Err(StampError::NotAnIdentifier);
        }
        Ok(Self {
            name: name.to_owned(),
            seating,
        })
    }

    /// The name material travels under.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The shape it travels in.
    #[must_use]
    pub const fn seating(&self) -> Seating {
        self.seating
    }
}

impl Pattern {
    /// Declare one pattern: the sentence its definition is documented with, the shape it is invoked in, and the body that shape expands into.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::SeatNameDoubled`] where two seats carry one name, [`StampError::PatternEmpty`] where no part was stated, and [`StampError::PatternUnbounded`] where the parts outgrow the declared magnitude.
    ///
    /// The namespace is closed before the magnitude, because a collision is a defect in what was declared and a caller repairing a magnitude first would repair the collision second.
    pub fn declared(note: &str, parts: Vec<Part>, body: GeneratedTree) -> Result<Self, StampError> {
        seat_names_closed(&parts)?;
        let admitted: NonEmpty<Part, PART_LIMIT> =
            NonEmpty::new(parts).map_err(|refusal| match refusal {
                NonEmptyError::Empty(_) => StampError::PatternEmpty,
                NonEmptyError::Overflow(overflow) => StampError::PatternUnbounded { overflow },
            })?;
        Ok(Self {
            note: note.to_owned(),
            parts: admitted,
            body,
        })
    }

    /// The sentence the definition is documented with.
    #[must_use]
    pub fn note(&self) -> &str {
        self.note.as_str()
    }

    /// The declared shape, in the order it is written; structurally at least one part.
    ///
    /// # Ordering
    ///
    /// This order is meaning: a matcher and every invocation are walks over it, so the same parts stated in another order are another grammar.
    #[must_use]
    pub fn parts(&self) -> &NonEmpty<Part, PART_LIMIT> {
        &self.parts
    }

    /// The body the shape expands into.
    #[must_use]
    pub const fn body(&self) -> &GeneratedTree {
        &self.body
    }

    /// The seats of the shape, in the order a site supplies arguments for them.
    pub fn seats(&self) -> impl Iterator<Item = &Seat> {
        self.parts.iter().filter_map(|part| match part {
            Part::Seat(seat) => Some(seat),
            Part::Literal(_) | Part::Reach => None,
        })
    }

    /// How many seats the shape declares.
    #[must_use]
    pub fn seat_count(&self) -> usize {
        self.seats().count()
    }

    /// Whether the shape gives a site's visibility a coordinate.
    #[must_use]
    pub fn reaches(&self) -> bool {
        self.parts.iter().any(|part| match part {
            Part::Reach => true,
            Part::Literal(_) | Part::Seat(_) => false,
        })
    }
}

impl StampName {
    /// The name one published stamp is exported under.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::NotAnIdentifier`] where the spelling is not one Rust identifier.
    pub fn declared(spelling: &str) -> Result<Self, StampError> {
        if !is_identifier(spelling) {
            return Err(StampError::NotAnIdentifier);
        }
        Ok(Self {
            spelling: spelling.to_owned(),
        })
    }

    /// The exported spelling a site invokes this stamp by.
    #[must_use]
    pub fn spelling(&self) -> &str {
        self.spelling.as_str()
    }
}

impl SiteRoot {
    /// The path one site reaches its stamp by, parsed from the segments the caller stated.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::NotAnIdentifier`] where a segment is not one Rust identifier, [`StampError::PathEmpty`] where no segment was stated, and [`StampError::PathUnbounded`] where the segments outgrow the declared magnitude.
    ///
    /// The checks are in that order, so exactly one cause is true of any refused root.
    pub fn spelled(segments: Vec<String>) -> Result<Self, StampError> {
        for segment in &segments {
            if !is_identifier(segment.as_str()) {
                return Err(StampError::NotAnIdentifier);
            }
        }
        let admitted: NonEmpty<String, PATH_SEGMENT_LIMIT> =
            NonEmpty::new(segments).map_err(|refusal| match refusal {
                NonEmptyError::Empty(_) => StampError::PathEmpty,
                NonEmptyError::Overflow(overflow) => StampError::PathUnbounded { overflow },
            })?;
        Ok(Self { segments: admitted })
    }

    /// The segments, in the order they were stated; structurally at least one.
    #[must_use]
    pub fn segments(&self) -> &NonEmpty<String, PATH_SEGMENT_LIMIT> {
        &self.segments
    }

    /// How many segments the root carries; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.segments.count()
    }
}

impl Site {
    /// Declare one site that adopts a stamp.
    ///
    /// The name is a label rather than a spelling: it is what the manifest calls this landing, and no token is ever written from it.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::ArgumentsUnbounded`] where the arguments outgrow the declared magnitude.
    /// Whether they are the RIGHT arguments is settled where the site meets its pattern, in [`Stamp::declared`].
    pub fn declared(
        name: &str,
        root: SiteRoot,
        reach: Visibility,
        arguments: Vec<GeneratedTree>,
    ) -> Result<Self, StampError> {
        let admitted: Bounded<GeneratedTree, PART_LIMIT> = Bounded::new(arguments)
            .map_err(|overflow| StampError::ArgumentsUnbounded { overflow })?;
        Ok(Self {
            name: name.to_owned(),
            root,
            reach,
            arguments: admitted,
        })
    }

    /// What the manifest calls this landing.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// The path this site reaches its stamp by.
    #[must_use]
    pub const fn root(&self) -> &SiteRoot {
        &self.root
    }

    /// The reach this site writes.
    #[must_use]
    pub const fn reach(&self) -> Visibility {
        self.reach
    }

    /// The material this site supplies, one argument per declared seat, in seat order.
    #[must_use]
    pub fn arguments(&self) -> &[GeneratedTree] {
        self.arguments.as_slice()
    }
}

impl Stamp {
    /// Declare the complete payload one published stamp is rendered from.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::SiteNameDoubled`] where two sites carry one name, [`StampError::ArgumentsUnmatched`] where a site supplies a different number of arguments than the pattern declares seats, [`StampError::ReachUnseated`] where a site declares a reach the pattern gives no coordinate to, [`StampError::SitesAbsent`] where no site was stated, and [`StampError::SitesUnbounded`] where the sites outgrow the declared magnitude.
    ///
    /// The namespace is closed first, then each site is settled against the pattern in the order the sites were stated, and the magnitude last.
    pub fn declared(
        name: StampName,
        pattern: Pattern,
        sites: Vec<Site>,
    ) -> Result<Self, StampError> {
        site_names_closed(&sites)?;
        sites_seated(&pattern, &sites)?;
        let admitted: NonEmpty<Site, SITE_LIMIT> =
            NonEmpty::new(sites).map_err(|refusal| match refusal {
                NonEmptyError::Empty(_) => StampError::SitesAbsent,
                NonEmptyError::Overflow(overflow) => StampError::SitesUnbounded { overflow },
            })?;
        Ok(Self {
            name,
            pattern,
            sites: admitted,
        })
    }

    /// The name this stamp is exported under.
    #[must_use]
    pub const fn name(&self) -> &StampName {
        &self.name
    }

    /// The pattern this stamp stamps.
    #[must_use]
    pub const fn pattern(&self) -> &Pattern {
        &self.pattern
    }

    /// The sites covered, in the order they were declared; structurally at least one.
    ///
    /// # Ordering
    ///
    /// This order is meaning for a migration: one invocation is rendered per site in the order this yields.
    #[must_use]
    pub fn sites(&self) -> &NonEmpty<Site, SITE_LIMIT> {
        &self.sites
    }

    /// How many sites this stamp covers; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.sites.count()
    }
}

impl Landing {
    /// The site this landing is for.
    #[must_use]
    pub fn site(&self) -> &str {
        self.site.as_str()
    }

    /// The invocation written there.
    #[must_use]
    pub const fn invocation(&self) -> &GeneratedTree {
        &self.invocation
    }
}

impl PublicationRecord {
    /// Why neither of the lighter roads expresses this output.
    #[must_use]
    pub const fn ground(&self) -> PublicationGround {
        self.ground
    }

    /// The planned member this artifact materializes.
    #[must_use]
    pub const fn unit(&self) -> Identity<identity::GeneratedUnit> {
        self.unit
    }

    /// What the eventual staged bytes' digest must satisfy.
    #[must_use]
    pub const fn staged(&self) -> DigestContract {
        self.staged
    }

    /// The stamp the artifact was rendered from, whole.
    #[must_use]
    pub const fn covered(&self) -> &Stamp {
        &self.stamp
    }

    /// What the unit contains, row by row.
    pub fn manifest(&self) -> impl Iterator<Item = &str> {
        self.stamp.sites().iter().map(Site::name)
    }
}

impl PublishedStamp {
    /// Render one published stamp over what the plan decided, what the caller declared, and why the lighter roads are insufficient.
    ///
    /// The order is the road: the definition first, then one invocation per covered site, then the record — and the artifact only after all three, so no half-rendered publication unit exists.
    ///
    /// # Errors
    ///
    /// Returns [`StampError::TokensUnbounded`] where the definition, one invocation, or the tree either is assembled into outgrows the declared token magnitude.
    pub fn rendered(
        planned: &StampedPlan,
        stamp: &Stamp,
        ground: PublicationGround,
    ) -> Result<Self, StampError> {
        let definition = GeneratedTree::assembled(render::definition(stamp)?)?;
        let mut landings: Vec<Landing> = Vec::new();
        for site in stamp.sites() {
            let invocation = GeneratedTree::assembled(render::invocation(stamp, site)?)?;
            landings.push(Landing {
                site: site.name().to_owned(),
                invocation,
            });
        }
        Ok(Self {
            definition,
            landings,
            record: PublicationRecord {
                ground,
                unit: planned.unit,
                staged: planned.staged,
                stamp: stamp.clone(),
            },
        })
    }

    /// The name the stamp is exported under, read out of the record.
    #[must_use]
    pub const fn name(&self) -> &StampName {
        self.record.covered().name()
    }

    /// The definition a publication road lands as visible source.
    #[must_use]
    pub const fn definition(&self) -> &GeneratedTree {
        &self.definition
    }

    /// Every covered site's landing, in the order the stamp declares them.
    ///
    /// # Bounds
    ///
    /// Exactly as many as the stamp declares sites, because the road that built them walked that stamp once.
    #[must_use]
    pub fn landings(&self) -> &[Landing] {
        self.landings.as_slice()
    }

    /// How many landings this artifact carries; structurally at least one.
    #[must_use]
    pub fn count(&self) -> usize {
        self.landings.len()
    }

    /// This side's record of the publication act.
    pub const fn record(&self) -> &PublicationRecord {
        &self.record
    }
}

/// The seat namespace one pattern closes.
///
/// Two seats under one name bind one metavariable twice, which the consumer's compiler would report inside an expansion nobody wrote.
fn seat_names_closed(parts: &[Part]) -> Result<(), StampError> {
    let mut named: BTreeSet<&str> = BTreeSet::new();
    for (position, part) in parts.iter().enumerate() {
        let Part::Seat(seat) = part else {
            continue;
        };
        if !named.insert(seat.name()) {
            return Err(StampError::SeatNameDoubled {
                at: counted(position),
            });
        }
    }
    Ok(())
}

/// The site namespace one stamp closes.
///
/// Two sites under one name are one manifest row written twice, and nothing downstream could tell which landing a row is about.
fn site_names_closed(sites: &[Site]) -> Result<(), StampError> {
    let mut named: BTreeSet<&str> = BTreeSet::new();
    for (position, site) in sites.iter().enumerate() {
        if !named.insert(site.name()) {
            return Err(StampError::SiteNameDoubled {
                at: counted(position),
            });
        }
    }
    Ok(())
}

/// Every site settled against the pattern it adopts: one argument per seat, and a reach only where the pattern writes one.
fn sites_seated(pattern: &Pattern, sites: &[Site]) -> Result<(), StampError> {
    let seats = counted(pattern.seat_count());
    let reaches = pattern.reaches();
    for (position, site) in sites.iter().enumerate() {
        let supplied = counted(site.arguments().len());
        if supplied != seats {
            return Err(StampError::ArgumentsUnmatched {
                at: counted(position),
                seats,
                supplied,
            });
        }
        if !reaches && site.reach() != Visibility::Private {
            return Err(StampError::ReachUnseated {
                at: counted(position),
            });
        }
    }
    Ok(())
}

/// Whether one spelling is a Rust identifier, which is what this home writes as a token.
fn is_identifier(spelling: &str) -> bool {
    let mut characters = spelling.chars();
    let Some(head) = characters.next() else {
        return false;
    };
    if !head.is_ascii_alphabetic() && head != '_' {
        return false;
    }
    if spelling == "_" {
        return false;
    }
    characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

/// One count as a refusal carries it.
fn counted(value: usize) -> u32 {
    u32::try_from(value).unwrap_or(u32::MAX)
}
