use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for duplicate members in a `typing.Literal[]` slice.
///
/// ## Why is this bad?
/// Duplicate literal members are redundant and should be removed.
///
/// ## Example
/// ```python
/// foo: Literal["a", "b", "a"]
/// ```
///
/// Use instead:
/// ```python
/// foo: Literal["a", "b"]
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as safe, unless the type annotation contains comments.
///
/// Note that while the fix may flatten nested literals into a single top-level literal,
/// the semantics of the annotation will remain unchanged.
///
/// ## References
/// - [Python documentation: `typing.Literal`](https://docs.python.org/3/library/typing.html#typing.Literal)
#[derive(ViolationMetadata)]
pub struct DuplicateLiteralMember {
    duplicate_name: String,
}

impl AlwaysFixableViolation for DuplicateLiteralMember {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Duplicate literal member `{}`", self.duplicate_name)
    }

    fn fix_title(&self) -> String {
        "Remove duplicates".to_string()
    }
}