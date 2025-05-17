use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for ternary `if` expressions that can be replaced with the `or`
/// operator.
///
/// ## Why is this bad?
/// Ternary `if` expressions are more verbose than `or` expressions while
/// providing the same functionality.
///
/// ## Example
/// ```python
/// z = x if x else y
/// ```
///
/// Use instead:
/// ```python
/// z = x or y
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe in the event that the body of the
/// `if` expression contains side effects.
///
/// For example, `foo` will be called twice in `foo() if foo() else bar()`
/// (assuming `foo()` returns a truthy value), but only once in
/// `foo() or bar()`.
#[derive(ViolationMetadata)]
pub struct IfExpInsteadOfOrOperator;

impl Violation for IfExpInsteadOfOrOperator {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Replace ternary `if` expression with `or` operator".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `or` operator".to_string())
    }
}