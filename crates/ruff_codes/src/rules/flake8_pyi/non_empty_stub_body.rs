use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for non-empty function stub bodies.
///
/// ## Why is this bad?
/// Stub files are never executed at runtime; they should be thought of as
/// "data files" for type checkers or IDEs. Function bodies are redundant
/// for this purpose.
///
/// ## Example
/// ```pyi
/// def double(x: int) -> int:
///     return x * 2
/// ```
///
/// Use instead:
/// ```pyi
/// def double(x: int) -> int: ...
/// ```
///
/// ## References
/// - [Typing documentation - Writing and Maintaining Stub Files](https://typing.python.org/en/latest/guides/writing_stubs.html)
#[derive(ViolationMetadata)]
pub struct NonEmptyStubBody;

impl AlwaysFixableViolation for NonEmptyStubBody {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Function body must contain only `...`".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace function body with `...`".to_string()
    }
}