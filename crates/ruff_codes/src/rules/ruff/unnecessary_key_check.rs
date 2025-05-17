use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary key checks prior to accessing a dictionary.
///
/// ## Why is this bad?
/// When working with dictionaries, the `get` can be used to access a value
/// without having to check if the dictionary contains the relevant key,
/// returning `None` if the key is not present.
///
/// ## Example
/// ```python
/// if "key" in dct and dct["key"]:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// if dct.get("key"):
///     ...
/// ```
#[derive(ViolationMetadata)]
pub struct UnnecessaryKeyCheck;

impl AlwaysFixableViolation for UnnecessaryKeyCheck {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary key check before dictionary access".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `dict.get`".to_string()
    }
}