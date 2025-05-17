use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `raise` statements in exception handlers that lack a `from`
/// clause.
///
/// ## Why is this bad?
/// In Python, `raise` can be used with or without an exception from which the
/// current exception is derived. This is known as exception chaining. When
/// printing the stack trace, chained exceptions are displayed in such a way
/// so as make it easier to trace the exception back to its root cause.
///
/// When raising an exception from within an `except` clause, always include a
/// `from` clause to facilitate exception chaining. If the exception is not
/// chained, it will be difficult to trace the exception back to its root cause.
///
/// ## Example
/// ```python
/// try:
///     ...
/// except FileNotFoundError:
///     if ...:
///         raise RuntimeError("...")
///     else:
///         raise UserWarning("...")
/// ```
///
/// Use instead:
/// ```python
/// try:
///     ...
/// except FileNotFoundError as exc:
///     if ...:
///         raise RuntimeError("...") from None
///     else:
///         raise UserWarning("...") from exc
/// ```
///
/// ## References
/// - [Python documentation: `raise` statement](https://docs.python.org/3/reference/simple_stmts.html#the-raise-statement)
#[derive(ViolationMetadata)]
pub struct RaiseWithoutFromInsideExcept {
    is_star: bool,
}

impl Violation for RaiseWithoutFromInsideExcept {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.is_star {
            "Within an `except*` clause, raise exceptions with `raise ... from err` or `raise ... \
                 from None` to distinguish them from errors in exception handling"
                .to_string()
        } else {
            "Within an `except` clause, raise exceptions with `raise ... from err` or `raise ... \
                 from None` to distinguish them from errors in exception handling"
                .to_string()
        }
    }
}