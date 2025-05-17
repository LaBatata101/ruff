use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `typing.Text`.
///
/// ## Why is this bad?
/// `typing.Text` is an alias for `str`, and only exists for Python 2
/// compatibility. As of Python 3.11, `typing.Text` is deprecated. Use `str`
/// instead.
///
/// ## Example
/// ```python
/// from typing import Text
///
/// foo: Text = "bar"
/// ```
///
/// Use instead:
/// ```python
/// foo: str = "bar"
/// ```
///
/// ## References
/// - [Python documentation: `typing.Text`](https://docs.python.org/3/library/typing.html#typing.Text)
#[derive(ViolationMetadata)]
pub struct TypingTextStrAlias;

impl Violation for TypingTextStrAlias {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`typing.Text` is deprecated, use `str`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `str`".to_string())
    }
}