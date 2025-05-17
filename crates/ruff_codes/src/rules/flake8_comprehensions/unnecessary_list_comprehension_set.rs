use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary list comprehensions.
///
/// ## Why is this bad?
/// It's unnecessary to use a list comprehension inside a call to `set()`,
/// since there is an equivalent comprehension for this type.
///
/// ## Example
/// ```python
/// set([f(x) for x in foo])
/// ```
///
/// Use instead:
/// ```python
/// {f(x) for x in foo}
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
#[derive(ViolationMetadata)]
pub struct UnnecessaryListComprehensionSet;

impl AlwaysFixableViolation for UnnecessaryListComprehensionSet {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary list comprehension (rewrite as a set comprehension)".to_string()
    }

    fn fix_title(&self) -> String {
        "Rewrite as a set comprehension".to_string()
    }
}