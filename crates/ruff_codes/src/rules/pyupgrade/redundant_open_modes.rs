use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for redundant `open` mode arguments.
///
/// ## Why is this bad?
/// Redundant `open` mode arguments are unnecessary and should be removed to
/// avoid confusion.
///
/// ## Example
/// ```python
/// with open("foo.txt", "r") as f:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// with open("foo.txt") as f:
///     ...
/// ```
///
/// ## References
/// - [Python documentation: `open`](https://docs.python.org/3/library/functions.html#open)
#[derive(ViolationMetadata)]
pub struct RedundantOpenModes {
    replacement: String,
}

impl AlwaysFixableViolation for RedundantOpenModes {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RedundantOpenModes { replacement } = self;
        if replacement.is_empty() {
            "Unnecessary mode argument".to_string()
        } else {
            format!("Unnecessary modes, use `{replacement}`")
        }
    }

    fn fix_title(&self) -> String {
        let RedundantOpenModes { replacement } = self;
        if replacement.is_empty() {
            "Remove mode argument".to_string()
        } else {
            format!("Replace with `{replacement}`")
        }
    }
}