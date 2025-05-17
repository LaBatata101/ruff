use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary `list()` calls around list comprehensions.
///
/// ## Why is this bad?
/// It is redundant to use a `list()` call around a list comprehension.
///
/// ## Example
/// ```python
/// list([f(x) for x in foo])
/// ```
///
/// Use instead
/// ```python
/// [f(x) for x in foo]
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
#[derive(ViolationMetadata)]
pub struct UnnecessaryListCall;

impl AlwaysFixableViolation for UnnecessaryListCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary `list()` call (remove the outer call to `list()`)".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove outer `list()` call".to_string()
    }
}