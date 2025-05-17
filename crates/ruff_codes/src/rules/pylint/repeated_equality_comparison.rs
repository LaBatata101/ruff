use ruff_diagnostics::AlwaysFixableViolation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for repeated equality comparisons that can be rewritten as a membership
/// test.
///
/// This rule will try to determine if the values are hashable
/// and the fix will use a `set` if they are. If unable to determine, the fix
/// will use a `tuple` and suggest the use of a `set`.
///
/// ## Why is this bad?
/// To check if a variable is equal to one of many values, it is common to
/// write a series of equality comparisons (e.g.,
/// `foo == "bar" or foo == "baz"`).
///
/// Instead, prefer to combine the values into a collection and use the `in`
/// operator to check for membership, which is more performant and succinct.
/// If the items are hashable, use a `set` for efficiency; otherwise, use a
/// `tuple`.
///
/// ## Example
/// ```python
/// foo == "bar" or foo == "baz" or foo == "qux"
/// ```
///
/// Use instead:
/// ```python
/// foo in {"bar", "baz", "qux"}
/// ```
///
/// ## References
/// - [Python documentation: Comparisons](https://docs.python.org/3/reference/expressions.html#comparisons)
/// - [Python documentation: Membership test operations](https://docs.python.org/3/reference/expressions.html#membership-test-operations)
/// - [Python documentation: `set`](https://docs.python.org/3/library/stdtypes.html#set)
#[derive(ViolationMetadata)]
pub struct RepeatedEqualityComparison {
    expression: SourceCodeSnippet,
    all_hashable: bool,
}

impl AlwaysFixableViolation for RepeatedEqualityComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        match (self.expression.full_display(), self.all_hashable) {
            (Some(expression), false) => {
                format!("Consider merging multiple comparisons: `{expression}`. Use a `set` if the elements are hashable.")
            }
            (Some(expression), true) => {
                format!("Consider merging multiple comparisons: `{expression}`.")
            }
            (None, false) => {
                "Consider merging multiple comparisons. Use a `set` if the elements are hashable."
                    .to_string()
            }
            (None, true) => "Consider merging multiple comparisons.".to_string(),
        }
    }

    fn fix_title(&self) -> String {
        "Merge multiple comparisons".to_string()
    }
}