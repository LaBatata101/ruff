use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for functions that end with an unnecessary `return` or
/// `return None`, and contain no other `return` statements.
///
/// ## Why is this bad?
/// Python implicitly assumes a `None` return at the end of a function, making
/// it unnecessary to explicitly write `return None`.
///
/// ## Example
/// ```python
/// def f():
///     print(5)
///     return None
/// ```
///
/// Use instead:
/// ```python
/// def f():
///     print(5)
/// ```
#[derive(ViolationMetadata)]
pub struct UselessReturn;

impl AlwaysFixableViolation for UselessReturn {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Useless `return` statement at end of function".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove useless `return` statement".to_string()
    }
}