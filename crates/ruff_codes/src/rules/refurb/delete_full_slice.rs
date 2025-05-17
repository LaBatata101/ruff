use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `del` statements that delete the entire slice of a list or
/// dictionary.
///
/// ## Why is this bad?
/// It is faster and more succinct to remove all items via the `clear()`
/// method.
///
/// ## Known problems
/// This rule is prone to false negatives due to type inference limitations,
/// as it will only detect lists and dictionaries that are instantiated as
/// literals or annotated with a type annotation.
///
/// ## Example
/// ```python
/// names = {"key": "value"}
/// nums = [1, 2, 3]
///
/// del names[:]
/// del nums[:]
/// ```
///
/// Use instead:
/// ```python
/// names = {"key": "value"}
/// nums = [1, 2, 3]
///
/// names.clear()
/// nums.clear()
/// ```
///
/// ## References
/// - [Python documentation: Mutable Sequence Types](https://docs.python.org/3/library/stdtypes.html?highlight=list#mutable-sequence-types)
/// - [Python documentation: `dict.clear()`](https://docs.python.org/3/library/stdtypes.html?highlight=list#dict.clear)
#[derive(ViolationMetadata)]
pub struct DeleteFullSlice;

impl Violation for DeleteFullSlice {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Prefer `clear` over deleting a full slice".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `clear()`".to_string())
    }
}