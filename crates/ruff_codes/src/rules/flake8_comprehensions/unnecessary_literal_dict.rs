use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::LiteralKind;

/// ## What it does
/// Checks for unnecessary list or tuple literals.
///
/// ## Why is this bad?
/// It's unnecessary to use a list or tuple literal within a call to `dict()`.
/// It can be rewritten as a dict literal (`{}`).
///
/// ## Example
/// ```python
/// dict([(1, 2), (3, 4)])
/// dict(((1, 2), (3, 4)))
/// dict([])
/// ```
///
/// Use instead:
/// ```python
/// {1: 2, 3: 4}
/// {1: 2, 3: 4}
/// {}
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
#[derive(ViolationMetadata)]
pub struct UnnecessaryLiteralDict {
    obj_type: LiteralKind,
}

impl AlwaysFixableViolation for UnnecessaryLiteralDict {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryLiteralDict { obj_type } = self;
        format!("Unnecessary {obj_type} literal (rewrite as a dict literal)")
    }

    fn fix_title(&self) -> String {
        "Rewrite as a dict literal".to_string()
    }
}