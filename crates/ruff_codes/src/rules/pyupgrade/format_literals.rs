use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary positional indices in format strings.
///
/// ## Why is this bad?
/// In Python 3.1 and later, format strings can use implicit positional
/// references. For example, `"{0}, {1}".format("Hello", "World")` can be
/// rewritten as `"{}, {}".format("Hello", "World")`.
///
/// If the positional indices appear exactly in-order, they can be omitted
/// in favor of automatic indices to improve readability.
///
/// ## Example
/// ```python
/// "{0}, {1}".format("Hello", "World")  # "Hello, World"
/// ```
///
/// Use instead:
/// ```python
/// "{}, {}".format("Hello", "World")  # "Hello, World"
/// ```
///
/// This fix is marked as unsafe because:
/// - Comments attached to arguments are not moved, which can cause comments to mismatch the actual arguments.
/// - If arguments have side effects (e.g., print), reordering may change program behavior.
///
/// ## References
/// - [Python documentation: Format String Syntax](https://docs.python.org/3/library/string.html#format-string-syntax)
/// - [Python documentation: `str.format`](https://docs.python.org/3/library/stdtypes.html#str.format)
#[derive(ViolationMetadata)]
pub struct FormatLiterals;

impl Violation for FormatLiterals {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use implicit references for positional format fields".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove explicit positional indices".to_string())
    }
}