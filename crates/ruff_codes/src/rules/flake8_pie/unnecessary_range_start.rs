use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `range` calls with an unnecessary `start` argument.
///
/// ## Why is this bad?
/// `range(0, x)` is equivalent to `range(x)`, as `0` is the default value for
/// the `start` argument. Omitting the `start` argument makes the code more
/// concise and idiomatic.
///
/// ## Example
/// ```python
/// range(0, 3)
/// ```
///
/// Use instead:
/// ```python
/// range(3)
/// ```
///
/// ## References
/// - [Python documentation: `range`](https://docs.python.org/3/library/stdtypes.html#range)
#[derive(ViolationMetadata)]
pub struct UnnecessaryRangeStart;

impl AlwaysFixableViolation for UnnecessaryRangeStart {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary `start` argument in `range`".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `start` argument".to_string()
    }
}