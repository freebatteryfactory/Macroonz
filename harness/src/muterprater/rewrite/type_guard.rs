//! The invariant nucleus of structural rewrite descriptors and rosters.

use super::{
    OperatorFamilyRef, RewriteCandidate, RewriteDescriptor, RewriteRefusal, RewriteRoster,
    RewriteTrust, RosterRefusal, ScopeShape,
};
impl RewriteDescriptor {
    /// One rewrite-mutation descriptor, as its author states it.
    ///
    /// # Errors
    ///
    /// Refuses an empty pattern, then an empty rewrite, then a pair whose two sides are one shape.
    pub fn declared(
        family: OperatorFamilyRef,
        pattern: &'static str,
        rewrite: &'static str,
    ) -> Result<Self, RewriteRefusal> {
        if pattern.is_empty() {
            return Err(RewriteRefusal::EmptyPattern);
        }
        if rewrite.is_empty() {
            return Err(RewriteRefusal::EmptyRewrite);
        }
        if pattern == rewrite {
            return Err(RewriteRefusal::RewriteIsPattern);
        }
        Ok(Self {
            family,
            pattern,
            rewrite,
        })
    }

    /// The operator family this pair realizes.
    #[must_use]
    pub const fn family(self) -> OperatorFamilyRef {
        self.family
    }

    /// The shape a damage matches.
    #[must_use]
    pub const fn pattern(self) -> &'static str {
        self.pattern
    }

    /// The shape it rewrites to.
    #[must_use]
    pub const fn rewrite(self) -> &'static str {
        self.rewrite
    }
}

impl RewriteRoster {
    /// The lane's declared descriptors.
    ///
    /// # Errors
    ///
    /// Refuses an empty roster, then two entries stating one pattern-and-rewrite pair — refused rather than folded away, because collapsing a duplicate silently would normalize an authoring defect out of sight.
    pub fn declared(descriptors: Vec<RewriteDescriptor>) -> Result<Self, RosterRefusal> {
        if descriptors.is_empty() {
            return Err(RosterRefusal::EmptyRoster);
        }
        for (at, descriptor) in descriptors.iter().enumerate() {
            if descriptors.iter().take(at).any(|earlier| {
                earlier.pattern() == descriptor.pattern()
                    && earlier.rewrite() == descriptor.rewrite()
            }) {
                return Err(RosterRefusal::DuplicateDescriptor { at });
            }
        }
        Ok(Self { descriptors })
    }

    /// Every descriptor the roster carries, in declared order.
    #[must_use]
    pub fn descriptors(&self) -> &[RewriteDescriptor] {
        &self.descriptors
    }
}

impl RewriteCandidate {
    /// One descriptor planned for the harness's audit.
    #[must_use]
    pub fn planned(descriptor: RewriteDescriptor, scope: ScopeShape) -> Self {
        Self {
            descriptor,
            scope,
            trust: RewriteTrust::AuditPending,
        }
    }

    /// The descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> RewriteDescriptor {
        self.descriptor
    }

    /// The scope its application was planned under.
    #[must_use]
    pub const fn scope(&self) -> &ScopeShape {
        &self.scope
    }

    /// The trust posture it stands under.
    #[must_use]
    pub const fn trust(&self) -> RewriteTrust {
        self.trust
    }
}
