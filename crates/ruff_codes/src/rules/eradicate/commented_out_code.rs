use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for commented-out Python code.
///
/// ## Why is this bad?
/// Commented-out code is dead code, and is often included inadvertently.
/// It should be removed.
///
/// ## Known problems
/// Prone to false positives when checking comments that resemble Python code,
/// but are not actually Python code ([#4845]).
///
/// ## Example
/// ```python
/// # print("Hello, world!")
/// ```
///
/// ## Options
/// - `lint.task-tags`
///
/// [#4845]: https://github.com/astral-sh/ruff/issues/4845
#[derive(ViolationMetadata)]
pub struct CommentedOutCode;

impl Violation for CommentedOutCode {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::None;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Found commented-out code".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove commented-out code".to_string())
    }
}
