use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::ComparisonLocationAt;

/// ## What it does
/// Checks for useless comparisons.
///
/// ## Why is this bad?
/// Useless comparisons have no effect on the program, and are often included
/// by mistake. If the comparison is intended to enforce an invariant, prepend
/// the comparison with an `assert`. Otherwise, remove it entirely.
///
/// ## Example
/// ```python
/// foo == bar
/// ```
///
/// Use instead:
/// ```python
/// assert foo == bar, "`foo` and `bar` should be equal."
/// ```
///
/// ## Notebook behavior
/// For Jupyter Notebooks, this rule is not applied to the last top-level expression in a cell.
/// This is because it's common to have a notebook cell that ends with an expression,
/// which will result in the `repr` of the evaluated expression being printed as the cell's output.
///
/// ## References
/// - [Python documentation: `assert` statement](https://docs.python.org/3/reference/simple_stmts.html#the-assert-statement)
#[derive(ViolationMetadata)]
pub struct UselessComparison {
    at: ComparisonLocationAt,
}

impl Violation for UselessComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.at {
            ComparisonLocationAt::MiddleBody => {
                "Pointless comparison. Did you mean to assign a value? \
                Otherwise, prepend `assert` or remove it."
                    .to_string()
            }
            ComparisonLocationAt::EndOfFunction => {
                "Pointless comparison at end of function scope. Did you mean \
                to return the expression result?"
                    .to_string()
            }
        }
    }
}