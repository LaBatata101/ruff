use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `os.getenv` calls with invalid default values.
///
/// ## Why is this bad?
/// If an environment variable is set, `os.getenv` will return its value as
/// a string. If the environment variable is _not_ set, `os.getenv` will
/// return `None`, or the default value if one is provided.
///
/// If the default value is not a string or `None`, then it will be
/// inconsistent with the return type of `os.getenv`, which can lead to
/// confusing behavior.
///
/// ## Example
/// ```python
/// import os
///
/// int(os.getenv("FOO", 1))
/// ```
///
/// Use instead:
/// ```python
/// import os
///
/// int(os.getenv("FOO", "1"))
/// ```
#[derive(ViolationMetadata)]
pub struct InvalidEnvvarDefault;

impl Violation for InvalidEnvvarDefault {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Invalid type for environment variable default; expected `str` or `None`".to_string()
    }
}