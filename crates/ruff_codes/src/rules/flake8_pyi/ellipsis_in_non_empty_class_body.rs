use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Removes ellipses (`...`) in otherwise non-empty class bodies.
///
/// ## Why is this bad?
/// An ellipsis in a class body is only necessary if the class body is
/// otherwise empty. If the class body is non-empty, then the ellipsis
/// is redundant.
///
/// ## Example
/// ```pyi
/// class Foo:
///     ...
///     value: int
/// ```
///
/// Use instead:
/// ```pyi
/// class Foo:
///     value: int
/// ```
#[derive(ViolationMetadata)]
pub struct EllipsisInNonEmptyClassBody;

impl Violation for EllipsisInNonEmptyClassBody {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Non-empty class body must not contain `...`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove unnecessary `...`".to_string())
    }
}