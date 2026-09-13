# Formatter execution

This home turns an admitted publication file into separately retained physical source through an explicitly selected rustfmt process.
`Formatter::qualified` executes `--version` through the native process owner and admits the `rustfmt 1.9.0-stable` profile.
The complete query observation remains available, including its exact build identifier.
An unsuccessful or unsupported query retains its actual command and finished output or pending cleanup resources.

The caller supplies an absolute regular configuration file, bounded to 65,536 bytes.
Formatting selects that file explicitly, Rust edition 2024, style edition 2024, Unix newlines, stdout emission and no color.
Configuration bytes are captured during qualification and compared before and after each file's execution.
The working directory and environment come only from the declared `ProcessTool`.
The formatter may interpret the selected configuration; this home does not infer configuration from an ambient parent directory.

`format` consumes an explicitly supplied disposable regular read/write file for stdin.
It truncates and writes that file with the inventory's actual source, rewinds it and hands it to the native process owner.
Execution, capture bounds, deadlines and retryable cleanup remain owned by the [native process owner](../../native_process/README.md).
Pending cleanup cannot supply formatted source.
Finished output retains the original source, canonical token commitment, declared destination, exact invocation, observed version, configuration bytes and every captured process result.

Admitted physical source requires successful ordinary exit, complete stdout and stderr capture, UTF-8 text, LF line endings with a terminal newline, no NUL and unchanged configuration.
The physical digest commits to exactly those stdout bytes through the [inventory owner](../inventory/README.md).
No normalization is applied after formatting, and the original canonical token commitment is retained unchanged.
Formatter diagnostics and failures remain inspectable and do not establish publishable text.

The selected executable, configuration and scratch file are caller trust inputs.
Before/after equality does not authenticate concurrent replacement, and the version text does not authenticate an executable.
Successful formatting establishes neither semantic equivalence nor compilability nor filesystem installation.
