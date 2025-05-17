use ruff_python_ast::{Expr, Stmt};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_text_size::Ranged;

use ruff_linter_checkers::ast::CheckerSnapshot;

/// ## What it does
/// Checks for `assert` statements that use non-empty tuples as test
/// conditions.
///
/// ## Why is this bad?
/// Non-empty tuples are always `True`, so an `assert` statement with a
/// non-empty tuple as its test condition will always pass. This is likely a
/// mistake.
///
/// ## Example
/// ```python
/// assert (some_condition,)
/// ```
///
/// Use instead:
/// ```python
/// assert some_condition
/// ```
///
/// ## References
/// - [Python documentation: The `assert` statement](https://docs.python.org/3/reference/simple_stmts.html#the-assert-statement)
#[derive(ViolationMetadata)]
pub struct AssertTuple;

impl Violation for AssertTuple {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Assert test is a non-empty tuple, which is always `True`".to_string()
    }
}

/// F631
pub fn assert_tuple(checker: &CheckerSnapshot, stmt: &Stmt, test: &Expr) {
    if let Expr::Tuple(tuple) = &test {
        if !tuple.is_empty() {
            checker.report_diagnostic(Diagnostic::new(AssertTuple, stmt.range()));
        }
    }
}
