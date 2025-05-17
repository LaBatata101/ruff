use ruff_diagnostics::AlwaysFixableViolation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `set.remove` that can be replaced with `set.discard`.
///
/// ## Why is this bad?
/// If an element should be removed from a set if it is present, it is more
/// succinct and idiomatic to use `discard`.
///
/// ## Known problems
/// This rule is prone to false negatives due to type inference limitations,
/// as it will only detect sets that are instantiated as literals or annotated
/// with a type annotation.
///
/// ## Example
/// ```python
/// nums = {123, 456}
///
/// if 123 in nums:
///     nums.remove(123)
/// ```
///
/// Use instead:
/// ```python
/// nums = {123, 456}
///
/// nums.discard(123)
/// ```
///
/// ## References
/// - [Python documentation: `set.discard()`](https://docs.python.org/3/library/stdtypes.html?highlight=list#frozenset.discard)
#[derive(ViolationMetadata)]
pub struct CheckAndRemoveFromSet {
    element: SourceCodeSnippet,
    set: String,
}

impl CheckAndRemoveFromSet {
    fn suggestion(&self) -> String {
        let set = &self.set;
        let element = self.element.truncated_display();
        format!("{set}.discard({element})")
    }
}

impl AlwaysFixableViolation for CheckAndRemoveFromSet {
    #[derive_message_formats]
    fn message(&self) -> String {
        let suggestion = self.suggestion();
        format!("Use `{suggestion}` instead of check and `remove`")
    }

    fn fix_title(&self) -> String {
        let suggestion = self.suggestion();
        format!("Replace with `{suggestion}`")
    }
}