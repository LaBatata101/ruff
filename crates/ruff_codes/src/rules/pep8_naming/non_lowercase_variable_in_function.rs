use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of non-lowercase variable names in functions.
///
/// ## Why is this bad?
/// [PEP 8] recommends that all function variables use lowercase names:
///
/// > Function names should be lowercase, with words separated by underscores as necessary to
/// > improve readability. Variable names follow the same convention as function names. mixedCase
/// > is allowed only in contexts where that's already the prevailing style (e.g. threading.py),
/// > to retain backwards compatibility.
///
/// ## Example
/// ```python
/// def my_function(a):
///     B = a + 3
///     return B
/// ```
///
/// Use instead:
/// ```python
/// def my_function(a):
///     b = a + 3
///     return b
/// ```
///
/// ## Options
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#function-and-variable-names
#[derive(ViolationMetadata)]
pub struct NonLowercaseVariableInFunction {
    name: String,
}

impl Violation for NonLowercaseVariableInFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NonLowercaseVariableInFunction { name } = self;
        format!("Variable `{name}` in function should be lowercase")
    }
}