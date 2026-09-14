use super::types::FormatContext;
use super::{FormatError, FormatOutput, FormatRun, Formatter, PendingFormat};
use crate::native_process::{ProcessRun, ProcessTool};
use std::fs::File;
use std::io::{Seek, Write};
use std::path::PathBuf;

pub(super) fn qualify(tool: &ProcessTool, config: PathBuf) -> Result<Formatter, FormatError> {
    if !config.is_absolute() {
        return Err(FormatError::Configuration(
            "an explicit absolute rustfmt configuration file is required".to_owned(),
        ));
    }
    let spelling = config.to_str().ok_or_else(|| {
        FormatError::Configuration("configuration path is not Unicode".to_owned())
    })?;
    let configuration = super::observe::configuration(&config)?;
    let process = tool
        .invocation(
            [
                "--emit",
                "stdout",
                "--edition",
                "2024",
                "--style-edition",
                "2024",
                "--config-path",
                spelling,
                "--config",
                "newline_style=Unix",
                "--color",
                "never",
            ]
            .map(str::to_owned)
            .to_vec(),
        )
        .map_err(FormatError::Process)?;
    let query = tool
        .invocation(vec!["--version".to_owned()])
        .map_err(FormatError::Process)?;
    let run = crate::native_process::run(&query, None).map_err(FormatError::Process)?;
    match run {
        ProcessRun::Finished(version) if super::observe::supported(&version) => Ok(Formatter {
            process,
            query,
            version,
            config,
            configuration,
        }),
        refused @ (ProcessRun::Finished(_) | ProcessRun::Pending(_)) => Err(FormatError::Query {
            request: Box::new(query),
            run: Box::new(refused),
        }),
    }
}

pub(super) fn format(context: FormatContext, mut scratch: File) -> Result<FormatRun, FormatError> {
    if super::observe::configuration(&context.config)? != context.configuration {
        return Err(FormatError::Configuration(
            "rustfmt configuration differs from qualification".to_owned(),
        ));
    }
    if !scratch
        .metadata()
        .map_err(FormatError::Filesystem)?
        .is_file()
    {
        return Err(FormatError::Configuration(
            "formatter stdin must be a disposable regular read/write file".to_owned(),
        ));
    }
    scratch.set_len(0).map_err(FormatError::Filesystem)?;
    scratch.rewind().map_err(FormatError::Filesystem)?;
    scratch
        .write_all(context.source.as_bytes())
        .map_err(FormatError::Filesystem)?;
    scratch.rewind().map_err(FormatError::Filesystem)?;
    let run = crate::native_process::run(&context.request, Some(scratch))
        .map_err(FormatError::Process)?;
    Ok(finish(context, run))
}

pub(super) fn finish(context: FormatContext, run: ProcessRun) -> FormatRun {
    match run {
        ProcessRun::Finished(process) => {
            let observation = super::observe::text(&context, &process);
            FormatRun::Finished(Box::new(FormatOutput {
                context,
                process,
                observation,
            }))
        }
        ProcessRun::Pending(process) => {
            FormatRun::Pending(Box::new(PendingFormat { context, process }))
        }
    }
}
