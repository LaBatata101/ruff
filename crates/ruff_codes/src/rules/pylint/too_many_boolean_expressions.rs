use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for too many Boolean expressions in an `if` statement.
///
/// By default, this rule allows up to 5 expressions. This can be configured
/// using the [`lint.pylint.max-bool-expr`] option.
///
/// ## Why is this bad?
/// `if` statements with many Boolean expressions are harder to understand
/// and maintain. Consider assigning the result of the Boolean expression,
/// or any of its sub-expressions, to a variable.
///
/// ## Example
/// ```python
/// if a and b and c and d and e and f and g and h:
///     ...
/// ```
///
/// ## Options
/// - `lint.pylint.max-bool-expr`
#[derive(ViolationMetadata)]
pub struct TooManyBooleanExpressions {
    expressions: usize,
    max_expressions: usize,
}

impl Violation for TooManyBooleanExpressions {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooManyBooleanExpressions {
            expressions,
            max_expressions,
        } = self;
        format!("Too many Boolean expressions ({expressions} > {max_expressions})")
    }
}