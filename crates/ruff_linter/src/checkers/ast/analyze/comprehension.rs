use ruff_python_ast::Comprehension;

use ruff_linter_checkers::ast::CheckerSnapshot;
use ruff_codes::Rule;

/// Run lint rules over a [`Comprehension`] syntax nodes.
pub(crate) fn comprehension(comprehension: &Comprehension, checker: &CheckerSnapshot) {
    if checker.enabled(Rule::InDictKeys) {
        ruff_rule_flake8_simplify::rules::key_in_dict_comprehension(checker, comprehension);
    }
    if checker.enabled(Rule::ReadlinesInFor) {
        ruff_rule_refurb::rules::readlines_in_comprehension(checker, comprehension);
    }
}
