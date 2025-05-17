use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `__str__` implementations that return a type other than `str`.
///
/// ## Why is this bad?
/// The `__str__` method should return a `str` object. Returning a different
/// type may cause unexpected behavior.
///
/// ## Example
/// ```python
/// class Foo:
///     def __str__(self):
///         return True
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     def __str__(self):
///         return "Foo"
/// ```
///
/// ## References
/// - [Python documentation: The `__str__` method](https://docs.python.org/3/reference/datamodel.html#object.__str__)
#[derive(ViolationMetadata)]
pub struct InvalidStrReturnType;

impl Violation for InvalidStrReturnType {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`__str__` does not return `str`".to_string()
    }
}