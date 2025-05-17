use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for key-based dict accesses during `.items()` iterations.
///
/// ## Why is this bad?
/// When iterating over a dict via `.items()`, the current value is already
/// available alongside its key. Using the key to look up the value is
/// unnecessary.
///
/// ## Example
/// ```python
/// FRUITS = {"apple": 1, "orange": 10, "berry": 22}
///
/// for fruit_name, fruit_count in FRUITS.items():
///     print(FRUITS[fruit_name])
/// ```
///
/// Use instead:
/// ```python
/// FRUITS = {"apple": 1, "orange": 10, "berry": 22}
///
/// for fruit_name, fruit_count in FRUITS.items():
///     print(fruit_count)
/// ```
#[derive(ViolationMetadata)]
pub struct UnnecessaryDictIndexLookup;

impl AlwaysFixableViolation for UnnecessaryDictIndexLookup {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary lookup of dictionary value by key".to_string()
    }

    fn fix_title(&self) -> String {
        "Use existing variable".to_string()
    }
}