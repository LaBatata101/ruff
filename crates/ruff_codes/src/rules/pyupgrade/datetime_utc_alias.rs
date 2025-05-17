use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `datetime.timezone.utc`.
///
/// ## Why is this bad?
/// As of Python 3.11, `datetime.UTC` is an alias for `datetime.timezone.utc`.
/// The alias is more readable and generally preferred over the full path.
///
/// ## Example
/// ```python
/// import datetime
///
/// datetime.timezone.utc
/// ```
///
/// Use instead:
/// ```python
/// import datetime
///
/// datetime.UTC
/// ```
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [Python documentation: `datetime.UTC`](https://docs.python.org/3/library/datetime.html#datetime.UTC)
#[derive(ViolationMetadata)]
pub struct DatetimeTimezoneUTC;

impl Violation for DatetimeTimezoneUTC {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `datetime.UTC` alias".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Convert to `datetime.UTC` alias".to_string())
    }
}