use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for class definitions that include unnecessary parentheses after
/// the class name.
///
/// ## Why is this bad?
/// If a class definition doesn't have any bases, the parentheses are
/// unnecessary.
///
/// ## Example
/// ```python
/// class Foo():
///     ...
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     ...
/// ```
#[derive(ViolationMetadata)]
pub struct UnnecessaryClassParentheses;

impl AlwaysFixableViolation for UnnecessaryClassParentheses {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary parentheses after class definition".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove parentheses".to_string()
    }
}