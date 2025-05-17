use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `raise` statements within `try` blocks. The only `raise`s
/// caught are those that throw exceptions caught by the `try` statement itself.
///
/// ## Why is this bad?
/// Raising and catching exceptions within the same `try` block is redundant,
/// as the code can be refactored to avoid the `try` block entirely.
///
/// Alternatively, the `raise` can be moved within an inner function, making
/// the exception reusable across multiple call sites.
///
/// ## Example
/// ```python
/// def bar():
///     pass
///
///
/// def foo():
///     try:
///         a = bar()
///         if not a:
///             raise ValueError
///     except ValueError:
///         raise
/// ```
///
/// Use instead:
/// ```python
/// def bar():
///     raise ValueError
///
///
/// def foo():
///     try:
///         a = bar()  # refactored `bar` to raise `ValueError`
///     except ValueError:
///         raise
/// ```
#[derive(ViolationMetadata)]
pub struct RaiseWithinTry;

impl Violation for RaiseWithinTry {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Abstract `raise` to an inner function".to_string()
    }
}