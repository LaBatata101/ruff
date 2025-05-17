use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for consistent style regarding whether nonempty tuples in subscripts
/// are parenthesized.
///
/// The exact nature of this violation depends on the setting
/// [`lint.ruff.parenthesize-tuple-in-subscript`]. By default, the use of
/// parentheses is considered a violation.
///
/// This rule is not applied inside "typing contexts" (type annotations,
/// type aliases and subscripted class bases), as these have their own specific
/// conventions around them.
///
/// ## Why is this bad?
/// It is good to be consistent and, depending on the codebase, one or the other
/// convention may be preferred.
///
/// ## Example
///
/// ```python
/// directions = {(0, 1): "North", (1, 0): "East", (0, -1): "South", (-1, 0): "West"}
/// directions[(0, 1)]
/// ```
///
/// Use instead (with default setting):
///
/// ```python
/// directions = {(0, 1): "North", (1, 0): "East", (0, -1): "South", (-1, 0): "West"}
/// directions[0, 1]
/// ```
///
/// ## Options
/// - `lint.ruff.parenthesize-tuple-in-subscript`
#[derive(ViolationMetadata)]
pub struct IncorrectlyParenthesizedTupleInSubscript {
    prefer_parentheses: bool,
}

impl AlwaysFixableViolation for IncorrectlyParenthesizedTupleInSubscript {
    #[derive_message_formats]
    fn message(&self) -> String {
        if self.prefer_parentheses {
            "Use parentheses for tuples in subscripts".to_string()
        } else {
            "Avoid parentheses for tuples in subscripts".to_string()
        }
    }

    fn fix_title(&self) -> String {
        if self.prefer_parentheses {
            "Parenthesize tuple".to_string()
        } else {
            "Remove parentheses".to_string()
        }
    }
}