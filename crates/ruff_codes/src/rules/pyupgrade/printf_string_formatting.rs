use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `printf`-style string formatting, and offers to replace it with
/// `str.format` calls.
///
/// ## Why is this bad?
/// `printf`-style string formatting has a number of quirks, and leads to less
/// readable code than using `str.format` calls or f-strings. In general, prefer
/// the newer `str.format` and f-strings constructs over `printf`-style string
/// formatting.
///
/// ## Example
///
/// ```python
/// "%s, %s" % ("Hello", "World")  # "Hello, World"
/// ```
///
/// Use instead:
///
/// ```python
/// "{}, {}".format("Hello", "World")  # "Hello, World"
/// ```
///
/// ```python
/// f"{'Hello'}, {'World'}"  # "Hello, World"
/// ```
///
/// ## Fix safety
///
/// In cases where the format string contains a single generic format specifier
/// (e.g. `%s`), and the right-hand side is an ambiguous expression,
/// we cannot offer a safe fix.
///
/// For example, given:
///
/// ```python
/// "%s" % val
/// ```
///
/// `val` could be a single-element tuple, _or_ a single value (not
/// contained in a tuple). Both of these would resolve to the same
/// formatted string when using `printf`-style formatting, but
/// resolve differently when using f-strings:
///
/// ```python
/// val = 1
/// print("%s" % val)  # "1"
/// print("{}".format(val))  # "1"
///
/// val = (1,)
/// print("%s" % val)  # "1"
/// print("{}".format(val))  # "(1,)"
/// ```
///
/// ## References
/// - [Python documentation: `printf`-style String Formatting](https://docs.python.org/3/library/stdtypes.html#old-string-formatting)
/// - [Python documentation: `str.format`](https://docs.python.org/3/library/stdtypes.html#str.format)
#[derive(ViolationMetadata)]
pub struct PrintfStringFormatting;

impl Violation for PrintfStringFormatting {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use format specifiers instead of percent format".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with format specifiers".to_string())
    }
}