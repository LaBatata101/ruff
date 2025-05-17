use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `else` clauses that only contains `pass` and `...` statements.
///
/// ## Why is this bad?
/// Such an else clause does nothing and can be removed.
///
/// ## Example
/// ```python
/// if foo:
///     bar()
/// else:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// if foo:
///     bar()
/// ```
#[derive(ViolationMetadata)]
pub struct NeedlessElse;

impl AlwaysFixableViolation for NeedlessElse {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Empty `else` clause".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove the `else` clause".to_string()
    }
}