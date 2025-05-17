use ruff_diagnostics::Violation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for mutations to an iterable during a loop iteration.
///
/// ## Why is this bad?
/// When iterating over an iterable, mutating the iterable can lead to unexpected
/// behavior, like skipping elements or infinite loops.
///
/// ## Example
/// ```python
/// items = [1, 2, 3]
///
/// for item in items:
///     print(item)
///
///     # Create an infinite loop by appending to the list.
///     items.append(item)
/// ```
///
/// ## References
/// - [Python documentation: Mutable Sequence Types](https://docs.python.org/3/library/stdtypes.html#typesseq-mutable)
#[derive(ViolationMetadata)]
pub struct LoopIteratorMutation {
    name: Option<SourceCodeSnippet>,
}

impl Violation for LoopIteratorMutation {
    #[derive_message_formats]
    fn message(&self) -> String {
        if let Some(name) = self.name.as_ref().and_then(SourceCodeSnippet::full_display) {
            format!("Mutation to loop iterable `{name}` during iteration")
        } else {
            "Mutation to loop iterable during iteration".to_string()
        }
    }
}