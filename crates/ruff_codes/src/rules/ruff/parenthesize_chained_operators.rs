use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for chained operators where adding parentheses could improve the
/// clarity of the code.
///
/// ## Why is this bad?
/// `and` always binds more tightly than `or` when chaining the two together,
/// but this can be hard to remember (and sometimes surprising).
/// Adding parentheses in these situations can greatly improve code readability,
/// with no change to semantics or performance.
///
/// For example:
/// ```python
/// a, b, c = 1, 0, 2
/// x = a or b and c
///
/// d, e, f = 0, 1, 2
/// y = d and e or f
/// ```
///
/// Use instead:
/// ```python
/// a, b, c = 1, 0, 2
/// x = a or (b and c)
///
/// d, e, f = 0, 1, 2
/// y = (d and e) or f
/// ```
#[derive(ViolationMetadata)]
pub struct ParenthesizeChainedOperators;

impl AlwaysFixableViolation for ParenthesizeChainedOperators {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Parenthesize `a and b` expressions when chaining `and` and `or` together, to make the precedence clear".to_string()
    }

    fn fix_title(&self) -> String {
        "Parenthesize the `and` subexpression".to_string()
    }
}