use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `setattr` that take a constant attribute value as an
/// argument (e.g., `setattr(obj, "foo", 42)`).
///
/// ## Why is this bad?
/// `setattr` is used to set attributes dynamically. If the attribute is
/// defined as a constant, it is no safer than a typical property access. When
/// possible, prefer property access over `setattr` calls, as the former is
/// more concise and idiomatic.
///
/// ## Example
/// ```python
/// setattr(obj, "foo", 42)
/// ```
///
/// Use instead:
/// ```python
/// obj.foo = 42
/// ```
///
/// ## References
/// - [Python documentation: `setattr`](https://docs.python.org/3/library/functions.html#setattr)
#[derive(ViolationMetadata)]
pub struct SetAttrWithConstant;

impl AlwaysFixableViolation for SetAttrWithConstant {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not call `setattr` with a constant attribute value. It is not any safer than \
             normal property access."
            .to_string()
    }

    fn fix_title(&self) -> String {
        "Replace `setattr` with assignment".to_string()
    }
}