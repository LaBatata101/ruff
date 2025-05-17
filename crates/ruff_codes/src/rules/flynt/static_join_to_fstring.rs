use ruff_diagnostics::AlwaysFixableViolation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `str.join` calls that can be replaced with f-strings.
///
/// ## Why is this bad?
/// f-strings are more readable and generally preferred over `str.join` calls.
///
/// ## Example
/// ```python
/// " ".join((foo, bar))
/// ```
///
/// Use instead:
/// ```python
/// f"{foo} {bar}"
/// ```
///
/// # Fix safety
/// The fix is always marked unsafe because the evaluation of the f-string
/// expressions will default to calling the `__format__` method of each
/// object, whereas `str.join` expects each object to be an instance of
/// `str` and uses the corresponding string. Therefore it is possible for
/// the values of the resulting strings to differ, or for one expression
/// to raise an exception while the other does not.
///
/// ## References
/// - [Python documentation: f-strings](https://docs.python.org/3/reference/lexical_analysis.html#f-strings)
#[derive(ViolationMetadata)]
pub struct StaticJoinToFString {
    expression: SourceCodeSnippet,
}

impl AlwaysFixableViolation for StaticJoinToFString {
    #[derive_message_formats]
    fn message(&self) -> String {
        let StaticJoinToFString { expression } = self;
        if let Some(expression) = expression.full_display() {
            format!("Consider `{expression}` instead of string join")
        } else {
            "Consider f-string instead of string join".to_string()
        }
    }

    fn fix_title(&self) -> String {
        let StaticJoinToFString { expression } = self;
        if let Some(expression) = expression.full_display() {
            format!("Replace with `{expression}`")
        } else {
            "Replace with f-string".to_string()
        }
    }
}