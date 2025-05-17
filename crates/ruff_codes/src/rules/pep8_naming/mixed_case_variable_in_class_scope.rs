use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for class variable names that follow the `mixedCase` convention.
///
/// ## Why is this bad?
/// [PEP 8] recommends that variable names should be lower case and separated
/// by underscores (also known as `snake_case`).
///
/// > Function names should be lowercase, with words separated by underscores
/// > as necessary to improve readability.
/// >
/// > Variable names follow the same convention as function names.
/// >
/// > mixedCase is allowed only in contexts where that’s already the
/// > prevailing style (e.g. threading.py), to retain backwards compatibility.
///
/// ## Example
/// ```python
/// class MyClass:
///     myVariable = "hello"
///     another_variable = "world"
/// ```
///
/// Use instead:
/// ```python
/// class MyClass:
///     my_variable = "hello"
///     another_variable = "world"
/// ```
///
/// ## Options
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#function-and-method-arguments
#[derive(ViolationMetadata)]
pub struct MixedCaseVariableInClassScope {
    name: String,
}

impl Violation for MixedCaseVariableInClassScope {
    #[derive_message_formats]
    fn message(&self) -> String {
        let MixedCaseVariableInClassScope { name } = self;
        format!("Variable `{name}` in class scope should not be mixedCase")
    }
}