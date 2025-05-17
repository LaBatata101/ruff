use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for duplicate base classes in class definitions.
///
/// ## Why is this bad?
/// Including duplicate base classes will raise a `TypeError` at runtime.
///
/// ## Example
/// ```python
/// class Foo:
///     pass
///
///
/// class Bar(Foo, Foo):
///     pass
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     pass
///
///
/// class Bar(Foo):
///     pass
/// ```
///
/// ## References
/// - [Python documentation: Class definitions](https://docs.python.org/3/reference/compound_stmts.html#class-definitions)
#[derive(ViolationMetadata)]
pub struct DuplicateBases {
    base: String,
    class: String,
}

impl Violation for DuplicateBases {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let DuplicateBases { base, class } = self;
        format!("Duplicate base `{base}` for class `{class}`")
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove duplicate base".to_string())
    }
}