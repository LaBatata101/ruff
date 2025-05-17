use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for three or more consecutive if-statements with direct returns
///
/// ## Why is this bad?
/// These can be simplified by using a dictionary
///
/// ## Example
/// ```python
/// if x == 1:
///     return "Hello"
/// elif x == 2:
///     return "Goodbye"
/// else:
///     return "Goodnight"
/// ```
///
/// Use instead:
/// ```python
/// return {1: "Hello", 2: "Goodbye"}.get(x, "Goodnight")
/// ```
#[derive(ViolationMetadata)]
pub struct IfElseBlockInsteadOfDictLookup;

impl Violation for IfElseBlockInsteadOfDictLookup {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use a dictionary instead of consecutive `if` statements".to_string()
    }
}