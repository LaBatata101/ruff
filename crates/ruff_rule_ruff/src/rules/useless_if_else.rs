use ruff_linter_checkers::ast::CheckerSnapshot;
use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast as ast;
use ruff_python_ast::comparable::ComparableExpr;

/// ## What it does
/// Checks for useless `if`-`else` conditions with identical arms.
///
/// ## Why is this bad?
/// Useless `if`-`else` conditions add unnecessary complexity to the code without
/// providing any logical benefit. Assigning the value directly is clearer.
///
/// ## Example
/// ```python
/// foo = x if y else x
/// ```
///
/// Use instead:
/// ```python
/// foo = x
/// ```
#[derive(ViolationMetadata)]
pub struct UselessIfElse;

impl Violation for UselessIfElse {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Useless `if`-`else` condition".to_string()
    }
}

/// RUF034
pub fn useless_if_else(checker: &CheckerSnapshot, if_expr: &ast::ExprIf) {
    let ast::ExprIf {
        body,
        orelse,
        range,
        ..
    } = if_expr;

    // Skip if the `body` and `orelse` are not the same.
    if ComparableExpr::from(body) != ComparableExpr::from(orelse) {
        return;
    }

    checker.report_diagnostic(Diagnostic::new(UselessIfElse, *range));
}
