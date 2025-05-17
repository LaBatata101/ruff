use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::DatetimeModuleAntipattern;

/// ## What it does
/// Checks for usage of `datetime.datetime.fromtimestamp()` that do not specify
/// a timezone.
///
/// ## Why is this bad?
/// Python datetime objects can be naive or timezone-aware. While an aware
/// object represents a specific moment in time, a naive object does not
/// contain enough information to unambiguously locate itself relative to other
/// datetime objects. Since this can lead to errors, it is recommended to
/// always use timezone-aware objects.
///
/// `datetime.datetime.fromtimestamp(ts)` or
/// `datetime.datetime.fromtimestampe(ts, tz=None)` returns a naive datetime
/// object. Instead, use `datetime.datetime.fromtimestamp(ts, tz=<timezone>)`
/// to create a timezone-aware object.
///
/// ## Example
/// ```python
/// import datetime
///
/// datetime.datetime.fromtimestamp(946684800)
/// ```
///
/// Use instead:
/// ```python
/// import datetime
///
/// datetime.datetime.fromtimestamp(946684800, tz=datetime.timezone.utc)
/// ```
///
/// Or, on Python 3.11 and later:
/// ```python
/// import datetime
///
/// datetime.datetime.fromtimestamp(946684800, tz=datetime.UTC)
/// ```
///
/// ## References
/// - [Python documentation: Aware and Naive Objects](https://docs.python.org/3/library/datetime.html#aware-and-naive-objects)
#[derive(ViolationMetadata)]
pub struct CallDatetimeFromtimestamp(DatetimeModuleAntipattern);

impl Violation for CallDatetimeFromtimestamp {
    #[derive_message_formats]
    fn message(&self) -> String {
        let CallDatetimeFromtimestamp(antipattern) = self;
        match antipattern {
            DatetimeModuleAntipattern::NoTzArgumentPassed => {
                "`datetime.datetime.fromtimestamp()` called without a `tz` argument".to_string()
            }
            DatetimeModuleAntipattern::NonePassedToTzArgument => {
                "`tz=None` passed to `datetime.datetime.fromtimestamp()`".to_string()
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        Some("Pass a `datetime.timezone` object to the `tz` parameter".to_string())
    }
}
