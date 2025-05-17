use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of the builtin `exec` function.
///
/// ## Why is this bad?
/// The `exec()` function is insecure as it allows for arbitrary code
/// execution.
///
/// ## Example
/// ```python
/// exec("print('Hello World')")
/// ```
///
/// ## References
/// - [Python documentation: `exec`](https://docs.python.org/3/library/functions.html#exec)
/// - [Common Weakness Enumeration: CWE-78](https://cwe.mitre.org/data/definitions/78.html)
#[derive(ViolationMetadata)]
pub struct ExecBuiltin;

impl Violation for ExecBuiltin {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of `exec` detected".to_string()
    }
}