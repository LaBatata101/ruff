use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `logging.error` instead of `logging.exception` when
/// logging an exception.
///
/// ## Why is this bad?
/// `logging.exception` logs the exception and the traceback, while
/// `logging.error` only logs the exception. The former is more appropriate
/// when logging an exception, as the traceback is often useful for debugging.
///
/// ## Example
/// ```python
/// import logging
///
///
/// def func():
///     try:
///         raise NotImplementedError
///     except NotImplementedError:
///         logging.error("Exception occurred")
/// ```
///
/// Use instead:
/// ```python
/// import logging
///
///
/// def func():
///     try:
///         raise NotImplementedError
///     except NotImplementedError:
///         logging.exception("Exception occurred")
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as safe when run against `logging.error` calls,
/// but unsafe when marked against other logger-like calls (e.g.,
/// `logger.error`), since the rule is prone to false positives when detecting
/// logger-like calls outside of the `logging` module.
///
/// ## References
/// - [Python documentation: `logging.exception`](https://docs.python.org/3/library/logging.html#logging.exception)
#[derive(ViolationMetadata)]
pub struct ErrorInsteadOfException;

impl Violation for ErrorInsteadOfException {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `logging.exception` instead of `logging.error`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `exception`".to_string())
    }
}