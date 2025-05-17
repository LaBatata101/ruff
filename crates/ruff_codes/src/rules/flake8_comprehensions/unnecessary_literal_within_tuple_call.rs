use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::TupleLiteralKind;

/// ## What it does
/// Checks for `tuple` calls that take unnecessary list or tuple literals as
/// arguments. In [preview], this also includes unnecessary list comprehensions
/// within tuple calls.
///
/// ## Why is this bad?
/// It's unnecessary to use a list or tuple literal within a `tuple()` call,
/// since there is a literal syntax for these types.
///
/// If a list literal was passed, then it should be rewritten as a `tuple`
/// literal. Otherwise, if a tuple literal was passed, then the outer call
/// to `tuple()` should be removed.
///
/// In [preview], this rule also checks for list comprehensions within `tuple()`
/// calls. If a list comprehension is found, it should be rewritten as a
/// generator expression.
///
/// ## Example
/// ```python
/// tuple([1, 2])
/// tuple((1, 2))
/// tuple([x for x in range(10)])
/// ```
///
/// Use instead:
/// ```python
/// (1, 2)
/// (1, 2)
/// tuple(x for x in range(10))
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
///
/// [preview]: https://docs.astral.sh/ruff/preview/
#[derive(ViolationMetadata)]
pub struct UnnecessaryLiteralWithinTupleCall {
    literal_kind: TupleLiteralKind,
}

impl AlwaysFixableViolation for UnnecessaryLiteralWithinTupleCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        match self.literal_kind {
            TupleLiteralKind::List => {
                "Unnecessary list literal passed to `tuple()` (rewrite as a tuple literal)"
                    .to_string()
            }
            TupleLiteralKind::Tuple => {
                "Unnecessary tuple literal passed to `tuple()` (remove the outer call to `tuple()`)"
                    .to_string()
            }
            TupleLiteralKind::ListComp => {
                "Unnecessary list comprehension passed to `tuple()` (rewrite as a generator)"
                    .to_string()
            }
        }
    }

    fn fix_title(&self) -> String {
        let title = match self.literal_kind {
            TupleLiteralKind::List => "Rewrite as a tuple literal",
            TupleLiteralKind::Tuple => "Remove the outer call to `tuple()`",
            TupleLiteralKind::ListComp => "Rewrite as a generator",
        };
        title.to_string()
    }
}