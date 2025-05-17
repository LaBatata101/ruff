use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for imports that use the same names as builtins.
///
/// ## Why is this bad?
/// Reusing a builtin for the name of an import increases the difficulty
/// of reading and maintaining the code, and can cause non-obvious errors,
/// as readers may mistake the variable for the builtin and vice versa.
///
/// Builtins can be marked as exceptions to this rule via the
/// [`lint.flake8-builtins.ignorelist`] configuration option.
///
/// ## Example
/// ```python
/// from rich import print
///
/// print("Some message")
/// ```
///
/// Use instead:
/// ```python
/// from rich import print as rich_print
///
/// rich_print("Some message")
/// ```
///
/// or:
/// ```python
/// import rich
///
/// rich.print("Some message")
/// ```
///
/// ## Options
/// - `lint.flake8-builtins.ignorelist`
/// - `target-version`
///
#[derive(ViolationMetadata)]
pub struct BuiltinImportShadowing {
    name: String,
}

impl Violation for BuiltinImportShadowing {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BuiltinImportShadowing { name } = self;
        format!("Import `{name}` is shadowing a Python builtin")
    }
}