use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for explicit casts to `list` on for-loop iterables.
///
/// ## Why is this bad?
/// Using a `list()` call to eagerly iterate over an already-iterable type
/// (like a tuple, list, or set) is inefficient, as it forces Python to create
/// a new list unnecessarily.
///
/// Removing the `list()` call will not change the behavior of the code, but
/// may improve performance.
///
/// Note that, as with all `perflint` rules, this is only intended as a
/// micro-optimization, and will have a negligible impact on performance in
/// most cases.
///
/// ## Example
/// ```python
/// items = (1, 2, 3)
/// for i in list(items):
///     print(i)
/// ```
///
/// Use instead:
/// ```python
/// items = (1, 2, 3)
/// for i in items:
///     print(i)
/// ```
#[derive(ViolationMetadata)]
pub struct UnnecessaryListCast;

impl AlwaysFixableViolation for UnnecessaryListCast {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not cast an iterable to `list` before iterating over it".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `list()` cast".to_string()
    }
}