use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for function definitions with default arguments set to insecure SSL
/// and TLS protocol versions.
///
/// ## Why is this bad?
/// Several highly publicized exploitable flaws have been discovered in all
/// versions of SSL and early versions of TLS. The following versions are
/// considered insecure, and should be avoided:
/// - SSL v2
/// - SSL v3
/// - TLS v1
/// - TLS v1.1
///
/// ## Example
///
/// ```python
/// import ssl
///
///
/// def func(version=ssl.PROTOCOL_TLSv1): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import ssl
///
///
/// def func(version=ssl.PROTOCOL_TLSv1_2): ...
/// ```
#[derive(ViolationMetadata)]
pub struct SslWithBadDefaults {
    protocol: String,
}

impl Violation for SslWithBadDefaults {
    #[derive_message_formats]
    fn message(&self) -> String {
        let SslWithBadDefaults { protocol } = self;
        format!("Argument default set to insecure SSL protocol: `{protocol}`")
    }
}