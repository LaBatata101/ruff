use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `dict.get(key, falsy_value)` calls in boolean test positions.
///
/// ## Why is this bad?
/// The default fallback `None` is already falsy.
///
/// ## Example
///
/// ```python
/// if dict.get(key, False):
///     ...
/// ```
///
/// Use instead:
///
/// ```python
/// if dict.get(key):
///     ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as safe, unless the `dict.get()` call contains comments between arguments.
#[derive(ViolationMetadata)]
pub struct FalsyDictGetFallback;

impl AlwaysFixableViolation for FalsyDictGetFallback {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Avoid providing a falsy fallback to `dict.get()` in boolean test positions. The default fallback `None` is already falsy.".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove falsy fallback from `dict.get()`".to_string()
    }
}