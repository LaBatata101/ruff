use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::UnaryPrefixOperatorType;

/// ## What it does
/// Checks for the attempted use of the unary prefix increment (`++`) or
/// decrement operator (`--`).
///
/// ## Why is this bad?
/// Python does not support the unary prefix increment or decrement operator.
/// Writing `++n` is equivalent to `+(+(n))` and writing `--n` is equivalent to
/// `-(-(n))`. In both cases, it is equivalent to `n`.
///
/// ## Example
/// ```python
/// ++x
/// --y
/// ```
///
/// Use instead:
/// ```python
/// x += 1
/// y -= 1
/// ```
///
/// ## References
/// - [Python documentation: Unary arithmetic and bitwise operations](https://docs.python.org/3/reference/expressions.html#unary-arithmetic-and-bitwise-operations)
/// - [Python documentation: Augmented assignment statements](https://docs.python.org/3/reference/simple_stmts.html#augmented-assignment-statements)
#[derive(ViolationMetadata)]
pub struct UnaryPrefixIncrementDecrement {
    operator: UnaryPrefixOperatorType,
}

impl Violation for UnaryPrefixIncrementDecrement {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.operator {
            UnaryPrefixOperatorType::Increment => {
                "Python does not support the unary prefix increment operator (`++`)".to_string()
            }
            UnaryPrefixOperatorType::Decrement => {
                "Python does not support the unary prefix decrement operator (`--`)".to_string()
            }
        }
    }
}