use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for missing whitespace after keywords.
///
/// ## Why is this bad?
/// Missing whitespace after keywords makes the code harder to read.
///
/// ## Example
/// ```python
/// if(True):
///     pass
/// ```
///
/// Use instead:
/// ```python
/// if (True):
///     pass
/// ```
///
/// ## References
/// - [Python documentation: Keywords](https://docs.python.org/3/reference/lexical_analysis.html#keywords)
#[derive(ViolationMetadata)]
pub struct MissingWhitespaceAfterKeyword;

impl AlwaysFixableViolation for MissingWhitespaceAfterKeyword {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing whitespace after keyword".to_string()
    }

    fn fix_title(&self) -> String {
        "Added missing whitespace after keyword".to_string()
    }
}
