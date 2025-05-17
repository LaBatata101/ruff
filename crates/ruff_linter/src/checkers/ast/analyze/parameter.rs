use ruff_python_ast::Parameter;
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;
use ruff_codes::Rule;

/// Run lint rules over a [`Parameter`] syntax node.
pub(crate) fn parameter(parameter: &Parameter, checker: &Checker) {
    if checker.enabled(Rule::AmbiguousVariableName) {
        ruff_rule_pycodestyle::rules::ambiguous_variable_name(
            &checker.snapshot(),
            &parameter.name,
            parameter.name.range(),
        );
    }
    if checker.enabled(Rule::BuiltinArgumentShadowing) {
        ruff_rule_flake8_builtins::rules::builtin_argument_shadowing(&checker.snapshot(), parameter);
    }
}
