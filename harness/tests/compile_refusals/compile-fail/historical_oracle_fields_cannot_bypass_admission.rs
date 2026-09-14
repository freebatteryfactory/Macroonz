//! Historical integrity and private disagreement relationships cannot be forged outside admission.

use macroonz_harness::identity::ContentAddress;
use macroonz_harness::oracle::{ByteDifference, VectorDisagreement};
use macroonz_harness::oracle::archive::{
    ArchivedOracle, ArchivedTranscriptDisagreement, ArchivedVectorDisagreement, ArchivedVerdict,
};
use macroonz_harness::report::archive::AddressClaim;

fn record(address: ContentAddress) -> ArchivedOracle {
    ArchivedOracle {
        encoded: vec![],
        address,
        verdict: ArchivedVerdict::VectorAgrees,
    }
}

fn vector() -> ArchivedVectorDisagreement {
    ArchivedVectorDisagreement {
        expected: vec![1],
        produced: vec![1],
        difference: ByteDifference::AtByte { at: 99 },
    }
}

fn current_vector() -> VectorDisagreement {
    VectorDisagreement {
        expected: vec![1],
        produced: vec![1],
        difference: ByteDifference::AtByte { at: 99 },
    }
}

fn transcript(claim: AddressClaim) -> ArchivedTranscriptDisagreement {
    ArchivedTranscriptDisagreement {
        rederived: claim,
        published: claim,
    }
}

fn main() {}
