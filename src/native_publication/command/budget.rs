use super::{BakePreparation, BakeToolBudget};
use crate::native_process::ProcessLimits;
use std::time::Duration;

pub(super) fn admit(
    preparation: &BakePreparation,
    files: usize,
    compiler: Option<ProcessLimits>,
) -> Result<(), String> {
    let formats = if preparation.formatter.is_some() {
        files.checked_add(1).ok_or("formatter count overflow")?
    } else {
        0
    };
    let selections = preparation
        .formatter
        .as_ref()
        .into_iter()
        .flat_map(|formatter| std::iter::repeat_n(formatter.tool.limits(), formats))
        .chain(compiler);
    let mut total = BakeToolBudget {
        runs: 0,
        time: Duration::ZERO,
        capture_bytes: 0,
    };
    for selection in selections {
        total.runs = total.runs.checked_add(1).ok_or("process count overflow")?;
        total.time = total
            .time
            .checked_add(selection.execution())
            .and_then(|time| time.checked_add(selection.cleanup()))
            .ok_or("process time overflow")?;
        total.capture_bytes = total
            .capture_bytes
            .checked_add(selection.stdout())
            .and_then(|bytes| bytes.checked_add(selection.stderr()))
            .ok_or("capture bound overflow")?;
    }
    if total.runs > preparation.tools.runs
        || total.time > preparation.tools.time
        || total.capture_bytes > preparation.tools.capture_bytes
    {
        return Err(
            "the complete selected process family exceeds its declared command allowance"
                .to_owned(),
        );
    }
    Ok(())
}
