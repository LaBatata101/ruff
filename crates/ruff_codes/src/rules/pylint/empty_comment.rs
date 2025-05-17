use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for a # symbol appearing on a line not followed by an actual comment.
///
/// ## Why is this bad?
/// Empty comments don't provide any clarity to the code, and just add clutter.
/// Either add a comment or delete the empty comment.
///
/// ## Example
/// ```python
/// class Foo:  #
///     pass
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     pass
/// ```
///
/// ## References
/// - [Pylint documentation](https://pylint.pycqa.org/en/latest/user_guide/messages/refactor/empty-comment.html)
#[derive(ViolationMetadata)]
pub struct EmptyComment;

impl Violation for EmptyComment {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Always;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Line with empty comment".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Delete the empty comment".to_string())
    }
}