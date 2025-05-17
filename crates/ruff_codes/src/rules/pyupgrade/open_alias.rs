use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `io.open`.
///
/// ## Why is this bad?
/// In Python 3, `io.open` is an alias for `open`. Prefer using `open` directly,
/// as it is more idiomatic.
///
/// ## Example
/// ```python
/// import io
///
/// with io.open("file.txt") as f:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// with open("file.txt") as f:
///     ...
/// ```
///
/// ## References
/// - [Python documentation: `io.open`](https://docs.python.org/3/library/io.html#io.open)
#[derive(ViolationMetadata)]
pub struct OpenAlias;

impl Violation for OpenAlias {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use builtin `open`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with builtin `open`".to_string())
    }
}