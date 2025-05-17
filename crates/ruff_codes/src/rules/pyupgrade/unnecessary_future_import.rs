use itertools::Itertools;
use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary `__future__` imports.
///
/// ## Why is this bad?
/// The `__future__` module is used to enable features that are not yet
/// available in the current Python version. If a feature is already
/// available in the minimum supported Python version, importing it
/// from `__future__` is unnecessary and should be removed to avoid
/// confusion.
///
/// ## Example
/// ```python
/// from __future__ import print_function
///
/// print("Hello, world!")
/// ```
///
/// Use instead:
/// ```python
/// print("Hello, world!")
/// ```
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `__future__` — Future statement definitions](https://docs.python.org/3/library/__future__.html)
#[derive(ViolationMetadata)]
pub struct UnnecessaryFutureImport {
    pub names: Vec<String>,
}

impl AlwaysFixableViolation for UnnecessaryFutureImport {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryFutureImport { names } = self;
        if names.len() == 1 {
            let import = &names[0];
            format!("Unnecessary `__future__` import `{import}` for target Python version")
        } else {
            let imports = names.iter().map(|name| format!("`{name}`")).join(", ");
            format!("Unnecessary `__future__` imports {imports} for target Python version")
        }
    }

    fn fix_title(&self) -> String {
        "Remove unnecessary `__future__` import".to_string()
    }
}