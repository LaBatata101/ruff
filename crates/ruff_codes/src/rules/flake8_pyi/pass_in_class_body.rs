use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of the `pass` statement in non-empty class bodies
/// in `.pyi` files.
///
/// ## Why is this bad?
/// The `pass` statement is always unnecessary in non-empty class bodies in
/// stubs.
///
/// ## Example
/// ```pyi
/// class MyClass:
///     x: int
///     pass
/// ```
///
/// Use instead:
/// ```pyi
/// class MyClass:
///     x: int
/// ```
#[derive(ViolationMetadata)]
pub struct PassInClassBody;

impl AlwaysFixableViolation for PassInClassBody {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Class body must not contain `pass`".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove unnecessary `pass`".to_string()
    }
}