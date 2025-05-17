use ruff_diagnostics::{FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `if` statements that can be replaced with `bool`.
///
/// ## Why is this bad?
/// `if` statements that return `True` for a truthy condition and `False` for
/// a falsy condition can be replaced with boolean casts.
///
/// ## Example
/// Given:
/// ```python
/// def foo(x: int) -> bool:
///     if x > 0:
///         return True
///     else:
///         return False
/// ```
///
/// Use instead:
/// ```python
/// def foo(x: int) -> bool:
///     return x > 0
/// ```
///
/// Or, given:
/// ```python
/// def foo(x: int) -> bool:
///     if x > 0:
///         return True
///     return False
/// ```
///
/// Use instead:
/// ```python
/// def foo(x: int) -> bool:
///     return x > 0
/// ```
///
/// ## Fix safety
///
/// This fix is marked as unsafe because it may change the program’s behavior if the condition does not
/// return a proper Boolean. While the fix will try to wrap non-boolean values in a call to bool,
/// custom implementations of comparison functions like `__eq__` can avoid the bool call and still
/// lead to altered behavior.
///
/// ## References
/// - [Python documentation: Truth Value Testing](https://docs.python.org/3/library/stdtypes.html#truth-value-testing)
#[derive(ViolationMetadata)]
pub struct NeedlessBool {
    condition: Option<SourceCodeSnippet>,
    negate: bool,
}

impl Violation for NeedlessBool {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let NeedlessBool { condition, negate } = self;
        if let Some(condition) = condition.as_ref().and_then(SourceCodeSnippet::full_display) {
            format!("Return the condition `{condition}` directly")
        } else if *negate {
            "Return the negated condition directly".to_string()
        } else {
            "Return the condition directly".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        let NeedlessBool { condition, .. } = self;
        Some(
            if let Some(condition) = condition.as_ref().and_then(SourceCodeSnippet::full_display) {
                format!("Replace with `return {condition}`")
            } else {
                "Inline condition".to_string()
            },
        )
    }
}