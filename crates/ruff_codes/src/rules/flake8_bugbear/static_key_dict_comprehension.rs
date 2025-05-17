use ruff_diagnostics::Violation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for dictionary comprehensions that use a static key, like a string
/// literal or a variable defined outside the comprehension.
///
/// ## Why is this bad?
/// Using a static key (like a string literal) in a dictionary comprehension
/// is usually a mistake, as it will result in a dictionary with only one key,
/// despite the comprehension iterating over multiple values.
///
/// ## Example
/// ```python
/// data = ["some", "Data"]
/// {"key": value.upper() for value in data}
/// ```
///
/// Use instead:
/// ```python
/// data = ["some", "Data"]
/// {value: value.upper() for value in data}
/// ```
#[derive(ViolationMetadata)]
pub struct StaticKeyDictComprehension {
    key: SourceCodeSnippet,
}

impl Violation for StaticKeyDictComprehension {
    #[derive_message_formats]
    fn message(&self) -> String {
        if let Some(key) = self.key.full_display() {
            format!("Dictionary comprehension uses static key: `{key}`")
        } else {
            "Dictionary comprehension uses static key".to_string()
        }
    }
}