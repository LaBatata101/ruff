use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of `.digest().hex()` on a hashlib hash, like `sha512`.
///
/// ## Why is this bad?
/// When generating a hex digest from a hash, it's preferable to use the
/// `.hexdigest()` method, rather than calling `.digest()` and then `.hex()`,
/// as the former is more concise and readable.
///
/// ## Example
/// ```python
/// from hashlib import sha512
///
/// hashed = sha512(b"some data").digest().hex()
/// ```
///
/// Use instead:
/// ```python
/// from hashlib import sha512
///
/// hashed = sha512(b"some data").hexdigest()
/// ```
///
/// ## References
/// - [Python documentation: `hashlib`](https://docs.python.org/3/library/hashlib.html)
#[derive(ViolationMetadata)]
pub struct HashlibDigestHex;

impl Violation for HashlibDigestHex {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of hashlib's `.digest().hex()`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `.hexdigest()`".to_string())
    }
}