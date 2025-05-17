use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `__hash__` implementations that return non-integer values.
///
/// ## Why is this bad?
/// The `__hash__` method should return an integer. Returning a different
/// type may cause unexpected behavior.
///
/// Note: `bool` is a subclass of `int`, so it's technically valid for `__hash__` to
/// return `True` or `False`. However, for consistency with other rules, Ruff will
/// still emit a diagnostic when `__hash__` returns a `bool`.
///
/// ## Example
/// ```python
/// class Foo:
///     def __hash__(self):
///         return "2"
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     def __hash__(self):
///         return 2
/// ```
///
/// ## References
/// - [Python documentation: The `__hash__` method](https://docs.python.org/3/reference/datamodel.html#object.__hash__)
#[derive(ViolationMetadata)]
pub struct InvalidHashReturnType;

impl Violation for InvalidHashReturnType {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`__hash__` does not return an integer".to_string()
    }
}