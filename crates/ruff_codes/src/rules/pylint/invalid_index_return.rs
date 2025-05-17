use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `__index__` implementations that return non-integer values.
///
/// ## Why is this bad?
/// The `__index__` method should return an integer. Returning a different
/// type may cause unexpected behavior.
///
/// Note: `bool` is a subclass of `int`, so it's technically valid for `__index__` to
/// return `True` or `False`. However, a `DeprecationWarning` (`DeprecationWarning:
/// __index__ returned non-int (type bool)`) for such cases was already introduced,
/// thus this is a conscious difference between the original pylint rule and the
/// current ruff implementation.
///
/// ## Example
/// ```python
/// class Foo:
///     def __index__(self):
///         return "2"
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     def __index__(self):
///         return 2
/// ```
///
/// ## References
/// - [Python documentation: The `__index__` method](https://docs.python.org/3/reference/datamodel.html#object.__index__)
#[derive(ViolationMetadata)]
pub struct InvalidIndexReturnType;

impl Violation for InvalidIndexReturnType {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`__index__` does not return an integer".to_string()
    }
}