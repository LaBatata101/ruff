use ruff_diagnostics::{FixAvailability, Violation};
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for redundant `Final[Literal[...]]` annotations.
///
/// ## Why is this bad?
/// All constant variables annotated as `Final` are understood as implicitly
/// having `Literal` types by a type checker. As such, a `Final[Literal[...]]`
/// annotation can often be replaced with a bare `Final`, annotation, which
/// will have the same meaning to the type checker while being more concise and
/// more readable.
///
/// ## Example
///
/// ```pyi
/// from typing import Final, Literal
///
/// x: Final[Literal[42]]
/// y: Final[Literal[42]] = 42
/// ```
///
/// Use instead:
/// ```pyi
/// from typing import Final, Literal
///
/// x: Final = 42
/// y: Final = 42
/// ```
#[derive(ViolationMetadata)]
pub struct RedundantFinalLiteral {
    literal: SourceCodeSnippet,
}

impl Violation for RedundantFinalLiteral {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let RedundantFinalLiteral { literal } = self;
        format!(
            "`Final[Literal[{literal}]]` can be replaced with a bare `Final`",
            literal = literal.truncated_display()
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `Final`".to_string())
    }
}