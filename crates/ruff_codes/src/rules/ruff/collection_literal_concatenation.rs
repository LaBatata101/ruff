use ruff_diagnostics::{FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of the `+` operator to concatenate collections.
///
/// ## Why is this bad?
/// In Python, the `+` operator can be used to concatenate collections (e.g.,
/// `x + y` to concatenate the lists `x` and `y`).
///
/// However, collections can be concatenated more efficiently using the
/// unpacking operator (e.g., `[*x, *y]` to concatenate `x` and `y`).
///
/// Prefer the unpacking operator to concatenate collections, as it is more
/// readable and flexible. The `*` operator can unpack any iterable, whereas
///  `+` operates only on particular sequences which, in many cases, must be of
/// the same type.
///
/// ## Example
/// ```python
/// foo = [2, 3, 4]
/// bar = [1] + foo + [5, 6]
/// ```
///
/// Use instead:
/// ```python
/// foo = [2, 3, 4]
/// bar = [1, *foo, 5, 6]
/// ```
///
/// ## Fix safety
///
/// The fix is always marked as unsafe because the `+` operator uses the `__add__` magic method and
/// `*`-unpacking uses the `__iter__` magic method. Both of these could have custom
/// implementations, causing the fix to change program behaviour.
///
/// ## References
/// - [PEP 448 – Additional Unpacking Generalizations](https://peps.python.org/pep-0448/)
/// - [Python documentation: Sequence Types — `list`, `tuple`, `range`](https://docs.python.org/3/library/stdtypes.html#sequence-types-list-tuple-range)
#[derive(ViolationMetadata)]
pub struct CollectionLiteralConcatenation {
    expression: SourceCodeSnippet,
}

impl Violation for CollectionLiteralConcatenation {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        if let Some(expression) = self.expression.full_display() {
            format!("Consider `{expression}` instead of concatenation")
        } else {
            "Consider iterable unpacking instead of concatenation".to_string()
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = match self.expression.full_display() {
            Some(expression) => format!("Replace with `{expression}`"),
            None => "Replace with iterable unpacking".to_string(),
        };
        Some(title)
    }
}