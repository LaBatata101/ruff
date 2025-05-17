use ruff_python_ast::Suite;

use crate::checkers::ast::Checker;
use ruff_codes::Rule;

/// Run lint rules over a module.
pub(crate) fn module(suite: &Suite, checker: &Checker) {
    if checker.enabled(Rule::FStringDocstring) {
        ruff_rule_flake8_bugbear::rules::f_string_docstring(&checker.snapshot(), suite);
    }
    if checker.enabled(Rule::InvalidFormatterSuppressionComment) {
        ruff_rule_ruff::rules::ignored_formatter_suppression_comment(&checker.snapshot(), suite);
    }
}
