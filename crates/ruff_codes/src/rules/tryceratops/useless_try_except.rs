use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for immediate uses of `raise` within exception handlers.
///
/// ## Why is this bad?
/// Capturing an exception, only to immediately reraise it, has no effect.
/// Instead, remove the error-handling code and let the exception propagate
/// upwards without the unnecessary `try`-`except` block.
///
/// ## Example
/// ```python
/// def foo():
///     try:
///         bar()
///     except NotImplementedError:
///         raise
/// ```
///
/// Use instead:
/// ```python
/// def foo():
///     bar()
/// ```
#[derive(ViolationMetadata)]
pub struct UselessTryExcept;

impl Violation for UselessTryExcept {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Remove exception handler; error is immediately re-raised".to_string()
    }
}