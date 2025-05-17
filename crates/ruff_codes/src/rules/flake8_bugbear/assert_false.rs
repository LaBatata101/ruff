use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `assert False`.
///
/// ## Why is this bad?
/// Python removes `assert` statements when running in optimized mode
/// (`python -O`), making `assert False` an unreliable means of
/// raising an `AssertionError`.
///
/// Instead, raise an `AssertionError` directly.
///
/// ## Example
/// ```python
/// assert False
/// ```
///
/// Use instead:
/// ```python
/// raise AssertionError
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as changing an `assert` to a
/// `raise` will change the behavior of your program when running in
/// optimized mode (`python -O`).
///
/// ## References
/// - [Python documentation: `assert`](https://docs.python.org/3/reference/simple_stmts.html#the-assert-statement)
#[derive(ViolationMetadata)]
pub struct AssertFalse;

impl AlwaysFixableViolation for AssertFalse {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not `assert False` (`python -O` removes these calls), raise `AssertionError()`"
            .to_string()
    }

    fn fix_title(&self) -> String {
        "Replace `assert False`".to_string()
    }
}