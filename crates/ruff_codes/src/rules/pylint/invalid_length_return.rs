use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `__len__` implementations that return values that are not non-negative
/// integers.
///
/// ## Why is this bad?
/// The `__len__` method should return a non-negative integer. Returning a different
/// value may cause unexpected behavior.
///
/// Note: `bool` is a subclass of `int`, so it's technically valid for `__len__` to
/// return `True` or `False`. However, for consistency with other rules, Ruff will
/// still emit a diagnostic when `__len__` returns a `bool`.
///
/// ## Example
/// ```python
/// class Foo:
///     def __len__(self):
///         return "2"
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     def __len__(self):
///         return 2
/// ```
///
/// ## References
/// - [Python documentation: The `__len__` method](https://docs.python.org/3/reference/datamodel.html#object.__len__)
#[derive(ViolationMetadata)]
pub struct InvalidLengthReturnType;

impl Violation for InvalidLengthReturnType {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`__len__` does not return a non-negative integer".to_string()
    }
}