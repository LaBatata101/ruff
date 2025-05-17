use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `os.getenv` calls with an invalid `key` argument.
///
/// ## Why is this bad?
/// `os.getenv` only supports strings as the first argument (`key`).
///
/// If the provided argument is not a string, `os.getenv` will throw a
/// `TypeError` at runtime.
///
/// ## Example
/// ```python
/// os.getenv(1)
/// ```
///
/// Use instead:
/// ```python
/// os.getenv("1")
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidEnvvarValue;

impl Violation for InvalidEnvvarValue {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Invalid type for initial `os.getenv` argument; expected `str`".to_string()
    }
}