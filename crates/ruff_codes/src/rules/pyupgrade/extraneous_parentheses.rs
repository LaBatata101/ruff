use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for extraneous parentheses.
///
/// ## Why is this bad?
/// Extraneous parentheses are redundant, and can be removed to improve
/// readability while retaining identical semantics.
///
/// ## Example
/// ```python
/// print(("Hello, world"))
/// ```
///
/// Use instead:
/// ```python
/// print("Hello, world")
/// ```
#[derive(ViolationMetadata)]
pub struct ExtraneousParentheses;

impl AlwaysFixableViolation for ExtraneousParentheses {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Avoid extraneous parentheses".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove extraneous parentheses".to_string()
    }
}