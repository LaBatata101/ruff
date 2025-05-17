use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `except` blocks that handle all exceptions, but are not the last
/// `except` block in a `try` statement.
///
/// ## Why is this bad?
/// When an exception is raised within a `try` block, the `except` blocks are
/// evaluated in order, and the first matching block is executed. If an `except`
/// block handles all exceptions, but isn't the last block, Python will raise a
/// `SyntaxError`, as the following blocks would never be executed.
///
/// ## Example
/// ```python
/// def reciprocal(n):
///     try:
///         reciprocal = 1 / n
///     except:
///         print("An exception occurred.")
///     except ZeroDivisionError:
///         print("Cannot divide by zero.")
///     else:
///         return reciprocal
/// ```
///
/// Use instead:
/// ```python
/// def reciprocal(n):
///     try:
///         reciprocal = 1 / n
///     except ZeroDivisionError:
///         print("Cannot divide by zero.")
///     except:
///         print("An exception occurred.")
///     else:
///         return reciprocal
/// ```
///
/// ## References
/// - [Python documentation: `except` clause](https://docs.python.org/3/reference/compound_stmts.html#except-clause)
#[derive(ViolationMetadata)]
pub struct DefaultExceptNotLast;

impl Violation for DefaultExceptNotLast {
    #[derive_message_formats]
    fn message(&self) -> String {
        "An `except` block as not the last exception handler".to_string()
    }
}