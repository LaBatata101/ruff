use ruff_python_ast::Expr;

use crate::checkers::ast::Checker;
use ruff_codes::Rule;

/// Run lint rules over all deferred lambdas in the [`SemanticModel`].
pub(crate) fn deferred_lambdas(checker: &mut Checker) {
    while !checker.analyze.lambdas.is_empty() {
        let lambdas = std::mem::take(&mut checker.analyze.lambdas);
        for snapshot in lambdas {
            checker.semantic.borrow_mut().restore(snapshot);

            let Some(Expr::Lambda(lambda)) = checker.semantic.borrow().current_expression() else {
                unreachable!("Expected Expr::Lambda");
            };

            if checker.enabled(Rule::UnnecessaryLambda) {
                ruff_rule_pylint::rules::unnecessary_lambda(&checker.snapshot(), lambda);
            }
            if checker.enabled(Rule::ReimplementedContainerBuiltin) {
                ruff_rule_flake8_pie::rules::reimplemented_container_builtin(&checker.snapshot(), lambda);
            }
            if checker.enabled(Rule::BuiltinLambdaArgumentShadowing) {
                ruff_rule_flake8_builtins::rules::builtin_lambda_argument_shadowing(&checker.snapshot(), lambda);
            }
        }
    }
}
