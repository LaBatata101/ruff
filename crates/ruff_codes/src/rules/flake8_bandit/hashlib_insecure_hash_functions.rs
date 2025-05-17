use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of weak or broken cryptographic hash functions in
/// `hashlib` and `crypt` libraries.
///
/// ## Why is this bad?
/// Weak or broken cryptographic hash functions may be susceptible to
/// collision attacks (where two different inputs produce the same hash) or
/// pre-image attacks (where an attacker can find an input that produces a
/// given hash). This can lead to security vulnerabilities in applications
/// that rely on these hash functions.
///
/// Avoid using weak or broken cryptographic hash functions in security
/// contexts. Instead, use a known secure hash function such as SHA256.
///
/// ## Example
/// ```python
/// import hashlib
///
///
/// def certificate_is_valid(certificate: bytes, known_hash: str) -> bool:
///     hash = hashlib.md5(certificate).hexdigest()
///     return hash == known_hash
/// ```
///
/// Use instead:
/// ```python
/// import hashlib
///
///
/// def certificate_is_valid(certificate: bytes, known_hash: str) -> bool:
///     hash = hashlib.sha256(certificate).hexdigest()
///     return hash == known_hash
/// ```
///
/// or add `usedforsecurity=False` if the hashing algorithm is not used in a security context, e.g.
/// as a non-cryptographic one-way compression function:
/// ```python
/// import hashlib
///
///
/// def certificate_is_valid(certificate: bytes, known_hash: str) -> bool:
///     hash = hashlib.md5(certificate, usedforsecurity=False).hexdigest()
///     return hash == known_hash
/// ```
///
///
/// ## References
/// - [Python documentation: `hashlib` — Secure hashes and message digests](https://docs.python.org/3/library/hashlib.html)
/// - [Python documentation: `crypt` — Function to check Unix passwords](https://docs.python.org/3/library/crypt.html)
/// - [Python documentation: `FIPS` - FIPS compliant hashlib implementation](https://docs.python.org/3/library/hashlib.html#hashlib.algorithms_guaranteed)
/// - [Common Weakness Enumeration: CWE-327](https://cwe.mitre.org/data/definitions/327.html)
/// - [Common Weakness Enumeration: CWE-328](https://cwe.mitre.org/data/definitions/328.html)
/// - [Common Weakness Enumeration: CWE-916](https://cwe.mitre.org/data/definitions/916.html)
#[derive(ViolationMetadata)]
pub struct HashlibInsecureHashFunction {
    library: String,
    string: String,
}

impl Violation for HashlibInsecureHashFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        let HashlibInsecureHashFunction { library, string } = self;
        format!("Probable use of insecure hash functions in `{library}`: `{string}`")
    }
}