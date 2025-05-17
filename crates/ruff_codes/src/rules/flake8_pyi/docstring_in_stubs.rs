use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of docstrings in stub files.
///
/// ## Why is this bad?
/// Stub files should omit docstrings, as they're intended to provide type
/// hints, rather than documentation.
///
/// ## Example
///
/// ```pyi
/// def func(param: int) -> str:
///     """This is a docstring."""
///     ...
/// ```
///
/// Use instead:
///
/// ```pyi
/// def func(param: int) -> str: ...
/// ```
#[derive(ViolationMetadata)]
pub struct DocstringInStub;

impl AlwaysFixableViolation for DocstringInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Docstrings should not be included in stubs".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove docstring".to_string()
    }
}