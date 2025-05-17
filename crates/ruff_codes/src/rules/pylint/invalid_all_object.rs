use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the inclusion of invalid objects in `__all__`.
///
/// ## Why is this bad?
/// In Python, `__all__` should contain a sequence of strings that represent
/// the names of all "public" symbols exported by a module.
///
/// Assigning anything other than a `tuple` or `list` of strings to `__all__`
/// is invalid.
///
/// ## Example
/// ```python
/// __all__ = [Foo, 1, None]
/// ```
///
/// Use instead:
/// ```python
/// __all__ = ["Foo", "Bar", "Baz"]
/// ```
///
/// ## References
/// - [Python documentation: The `import` statement](https://docs.python.org/3/reference/simple_stmts.html#the-import-statement)
#[derive(ViolationMetadata)]
pub struct InvalidAllObject;

impl Violation for InvalidAllObject {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Invalid object in `__all__`, must contain only strings".to_string()
    }
}