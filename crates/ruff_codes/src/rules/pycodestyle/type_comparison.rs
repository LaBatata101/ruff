use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for object type comparisons using `==` and other comparison
/// operators.
///
/// ## Why is this bad?
/// Unlike a direct type comparison, `isinstance` will also check if an object
/// is an instance of a class or a subclass thereof.
///
/// If you want to check for an exact type match, use `is` or `is not`.
///
/// ## Known problems
/// When using libraries that override the `==` (`__eq__`) operator (such as NumPy,
/// Pandas, and SQLAlchemy), this rule may produce false positives, as converting
/// from `==` to `is` or `is not` will change the behavior of the code.
///
/// For example, the following operations are _not_ equivalent:
/// ```python
/// import numpy as np
///
/// np.array([True, False]) == False
/// # array([False,  True])
///
/// np.array([True, False]) is False
/// # False
/// ```
///
/// ## Example
/// ```python
/// if type(obj) == type(1):
///     pass
///
/// if type(obj) == int:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// if isinstance(obj, int):
///     pass
/// ```
#[derive(ViolationMetadata)]
pub struct TypeComparison;

impl Violation for TypeComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `is` and `is not` for type comparisons, or `isinstance()` for isinstance checks"
            .to_string()
    }
}