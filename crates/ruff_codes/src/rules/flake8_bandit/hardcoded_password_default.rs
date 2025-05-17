use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for potential uses of hardcoded passwords in function argument
/// defaults.
///
/// ## Why is this bad?
/// Including a hardcoded password in source code is a security risk, as an
/// attacker could discover the password and use it to gain unauthorized
/// access.
///
/// Instead, store passwords and other secrets in configuration files,
/// environment variables, or other sources that are excluded from version
/// control.
///
/// ## Example
///
/// ```python
/// def connect_to_server(password="hunter2"): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import os
///
///
/// def connect_to_server(password=os.environ["PASSWORD"]): ...
/// ```
///
/// ## References
/// - [Common Weakness Enumeration: CWE-259](https://cwe.mitre.org/data/definitions/259.html)
#[derive(ViolationMetadata)]
pub struct HardcodedPasswordDefault {
    name: String,
}

impl Violation for HardcodedPasswordDefault {
    #[derive_message_formats]
    fn message(&self) -> String {
        let HardcodedPasswordDefault { name } = self;
        format!(
            "Possible hardcoded password assigned to function default: \"{}\"",
            name.escape_debug()
        )
    }
}