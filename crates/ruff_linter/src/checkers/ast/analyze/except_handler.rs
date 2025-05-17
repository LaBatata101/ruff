use ruff_python_ast::{self as ast, ExceptHandler};
use ruff_text_size::Ranged;

use crate::checkers::ast::Checker;
use ruff_codes::Rule;
use ruff_rule_flake8_bandit::{self as flake8_bandit};
use ruff_rule_flake8_blind_except::{self as flake8_blind_except};
use ruff_rule_flake8_bugbear::{self as flake8_bugbear};
use ruff_rule_flake8_builtins::{self as flake8_builtins};
use ruff_rule_pycodestyle::{self as pycodestyle};
use ruff_rule_pylint::{self as pylint};


/// Run lint rules over an [`ExceptHandler`] syntax node.
pub(crate) fn except_handler(except_handler: &ExceptHandler, checker: &Checker) {
    match except_handler {
        ExceptHandler::ExceptHandler(ast::ExceptHandlerExceptHandler {
            type_,
            name,
            body,
            range: _,
        }) => {
            if checker.enabled(Rule::BareExcept) {
                if let Some(diagnostic) = pycodestyle::rules::bare_except(
                    type_.as_deref(),
                    body,
                    except_handler,
                    checker.locator,
                ) {
                    checker.report_diagnostic(diagnostic);
                }
            }
            if checker.enabled(Rule::RaiseWithoutFromInsideExcept) {
                flake8_bugbear::rules::raise_without_from_inside_except(
                    &checker.snapshot(),
                    name.as_deref(),
                    body,
                );
            }
            if checker.enabled(Rule::BlindExcept) {
                flake8_blind_except::rules::blind_except(
                    &checker.snapshot(),
                    type_.as_deref(),
                    name.as_deref(),
                    body,
                );
            }
            if checker.enabled(Rule::TryExceptPass) {
                flake8_bandit::rules::try_except_pass(
                    &checker.snapshot(),
                    except_handler,
                    type_.as_deref(),
                    body,
                    checker.settings.flake8_bandit.check_typed_exception,
                );
            }
            if checker.enabled(Rule::TryExceptContinue) {
                flake8_bandit::rules::try_except_continue(
                    &checker.snapshot(),
                    except_handler,
                    type_.as_deref(),
                    body,
                    checker.settings.flake8_bandit.check_typed_exception,
                );
            }
            if checker.enabled(Rule::ExceptWithEmptyTuple) {
                flake8_bugbear::rules::except_with_empty_tuple(&checker.snapshot(), except_handler);
            }
            if checker.enabled(Rule::ExceptWithNonExceptionClasses) {
                flake8_bugbear::rules::except_with_non_exception_classes(&checker.snapshot(), except_handler);
            }
            if checker.enabled(Rule::BinaryOpException) {
                pylint::rules::binary_op_exception(&checker.snapshot(), except_handler);
            }
            if let Some(name) = name {
                if checker.enabled(Rule::AmbiguousVariableName) {
                    pycodestyle::rules::ambiguous_variable_name(
                        &checker.snapshot(),
                        name.as_str(),
                        name.range(),
                    );
                }
                if checker.enabled(Rule::BuiltinVariableShadowing) {
                    flake8_builtins::rules::builtin_variable_shadowing(&checker.snapshot(), name, name.range());
                }
            }
        }
    }
}
