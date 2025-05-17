use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for redundant `Literal[None]` annotations.
///
/// ## Why is this bad?
/// While `Literal[None]` is a valid type annotation, it is semantically equivalent to `None`.
/// Prefer `None` over `Literal[None]` for both consistency and readability.
///
/// ## Example
/// ```python
/// from typing import Literal
///
/// Literal[None]
/// Literal[1, 2, 3, "foo", 5, None]
/// ```
///
/// Use instead:
/// ```python
/// from typing import Literal
///
/// None
/// Literal[1, 2, 3, "foo", 5] | None
/// ```
///
/// ## Fix safety and availability
/// This rule's fix is marked as safe unless the literal contains comments.
///
/// There is currently no fix available when applying the fix would lead to
/// a `TypeError` from an expression of the form `None | None` or when we
/// are unable to import the symbol `typing.Optional` and the Python version
/// is 3.9 or below.
///
/// ## References
/// - [Typing documentation: Legal parameters for `Literal` at type check time](https://typing.python.org/en/latest/spec/literal.html#legal-parameters-for-literal-at-type-check-time)
#[derive(ViolationMetadata)]
pub struct RedundantNoneLiteral {
    union_kind: UnionKind,
}

impl Violation for RedundantNoneLiteral {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.union_kind {
            UnionKind::NoUnion => "Use `None` rather than `Literal[None]`".to_string(),
            UnionKind::TypingOptional => {
                "Use `Optional[Literal[...]]` rather than `Literal[None, ...]` ".to_string()
            }
            UnionKind::BitOr => {
                "Use `Literal[...] | None` rather than `Literal[None, ...]` ".to_string()
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        Some(match self.union_kind {
            UnionKind::NoUnion => "Replace with `None`".to_string(),
            UnionKind::TypingOptional => "Replace with `Optional[Literal[...]]`".to_string(),
            UnionKind::BitOr => "Replace with `Literal[...] | None`".to_string(),
        })
    }
}

// FIX: duplicated with ruff_rule_flake8_pyi::redundant_none_literal
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum UnionKind {
    NoUnion,
    TypingOptional,
    BitOr,
}