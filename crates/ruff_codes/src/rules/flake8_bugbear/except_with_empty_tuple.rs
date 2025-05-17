use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for exception handlers that catch an empty tuple.
///
/// ## Why is this bad?
/// An exception handler that catches an empty tuple will not catch anything,
/// and is indicative of a mistake. Instead, add exceptions to the `except`
/// clause.
///
/// ## Example
/// ```python
/// try:
///     1 / 0
/// except ():
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
#[derive(ViolationMetadata)]
pub struct ExceptWithEmptyTuple {
    is_star: bool,
}

impl Violation for ExceptWithEmptyTuple {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.is_star {
            "Using `except* ():` with an empty tuple does not catch anything; add exceptions to handle".to_string()
        } else {
            "Using `except ():` with an empty tuple does not catch anything; add exceptions to handle".to_string()
        }
    }
}