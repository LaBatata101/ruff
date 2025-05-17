use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::Stmt;
use ruff_text_size::Ranged;

use ruff_linter_checkers::ast::CheckerSnapshot;

/// ## What it does
/// Checks for uses of the `global` keyword at the module level.
///
/// ## Why is this bad?
/// The `global` keyword is used within functions to indicate that a name
/// refers to a global variable, rather than a local variable.
///
/// At the module level, all names are global by default, so the `global`
/// keyword is redundant.
#[derive(ViolationMetadata)]
pub struct GlobalAtModuleLevel;

impl Violation for GlobalAtModuleLevel {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`global` at module level is redundant".to_string()
    }
}

/// PLW0604
pub fn global_at_module_level(checker: &CheckerSnapshot, stmt: &Stmt) {
    if checker.semantic().current_scope().kind.is_module() {
        checker.report_diagnostic(Diagnostic::new(GlobalAtModuleLevel, stmt.range()));
    }
}
