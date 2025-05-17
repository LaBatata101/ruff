use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for functions with "dunder" names (that is, names with two
/// leading and trailing underscores) that are not documented.
///
/// ## Why is this bad?
/// [PEP 8] recommends that only documented "dunder" methods are used:
///
/// > ..."magic" objects or attributes that live in user-controlled
/// > namespaces. E.g. `__init__`, `__import__` or `__file__`. Never invent
/// > such names; only use them as documented.
///
/// ## Example
/// ```python
/// def __my_function__():
///     pass
/// ```
///
/// Use instead:
/// ```python
/// def my_function():
///     pass
/// ```
///
/// ## Options
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/
#[derive(ViolationMetadata)]
pub struct DunderFunctionName;

impl Violation for DunderFunctionName {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Function name should not start and end with `__`".to_string()
    }
}