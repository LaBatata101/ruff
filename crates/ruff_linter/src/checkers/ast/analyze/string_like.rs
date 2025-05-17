use ruff_python_ast::StringLike;

use crate::checkers::ast::Checker;
use ruff_codes::Rule;
use ruff_rule_flake8_bandit::{self as flake8_bandit};
use ruff_rule_flake8_pyi::{self as flake8_pyi};
use ruff_rule_flake8_quotes::{self as flake8_quotes};
use ruff_rule_pycodestyle::{self as pycodestyle};
use ruff_rule_ruff::{self as ruff};


/// Run lint rules over a [`StringLike`] syntax nodes.
pub(crate) fn string_like(string_like: StringLike, checker: &Checker) {
    if checker.any_enabled(&[
        Rule::AmbiguousUnicodeCharacterString,
        Rule::AmbiguousUnicodeCharacterDocstring,
    ]) {
        ruff::rules::ambiguous_unicode_character_string(&checker.snapshot(), string_like);
    }
    if checker.enabled(Rule::HardcodedBindAllInterfaces) {
        flake8_bandit::rules::hardcoded_bind_all_interfaces(&checker.snapshot(), string_like);
    }
    if checker.enabled(Rule::HardcodedTempFile) {
        flake8_bandit::rules::hardcoded_tmp_directory(&checker.snapshot(), string_like);
    }
    if checker.source_type.is_stub() {
        if checker.enabled(Rule::StringOrBytesTooLong) {
            flake8_pyi::rules::string_or_bytes_too_long(&checker.snapshot(), string_like);
        }
    }
    if checker.any_enabled(&[
        Rule::BadQuotesInlineString,
        Rule::BadQuotesMultilineString,
        Rule::BadQuotesDocstring,
    ]) {
        flake8_quotes::rules::check_string_quotes(&checker.snapshot(), string_like);
    }
    if checker.enabled(Rule::UnnecessaryEscapedQuote) {
        flake8_quotes::rules::unnecessary_escaped_quote(&checker.snapshot(), string_like);
    }
    if checker.enabled(Rule::AvoidableEscapedQuote) && checker.settings.flake8_quotes.avoid_escape {
        flake8_quotes::rules::avoidable_escaped_quote(&checker.snapshot(), string_like);
    }
    if checker.enabled(Rule::InvalidEscapeSequence) {
        pycodestyle::rules::invalid_escape_sequence(&checker.snapshot(), string_like);
    }
}
