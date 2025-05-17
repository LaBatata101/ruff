use ruff_diagnostics::{AlwaysFixableViolation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use super::types::Parentheses;

/// ## What it does
/// Checks for argument-free `@pytest.mark.<marker>()` decorators with or
/// without parentheses, depending on the [`lint.flake8-pytest-style.mark-parentheses`]
/// setting.
///
/// The rule defaults to removing unnecessary parentheses,
/// to match the documentation of the official pytest projects.
///
/// ## Why is this bad?
/// If a `@pytest.mark.<marker>()` doesn't take any arguments, the parentheses are
/// optional.
///
/// Either removing those unnecessary parentheses _or_ requiring them for all
/// fixtures is fine, but it's best to be consistent.
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// @pytest.mark.foo
/// def test_something(): ...
/// ```
///
/// Use instead:
///
/// ```python
/// import pytest
///
///
/// @pytest.mark.foo()
/// def test_something(): ...
/// ```
///
/// ## Options
/// - `lint.flake8-pytest-style.mark-parentheses`
///
/// ## References
/// - [`pytest` documentation: Marks](https://docs.pytest.org/en/latest/reference/reference.html#marks)
#[derive(ViolationMetadata)]
pub struct PytestIncorrectMarkParenthesesStyle {
    mark_name: String,
    expected_parens: Parentheses,
    actual_parens: Parentheses,
}

impl AlwaysFixableViolation for PytestIncorrectMarkParenthesesStyle {
    #[derive_message_formats]
    fn message(&self) -> String {
        let PytestIncorrectMarkParenthesesStyle {
            mark_name,
            expected_parens,
            actual_parens,
        } = self;
        format!(
            "Use `@pytest.mark.{mark_name}{expected_parens}` over \
             `@pytest.mark.{mark_name}{actual_parens}`"
        )
    }

    fn fix_title(&self) -> String {
        match &self.expected_parens {
            Parentheses::None => "Remove parentheses".to_string(),
            Parentheses::Empty => "Add parentheses".to_string(),
        }
    }
}

/// ## What it does
/// Checks for `@pytest.mark.usefixtures()` decorators that aren't passed any
/// arguments.
///
/// ## Why is this bad?
/// A `@pytest.mark.usefixtures()` decorator that isn't passed any arguments is
/// useless and should be removed.
///
/// ## Example
///
/// ```python
/// import pytest
///
///
/// @pytest.mark.usefixtures()
/// def test_something(): ...
/// ```
///
/// Use instead:
///
/// ```python
/// def test_something(): ...
/// ```
///
/// ## References
/// - [`pytest` documentation: `pytest.mark.usefixtures`](https://docs.pytest.org/en/latest/reference/reference.html#pytest-mark-usefixtures)
#[derive(ViolationMetadata)]
pub struct PytestUseFixturesWithoutParameters;

impl AlwaysFixableViolation for PytestUseFixturesWithoutParameters {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Useless `pytest.mark.usefixtures` without parameters".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `usefixtures` decorator or pass parameters".to_string()
    }
}
