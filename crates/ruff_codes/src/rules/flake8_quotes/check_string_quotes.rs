use ruff_diagnostics::{AlwaysFixableViolation, FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};
use ruff_python_ast::str::Quote;

/// ## What it does
/// Checks for inline strings that use single quotes or double quotes,
/// depending on the value of the [`lint.flake8-quotes.inline-quotes`] option.
///
/// ## Why is this bad?
/// Consistency is good. Use either single or double quotes for inline
/// strings, but be consistent.
///
/// ## Example
/// ```python
/// foo = 'bar'
/// ```
///
/// Assuming `inline-quotes` is set to `double`, use instead:
/// ```python
/// foo = "bar"
/// ```
///
/// ## Options
/// - `lint.flake8-quotes.inline-quotes`
///
/// ## Formatter compatibility
/// We recommend against using this rule alongside the [formatter]. The
/// formatter enforces consistent quotes for inline strings, making the rule
/// redundant.
///
/// [formatter]: https://docs.astral.sh/ruff/formatter
#[derive(ViolationMetadata)]
pub struct BadQuotesInlineString {
    preferred_quote: Quote,
}

impl Violation for BadQuotesInlineString {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.preferred_quote {
            Quote::Double => "Single quotes found but double quotes preferred".to_string(),
            Quote::Single => "Double quotes found but single quotes preferred".to_string(),
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = match self.preferred_quote {
            Quote::Double => "Replace single quotes with double quotes",
            Quote::Single => "Replace double quotes with single quotes",
        };
        Some(title.to_string())
    }
}

/// ## What it does
/// Checks for multiline strings that use single quotes or double quotes,
/// depending on the value of the [`lint.flake8-quotes.multiline-quotes`]
/// setting.
///
/// ## Why is this bad?
/// Consistency is good. Use either single or double quotes for multiline
/// strings, but be consistent.
///
/// ## Example
/// ```python
/// foo = '''
/// bar
/// '''
/// ```
///
/// Assuming `multiline-quotes` is set to `double`, use instead:
/// ```python
/// foo = """
/// bar
/// """
/// ```
///
/// ## Options
/// - `lint.flake8-quotes.multiline-quotes`
///
/// ## Formatter compatibility
/// We recommend against using this rule alongside the [formatter]. The
/// formatter enforces double quotes for multiline strings, making the rule
/// redundant.
///
/// [formatter]: https://docs.astral.sh/ruff/formatter
#[derive(ViolationMetadata)]
pub struct BadQuotesMultilineString {
    preferred_quote: Quote,
}

impl AlwaysFixableViolation for BadQuotesMultilineString {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BadQuotesMultilineString { preferred_quote } = self;
        match preferred_quote {
            Quote::Double => "Single quote multiline found but double quotes preferred".to_string(),
            Quote::Single => "Double quote multiline found but single quotes preferred".to_string(),
        }
    }

    fn fix_title(&self) -> String {
        let BadQuotesMultilineString { preferred_quote } = self;
        match preferred_quote {
            Quote::Double => "Replace single multiline quotes with double quotes".to_string(),
            Quote::Single => "Replace double multiline quotes with single quotes".to_string(),
        }
    }
}

/// ## What it does
/// Checks for docstrings that use single quotes or double quotes, depending
/// on the value of the [`lint.flake8-quotes.docstring-quotes`] setting.
///
/// ## Why is this bad?
/// Consistency is good. Use either single or double quotes for docstring
/// strings, but be consistent.
///
/// ## Example
/// ```python
/// '''
/// bar
/// '''
/// ```
///
/// Assuming `docstring-quotes` is set to `double`, use instead:
/// ```python
/// """
/// bar
/// """
/// ```
///
/// ## Options
/// - `lint.flake8-quotes.docstring-quotes`
///
/// ## Formatter compatibility
/// We recommend against using this rule alongside the [formatter]. The
/// formatter enforces double quotes for docstrings, making the rule
/// redundant.
///
/// [formatter]: https://docs.astral.sh/ruff/formatter
#[derive(ViolationMetadata)]
pub struct BadQuotesDocstring {
    preferred_quote: Quote,
}

impl Violation for BadQuotesDocstring {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.preferred_quote {
            Quote::Double => "Single quote docstring found but double quotes preferred".to_string(),
            Quote::Single => "Double quote docstring found but single quotes preferred".to_string(),
        }
    }

    fn fix_title(&self) -> Option<String> {
        match self.preferred_quote {
            Quote::Double => Some("Replace single quotes docstring with double quotes".to_string()),
            Quote::Single => Some("Replace double quotes docstring with single quotes".to_string()),
        }
    }
}
