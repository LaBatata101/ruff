use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `return` statements in `try`-`except` and `finally` blocks.
///
/// ## Why is this bad?
/// The `return` statement in a `finally` block will always be executed, even if
/// an exception is raised in the `try` or `except` block. This can lead to
/// unexpected behavior.
///
/// ## Example
/// ```python
/// def squared(n):
///     try:
///         sqr = n**2
///         return sqr
///     except Exception:
///         return "An exception occurred"
///     finally:
///         return -1  # Always returns -1.
/// ```
///
/// Use instead:
/// ```python
/// def squared(n):
///     try:
///         return_value = n**2
///     except Exception:
///         return_value = "An exception occurred"
///     finally:
///         return_value = -1
///     return return_value
/// ```
///
/// ## References
/// - [Python documentation: Defining Clean-up Actions](https://docs.python.org/3/tutorial/errors.html#defining-clean-up-actions)
#[derive(ViolationMetadata)]
pub struct ReturnInTryExceptFinally;

impl Violation for ReturnInTryExceptFinally {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Don't use `return` in `try`-`except` and `finally`".to_string()
    }
}