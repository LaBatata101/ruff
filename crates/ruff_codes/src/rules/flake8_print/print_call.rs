use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `print` statements.
///
/// ## Why is this bad?
/// `print` statements are useful in some situations (e.g., debugging), but
/// should typically be omitted from production code. `print` statements can
/// lead to the accidental inclusion of sensitive information in logs, and are
/// not configurable by clients, unlike `logging` statements.
///
/// ## Example
/// ```python
/// def add_numbers(a, b):
///     print(f"The sum of {a} and {b} is {a + b}")
///     return a + b
/// ```
///
/// Use instead:
/// ```python
/// def add_numbers(a, b):
///     return a + b
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may remove `print` statements
/// that are used beyond debugging purposes.
#[derive(ViolationMetadata)]
pub struct Print;

impl Violation for Print {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`print` found".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove `print`".to_string())
    }
}

/// ## What it does
/// Checks for `pprint` statements.
///
/// ## Why is this bad?
/// Like `print` statements, `pprint` statements are useful in some situations
/// (e.g., debugging), but should typically be omitted from production code.
/// `pprint` statements can lead to the accidental inclusion of sensitive
/// information in logs, and are not configurable by clients, unlike `logging`
/// statements.
///
/// ## Example
/// ```python
/// import pprint
///
///
/// def merge_dicts(dict_a, dict_b):
///     dict_c = {**dict_a, **dict_b}
///     pprint.pprint(dict_c)
///     return dict_c
/// ```
///
/// Use instead:
/// ```python
/// def merge_dicts(dict_a, dict_b):
///     dict_c = {**dict_a, **dict_b}
///     return dict_c
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may remove `pprint` statements
/// that are used beyond debugging purposes.
#[derive(ViolationMetadata)]
pub struct PPrint;

impl Violation for PPrint {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`pprint` found".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove `pprint`".to_string())
    }
}
