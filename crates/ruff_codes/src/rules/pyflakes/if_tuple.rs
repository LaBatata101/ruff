use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `if` statements that use non-empty tuples as test conditions.
///
/// ## Why is this bad?
/// Non-empty tuples are always `True`, so an `if` statement with a non-empty
/// tuple as its test condition will always pass. This is likely a mistake.
///
/// ## Example
/// ```python
/// if (False,):
///     print("This will always run")
/// ```
///
/// Use instead:
/// ```python
/// if False:
///     print("This will never run")
/// ```
///
/// ## References
/// - [Python documentation: The `if` statement](https://docs.python.org/3/reference/compound_stmts.html#the-if-statement)
#[derive(ViolationMetadata)]
pub struct IfTuple;

impl Violation for IfTuple {
    #[derive_message_formats]
    fn message(&self) -> String {
        "If test is a tuple, which is always `True`".to_string()
    }
}