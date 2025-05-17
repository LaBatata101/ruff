use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::MinMax;

/// ## What it does
/// Checks for uses of `datetime.datetime.min` and `datetime.datetime.max`.
///
/// ## Why is this bad?
/// `datetime.min` and `datetime.max` are non-timezone-aware datetime objects.
///
/// As such, operations on `datetime.min` and `datetime.max` may behave
/// unexpectedly, as in:
///
/// ```python
/// # Timezone: UTC-14
/// datetime.min.timestamp()  # ValueError: year 0 is out of range
/// datetime.max.timestamp()  # ValueError: year 10000 is out of range
/// ```
///
/// ## Example
/// ```python
/// datetime.max
/// ```
///
/// Use instead:
/// ```python
/// datetime.max.replace(tzinfo=datetime.UTC)
/// ```
#[derive(ViolationMetadata)]
pub struct DatetimeMinMax {
    min_max: MinMax,
}

impl Violation for DatetimeMinMax {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DatetimeMinMax { min_max } = self;
        format!("Use of `datetime.datetime.{min_max}` without timezone information")
    }

    fn fix_title(&self) -> Option<String> {
        let DatetimeMinMax { min_max } = self;
        Some(format!(
            "Replace with `datetime.datetime.{min_max}.replace(tzinfo=...)`"
        ))
    }
}