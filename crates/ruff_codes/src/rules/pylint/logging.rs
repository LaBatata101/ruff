use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for too few positional arguments for a `logging` format string.
///
/// ## Why is this bad?
/// A `TypeError` will be raised if the statement is run.
///
/// ## Example
/// ```python
/// import logging
///
/// try:
///     function()
/// except Exception as e:
///     logging.error("%s error occurred: %s", e)
///     raise
/// ```
///
/// Use instead:
/// ```python
/// import logging
///
/// try:
///     function()
/// except Exception as e:
///     logging.error("%s error occurred: %s", type(e), e)
///     raise
/// ```
#[derive(ViolationMetadata)]
pub struct LoggingTooFewArgs;

impl Violation for LoggingTooFewArgs {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Not enough arguments for `logging` format string".to_string()
    }
}

/// ## What it does
/// Checks for too many positional arguments for a `logging` format string.
///
/// ## Why is this bad?
/// A `TypeError` will be raised if the statement is run.
///
/// ## Example
/// ```python
/// import logging
///
/// try:
///     function()
/// except Exception as e:
///     logging.error("Error occurred: %s", type(e), e)
///     raise
/// ```
///
/// Use instead:
/// ```python
/// import logging
///
/// try:
///     function()
/// except Exception as e:
///     logging.error("%s error occurred: %s", type(e), e)
///     raise
/// ```
#[derive(ViolationMetadata)]
pub struct LoggingTooManyArgs;

impl Violation for LoggingTooManyArgs {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Too many arguments for `logging` format string".to_string()
    }
}
