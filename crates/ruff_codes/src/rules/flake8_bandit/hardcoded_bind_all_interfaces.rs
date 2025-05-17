use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for hardcoded bindings to all network interfaces (`0.0.0.0`).
///
/// ## Why is this bad?
/// Binding to all network interfaces is insecure as it allows access from
/// unintended interfaces, which may be poorly secured or unauthorized.
///
/// Instead, bind to specific interfaces.
///
/// ## Example
/// ```python
/// ALLOWED_HOSTS = ["0.0.0.0"]
/// ```
///
/// Use instead:
/// ```python
/// ALLOWED_HOSTS = ["127.0.0.1", "localhost"]
/// ```
///
/// ## References
/// - [Common Weakness Enumeration: CWE-200](https://cwe.mitre.org/data/definitions/200.html)
#[derive(ViolationMetadata)]
pub struct HardcodedBindAllInterfaces;

impl Violation for HardcodedBindAllInterfaces {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Possible binding to all interfaces".to_string()
    }
}