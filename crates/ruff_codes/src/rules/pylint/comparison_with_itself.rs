use ruff_diagnostics::Violation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for operations that compare a name to itself.
///
/// ## Why is this bad?
/// Comparing a name to itself always results in the same value, and is likely
/// a mistake.
///
/// ## Example
/// ```python
/// foo == foo
/// ```
///
/// In some cases, self-comparisons are used to determine whether a float is
/// NaN. Instead, prefer `math.isnan`:
/// ```python
/// import math
///
/// math.isnan(foo)
/// ```
///
/// ## References
/// - [Python documentation: Comparisons](https://docs.python.org/3/reference/expressions.html#comparisons)
#[derive(ViolationMetadata)]
pub struct ComparisonWithItself {
    actual: SourceCodeSnippet,
}

impl Violation for ComparisonWithItself {
    #[derive_message_formats]
    fn message(&self) -> String {
        if let Some(actual) = self.actual.full_display() {
            format!("Name compared with itself, consider replacing `{actual}`")
        } else {
            "Name compared with itself".to_string()
        }
    }
}