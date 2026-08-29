//! Closed public-path mapping and deterministic source rendering for type-separation challenges.

use macroonz_harness::depot::types::SwapPair;
use macroonz_harness::oracle::{
    DiagnosticAnchor, PrimarySourceSpan, RelativeSourcePath, RustcErrorCode, SourcePosition,
};

/// One rendered compiler input.
pub(super) struct RenderedSource {
    pub(super) bin_name: String,
    pub(super) file_name: String,
    pub(super) source: String,
    pub(super) locus: PrimarySourceSpan,
}

/// The lawful and hostile sources for one directional row.
pub(super) struct Challenge {
    pub(super) ordinal: usize,
    pub(super) seat: &'static str,
    pub(super) substitute: &'static str,
    pub(super) lawful: RenderedSource,
    pub(super) hostile: RenderedSource,
    pub(super) expected_hostile_refusal: DiagnosticAnchor,
}

impl Challenge {
    pub(super) fn for_pair(ordinal: usize, pair: SwapPair) -> Result<Self, String> {
        let seat_path = public_path(pair.seat)?;
        let substitute_path = public_path(pair.substitute)?;
        let lawful = render(ordinal, "lawful", seat_path, seat_path, "seat")?;
        let hostile = render(ordinal, "hostile", seat_path, substitute_path, "substitute")?;
        let code = RustcErrorCode::informed("E0308")
            .map_err(|refusal| format!("declared mismatch code was refused: {refusal:?}"))?;
        let expected_hostile_refusal = DiagnosticAnchor::at(code, hostile.locus.clone());
        Ok(Self {
            ordinal,
            seat: pair.seat,
            substitute: pair.substitute,
            lawful,
            hostile,
            expected_hostile_refusal,
        })
    }
}

fn public_path(name: &str) -> Result<&'static str, String> {
    match name {
        "TrialId" => Ok("macroonz_harness::report::TrialId"),
        "TrialSite" => Ok("macroonz_harness::report::TrialSite"),
        "ExecutionKey" => Ok("macroonz_harness::report::ExecutionKey"),
        "RowRevisionId" => Ok("macroonz_harness::report::RowRevisionId"),
        "CheckRevisionId" => Ok("macroonz_harness::report::CheckRevisionId"),
        "SubjectRevisionId" => Ok("macroonz_harness::report::SubjectRevisionId"),
        "ProposalId" => Ok("macroonz_harness::descriptor::ProposalId"),
        "StoredProposalRef" => Ok("macroonz_harness::muterprater::StoredProposalRef"),
        "Fingerprint" => Ok("macroonz_harness::report::Fingerprint"),
        "ReplayRef" => Ok("macroonz_harness::descriptor::ReplayRef"),
        unknown => Err(format!(
            "the swap-pair bank named unmapped public type {unknown}"
        )),
    }
}

fn render(
    ordinal: usize,
    posture: &str,
    seat_path: &str,
    offered_path: &str,
    offered_name: &str,
) -> Result<RenderedSource, String> {
    let bin_name = format!("{posture}-{ordinal}");
    let file_name = format!("{bin_name}.rs");
    let call = format!("    require({offered_name});");
    let column_start = call
        .find(offered_name)
        .and_then(|column| column.checked_add(1))
        .ok_or_else(|| "the rendered call lost its offered value".to_owned())?;
    let column_end = column_start
        .checked_add(offered_name.len())
        .ok_or_else(|| "the rendered call column overflowed".to_owned())?;
    let lines = [
        format!("pub fn require(_: {seat_path}) {{}}"),
        String::new(),
        format!("pub fn challenge({offered_name}: {offered_path}) {{"),
        call,
        "}".to_owned(),
        String::new(),
        "fn main() {}".to_owned(),
    ];
    let source = lines.join("\n") + "\n";

    let relative = RelativeSourcePath::informed(&format!("src/bin/{file_name}"))
        .map_err(|refusal| format!("rendered source path was refused: {refusal:?}"))?;
    let start = SourcePosition::informed(
        4u64,
        u64::try_from(column_start)
            .map_err(|error| format!("rendered start column does not fit u64: {error}"))?,
    )
    .map_err(|refusal| format!("rendered start position was refused: {refusal:?}"))?;
    let end = SourcePosition::informed(
        4u64,
        u64::try_from(column_end)
            .map_err(|error| format!("rendered end column does not fit u64: {error}"))?,
    )
    .map_err(|refusal| format!("rendered end position was refused: {refusal:?}"))?;
    let primary = PrimarySourceSpan::informed(relative, start, end)
        .map_err(|refusal| format!("rendered primary span was refused: {refusal:?}"))?;
    Ok(RenderedSource {
        bin_name,
        file_name,
        source,
        locus: primary,
    })
}
