//! Historical interpreted records cannot create current trust or execution evidence.

use macroonz_harness::muterprater::interpretation_archive::{ArchivedInterpretedEvidence, ArchivedInterpretedTrust};
use macroonz_harness::muterprater::{InterpretedMutationEvidence, InterpretedTrust};

fn trust(historical: ArchivedInterpretedTrust) -> InterpretedTrust<'static, 'static, 'static, 'static, 'static, 'static, (), ()> {
    historical
}

fn evidence(historical: ArchivedInterpretedEvidence) -> InterpretedMutationEvidence<'static, 'static, 'static, 'static, 'static, 'static, (), ()> {
    historical
}

fn main() {}
