use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for membership tests using `not {element} in {collection}`.
///
/// ## Why is this bad?
/// Testing membership with `{element} not in {collection}` is more readable.
///
/// ## Example
/// ```python
/// Z = not X in Y
/// if not X.B in Y:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// Z = X not in Y
/// if X.B not in Y:
///     pass
/// ```
#[derive(ViolationMetadata)]
pub struct NotInTest;

impl AlwaysFixableViolation for NotInTest {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Test for membership should be `not in`".to_string()
    }

    fn fix_title(&self) -> String {
        "Convert to `not in`".to_string()
    }
}

/// ## What it does
/// Checks for identity comparisons using `not {foo} is {bar}`.
///
/// ## Why is this bad?
/// According to [PEP8], testing for an object's identity with `is not` is more
/// readable.
///
/// ## Example
/// ```python
/// if not X is Y:
///     pass
/// Z = not X.B is Y
/// ```
///
/// Use instead:
/// ```python
/// if X is not Y:
///     pass
/// Z = X.B is not Y
/// ```
///
/// [PEP8]: https://peps.python.org/pep-0008/#programming-recommendations
#[derive(ViolationMetadata)]
pub struct NotIsTest;

impl AlwaysFixableViolation for NotIsTest {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Test for object identity should be `is not`".to_string()
    }

    fn fix_title(&self) -> String {
        "Convert to `is not`".to_string()
    }
}
