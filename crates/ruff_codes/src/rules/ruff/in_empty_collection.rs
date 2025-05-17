use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for membership tests on empty collections (such as `list`, `tuple`, `set` or `dict`).
///
/// ## Why is this bad?
/// If the collection is always empty, the check is unnecessary, and can be removed.
///
/// ## Example
///
/// ```python
/// if 1 not in set():
///     print("got it!")
/// ```
///
/// Use instead:
///
/// ```python
/// print("got it!")
/// ```
#[derive(ViolationMetadata)]
pub struct InEmptyCollection;

impl Violation for InEmptyCollection {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary membership test on empty collection".to_string()
    }
}