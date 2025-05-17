use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::UnnecessaryLiteral;

/// ## What it does
/// Checks for `set()` calls that take unnecessary list or tuple literals
/// as arguments.
///
/// ## Why is this bad?
/// It's unnecessary to use a list or tuple literal within a call to `set()`.
/// Instead, the expression can be rewritten as a set literal.
///
/// ## Example
/// ```python
/// set([1, 2])
/// set((1, 2))
/// set([])
/// ```
///
/// Use instead:
/// ```python
/// {1, 2}
/// {1, 2}
/// set()
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
#[derive(ViolationMetadata)]
pub struct UnnecessaryLiteralSet {
    kind: UnnecessaryLiteral,
}

impl AlwaysFixableViolation for UnnecessaryLiteralSet {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryLiteralSet { kind } = self;
        format!("Unnecessary {kind} literal (rewrite as a set literal)")
    }

    fn fix_title(&self) -> String {
        "Rewrite as a set literal".to_string()
    }
}