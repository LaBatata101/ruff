use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary dict comprehension when creating a dictionary from
/// an iterable.
///
/// ## Why is this bad?
/// It's unnecessary to use a dict comprehension to build a dictionary from
/// an iterable when the value is static.
///
/// Prefer `dict.fromkeys(iterable)` over `{value: None for value in iterable}`,
/// as `dict.fromkeys` is more readable and efficient.
///
/// ## Example
/// ```python
/// {a: None for a in iterable}
/// {a: 1 for a in iterable}
/// ```
///
/// Use instead:
/// ```python
/// dict.fromkeys(iterable)
/// dict.fromkeys(iterable, 1)
/// ```
///
/// ## References
/// - [Python documentation: `dict.fromkeys`](https://docs.python.org/3/library/stdtypes.html#dict.fromkeys)
#[derive(ViolationMetadata)]
pub struct UnnecessaryDictComprehensionForIterable {
    is_value_none_literal: bool,
}

impl Violation for UnnecessaryDictComprehensionForIterable {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary dict comprehension for iterable; use `dict.fromkeys` instead".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        let title = if self.is_value_none_literal {
            "Replace with `dict.fromkeys(iterable, value)`)"
        } else {
            "Replace with `dict.fromkeys(iterable)`)"
        };
        Some(title.to_string())
    }
}