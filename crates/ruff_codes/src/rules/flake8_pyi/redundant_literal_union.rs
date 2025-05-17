use std::fmt;

use ruff_diagnostics::Violation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for redundant unions between a `Literal` and a builtin supertype of
/// that `Literal`.
///
/// ## Why is this bad?
/// Using a `Literal` type in a union with its builtin supertype is redundant,
/// as the supertype will be strictly more general than the `Literal` type.
/// For example, `Literal["A"] | str` is equivalent to `str`, and
/// `Literal[1] | int` is equivalent to `int`, as `str` and `int` are the
/// supertypes of `"A"` and `1` respectively.
///
/// ## Example
/// ```pyi
/// from typing import Literal
///
/// x: Literal["A", b"B"] | str
/// ```
///
/// Use instead:
/// ```pyi
/// from typing import Literal
///
/// x: Literal[b"B"] | str
/// ```
#[derive(ViolationMetadata)]
pub struct RedundantLiteralUnion {
    literal: SourceCodeSnippet,
    builtin_type: ExprType,
}

impl Violation for RedundantLiteralUnion {
    #[derive_message_formats]
    fn message(&self) -> String {
        let RedundantLiteralUnion {
            literal,
            builtin_type,
        } = self;
        if let Some(literal) = literal.full_display() {
            format!("`Literal[{literal}]` is redundant in a union with `{builtin_type}`")
        } else {
            format!("`Literal` is redundant in a union with `{builtin_type}`")
        }
    }
}

// FIX: duplicated with ruff_rule_flake8_pyi::redundant_literal_union
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum ExprType {
    Int,
    Str,
    Bool,
    Float,
    Bytes,
    Complex,
}

impl fmt::Display for ExprType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Int => fmt.write_str("int"),
            Self::Str => fmt.write_str("str"),
            Self::Bool => fmt.write_str("bool"),
            Self::Float => fmt.write_str("float"),
            Self::Bytes => fmt.write_str("bytes"),
            Self::Complex => fmt.write_str("complex"),
        }
    }
}
