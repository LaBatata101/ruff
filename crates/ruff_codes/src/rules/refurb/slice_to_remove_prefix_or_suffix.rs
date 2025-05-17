use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for code that could be written more idiomatically using
/// [`str.removeprefix()`](https://docs.python.org/3/library/stdtypes.html#str.removeprefix)
/// or [`str.removesuffix()`](https://docs.python.org/3/library/stdtypes.html#str.removesuffix).
///
/// Specifically, the rule flags code that conditionally removes a prefix or suffix
/// using a slice operation following an `if` test that uses `str.startswith()` or `str.endswith()`.
///
/// The rule is only applied if your project targets Python 3.9 or later.
///
/// ## Why is this bad?
/// The methods [`str.removeprefix()`](https://docs.python.org/3/library/stdtypes.html#str.removeprefix)
/// and [`str.removesuffix()`](https://docs.python.org/3/library/stdtypes.html#str.removesuffix),
/// introduced in Python 3.9, have the same behavior while being more readable and efficient.
///
/// ## Example
/// ```python
/// def example(filename: str, text: str):
///     filename = filename[:-4] if filename.endswith(".txt") else filename
///
///     if text.startswith("pre"):
///         text = text[3:]
/// ```
///
/// Use instead:
/// ```python
/// def example(filename: str, text: str):
///     filename = filename.removesuffix(".txt")
///     text = text.removeprefix("pre")
/// ```
#[derive(ViolationMetadata)]
pub struct SliceToRemovePrefixOrSuffix {
    affix_kind: AffixKind,
    stmt_or_expression: StmtOrExpr,
}

impl AlwaysFixableViolation for SliceToRemovePrefixOrSuffix {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.affix_kind {
            AffixKind::StartsWith => {
                "Prefer `str.removeprefix()` over conditionally replacing with slice.".to_string()
            }
            AffixKind::EndsWith => {
                "Prefer `str.removesuffix()` over conditionally replacing with slice.".to_string()
            }
        }
    }

    fn fix_title(&self) -> String {
        let method_name = self.affix_kind.as_str();
        let replacement = self.affix_kind.replacement();
        let context = match self.stmt_or_expression {
            StmtOrExpr::Statement => "assignment",
            StmtOrExpr::Expression => "ternary expression",
        };
        format!("Use {replacement} instead of {context} conditional upon {method_name}.")
    }
}

// FIX: duplicated with ruff_rule_refurb::slice_to_remove_prefix_or_suffix
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum StmtOrExpr {
    Statement,
    Expression,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum AffixKind {
    StartsWith,
    EndsWith,
}

impl AffixKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::StartsWith => "startswith",
            Self::EndsWith => "endswith",
        }
    }

    const fn replacement(self) -> &'static str {
        match self {
            Self::StartsWith => "removeprefix",
            Self::EndsWith => "removesuffix",
        }
    }
}