use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of type comments (e.g., `x = 1  # type: int`) in stub
/// files.
///
/// ## Why is this bad?
/// Stub (`.pyi`) files should use type annotations directly, rather
/// than type comments, even if they're intended to support Python 2, since
/// stub files are not executed at runtime. The one exception is `# type: ignore`.
///
/// ## Example
/// ```pyi
/// x = 1  # type: int
/// ```
///
/// Use instead:
/// ```pyi
/// x: int = 1
/// ```
#[derive(ViolationMetadata)]
pub struct TypeCommentInStub;

impl Violation for TypeCommentInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Don't use type comments in stub file".to_string()
    }
}