use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `str.format` calls that can be replaced with f-strings.
///
/// ## Why is this bad?
/// f-strings are more readable and generally preferred over `str.format`
/// calls.
///
/// ## Example
/// ```python
/// "{}".format(foo)
/// ```
///
/// Use instead:
/// ```python
/// f"{foo}"
/// ```
///
/// ## References
/// - [Python documentation: f-strings](https://docs.python.org/3/reference/lexical_analysis.html#f-strings)
#[derive(ViolationMetadata)]
pub struct FString;

impl Violation for FString {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use f-string instead of `format` call".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Convert to f-string".to_string())
    }
}