use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `datetime.fromisoformat()` calls
/// where the only argument is an inline replacement
/// of `Z` with a zero offset timezone.
///
/// ## Why is this bad?
/// On Python 3.11 and later, `datetime.fromisoformat()` can handle most [ISO 8601][iso-8601]
/// formats including ones affixed with `Z`, so such an operation is unnecessary.
///
/// More information on unsupported formats
/// can be found in [the official documentation][fromisoformat].
///
/// ## Example
///
/// ```python
/// from datetime import datetime
///
///
/// date = "2025-01-01T00:00:00Z"
///
/// datetime.fromisoformat(date.replace("Z", "+00:00"))
/// datetime.fromisoformat(date[:-1] + "-00")
/// datetime.fromisoformat(date.strip("Z", "-0000"))
/// datetime.fromisoformat(date.rstrip("Z", "-00:00"))
/// ```
///
/// Use instead:
///
/// ```python
/// from datetime import datetime
///
///
/// date = "2025-01-01T00:00:00Z"
///
/// datetime.fromisoformat(date)
/// ```
///
/// ## Fix safety
/// The fix is always marked as unsafe,
/// as it might change the program's behaviour.
///
/// For example, working code might become non-working:
///
/// ```python
/// d = "Z2025-01-01T00:00:00Z"  # Note the leading `Z`
///
/// datetime.fromisoformat(d.strip("Z") + "+00:00")  # Fine
/// datetime.fromisoformat(d)  # Runtime error
/// ```
///
/// ## References
/// * [What’s New In Python 3.11 &sect; `datetime`](https://docs.python.org/3/whatsnew/3.11.html#datetime)
/// * [`fromisoformat`](https://docs.python.org/3/library/datetime.html#datetime.date.fromisoformat)
///
/// [iso-8601]: https://www.iso.org/obp/ui/#iso:std:iso:8601
/// [fromisoformat]: https://docs.python.org/3/library/datetime.html#datetime.date.fromisoformat
#[derive(ViolationMetadata)]
pub struct FromisoformatReplaceZ;

impl AlwaysFixableViolation for FromisoformatReplaceZ {
    #[derive_message_formats]
    fn message(&self) -> String {
        r#"Unnecessary timezone replacement with zero offset"#.to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `.replace()` call".to_string()
    }
}