use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::Kind;

/// ## What it does
/// Checks for useless expressions.
///
/// ## Why is this bad?
/// Useless expressions have no effect on the program, and are often included
/// by mistake. Assign a useless expression to a variable, or remove it
/// entirely.
///
/// ## Example
/// ```python
/// 1 + 1
/// ```
///
/// Use instead:
/// ```python
/// foo = 1 + 1
/// ```
///
/// ## Notebook behavior
/// For Jupyter Notebooks, this rule is not applied to the last top-level expression in a cell.
/// This is because it's common to have a notebook cell that ends with an expression,
/// which will result in the `repr` of the evaluated expression being printed as the cell's output.
///
/// ## Known problems
/// This rule ignores expression types that are commonly used for their side
/// effects, such as function calls.
///
/// However, if a seemingly useless expression (like an attribute access) is
/// needed to trigger a side effect, consider assigning it to an anonymous
/// variable, to indicate that the return value is intentionally ignored.
///
/// For example, given:
/// ```python
/// with errors.ExceptionRaisedContext():
///     obj.attribute
/// ```
///
/// Use instead:
/// ```python
/// with errors.ExceptionRaisedContext():
///     _ = obj.attribute
/// ```
#[derive(ViolationMetadata)]
pub struct UselessExpression {
    kind: Kind,
}

impl Violation for UselessExpression {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.kind {
            Kind::Expression => {
                "Found useless expression. Either assign it to a variable or remove it.".to_string()
            }
            Kind::Attribute => {
                "Found useless attribute access. Either assign it to a variable or remove it."
                    .to_string()
            }
        }
    }
}