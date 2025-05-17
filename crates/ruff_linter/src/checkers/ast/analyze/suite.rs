use ruff_python_ast::Stmt;

use crate::checkers::ast::Checker;
use ruff_codes::Rule;

/// Run lint rules over a suite of [`Stmt`] syntax nodes.
pub(crate) fn suite(suite: &[Stmt], checker: &Checker) {
    if checker.enabled(Rule::UnnecessaryPlaceholder) {
        ruff_rule_flake8_pie::rules::unnecessary_placeholder(&checker.snapshot(), suite);
    }
    if checker.enabled(Rule::RepeatedGlobal) {
        ruff_rule_refurb::rules::repeated_global(&checker.snapshot(), suite);
    }
}
