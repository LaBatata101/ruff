use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of the SNMPv3 protocol without encryption.
///
/// ## Why is this bad?
/// Unencrypted SNMPv3 communication can be intercepted and read by
/// unauthorized parties. Instead, enable encryption when using SNMPv3.
///
/// ## Example
/// ```python
/// from pysnmp.hlapi import UsmUserData
///
/// UsmUserData("user")
/// ```
///
/// Use instead:
/// ```python
/// from pysnmp.hlapi import UsmUserData
///
/// UsmUserData("user", "authkey", "privkey")
/// ```
///
/// ## References
/// - [Common Weakness Enumeration: CWE-319](https://cwe.mitre.org/data/definitions/319.html)
#[derive(ViolationMetadata)]
pub struct SnmpWeakCryptography;

impl Violation for SnmpWeakCryptography {
    #[derive_message_formats]
    fn message(&self) -> String {
        "You should not use SNMPv3 without encryption. `noAuthNoPriv` & `authNoPriv` is insecure."
            .to_string()
    }
}