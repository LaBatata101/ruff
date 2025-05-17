use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `readlines()` when iterating over a file line-by-line.
///
/// ## Why is this bad?
/// Rather than iterating over all lines in a file by calling `readlines()`,
/// it's more convenient and performant to iterate over the file object
/// directly.
///
/// ## Example
/// ```python
/// with open("file.txt") as fp:
///     for line in fp.readlines():
///         ...
/// ```
///
/// Use instead:
/// ```python
/// with open("file.txt") as fp:
///     for line in fp:
///         ...
/// ```
///
/// ## References
/// - [Python documentation: `io.IOBase.readlines`](https://docs.python.org/3/library/io.html#io.IOBase.readlines)
#[derive(ViolationMetadata)]
pub struct ReadlinesInFor;

impl AlwaysFixableViolation for ReadlinesInFor {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Instead of calling `readlines()`, iterate over file object directly".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `readlines()`".into()
    }
}