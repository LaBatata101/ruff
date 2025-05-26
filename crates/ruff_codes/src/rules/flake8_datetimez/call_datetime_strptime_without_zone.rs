use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::DatetimeModuleAntipattern;

/// ## What it does
/// Checks for uses of `datetime.datetime.strptime()` that lead to naive
/// datetime objects.
///
/// ## Why is this bad?
/// Python datetime objects can be naive or timezone-aware. While an aware
/// object represents a specific moment in time, a naive object does not
/// contain enough information to unambiguously locate itself relative to other
/// datetime objects. Since this can lead to errors, it is recommended to
/// always use timezone-aware objects.
///
/// `datetime.datetime.strptime()` without `%z` returns a naive datetime
/// object. Follow it with `.replace(tzinfo=<timezone>)` or `.astimezone()`.
///
/// ## Example
/// ```python
/// import datetime
///
/// datetime.datetime.strptime("2022/01/31", "%Y/%m/%d")
/// ```
///
/// Instead, use `.replace(tzinfo=<timezone>)`:
/// ```python
/// import datetime
///
/// datetime.datetime.strptime("2022/01/31", "%Y/%m/%d").replace(
///     tzinfo=datetime.timezone.utc
/// )
/// ```
///
/// Or, use `.astimezone()`:
/// ```python
/// import datetime
///
/// datetime.datetime.strptime("2022/01/31", "%Y/%m/%d").astimezone(datetime.timezone.utc)
/// ```
///
/// On Python 3.11 and later, `datetime.timezone.utc` can be replaced with
/// `datetime.UTC`.
///
/// ## References
/// - [Python documentation: Aware and Naive Objects](https://docs.python.org/3/library/datetime.html#aware-and-naive-objects)
/// - [Python documentation: `strftime()` and `strptime()` Behavior](https://docs.python.org/3/library/datetime.html#strftime-and-strptime-behavior)
#[derive(ViolationMetadata)]
pub struct CallDatetimeStrptimeWithoutZone(DatetimeModuleAntipattern);

impl Violation for CallDatetimeStrptimeWithoutZone {
    #[derive_message_formats]
    fn message(&self) -> String {
        let CallDatetimeStrptimeWithoutZone(antipattern) = self;
        match antipattern {
            DatetimeModuleAntipattern::NoTzArgumentPassed => {
                "Naive datetime constructed using `datetime.datetime.strptime()` without %z"
                    .to_string()
            }
            DatetimeModuleAntipattern::NonePassedToTzArgument => {
                "`datetime.datetime.strptime(...).replace(tz=None)` used".to_string()
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let CallDatetimeStrptimeWithoutZone(antipattern) = self;
        let title = match antipattern {
            DatetimeModuleAntipattern::NoTzArgumentPassed => {
                "Call `.replace(tzinfo=<timezone>)` or `.astimezone()` \
                to convert to an aware datetime"
            }
            DatetimeModuleAntipattern::NonePassedToTzArgument => {
                "Pass a `datetime.timezone` object to the `tzinfo` parameter"
            }
        };
        Some(title.to_string())
    }
}