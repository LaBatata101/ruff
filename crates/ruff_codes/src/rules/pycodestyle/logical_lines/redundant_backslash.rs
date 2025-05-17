use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for redundant backslashes between brackets.
///
/// ## Why is this bad?
/// Explicit line joins using a backslash are redundant between brackets.
///
/// ## Example
/// ```python
/// x = (2 + \
///     2)
/// ```
///
/// Use instead:
/// ```python
/// x = (2 +
///     2)
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#maximum-line-length
#[derive(ViolationMetadata)]
pub struct RedundantBackslash;

impl AlwaysFixableViolation for RedundantBackslash {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Redundant backslash".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove redundant backslash".to_string()
    }
}
