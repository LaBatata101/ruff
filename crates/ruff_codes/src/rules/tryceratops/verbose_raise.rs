use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for needless exception names in `raise` statements.
///
/// ## Why is this bad?
/// It's redundant to specify the exception name in a `raise` statement if the
/// exception is being re-raised.
///
/// ## Example
/// ```python
/// def foo():
///     try:
///         ...
///     except ValueError as exc:
///         raise exc
/// ```
///
/// Use instead:
/// ```python
/// def foo():
///     try:
///         ...
///     except ValueError:
///         raise
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it doesn't properly handle bound
/// exceptions that are shadowed between the `except` and `raise` statements.
#[derive(ViolationMetadata)]
pub struct VerboseRaise;

impl AlwaysFixableViolation for VerboseRaise {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `raise` without specifying exception name".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove exception name".to_string()
    }
}