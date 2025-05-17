use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for useless `if`-`else` conditions with identical arms.
///
/// ## Why is this bad?
/// Useless `if`-`else` conditions add unnecessary complexity to the code without
/// providing any logical benefit. Assigning the value directly is clearer.
///
/// ## Example
/// ```python
/// foo = x if y else x
/// ```
///
/// Use instead:
/// ```python
/// foo = x
/// ```
#[derive(ViolationMetadata)]
pub struct UselessIfElse;

impl Violation for UselessIfElse {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Useless `if`-`else` condition".to_string()
    }
}