use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for HTTPS requests that disable SSL certificate checks.
///
/// ## Why is this bad?
/// If SSL certificates are not verified, an attacker could perform a "man in
/// the middle" attack by intercepting and modifying traffic between the client
/// and server.
///
/// ## Example
/// ```python
/// import requests
///
/// requests.get("https://www.example.com", verify=False)
/// ```
///
/// Use instead:
/// ```python
/// import requests
///
/// requests.get("https://www.example.com")  # By default, `verify=True`.
/// ```
///
/// ## References
/// - [Common Weakness Enumeration: CWE-295](https://cwe.mitre.org/data/definitions/295.html)
#[derive(ViolationMetadata)]
pub struct RequestWithNoCertValidation {
    string: String,
}

impl Violation for RequestWithNoCertValidation {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RequestWithNoCertValidation { string } = self;
        format!(
            "Probable use of `{string}` call with `verify=False` disabling SSL certificate checks"
        )
    }
}