use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for undefined local variables.
///
/// ## Why is this bad?
/// Referencing a local variable before it has been assigned will raise
/// an `UnboundLocalError` at runtime.
///
/// ## Example
/// ```python
/// x = 1
///
///
/// def foo():
///     x += 1
/// ```
///
/// Use instead:
/// ```python
/// x = 1
///
///
/// def foo():
///     global x
///     x += 1
/// ```
#[derive(ViolationMetadata)]
pub struct UndefinedLocal {
    name: String,
}

impl Violation for UndefinedLocal {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UndefinedLocal { name } = self;
        format!("Local variable `{name}` referenced before assignment")
    }
}