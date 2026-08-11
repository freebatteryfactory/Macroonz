//! Specimen A's owed red twin, discharged.
//!
//! The declarative stamp writes the Class-C guard, and the guard's wall comes
//! with it: two guards over different scopes are different types, so comparing
//! one against the other is not a runtime edge case to refuse — it is a
//! category error the compiler rejects. No value is constructed below; the
//! signature alone is the proof.

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AlphaScopeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BetaScopeId;

threadpak::scope_guard_version! {
    /// Alpha's version, positioned by Alpha's authority.
    pub struct AlphaVersion over AlphaScopeId;
}

threadpak::scope_guard_version! {
    /// Beta's version, positioned by Beta's authority.
    pub struct BetaVersion over BetaScopeId;
}

fn main() {
    let across: fn(&AlphaVersion, &BetaVersion) = |left, right| {
        let _ = left.try_cmp_same_scope(right);
    };
    let _ = across;
}
