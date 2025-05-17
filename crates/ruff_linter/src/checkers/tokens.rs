//! Lint rules based on token traversal.

use std::path::Path;

use ruff_diagnostics::Diagnostic;
use ruff_notebook::CellOffsets;
use ruff_python_ast::PySourceType;
use ruff_python_codegen::Stylist;
use ruff_python_index::Indexer;
use ruff_python_parser::Tokens;

use ruff_linter_directives::TodoComment;
use ruff_codes::{AsRule, Rule};
use ruff_rule_pycodestyle::rules::BlankLinesChecker;
use ruff_rule_eradicate::{self as eradicate};
use ruff_rule_flake8_commas::{self as flake8_commas};
use ruff_rule_flake8_executable::{self as flake8_executable};
use ruff_rule_flake8_fixme::{self as flake8_fixme};
use ruff_rule_flake8_implicit_str_concat::{self as flake8_implicit_str_concat};
use ruff_rule_flake8_pyi::{self as flake8_pyi};
use ruff_rule_flake8_todos::{self as flake8_todos};
use ruff_rule_pycodestyle::{self as pycodestyle};
use ruff_rule_pygrep_hooks::{self as pygrep_hooks};
use ruff_rule_pylint::{self as pylint};
use ruff_rule_pyupgrade::{self as pyupgrade};
use ruff_rule_ruff::{self as ruff};


use ruff_linter_settings::LinterSettings;
use ruff_linter_commons::Locator;

#[expect(clippy::too_many_arguments)]
pub(crate) fn check_tokens(
    tokens: &Tokens,
    path: &Path,
    locator: &Locator,
    indexer: &Indexer,
    stylist: &Stylist,
    settings: &LinterSettings,
    source_type: PySourceType,
    cell_offsets: Option<&CellOffsets>,
) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<Diagnostic> = vec![];
    let comment_ranges = indexer.comment_ranges();

    if settings.rules.any_enabled(&[
        Rule::BlankLineBetweenMethods,
        Rule::BlankLinesTopLevel,
        Rule::TooManyBlankLines,
        Rule::BlankLineAfterDecorator,
        Rule::BlankLinesAfterFunctionOrClass,
        Rule::BlankLinesBeforeNestedDefinition,
    ]) {
        BlankLinesChecker::new(locator, stylist, settings, source_type, cell_offsets)
            .check_lines(tokens, &mut diagnostics);
    }

    if settings.rules.enabled(Rule::BlanketTypeIgnore) {
        pygrep_hooks::rules::blanket_type_ignore(&mut diagnostics, comment_ranges, locator);
    }

    if settings.rules.enabled(Rule::EmptyComment) {
        pylint::rules::empty_comments(&mut diagnostics, comment_ranges, locator);
    }

    if settings
        .rules
        .enabled(Rule::AmbiguousUnicodeCharacterComment)
    {
        for range in comment_ranges {
            ruff::rules::ambiguous_unicode_character_comment(
                &mut diagnostics,
                locator,
                range,
                settings,
            );
        }
    }

    if settings.rules.enabled(Rule::CommentedOutCode) {
        eradicate::rules::commented_out_code(&mut diagnostics, locator, comment_ranges, settings);
    }

    if settings.rules.enabled(Rule::UTF8EncodingDeclaration) {
        pyupgrade::rules::unnecessary_coding_comment(&mut diagnostics, locator, comment_ranges);
    }

    if settings.rules.enabled(Rule::TabIndentation) {
        pycodestyle::rules::tab_indentation(&mut diagnostics, locator, indexer);
    }

    if settings.rules.any_enabled(&[
        Rule::InvalidCharacterBackspace,
        Rule::InvalidCharacterSub,
        Rule::InvalidCharacterEsc,
        Rule::InvalidCharacterNul,
        Rule::InvalidCharacterZeroWidthSpace,
    ]) {
        for token in tokens {
            pylint::rules::invalid_string_characters(&mut diagnostics, token, locator);
        }
    }

    if settings.rules.any_enabled(&[
        Rule::MultipleStatementsOnOneLineColon,
        Rule::MultipleStatementsOnOneLineSemicolon,
        Rule::UselessSemicolon,
    ]) {
        pycodestyle::rules::compound_statements(
            &mut diagnostics,
            tokens,
            locator,
            indexer,
            source_type,
            cell_offsets,
        );
    }

    if settings.rules.any_enabled(&[
        Rule::SingleLineImplicitStringConcatenation,
        Rule::MultiLineImplicitStringConcatenation,
    ]) {
        flake8_implicit_str_concat::rules::implicit(
            &mut diagnostics,
            tokens,
            locator,
            indexer,
            settings,
        );
    }

    if settings.rules.any_enabled(&[
        Rule::MissingTrailingComma,
        Rule::TrailingCommaOnBareTuple,
        Rule::ProhibitedTrailingComma,
    ]) {
        flake8_commas::rules::trailing_commas(&mut diagnostics, tokens, locator, indexer);
    }

    if settings.rules.enabled(Rule::ExtraneousParentheses) {
        pyupgrade::rules::extraneous_parentheses(&mut diagnostics, tokens, locator);
    }

    if source_type.is_stub() && settings.rules.enabled(Rule::TypeCommentInStub) {
        flake8_pyi::rules::type_comment_in_stub(&mut diagnostics, locator, comment_ranges);
    }

    if settings.rules.any_enabled(&[
        Rule::ShebangNotExecutable,
        Rule::ShebangMissingExecutableFile,
        Rule::ShebangLeadingWhitespace,
        Rule::ShebangNotFirstLine,
        Rule::ShebangMissingPython,
    ]) {
        flake8_executable::rules::from_tokens(
            &mut diagnostics,
            path,
            locator,
            comment_ranges,
            settings,
        );
    }

    if settings.rules.any_enabled(&[
        Rule::InvalidTodoTag,
        Rule::MissingTodoAuthor,
        Rule::MissingTodoLink,
        Rule::MissingTodoColon,
        Rule::MissingTodoDescription,
        Rule::InvalidTodoCapitalization,
        Rule::MissingSpaceAfterTodoColon,
        Rule::LineContainsFixme,
        Rule::LineContainsXxx,
        Rule::LineContainsTodo,
        Rule::LineContainsHack,
    ]) {
        let todo_comments: Vec<TodoComment> = comment_ranges
            .iter()
            .enumerate()
            .filter_map(|(i, comment_range)| {
                let comment = locator.slice(*comment_range);
                TodoComment::from_comment(comment, *comment_range, i)
            })
            .collect();
        flake8_todos::rules::todos(&mut diagnostics, &todo_comments, locator, comment_ranges);
        flake8_fixme::rules::todos(&mut diagnostics, &todo_comments);
    }

    if settings.rules.enabled(Rule::TooManyNewlinesAtEndOfFile) {
        pycodestyle::rules::too_many_newlines_at_end_of_file(
            &mut diagnostics,
            tokens,
            cell_offsets,
        );
    }

    diagnostics.retain(|diagnostic| settings.rules.enabled(diagnostic.rule()));

    diagnostics
}
