use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `CamelCase` imports that are aliased as acronyms.
///
/// ## Why is this bad?
/// [PEP 8] recommends naming conventions for classes, functions,
/// constants, and more. The use of inconsistent naming styles between
/// import and alias names may lead readers to expect an import to be of
/// another type (e.g., confuse a Python class with a constant).
///
/// Import aliases should thus follow the same naming style as the member
/// being imported.
///
/// Note that this rule is distinct from `camelcase-imported-as-constant`
/// to accommodate selective enforcement.
///
/// Also note that import aliases following an import convention according to the
/// [`lint.flake8-import-conventions.aliases`] option are allowed.
///
/// ## Example
/// ```python
/// from example import MyClassName as MCN
/// ```
///
/// Use instead:
/// ```python
/// from example import MyClassName
/// ```
///
/// ## Options
/// - `lint.flake8-import-conventions.aliases`
/// - `lint.pep8-naming.ignore-names`
/// - `lint.pep8-naming.extend-ignore-names`
///
/// [PEP 8]: https://peps.python.org/pep-0008/
#[derive(ViolationMetadata)]
pub struct CamelcaseImportedAsAcronym {
    name: String,
    asname: String,
}

impl Violation for CamelcaseImportedAsAcronym {
    #[derive_message_formats]
    fn message(&self) -> String {
        let CamelcaseImportedAsAcronym { name, asname } = self;
        format!("CamelCase `{name}` imported as acronym `{asname}`")
    }
}