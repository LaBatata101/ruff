use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for calls to `ssl.wrap_socket()` without an `ssl_version`.
///
/// ## Why is this bad?
/// This method is known to provide a default value that maximizes
/// compatibility, but permits use of insecure protocols.
///
/// ## Example
/// ```python
/// import ssl
///
/// ssl.wrap_socket()
/// ```
///
/// Use instead:
/// ```python
/// import ssl
///
/// ssl.wrap_socket(ssl_version=ssl.PROTOCOL_TLSv1_2)
/// ```
#[derive(ViolationMetadata)]
pub struct SslWithNoVersion;

impl Violation for SslWithNoVersion {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`ssl.wrap_socket` called without an `ssl_version``".to_string()
    }
}