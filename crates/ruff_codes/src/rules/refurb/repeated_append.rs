use ruff_diagnostics::{FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for consecutive calls to `append`.
///
/// ## Why is this bad?
/// Consecutive calls to `append` can be less efficient than batching them into
/// a single `extend`. Each `append` resizes the list individually, whereas an
/// `extend` can resize the list once for all elements.
///
/// ## Known problems
/// This rule is prone to false negatives due to type inference limitations,
/// as it will only detect lists that are instantiated as literals or annotated
/// with a type annotation.
///
/// ## Example
/// ```python
/// nums = [1, 2, 3]
///
/// nums.append(4)
/// nums.append(5)
/// nums.append(6)
/// ```
///
/// Use instead:
/// ```python
/// nums = [1, 2, 3]
///
/// nums.extend((4, 5, 6))
/// ```
///
/// ## References
/// - [Python documentation: More on Lists](https://docs.python.org/3/tutorial/datastructures.html#more-on-lists)
#[derive(ViolationMetadata)]
pub struct RepeatedAppend {
    name: String,
    replacement: SourceCodeSnippet,
}

impl RepeatedAppend {
    fn suggestion(&self) -> String {
        let name = &self.name;
        self.replacement
            .full_display()
            .map_or(format!("{name}.extend(...)"), ToString::to_string)
    }
}

impl Violation for RepeatedAppend {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let name = &self.name;
        let suggestion = self.suggestion();
        format!("Use `{suggestion}` instead of repeatedly calling `{name}.append()`")
    }

    fn fix_title(&self) -> Option<String> {
        let suggestion = self.suggestion();
        Some(format!("Replace with `{suggestion}`"))
    }
}