use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for argument names that do not follow the `snake_case` convention.
///
/// ## Why is this bad?
/// [PEP 8] recommends that function names should be lower case and separated
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
/// Methods decorated with `@typing.override` are ignored.
///
/// ## Example
/// ```python
/// def my_function(A, myArg):
///     pass
/// ```
///
/// Use instead:
/// ```python
/// def my_function(a, my_arg):
///     pass
/// ```
///
/// ## Options
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/#function-and-method-arguments
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct InvalidArgumentName {
    name: String,
}

impl Violation for InvalidArgumentName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let InvalidArgumentName { name } = self;
        format!("Argument name `{name}` should be lowercase")
    }
}