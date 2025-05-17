use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for index-based list accesses during `enumerate` iterations.
///
/// ## Why is this bad?
/// When iterating over a list with `enumerate`, the current item is already
/// available alongside its index. Using the index to look up the item is
/// unnecessary.
///
/// ## Example
/// ```python
/// letters = ["a", "b", "c"]
///
/// for index, letter in enumerate(letters):
///     print(letters[index])
/// ```
///
/// Use instead:
/// ```python
/// letters = ["a", "b", "c"]
///
/// for index, letter in enumerate(letters):
///     print(letter)
/// ```
#[derive(ViolationMetadata)]
pub struct UnnecessaryListIndexLookup;

impl AlwaysFixableViolation for UnnecessaryListIndexLookup {
    #[derive_message_formats]
    fn message(&self) -> String {
        "List index lookup in `enumerate()` loop".to_string()
    }

    fn fix_title(&self) -> String {
        "Use the loop variable directly".to_string()
    }
}