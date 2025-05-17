use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `__bool__` implementations that return a type other than `bool`.
///
/// ## Why is this bad?
/// The `__bool__` method should return a `bool` object. Returning a different
/// type may cause unexpected behavior.
///
/// ## Example
/// ```python
/// class Foo:
///     def __bool__(self):
///         return 2
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     def __bool__(self):
///         return True
/// ```
///
/// ## References
/// - [Python documentation: The `__bool__` method](https://docs.python.org/3/reference/datamodel.html#object.__bool__)
#[derive(ViolationMetadata)]
pub struct InvalidBoolReturnType;

impl Violation for InvalidBoolReturnType {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`__bool__` does not return `bool`".to_string()
    }
}