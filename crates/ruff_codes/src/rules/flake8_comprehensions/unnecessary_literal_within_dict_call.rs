use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::DictKind;

/// ## What it does
/// Checks for `dict()` calls that take unnecessary dict literals or dict
/// comprehensions as arguments.
///
/// ## Why is this bad?
/// It's unnecessary to wrap a dict literal or comprehension within a `dict()`
/// call, since the literal or comprehension syntax already returns a
/// dictionary.
///
/// ## Example
/// ```python
/// dict({})
/// dict({"a": 1})
/// ```
///
/// Use instead:
/// ```python
/// {}
/// {"a": 1}
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
#[derive(ViolationMetadata)]
pub struct UnnecessaryLiteralWithinDictCall {
    kind: DictKind,
}

impl AlwaysFixableViolation for UnnecessaryLiteralWithinDictCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryLiteralWithinDictCall { kind } = self;
        format!("Unnecessary dict {kind} passed to `dict()` (remove the outer call to `dict()`)")
    }

    fn fix_title(&self) -> String {
        "Remove outer `dict()` call".to_string()
    }
}