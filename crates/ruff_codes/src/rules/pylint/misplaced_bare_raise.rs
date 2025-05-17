use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for bare `raise` statements outside of exception handlers.
///
/// ## Why is this bad?
/// A bare `raise` statement without an exception object will re-raise the last
/// exception that was active in the current scope, and is typically used
/// within an exception handler to re-raise the caught exception.
///
/// If a bare `raise` is used outside of an exception handler, it will generate
/// an error due to the lack of an active exception.
///
/// Note that a bare `raise` within a  `finally` block will work in some cases
/// (namely, when the exception is raised within the `try` block), but should
/// be avoided as it can lead to confusing behavior.
///
/// ## Example
/// ```python
/// from typing import Any
///
///
/// def is_some(obj: Any) -> bool:
///     if obj is None:
///         raise
/// ```
///
/// Use instead:
/// ```python
/// from typing import Any
///
///
/// def is_some(obj: Any) -> bool:
///     if obj is None:
///         raise ValueError("`obj` cannot be `None`")
/// ```
#[derive(ViolationMetadata)]
pub struct MisplacedBareRaise;

impl Violation for MisplacedBareRaise {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Bare `raise` statement is not inside an exception handler".to_string()
    }
}