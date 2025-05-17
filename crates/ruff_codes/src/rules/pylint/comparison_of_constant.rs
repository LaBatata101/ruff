use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::CmpOp;

/// ## What it does
/// Checks for comparisons between constants.
///
/// ## Why is this bad?
/// Comparing two constants will always resolve to the same value, so the
/// comparison is redundant. Instead, the expression should be replaced
/// with the result of the comparison.
///
/// ## Example
/// ```python
/// foo = 1 == 1
/// ```
///
/// Use instead:
/// ```python
/// foo = True
/// ```
///
/// ## References
/// - [Python documentation: Comparisons](https://docs.python.org/3/reference/expressions.html#comparisons)
#[derive(ViolationMetadata)]
pub struct ComparisonOfConstant {
    left_constant: String,
    op: CmpOp,
    right_constant: String,
}

impl Violation for ComparisonOfConstant {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ComparisonOfConstant {
            left_constant,
            op,
            right_constant,
        } = self;

        format!(
            "Two constants compared in a comparison, consider replacing `{left_constant} {op} {right_constant}`",
        )
    }
}