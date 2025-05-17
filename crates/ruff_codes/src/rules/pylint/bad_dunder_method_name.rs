use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for dunder methods that have no special meaning in Python 3.
///
/// ## Why is this bad?
/// Misspelled or no longer supported dunder name methods may cause your code to not function
/// as expected.
///
/// Since dunder methods are associated with customizing the behavior
/// of a class in Python, introducing a dunder method such as `__foo__`
/// that diverges from standard Python dunder methods could potentially
/// confuse someone reading the code.
///
/// This rule will detect all methods starting and ending with at least
/// one underscore (e.g., `_str_`), but ignores known dunder methods (like
/// `__init__`), as well as methods that are marked with `@override`.
///
/// Additional dunder methods names can be allowed via the
/// [`lint.pylint.allow-dunder-method-names`] setting.
///
/// ## Example
///
/// ```python
/// class Foo:
///     def __init_(self): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class Foo:
///     def __init__(self): ...
/// ```
///
/// ## Options
/// - `lint.pylint.allow-dunder-method-names`
#[derive(ViolationMetadata)]
pub struct BadDunderMethodName {
    name: String,
}

impl Violation for BadDunderMethodName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BadDunderMethodName { name } = self;
        format!("Dunder method `{name}` has no special meaning in Python 3")
    }
}