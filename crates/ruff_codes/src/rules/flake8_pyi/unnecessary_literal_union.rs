use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of multiple literal types in a union.
///
/// ## Why is this bad?
/// `Literal["foo", 42]` has identical semantics to
/// `Literal["foo"] | Literal[42]`, but is clearer and more concise.
///
/// ## Example
/// ```pyi
/// from typing import Literal
///
/// field: Literal[1] | Literal[2] | str
/// ```
///
/// Use instead:
/// ```pyi
/// from typing import Literal
///
/// field: Literal[1, 2] | str
/// ```
///
/// ## Fix safety
/// This fix is marked unsafe if it would delete any comments within the replacement range.
///
/// An example to illustrate where comments are preserved and where they are not:
///
/// ```pyi
/// from typing import Literal
///
/// field: (
///     # deleted comment
///     Literal["a", "b"]  # deleted comment
///     # deleted comment
///     | Literal["c", "d"]  # preserved comment
/// )
/// ```
///
/// ## References
/// - [Python documentation: `typing.Literal`](https://docs.python.org/3/library/typing.html#typing.Literal)
#[derive(ViolationMetadata)]
pub struct UnnecessaryLiteralUnion {
    members: Vec<String>,
}

impl Violation for UnnecessaryLiteralUnion {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "Multiple literal members in a union. Use a single literal, e.g. `Literal[{}]`",
            self.members.join(", ")
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with a single `Literal`".to_string())
    }
}