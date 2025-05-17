use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for custom exception definitions that omit the `Error` suffix.
///
/// ## Why is this bad?
/// The `Error` suffix is recommended by [PEP 8]:
///
/// > Because exceptions should be classes, the class naming convention
/// > applies here. However, you should use the suffix `"Error"` on your
/// > exception names (if the exception actually is an error).
///
/// ## Example
///
/// ```python
/// class Validation(Exception): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class ValidationError(Exception): ...
/// ```
///
/// ## Options
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#exception-names
#[derive(ViolationMetadata)]
pub struct ErrorSuffixOnExceptionName {
    name: String,
}

impl Violation for ErrorSuffixOnExceptionName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ErrorSuffixOnExceptionName { name } = self;
        format!("Exception name `{name}` should be named with an Error suffix")
    }
}