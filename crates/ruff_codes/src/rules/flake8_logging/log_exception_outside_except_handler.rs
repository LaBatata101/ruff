use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `.exception()` logging calls outside of exception handlers.
///
/// ## Why is this bad?
/// [The documentation] states:
/// > This function should only be called from an exception handler.
///
/// Calling `.exception()` outside of an exception handler
/// attaches `None` as exception information, leading to confusing messages:
///
/// ```pycon
/// >>> logging.exception("example")
/// ERROR:root:example
/// NoneType: None
/// ```
///
/// ## Example
///
/// ```python
/// import logging
///
/// logging.exception("Foobar")
/// ```
///
/// Use instead:
///
/// ```python
/// import logging
///
/// logging.error("Foobar")
/// ```
///
/// ## Fix safety
/// The fix, if available, will always be marked as unsafe, as it changes runtime behavior.
///
/// [The documentation]: https://docs.python.org/3/library/logging.html#logging.exception
#[derive(ViolationMetadata)]
pub struct LogExceptionOutsideExceptHandler;

impl Violation for LogExceptionOutsideExceptHandler {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`.exception()` call outside exception handlers".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `.error()`".to_string())
    }
}