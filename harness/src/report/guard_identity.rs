//! The three identity rails: what a trial means, which revisions it stood on, and what one execution was keyed by.

use crate::descriptor::{CanonicalRowBytes, RevisionBinding, TrialKey};
use crate::identity::ContentAddress;
use crate::report::{
    ByteBudget, CaseBudget, CheckRevisionId, EXECUTION_KEY_TAG, ExecutionKey, InvocationProfile,
    ProfiledTrial, ROW_REVISION_TAG, RowRevisionId, SubjectRevisionId, TRIAL_IDENTITY_TAG,
    TargetBinding, TargetTriple, TimeBudget, ToolchainIdentity, TrialId, TrialProfile, TrialSite,
    execution_key_preimage, trial_preimage,
};

impl ProfiledTrial {
    /// One trial's key under one profile.
    ///
    /// Standing on the key rather than on the whole row is what keeps the execution suite, the roles, and the tags out of a trial's identity.
    #[must_use]
    pub const fn of_key(key: TrialKey, profile: TrialProfile) -> Self {
        Self { key, profile }
    }

    /// The trial's compact key.
    #[must_use]
    pub const fn key(self) -> TrialKey {
        self.key
    }

    /// The profile coordinate.
    #[must_use]
    pub const fn profile(self) -> TrialProfile {
        self.profile
    }
}

impl TrialId {
    /// Derive one trial's semantic identity from its complete preimage.
    ///
    /// Deterministic and total: every key under every profile names a trial.
    #[must_use]
    pub fn over(profiled: ProfiledTrial) -> Self {
        Self(ContentAddress::derived(
            TRIAL_IDENTITY_TAG,
            &trial_preimage(profiled),
        ))
    }

    /// Derive the identity of the trial one descriptor row declares.
    #[must_use]
    pub fn of_key(key: TrialKey, profile: TrialProfile) -> Self {
        Self::over(ProfiledTrial::of_key(key, profile))
    }

    /// The identity's address, for comparison and for rendering.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

impl TrialSite {
    /// Where one trial is written.
    #[must_use]
    pub const fn located(
        module_path: &'static str,
        file: &'static str,
        line: u32,
        name: &'static str,
    ) -> Self {
        Self {
            module_path,
            file,
            line,
            name,
        }
    }

    /// The module path the trial is declared under.
    #[must_use]
    pub const fn module_path(&self) -> &'static str {
        self.module_path
    }

    /// The file the trial is declared in.
    #[must_use]
    pub const fn file(&self) -> &'static str {
        self.file
    }

    /// The line the trial is declared on.
    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    /// The trial's display name, which is what a person filters on.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }
}

impl RowRevisionId {
    /// Derive one row revision identity from the row's canonical bytes.
    ///
    /// Total, and typed on the bytes rather than on a slice: holding them is holding everything the derivation needs, so there is nothing left here to refuse.
    #[must_use]
    pub fn over(canonical_row: &CanonicalRowBytes) -> Self {
        Self(ContentAddress::derived(
            ROW_REVISION_TAG,
            canonical_row.as_bytes(),
        ))
    }

    /// The identity's address.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

impl SubjectRevisionId {
    /// The subject revision one attachment bound, under this home's name for it.
    ///
    /// The address crosses unchanged; nothing is derived a second time.
    #[must_use]
    pub const fn of_binding(binding: RevisionBinding) -> Self {
        Self(binding.revision())
    }

    /// The identity's address.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

impl CheckRevisionId {
    /// The check revision one attachment bound, under this home's name for it.
    #[must_use]
    pub const fn of_binding(binding: RevisionBinding) -> Self {
        Self(binding.revision())
    }

    /// The identity's address.
    #[must_use]
    pub const fn address(&self) -> &ContentAddress {
        &self.0
    }
}

impl TargetTriple {
    /// The target triple the run declared.
    #[must_use]
    pub fn declared(spelling: &str) -> Self {
        Self(spelling.to_owned())
    }

    /// The declared spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl ToolchainIdentity {
    /// The toolchain identity the run declared.
    #[must_use]
    pub fn declared(spelling: &str) -> Self {
        Self(spelling.to_owned())
    }

    /// The declared spelling.
    #[must_use]
    pub fn spelling(&self) -> &str {
        &self.0
    }
}

impl TargetBinding {
    /// The target and toolchain one execution ran under.
    #[must_use]
    pub fn bound(target: TargetTriple, toolchain: ToolchainIdentity) -> Self {
        Self { target, toolchain }
    }

    /// The compilation target.
    #[must_use]
    pub const fn target(&self) -> &TargetTriple {
        &self.target
    }

    /// The toolchain.
    #[must_use]
    pub const fn toolchain(&self) -> &ToolchainIdentity {
        &self.toolchain
    }
}

impl CaseBudget {
    /// The case budget the invocation declared.
    #[must_use]
    pub const fn declared(cases: u32) -> Self {
        Self(cases)
    }

    /// The declared number of cases.
    #[must_use]
    pub const fn cases(self) -> u32 {
        self.0
    }
}

impl ByteBudget {
    /// The input-byte budget the invocation declared.
    #[must_use]
    pub const fn declared(bytes: u64) -> Self {
        Self(bytes)
    }

    /// The declared number of bytes.
    #[must_use]
    pub const fn bytes(self) -> u64 {
        self.0
    }
}

impl TimeBudget {
    /// The time budget the invocation declared, in nanoseconds.
    #[must_use]
    pub const fn declared(nanoseconds: u64) -> Self {
        Self(nanoseconds)
    }

    /// The declared bound, in nanoseconds.
    #[must_use]
    pub const fn nanoseconds(self) -> u64 {
        self.0
    }
}

impl InvocationProfile {
    /// The invocation's conclusion-relevant facts.
    #[must_use]
    pub const fn declared(cases: CaseBudget, bytes: ByteBudget, time: TimeBudget) -> Self {
        Self { cases, bytes, time }
    }

    /// The case budget.
    #[must_use]
    pub const fn cases(self) -> CaseBudget {
        self.cases
    }

    /// The input-byte budget.
    #[must_use]
    pub const fn bytes(self) -> ByteBudget {
        self.bytes
    }

    /// The time budget.
    #[must_use]
    pub const fn time(self) -> TimeBudget {
        self.time
    }
}

impl ExecutionKey {
    /// The key one execution of one trial is looked up under.
    ///
    /// The target binding is taken unconditionally, so a cross-target hit is not a policy the harness applies but a value it cannot construct.
    #[must_use]
    pub fn over(
        trial: TrialId,
        subject: SubjectRevisionId,
        check: CheckRevisionId,
        invocation: InvocationProfile,
        target: TargetBinding,
    ) -> Self {
        Self {
            trial,
            subject,
            check,
            invocation,
            target,
        }
    }

    /// The trial this key executes.
    #[must_use]
    pub const fn trial(&self) -> TrialId {
        self.trial
    }

    /// The subject revision it stood on.
    #[must_use]
    pub const fn subject(&self) -> SubjectRevisionId {
        self.subject
    }

    /// The check revision it stood on.
    #[must_use]
    pub const fn check(&self) -> CheckRevisionId {
        self.check
    }

    /// The invocation profile it ran under.
    #[must_use]
    pub const fn invocation(&self) -> InvocationProfile {
        self.invocation
    }

    /// The target and toolchain it ran on.
    #[must_use]
    pub const fn target(&self) -> &TargetBinding {
        &self.target
    }

    /// The key's address, derived from its parts.
    #[must_use]
    pub fn address(&self) -> ContentAddress {
        ContentAddress::derived(
            EXECUTION_KEY_TAG,
            &execution_key_preimage(
                self.trial,
                self.subject,
                self.check,
                self.invocation,
                &self.target,
            ),
        )
    }
}
