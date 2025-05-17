use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `return` statements in `try` blocks.
///
/// ## Why is this bad?
/// The `try`-`except` statement has an `else` clause for code that should
/// run _only_ if no exceptions were raised. Returns in `try` blocks may
/// exhibit confusing or unwanted behavior, such as being overridden by
/// control flow in `except` and `finally` blocks, or unintentionally
/// suppressing an exception.
///
/// ## Example
/// ```python
/// import logging
///
///
/// def reciprocal(n):
///     try:
///         rec = 1 / n
///         print(f"reciprocal of {n} is {rec}")
///         return rec
///     except ZeroDivisionError:
///         logging.exception("Exception occurred")
/// ```
///
/// Use instead:
/// ```python
/// import logging
///
///
/// def reciprocal(n):
///     try:
///         rec = 1 / n
///     except ZeroDivisionError:
///         logging.exception("Exception occurred")
///     else:
///         print(f"reciprocal of {n} is {rec}")
///         return rec
/// ```
///
/// ## References
/// - [Python documentation: Errors and Exceptions](https://docs.python.org/3/tutorial/errors.html)
#[derive(ViolationMetadata)]
pub struct TryConsiderElse;

impl Violation for TryConsiderElse {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Consider moving this statement to an `else` block".to_string()
    }
}