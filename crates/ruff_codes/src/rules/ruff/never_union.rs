use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::Expr;

/// ## What it does
/// Checks for uses of `typing.NoReturn` and `typing.Never` in union types.
///
/// ## Why is this bad?
/// `typing.NoReturn` and `typing.Never` are special types, used to indicate
/// that a function never returns, or that a type has no values.
///
/// Including `typing.NoReturn` or `typing.Never` in a union type is redundant,
/// as, e.g., `typing.Never | T` is equivalent to `T`.
///
/// ## Example
///
/// ```python
/// from typing import Never
///
///
/// def func() -> Never | int: ...
/// ```
///
/// Use instead:
///
/// ```python
/// def func() -> int: ...
/// ```
///
/// ## References
/// - [Python documentation: `typing.Never`](https://docs.python.org/3/library/typing.html#typing.Never)
/// - [Python documentation: `typing.NoReturn`](https://docs.python.org/3/library/typing.html#typing.NoReturn)
#[derive(ViolationMetadata)]
pub struct NeverUnion {
    never_like: NeverLike,
    union_like: UnionLike,
}

impl Violation for NeverUnion {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self {
            never_like,
            union_like,
        } = self;
        match union_like {
            UnionLike::PEP604 => {
                format!("`{never_like} | T` is equivalent to `T`")
            }
            UnionLike::TypingUnion => {
                format!("`Union[{never_like}, T]` is equivalent to `T`")
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let Self { never_like, .. } = self;
        Some(format!("Remove `{never_like}`"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UnionLike {
    /// E.g., `typing.Union[int, str]`
    TypingUnion,
    /// E.g., `int | str`
    PEP604,
}

// FIX: duplicated with ruff_rule_ruff::never_union
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NeverLike {
    /// E.g., `typing.NoReturn`
    NoReturn,
    /// E.g., `typing.Never`
    Never,
}

impl NeverLike {
    fn from_expr(expr: &Expr, semantic: &ruff_python_semantic::SemanticModel) -> Option<Self> {
        let qualified_name = semantic.resolve_qualified_name(expr)?;
        if semantic.match_typing_qualified_name(&qualified_name, "NoReturn") {
            Some(NeverLike::NoReturn)
        } else if semantic.match_typing_qualified_name(&qualified_name, "Never") {
            Some(NeverLike::Never)
        } else {
            None
        }
    }
}

impl std::fmt::Display for NeverLike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NeverLike::NoReturn => f.write_str("NoReturn"),
            NeverLike::Never => f.write_str("Never"),
        }
    }
}