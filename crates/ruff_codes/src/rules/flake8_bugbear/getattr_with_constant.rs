use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `getattr` that take a constant attribute value as an
/// argument (e.g., `getattr(obj, "foo")`).
///
/// ## Why is this bad?
/// `getattr` is used to access attributes dynamically. If the attribute is
/// defined as a constant, it is no safer than a typical property access. When
/// possible, prefer property access over `getattr` calls, as the former is
/// more concise and idiomatic.
///
///
/// ## Example
/// ```python
/// getattr(obj, "foo")
/// ```
///
/// Use instead:
/// ```python
/// obj.foo
/// ```
///
/// ## References
/// - [Python documentation: `getattr`](https://docs.python.org/3/library/functions.html#getattr)
#[derive(ViolationMetadata)]
pub struct GetAttrWithConstant;

impl AlwaysFixableViolation for GetAttrWithConstant {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not call `getattr` with a constant attribute value. It is not any safer than \
            normal property access."
            .to_string()
    }

    fn fix_title(&self) -> String {
        "Replace `getattr` with attribute access".to_string()
    }
}