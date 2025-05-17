use ruff_python_ast::Stmt;

use crate::checkers::ast::Checker;
use ruff_codes::Rule;

/// Run lint rules over all deferred for-loops in the [`SemanticModel`].
pub(crate) fn deferred_for_loops(checker: &mut Checker) {
    while !checker.analyze.for_loops.is_empty() {
        let for_loops = std::mem::take(&mut checker.analyze.for_loops);
        for snapshot in for_loops {
            checker.semantic.borrow_mut().restore(snapshot);

            let Stmt::For(stmt_for) = checker.semantic.borrow().current_statement() else {
                unreachable!("Expected Stmt::For");
            };
            if checker.enabled(Rule::UnusedLoopControlVariable) {
                ruff_rule_flake8_bugbear::rules::unused_loop_control_variable(&checker.snapshot(), stmt_for);
            }
            if checker.enabled(Rule::IncorrectDictIterator) {
                ruff_rule_perflint::rules::incorrect_dict_iterator(&checker.snapshot(), stmt_for);
            }
            if checker.enabled(Rule::YieldInForLoop) {
                ruff_rule_pyupgrade::rules::yield_in_for_loop(&checker.snapshot(), stmt_for);
            }
            if checker.enabled(Rule::UnnecessaryEnumerate) {
                ruff_rule_refurb::rules::unnecessary_enumerate(&checker.snapshot(), stmt_for);
            }
            if checker.enabled(Rule::EnumerateForLoop) {
                ruff_rule_flake8_simplify::rules::enumerate_for_loop(&checker.snapshot(), stmt_for);
            }
            if checker.enabled(Rule::LoopIteratorMutation) {
                ruff_rule_flake8_bugbear::rules::loop_iterator_mutation(&checker.snapshot(), stmt_for);
            }
            if checker.enabled(Rule::DictIndexMissingItems) {
                ruff_rule_pylint::rules::dict_index_missing_items(&checker.snapshot(), stmt_for);
            }
            if checker.enabled(Rule::ManualDictComprehension) {
                ruff_rule_perflint::rules::manual_dict_comprehension(&checker.snapshot(), stmt_for);
            }
            if checker.enabled(Rule::ManualListComprehension) {
                ruff_rule_perflint::rules::manual_list_comprehension(&checker.snapshot(), stmt_for);
            }
        }
    }
}
