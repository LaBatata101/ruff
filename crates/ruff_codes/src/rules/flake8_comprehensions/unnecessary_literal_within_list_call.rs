use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::LiteralKind;

/// ## What it does
/// Checks for `list()` calls that take unnecessary list or tuple literals as
/// arguments.
///
/// ## Why is this bad?
/// It's unnecessary to use a list or tuple literal within a `list()` call,
/// since there is a literal syntax for these types.
///
/// If a list literal is passed in, then the outer call to `list()` should be
/// removed. Otherwise, if a tuple literal is passed in, then it should be
/// rewritten as a list literal.
///
/// ## Example
/// ```python
/// list([1, 2])
/// list((1, 2))
/// ```
///
/// Use instead:
/// ```python
/// [1, 2]
/// [1, 2]
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
#[derive(ViolationMetadata)]
pub struct UnnecessaryLiteralWithinListCall {
    kind: LiteralKind,
}

impl AlwaysFixableViolation for UnnecessaryLiteralWithinListCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.kind {
            LiteralKind::List => {
                "Unnecessary list literal passed to `list()` (remove the outer call to `list()`)"
                    .to_string()
            }
            LiteralKind::Tuple => {
                "Unnecessary tuple literal passed to `list()` (rewrite as a single list literal)"
                    .to_string()
            }
        }
    }

    fn fix_title(&self) -> String {
        match self.kind {
            LiteralKind::List => "Remove outer `list()` call".to_string(),
            LiteralKind::Tuple => "Rewrite as a single list literal".to_string(),
        }
    }
}