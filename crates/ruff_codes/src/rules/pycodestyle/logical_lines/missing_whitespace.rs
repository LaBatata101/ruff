use ruff_diagnostics::{AlwaysFixableViolation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_parser::TokenKind;

/// ## What it does
/// Checks for missing whitespace after `,`, `;`, and `:`.
///
/// ## Why is this bad?
/// Missing whitespace after `,`, `;`, and `:` makes the code harder to read.
///
/// ## Example
/// ```python
/// a = (1,2)
/// ```
///
/// Use instead:
/// ```python
/// a = (1, 2)
/// ```
#[derive(ViolationMetadata)]
pub struct MissingWhitespace {
    token: TokenKind,
}

impl AlwaysFixableViolation for MissingWhitespace {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Missing whitespace after {}", self.token)
    }

    fn fix_title(&self) -> String {
        "Add missing whitespace".to_string()
    }
}