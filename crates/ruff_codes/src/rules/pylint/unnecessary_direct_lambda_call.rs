use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary direct calls to lambda expressions.
///
/// ## Why is this bad?
/// Calling a lambda expression directly is unnecessary. The expression can be
/// executed inline instead to improve readability.
///
/// ## Example
/// ```python
/// area = (lambda r: 3.14 * r**2)(radius)
/// ```
///
/// Use instead:
/// ```python
/// area = 3.14 * radius**2
/// ```
///
/// ## References
/// - [Python documentation: Lambdas](https://docs.python.org/3/reference/expressions.html#lambda)
#[derive(ViolationMetadata)]
pub struct UnnecessaryDirectLambdaCall;

impl Violation for UnnecessaryDirectLambdaCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Lambda expression called directly. Execute the expression inline instead.".to_string()
    }
}