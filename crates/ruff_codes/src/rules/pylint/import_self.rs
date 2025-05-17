use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for import statements that import the current module.
///
/// ## Why is this bad?
/// Importing a module from itself is a circular dependency and results
/// in an `ImportError` exception.
///
/// ## Example
///
/// ```python
/// # file: this_file.py
/// from this_file import foo
///
///
/// def foo(): ...
/// ```
#[derive(ViolationMetadata)]
pub struct ImportSelf {
    name: String,
}

impl Violation for ImportSelf {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { name } = self;
        format!("Module `{name}` imports itself")
    }
}