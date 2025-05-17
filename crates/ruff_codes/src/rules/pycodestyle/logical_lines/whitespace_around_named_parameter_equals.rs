use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for missing whitespace around the equals sign in an unannotated
/// function keyword parameter.
///
/// ## Why is this bad?
/// According to [PEP 8], there should be no spaces around the equals sign in a
/// keyword parameter, if it is unannotated:
///
/// > Don’t use spaces around the = sign when used to indicate a keyword
/// > argument, or when used to indicate a default value for an unannotated
/// > function parameter.
///
/// ## Example
/// ```python
/// def add(a = 0) -> int:
///     return a + 1
/// ```
///
/// Use instead:
/// ```python
/// def add(a=0) -> int:
///     return a + 1
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#whitespace-in-expressions-and-statements
#[derive(ViolationMetadata)]
pub struct UnexpectedSpacesAroundKeywordParameterEquals;

impl AlwaysFixableViolation for UnexpectedSpacesAroundKeywordParameterEquals {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unexpected spaces around keyword / parameter equals".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove whitespace".to_string()
    }
}

/// ## What it does
/// Checks for missing whitespace around the equals sign in an annotated
/// function keyword parameter.
///
/// ## Why is this bad?
/// According to [PEP 8], the spaces around the equals sign in a keyword
/// parameter should only be omitted when the parameter is unannotated:
///
/// > Don’t use spaces around the = sign when used to indicate a keyword
/// > argument, or when used to indicate a default value for an unannotated
/// > function parameter.
///
/// ## Example
/// ```python
/// def add(a: int=0) -> int:
///     return a + 1
/// ```
///
/// Use instead:
/// ```python
/// def add(a: int = 0) -> int:
///     return a + 1
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#whitespace-in-expressions-and-statements
#[derive(ViolationMetadata)]
pub struct MissingWhitespaceAroundParameterEquals;

impl AlwaysFixableViolation for MissingWhitespaceAroundParameterEquals {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Missing whitespace around parameter equals".to_string()
    }

    fn fix_title(&self) -> String {
        "Add missing whitespace".to_string()
    }
}
