use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the unintentional use of type annotations.
///
/// ## Why is this bad?
/// The use of a colon (`:`) in lieu of an assignment (`=`) can be syntactically valid, but
/// is almost certainly a mistake when used in a subscript or attribute assignment.
///
/// ## Example
/// ```python
/// a["b"]: 1
/// ```
///
/// Use instead:
/// ```python
/// a["b"] = 1
/// ```
#[derive(ViolationMetadata)]
pub struct UnintentionalTypeAnnotation;

impl Violation for UnintentionalTypeAnnotation {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Possible unintentional type annotation (using `:`). Did you mean to assign (using `=`)?"
            .to_string()
    }
}