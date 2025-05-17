use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for `type: ignore` annotations that suppress all type warnings, as
/// opposed to targeting specific type warnings.
///
/// ## Why is this bad?
/// Suppressing all warnings can hide issues in the code.
///
/// Blanket `type: ignore` annotations are also more difficult to interpret and
/// maintain, as the annotation does not clarify which warnings are intended
/// to be suppressed.
///
/// ## Example
/// ```python
/// from foo import secrets  # type: ignore
/// ```
///
/// Use instead:
/// ```python
/// from foo import secrets  # type: ignore[attr-defined]
/// ```
///
/// ## References
/// Mypy supports a [built-in setting](https://mypy.readthedocs.io/en/stable/error_code_list2.html#check-that-type-ignore-include-an-error-code-ignore-without-code)
/// to enforce that all `type: ignore` annotations include an error code, akin
/// to enabling this rule:
/// ```toml
/// [tool.mypy]
/// enable_error_code = ["ignore-without-code"]
/// ```
#[derive(ViolationMetadata)]
pub struct BlanketTypeIgnore;

impl Violation for BlanketTypeIgnore {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use specific rule codes when ignoring type issues".to_string()
    }
}