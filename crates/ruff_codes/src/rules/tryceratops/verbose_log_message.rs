use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for excessive logging of exception objects.
///
/// ## Why is this bad?
/// When logging exceptions via `logging.exception`, the exception object
/// is logged automatically. Including the exception object in the log
/// message is redundant and can lead to excessive logging.
///
/// ## Example
/// ```python
/// try:
///     ...
/// except ValueError as e:
///     logger.exception(f"Found an error: {e}")
/// ```
///
/// Use instead:
/// ```python
/// try:
///     ...
/// except ValueError:
///     logger.exception("Found an error")
/// ```
#[derive(ViolationMetadata)]
pub struct VerboseLogMessage;

impl Violation for VerboseLogMessage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Redundant exception object included in `logging.exception` call".to_string()
    }
}