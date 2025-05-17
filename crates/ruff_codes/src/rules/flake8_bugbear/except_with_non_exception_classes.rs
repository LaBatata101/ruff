use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for exception handlers that catch non-exception classes.
///
/// ## Why is this bad?
/// Catching classes that do not inherit from `BaseException` will raise a
/// `TypeError`.
///
/// ## Example
/// ```python
/// try:
///     1 / 0
/// except 1:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// try:
///     1 / 0
/// except ZeroDivisionError:
///     ...
/// ```
///
/// ## References
/// - [Python documentation: `except` clause](https://docs.python.org/3/reference/compound_stmts.html#except-clause)
/// - [Python documentation: Built-in Exceptions](https://docs.python.org/3/library/exceptions.html#built-in-exceptions)
#[derive(ViolationMetadata)]
pub struct ExceptWithNonExceptionClasses {
    is_star: bool,
}

impl Violation for ExceptWithNonExceptionClasses {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.is_star {
            "`except*` handlers should only be exception classes or tuples of exception classes"
                .to_string()
        } else {
            "`except` handlers should only be exception classes or tuples of exception classes"
                .to_string()
        }
    }
}