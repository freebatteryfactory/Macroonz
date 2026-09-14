//! Transcript records keep unequal digest claims without reconstructing their preimages.

use super::wire;
use macroonz_harness::oracle::archive::{
    ArchivedMethod, ArchivedVerdict, read_verdict, retain_transcript,
};
use macroonz_harness::oracle::{SpecifiedContext, TranscriptDerivation, TranscriptVerdict};
use macroonz_harness::report::archive::ArchiveLimits;

#[test]
fn transcript_retention_preserves_both_claims_and_agreement_absence() -> Result<(), ()> {
    let limits = ArchiveLimits::declared(512, 32);
    let context = SpecifiedContext::spelled(&["outside", "transcript"]).map_err(|_| ())?;
    let derivation = TranscriptDerivation::opened().framed_text("material");
    let derived = derivation.derived(&context);
    let independent = blake3::derive_key(
        "outside/transcript",
        b"\x00\x00\x00\x00\x00\x00\x00\x08material",
    );
    assert_eq!(derived.as_bytes(), &independent);
    let published = [17; 32];
    let verdict = derived.compared(&published);
    let mut payload = Vec::new();
    wire::frame(&independent, &mut payload);
    wire::frame(&published, &mut payload);
    let written = retain_transcript(verdict, limits).map_err(|_| ())?;
    assert_eq!(written.encoded(), wire::envelope(2, 1, &payload));
    let loaded =
        read_verdict(written.encoded(), ArchivedMethod::Transcript, limits).map_err(|_| ())?;
    let ArchivedVerdict::TranscriptDisagrees(found) = loaded.verdict() else {
        return Err(());
    };
    assert_eq!(found.rederived().as_bytes(), &independent);
    assert_eq!(found.published().as_bytes(), &published);

    assert_eq!(derived.compared(&independent), TranscriptVerdict::Agrees);
    let agreement = retain_transcript(derived.compared(&independent), limits).map_err(|_| ())?;
    assert_eq!(agreement.encoded(), wire::envelope(2, 0, &[]));
    assert_eq!(agreement.verdict(), &ArchivedVerdict::TranscriptAgrees);
    Ok(())
}
