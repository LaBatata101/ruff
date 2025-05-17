use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for bidirectional unicode characters.
///
/// ## Why is this bad?
/// The interaction between bidirectional unicode characters and the
/// surrounding code can be surprising to those that are unfamiliar
/// with right-to-left writing systems.
///
/// In some cases, bidirectional unicode characters can also be used to
/// obfuscate code and introduce or mask security vulnerabilities.
///
/// ## Example
/// ```python
/// s = "א" * 100  #  "א" is assigned
/// print(s)  # prints a 100-character string
/// ```
///
/// ## References
/// - [PEP 672: Bidirectional Text](https://peps.python.org/pep-0672/#bidirectional-text)
#[derive(ViolationMetadata)]
pub struct BidirectionalUnicode;

impl Violation for BidirectionalUnicode {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Contains control characters that can permit obfuscated code".to_string()
    }
}