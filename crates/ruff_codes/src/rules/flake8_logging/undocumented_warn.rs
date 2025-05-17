use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `logging.WARN`.
///
/// ## Why is this bad?
/// The `logging.WARN` constant is an undocumented alias for `logging.WARNING`.
///
/// Although it’s not explicitly deprecated, `logging.WARN` is not mentioned
/// in the `logging` documentation. Prefer `logging.WARNING` instead.
///
/// ## Example
/// ```python
/// import logging
///
///
/// logging.basicConfig(level=logging.WARN)
/// ```
///
/// Use instead:
/// ```python
/// import logging
///
///
/// logging.basicConfig(level=logging.WARNING)
/// ```
#[derive(ViolationMetadata)]
pub struct UndocumentedWarn;

impl Violation for UndocumentedWarn {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of undocumented `logging.WARN` constant".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace `logging.WARN` with `logging.WARNING`".to_string())
    }
}