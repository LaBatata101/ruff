use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of the characters 'l', 'O', or 'I' as function names.
///
/// ## Why is this bad?
/// In some fonts, these characters are indistinguishable from the
/// numerals one and zero. When tempted to use 'l', use 'L' instead.
///
/// ## Example
///
/// ```python
/// def l(x): ...
/// ```
///
/// Use instead:
///
/// ```python
/// def long_name(x): ...
/// ```
#[derive(ViolationMetadata)]
pub struct AmbiguousFunctionName(pub String);

impl Violation for AmbiguousFunctionName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let AmbiguousFunctionName(name) = self;
        format!("Ambiguous function name: `{name}`")
    }
}