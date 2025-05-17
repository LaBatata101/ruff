use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use super::common::CryptographicKey;

/// ## What it does
/// Checks for uses of cryptographic keys with vulnerable key sizes.
///
/// ## Why is this bad?
/// Small keys are easily breakable. For DSA and RSA, keys should be at least
/// 2048 bits long. For EC, keys should be at least 224 bits long.
///
/// ## Example
/// ```python
/// from cryptography.hazmat.primitives.asymmetric import dsa, ec
///
/// dsa.generate_private_key(key_size=512)
/// ec.generate_private_key(curve=ec.SECT163K1())
/// ```
///
/// Use instead:
/// ```python
/// from cryptography.hazmat.primitives.asymmetric import dsa, ec
///
/// dsa.generate_private_key(key_size=4096)
/// ec.generate_private_key(curve=ec.SECP384R1())
/// ```
///
/// ## References
/// - [CSRC: Transitioning the Use of Cryptographic Algorithms and Key Lengths](https://csrc.nist.gov/pubs/sp/800/131/a/r2/final)
#[derive(ViolationMetadata)]
pub struct WeakCryptographicKey {
    cryptographic_key: CryptographicKey,
}

impl Violation for WeakCryptographicKey {
    #[derive_message_formats]
    fn message(&self) -> String {
        let WeakCryptographicKey { cryptographic_key } = self;
        let minimum_key_size = cryptographic_key.minimum_key_size();
        format!(
            "{cryptographic_key} key sizes below {minimum_key_size} bits are considered breakable"
        )
    }
}