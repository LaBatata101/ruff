use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of non-ASCII characters in import statements.
///
/// ## Why is this bad?
/// The use of non-ASCII characters in import statements can cause confusion
/// and compatibility issues (see: [PEP 672]).
///
/// ## Example
/// ```python
/// import bár
/// ```
///
/// Use instead:
/// ```python
/// import bar
/// ```
///
/// If the module is third-party, use an ASCII-only alias:
/// ```python
/// import bár as bar
/// ```
///
/// [PEP 672]: https://peps.python.org/pep-0672/
#[derive(ViolationMetadata)]
pub struct NonAsciiImportName {
    name: String,
    kind: Kind,
}

impl Violation for NonAsciiImportName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name, kind } = self;
        match kind {
            Kind::Aliased => {
                format!("Module alias `{name}` contains a non-ASCII character")
            }
            Kind::Unaliased => {
                format!("Module name `{name}` contains a non-ASCII character")
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        Some("Use an ASCII-only alias".to_string())
    }
}

// FIX: duplicated with rufe_rule_pylint::non_ascii_module_import
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Kind {
    /// The import uses a non-ASCII alias (e.g., `import foo as bár`).
    Aliased,
    /// The imported module is non-ASCII, and could be given an ASCII alias (e.g., `import bár`).
    Unaliased,
}