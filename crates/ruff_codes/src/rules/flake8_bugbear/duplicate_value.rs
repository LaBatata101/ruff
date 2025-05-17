use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for set literals that contain duplicate items.
///
/// ## Why is this bad?
/// In Python, sets are unordered collections of unique elements. Including a
/// duplicate item in a set literal is redundant, as the duplicate item will be
/// replaced with a single item at runtime.
///
/// ## Example
/// ```python
/// {1, 2, 3, 1}
/// ```
///
/// Use instead:
/// ```python
/// {1, 2, 3}
/// ```
#[derive(ViolationMetadata)]
pub struct DuplicateValue {
    value: String,
    existing: String,
}

impl Violation for DuplicateValue {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let DuplicateValue { value, existing } = self;
        if value == existing {
            format!("Sets should not contain duplicate item `{value}`")
        } else {
            format!(
                "Sets should not contain duplicate items, but `{existing}` and `{value}` has the same value"
            )
        }
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove duplicate item".to_string())
    }
}