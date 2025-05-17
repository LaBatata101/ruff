use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `pass` statements in empty stub bodies.
///
/// ## Why is this bad?
/// For stylistic consistency, `...` should always be used rather than `pass`
/// in stub files.
///
/// ## Example
/// ```pyi
/// def foo(bar: int) -> list[int]: pass
/// ```
///
/// Use instead:
/// ```pyi
/// def foo(bar: int) -> list[int]: ...
/// ```
///
/// ## References
/// - [Typing documentation - Writing and Maintaining Stub Files](https://typing.python.org/en/latest/guides/writing_stubs.html)
#[derive(ViolationMetadata)]
pub struct PassStatementStubBody;

impl AlwaysFixableViolation for PassStatementStubBody {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Empty body should contain `...`, not `pass`".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace `pass` with `...`".to_string()
    }
}