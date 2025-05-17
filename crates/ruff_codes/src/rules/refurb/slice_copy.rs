use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unbounded slice expressions to copy a list.
///
/// ## Why is this bad?
/// The `list.copy` method is more readable and consistent with copying other
/// types.
///
/// ## Known problems
/// This rule is prone to false negatives due to type inference limitations,
/// as it will only detect lists that are instantiated as literals or annotated
/// with a type annotation.
///
/// ## Example
/// ```python
/// a = [1, 2, 3]
/// b = a[:]
/// ```
///
/// Use instead:
/// ```python
/// a = [1, 2, 3]
/// b = a.copy()
/// ```
///
/// ## References
/// - [Python documentation: Mutable Sequence Types](https://docs.python.org/3/library/stdtypes.html#mutable-sequence-types)
#[derive(ViolationMetadata)]
pub struct SliceCopy;

impl Violation for SliceCopy {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Prefer `copy` method over slicing".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `copy()`".to_string())
    }
}