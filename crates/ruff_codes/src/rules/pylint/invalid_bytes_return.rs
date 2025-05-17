use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `__bytes__` implementations that return types other than `bytes`.
///
/// ## Why is this bad?
/// The `__bytes__` method should return a `bytes` object. Returning a different
/// type may cause unexpected behavior.
///
/// ## Example
/// ```python
/// class Foo:
///     def __bytes__(self):
///         return 2
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     def __bytes__(self):
///         return b"2"
/// ```
///
/// ## References
/// - [Python documentation: The `__bytes__` method](https://docs.python.org/3/reference/datamodel.html#object.__bytes__)
#[derive(ViolationMetadata)]
pub struct InvalidBytesReturnType;

impl Violation for InvalidBytesReturnType {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`__bytes__` does not return `bytes`".to_string()
    }
}