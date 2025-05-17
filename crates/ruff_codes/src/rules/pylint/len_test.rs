use ruff_diagnostics::AlwaysFixableViolation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `len` calls on sequences in a boolean test context.
///
/// ## Why is this bad?
/// Empty sequences are considered false in a boolean context.
/// You can either remove the call to `len`
/// or compare the length against a scalar.
///
/// ## Example
/// ```python
/// fruits = ["orange", "apple"]
/// vegetables = []
///
/// if len(fruits):
///     print(fruits)
///
/// if not len(vegetables):
///     print(vegetables)
/// ```
///
/// Use instead:
/// ```python
/// fruits = ["orange", "apple"]
/// vegetables = []
///
/// if fruits:
///     print(fruits)
///
/// if not vegetables:
///     print(vegetables)
/// ```
///
/// ## References
/// [PEP 8: Programming Recommendations](https://peps.python.org/pep-0008/#programming-recommendations)
#[derive(ViolationMetadata)]
pub struct LenTest {
    expression: SourceCodeSnippet,
}

impl AlwaysFixableViolation for LenTest {
    #[derive_message_formats]
    fn message(&self) -> String {
        if let Some(expression) = self.expression.full_display() {
            format!("`len({expression})` used as condition without comparison")
        } else {
            "`len(SEQUENCE)` used as condition without comparison".to_string()
        }
    }

    fn fix_title(&self) -> String {
        "Remove `len`".to_string()
    }
}