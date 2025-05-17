use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `round()` calls that have no effect on the input.
///
/// ## Why is this bad?
/// Rounding a value that's already an integer is unnecessary.
/// It's clearer to use the value directly.
///
/// ## Example
///
/// ```python
/// a = round(1, 0)
/// ```
///
/// Use instead:
///
/// ```python
/// a = 1
/// ```
///
/// ## Fix safety
///
/// The fix is marked unsafe if it is not possible to guarantee that the first argument of
/// `round()` is of type `int`, or if the fix deletes comments.
///
#[derive(ViolationMetadata)]
pub struct UnnecessaryRound;

impl AlwaysFixableViolation for UnnecessaryRound {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Value being rounded is already an integer".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove unnecessary `round` call".to_string()
    }
}