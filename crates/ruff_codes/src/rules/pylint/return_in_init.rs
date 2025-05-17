use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `__init__` methods that return values.
///
/// ## Why is this bad?
/// The `__init__` method is the constructor for a given Python class,
/// responsible for initializing, rather than creating, new objects.
///
/// The `__init__` method has to return `None`. Returning any value from
/// an `__init__` method will result in a runtime error.
///
/// ## Example
/// ```python
/// class Example:
///     def __init__(self):
///         return []
/// ```
///
/// Use instead:
/// ```python
/// class Example:
///     def __init__(self):
///         self.value = []
/// ```
///
/// ## References
/// - [CodeQL: `py-explicit-return-in-init`](https://codeql.github.com/codeql-query-help/python/py-explicit-return-in-init/)
#[derive(ViolationMetadata)]
pub struct ReturnInInit;

impl Violation for ReturnInInit {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Explicit return in `__init__`".to_string()
    }
}