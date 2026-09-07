use super::ProposalArchiveRefusal;
use crate::descriptor::archive::CandidateArchiveRefusal;
use crate::report::archive::ArchiveRefusal;

impl From<ArchiveRefusal> for ProposalArchiveRefusal {
    fn from(refusal: ArchiveRefusal) -> Self {
        Self::Historical(refusal)
    }
}

impl From<CandidateArchiveRefusal> for ProposalArchiveRefusal {
    fn from(refusal: CandidateArchiveRefusal) -> Self {
        Self::Candidate(refusal)
    }
}
