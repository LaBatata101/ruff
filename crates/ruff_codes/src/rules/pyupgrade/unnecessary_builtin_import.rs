use itertools::Itertools;
use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary imports of builtins.
///
/// ## Why is this bad?
/// Builtins are always available. Importing them is unnecessary and should be
/// removed to avoid confusion.
///
/// ## Example
/// ```python
/// from builtins import str
///
/// str(1)
/// ```
///
/// Use instead:
/// ```python
/// str(1)
/// ```
///
/// ## References
/// - [Python documentation: The Python Standard Library](https://docs.python.org/3/library/index.html)
#[derive(ViolationMetadata)]
pub struct UnnecessaryBuiltinImport {
    pub names: Vec<String>,
}

impl AlwaysFixableViolation for UnnecessaryBuiltinImport {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryBuiltinImport { names } = self;
        if names.len() == 1 {
            let import = &names[0];
            format!("Unnecessary builtin import: `{import}`")
        } else {
            let imports = names.iter().map(|name| format!("`{name}`")).join(", ");
            format!("Unnecessary builtin imports: {imports}")
        }
    }

    fn fix_title(&self) -> String {
        "Remove unnecessary builtin import".to_string()
    }
}