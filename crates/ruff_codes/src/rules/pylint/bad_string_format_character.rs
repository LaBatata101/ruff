use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unsupported format types in format strings.
///
/// ## Why is this bad?
/// An invalid format string character will result in an error at runtime.
///
/// ## Example
/// ```python
/// # `z` is not a valid format type.
/// print("%z" % "1")
///
/// print("{:z}".format("1"))
/// ```
#[derive(ViolationMetadata)]
pub struct BadStringFormatCharacter {
    format_char: char,
}

impl Violation for BadStringFormatCharacter {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BadStringFormatCharacter { format_char } = self;
        format!("Unsupported format character '{format_char}'")
    }
}