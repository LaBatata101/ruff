use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for an exception that is not raised.
///
/// ## Why is this bad?
/// It's unnecessary to create an exception without raising it. For example,
/// `ValueError("...")` on its own will have no effect (unlike
/// `raise ValueError("...")`) and is likely a mistake.
///
/// ## Known problems
/// This rule only detects built-in exceptions, like `ValueError`, and does
/// not catch user-defined exceptions.
///
/// ## Example
/// ```python
/// ValueError("...")
/// ```
///
/// Use instead:
/// ```python
/// raise ValueError("...")
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as converting a useless exception
/// statement to a `raise` statement will change the program's behavior.
#[derive(ViolationMetadata)]
pub struct UselessExceptionStatement;

impl Violation for UselessExceptionStatement {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing `raise` statement on exception".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Add `raise` keyword".to_string())
    }
}