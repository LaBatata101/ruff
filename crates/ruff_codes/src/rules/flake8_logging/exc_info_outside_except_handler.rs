use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for logging calls with `exc_info=` outside exception handlers.
///
/// ## Why is this bad?
/// Using `exc_info=True` outside of an exception handler
/// attaches `None` as the exception information, leading to confusing messages:
///
/// ```pycon
/// >>> logging.warning("Uh oh", exc_info=True)
/// WARNING:root:Uh oh
/// NoneType: None
/// ```
///
/// ## Example
///
/// ```python
/// import logging
///
///
/// logging.warning("Foobar", exc_info=True)
/// ```
///
/// Use instead:
///
/// ```python
/// import logging
///
///
/// logging.warning("Foobar")
/// ```
///
/// ## Fix safety
/// The fix is always marked as unsafe, as it changes runtime behavior.
#[derive(ViolationMetadata)]
pub struct ExcInfoOutsideExceptHandler;

impl Violation for ExcInfoOutsideExceptHandler {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`exc_info=` outside exception handlers".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove `exc_info=`".to_string())
    }
}