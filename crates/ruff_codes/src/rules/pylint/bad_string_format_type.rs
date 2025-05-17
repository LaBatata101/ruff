use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for mismatched argument types in "old-style" format strings.
///
/// ## Why is this bad?
/// The format string is not checked at compile time, so it is easy to
/// introduce bugs by mistyping the format string.
///
/// ## Example
/// ```python
/// print("%d" % "1")
/// ```
///
/// Use instead:
/// ```python
/// print("%d" % 1)
/// ```
#[derive(ViolationMetadata)]
pub struct BadStringFormatType;

impl Violation for BadStringFormatType {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Format type does not match argument type".to_string()
    }
}