use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `for` loops that can be replaced with a builtin function, like
/// `any` or `all`.
///
/// ## Why is this bad?
/// Using a builtin function is more concise and readable.
///
/// ## Example
/// ```python
/// for item in iterable:
///     if predicate(item):
///         return True
/// return False
/// ```
///
/// Use instead:
/// ```python
/// return any(predicate(item) for item in iterable)
/// ```
///
/// ## References
/// - [Python documentation: `any`](https://docs.python.org/3/library/functions.html#any)
/// - [Python documentation: `all`](https://docs.python.org/3/library/functions.html#all)
#[derive(ViolationMetadata)]
pub struct ReimplementedBuiltin {
    replacement: String,
}

impl Violation for ReimplementedBuiltin {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let ReimplementedBuiltin { replacement } = self;
        format!("Use `{replacement}` instead of `for` loop")
    }

    fn fix_title(&self) -> Option<String> {
        let ReimplementedBuiltin { replacement } = self;
        Some(format!("Replace with `{replacement}`"))
    }
}