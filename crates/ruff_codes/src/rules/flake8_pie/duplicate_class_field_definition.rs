use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for duplicate field definitions in classes.
///
/// ## Why is this bad?
/// Defining a field multiple times in a class body is redundant and likely a
/// mistake.
///
/// ## Example
/// ```python
/// class Person:
///     name = Tom
///     ...
///     name = Ben
/// ```
///
/// Use instead:
/// ```python
/// class Person:
///     name = Tom
///     ...
/// ```
#[derive(ViolationMetadata)]
pub struct DuplicateClassFieldDefinition {
    name: String,
}

impl AlwaysFixableViolation for DuplicateClassFieldDefinition {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DuplicateClassFieldDefinition { name } = self;
        format!("Class field `{name}` is defined multiple times")
    }

    fn fix_title(&self) -> String {
        let DuplicateClassFieldDefinition { name } = self;
        format!("Remove duplicate field definition for `{name}`")
    }
}