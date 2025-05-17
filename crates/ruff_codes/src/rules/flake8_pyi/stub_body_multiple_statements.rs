use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for functions in stub (`.pyi`) files that contain multiple
/// statements.
///
/// ## Why is this bad?
/// Stub files are never executed, and are only intended to define type hints.
/// As such, functions in stub files should not contain functional code, and
/// should instead contain only a single statement (e.g., `...`).
///
/// ## Example
///
/// ```pyi
/// def function():
///     x = 1
///     y = 2
///     return x + y
/// ```
///
/// Use instead:
///
/// ```pyi
/// def function(): ...
/// ```
#[derive(ViolationMetadata)]
pub struct StubBodyMultipleStatements;

impl Violation for StubBodyMultipleStatements {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Function body must contain exactly one statement".to_string()
    }
}