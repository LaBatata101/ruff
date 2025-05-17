use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for membership tests on `list` and `tuple` literals.
///
/// ## Why is this bad?
/// When testing for membership in a static sequence, prefer a `set` literal
/// over a `list` or `tuple`, as Python optimizes `set` membership tests.
///
/// ## Example
/// ```python
/// 1 in [1, 2, 3]
/// ```
///
/// Use instead:
/// ```python
/// 1 in {1, 2, 3}
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as the use of a `set` literal will
/// error at runtime if the sequence contains unhashable elements (like lists
/// or dictionaries). While Ruff will attempt to infer the hashability of the
/// elements, it may not always be able to do so.
///
/// ## References
/// - [What’s New In Python 3.2](https://docs.python.org/3/whatsnew/3.2.html#optimizations)
#[derive(ViolationMetadata)]
pub struct LiteralMembership;

impl AlwaysFixableViolation for LiteralMembership {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use a set literal when testing for membership".to_string()
    }

    fn fix_title(&self) -> String {
        "Convert to `set`".to_string()
    }
}