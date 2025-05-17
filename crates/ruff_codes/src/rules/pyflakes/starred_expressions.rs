use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of too many expressions in starred assignment statements.
///
/// ## Why is this bad?
/// In assignment statements, starred expressions can be used to unpack iterables.
///
/// In Python 3, no more than 1 << 8 assignments are allowed before a starred
/// expression, and no more than 1 << 24 expressions are allowed after a starred
/// expression.
///
/// ## References
/// - [PEP 3132 – Extended Iterable Unpacking](https://peps.python.org/pep-3132/)
#[derive(ViolationMetadata)]
pub struct ExpressionsInStarAssignment;

impl Violation for ExpressionsInStarAssignment {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Too many expressions in star-unpacking assignment".to_string()
    }
}

/// ## What it does
/// Checks for the use of multiple starred expressions in assignment statements.
///
/// ## Why is this bad?
/// In assignment statements, starred expressions can be used to unpack iterables.
/// Including more than one starred expression on the left-hand-side of an
/// assignment will cause a `SyntaxError`, as it is unclear which expression
/// should receive the remaining values.
///
/// ## Example
/// ```python
/// *foo, *bar, baz = (1, 2, 3)
/// ```
///
/// ## References
/// - [PEP 3132 – Extended Iterable Unpacking](https://peps.python.org/pep-3132/)
#[derive(ViolationMetadata)]
pub struct MultipleStarredExpressions;

impl Violation for MultipleStarredExpressions {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Two starred expressions in assignment".to_string()
    }
}
