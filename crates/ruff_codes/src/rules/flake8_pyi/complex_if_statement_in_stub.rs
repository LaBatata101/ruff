use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `if` statements with complex conditionals in stubs.
///
/// ## Why is this bad?
/// Type checkers understand simple conditionals to express variations between
/// different Python versions and platforms. However, complex tests may not be
/// understood by a type checker, leading to incorrect inferences when they
/// analyze your code.
///
/// ## Example
/// ```pyi
/// import sys
///
/// if (3, 10) <= sys.version_info < (3, 12): ...
/// ```
///
/// Use instead:
/// ```pyi
/// import sys
///
/// if sys.version_info >= (3, 10) and sys.version_info < (3, 12): ...
/// ```
///
/// ## References
/// - [Typing documentation: Version and platform checking](https://typing.python.org/en/latest/spec/directives.html#version-and-platform-checks)
#[derive(ViolationMetadata)]
pub struct ComplexIfStatementInStub;

impl Violation for ComplexIfStatementInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`if` test must be a simple comparison against `sys.platform` or `sys.version_info`"
            .to_string()
    }
}