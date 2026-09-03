//! The compiled-mutation lane: the console grammar of one wrapped backend, the defensive parser that reads it, and the witness runs a reading is planned into.
//!
//! The backend is external and it runs outside the wall — it mutates real source and invokes the test command itself.
//! Nothing here executes anything: this file reads text a caller already holds, and plans runs the one report engine performs.
//!
//! # The line grammar this parser reads
//!
//! The grammar is line-oriented and the reading is defensive: a line is read only when it matches a shape stated here, and every other line becomes an [`UnparsedLine`](crate::muterprater::UnparsedLine) that travels with the reading rather than being dropped.
//!
//! - A roster line is `Found <count> mutant…`: the word `Found`, a decimal count, and a third word beginning `mutant`.
//! - A baseline line is an outcome word followed by `Unmutated baseline…`, and only `ok` reads as a qualified baseline.
//! - A mutant line is an outcome word, then `<file>:<line>:<column>:`, then the backend's own damage text.
//!
//! The coordinate's file part is everything before the last two colon-separated fields, so a drive-lettered path stays whole.
//! The outcome words are `caught`, `missed`, `unviable`, `timeout`, and `failed`, matched without regard to case.
//!
//! # What a reading may claim
//!
//! Every reading is stated under an [`AdapterProfile`](crate::muterprater::AdapterProfile) naming the backend, the version posture the running party states, the output it was taken from, and this adapter's own grammar version.
//! Three of those four are this file's own facts, so [`console_profile`] states them and only the backend version is the caller's word.
//!
//! The backend says which of its own mutants its command rejected and says nothing about whether a damaged expression was ever reached, so every mutant read here carries [`ActivationDisposition::UnobservableUnderBackend`](crate::muterprater::ActivationDisposition::UnobservableUnderBackend).
//! Two consequences follow, both structural: a kill under this lane asserts witness rejection and never observed activation, and a non-kill can never earn survived — it is inconclusive, and [`MutationRun::non_kills`](crate::muterprater::MutationRun::non_kills) is the roster a reader means by "what got through".
//! The same fact at run width is the profile's [`ClaimCeiling::WitnessRejection`](crate::muterprater::ClaimCeiling::WitnessRejection), and a reading whose run carries a survivor is refused rather than believed.
//!
//! A kill's rejection is the backend's word ([`IntendedRejection::ReportedByBackend`](crate::muterprater::IntendedRejection::ReportedByBackend)), because it named neither a trial nor a cause, so no fingerprint exists for it.
//!
//! # What stands behind the grammar
//!
//! The grammar is an inspectable assumption about one tool's rendering, and it qualifies nothing until a party states that these shapes were checked against real output of the exact backend version a reading names.
//! That statement is what [`AdapterQualification`](crate::muterprater::AdapterQualification) carries, and [`CompiledSuitePressure`](crate::muterprater::CompiledSuitePressure) is a lawful kill read out of a current-source-qualified artifact carrying that exact profile.
//! A different machine-readable backend surface, if adopted, earns its own profile and its own qualification rather than inheriting this grammar's standing.
//!
//! # The caller-supplied seams
//!
//! External mutants arrive as source coordinates rather than as claims, so the reading from a coordinate to its owning claim is the caller's ([`OwnerLookup`](crate::muterprater::OwnerLookup)), and so is the reading from damage text to operator family ([`FamilyLookup`](crate::muterprater::FamilyLookup)).
//! Neither answer is invented here: an unanswered lookup produces [`MappingPosture::OwnerUnmapped`](crate::muterprater::MappingPosture::OwnerUnmapped) and [`FamilyAttribution::OutsideTheBank`](crate::muterprater::FamilyAttribution::OutsideTheBank), and the witness selection widens accordingly.

mod parse;
mod plan;
mod read;

pub use plan::{mutant_scoped, plan_pass};
pub use read::{console_profile, read_artifact, read_output};
