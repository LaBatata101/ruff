use ruff_python_ast::Parameters;

use crate::checkers::ast::Checker;
use ruff_codes::Rule;

/// Run lint rules over a [`Parameters`] syntax node.
pub(crate) fn parameters(parameters: &Parameters, checker: &Checker) {
    if checker.enabled(Rule::FunctionCallInDefaultArgument) {
        ruff_rule_flake8_bugbear::rules::function_call_in_argument_default(&checker.snapshot(), parameters);
    }
    if checker.settings.rules.enabled(Rule::ImplicitOptional) {
        ruff_rule_ruff::rules::implicit_optional(&checker.snapshot(), parameters);
    }
    if checker.source_type.is_stub() {
        if checker.enabled(Rule::TypedArgumentDefaultInStub) {
            ruff_rule_flake8_pyi::rules::typed_argument_simple_defaults(&checker.snapshot(), parameters);
        }
        if checker.enabled(Rule::ArgumentDefaultInStub) {
            ruff_rule_flake8_pyi::rules::argument_simple_defaults(&checker.snapshot(), parameters);
        }
    }
}
