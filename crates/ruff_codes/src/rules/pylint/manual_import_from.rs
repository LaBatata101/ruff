use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for submodule imports that are aliased to the submodule name.
///
/// ## Why is this bad?
/// Using the `from` keyword to import the submodule is more concise and
/// readable.
///
/// ## Example
/// ```python
/// import concurrent.futures as futures
/// ```
///
/// Use instead:
/// ```python
/// from concurrent import futures
/// ```
///
/// ## References
/// - [Python documentation: Submodules](https://docs.python.org/3/reference/import.html#submodules)
#[derive(ViolationMetadata)]
pub struct ManualFromImport {
    module: String,
    name: String,
}

impl Violation for ManualFromImport {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let ManualFromImport { module, name } = self;
        format!("Use `from {module} import {name}` in lieu of alias")
    }

    fn fix_title(&self) -> Option<String> {
        let ManualFromImport { module, name } = self;
        Some(format!("Replace with `from {module} import {name}`"))
    }
}