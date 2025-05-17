use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for docstring summary lines that are not positioned on the first
/// physical line of the docstring.
///
/// ## Why is this bad?
/// [PEP 257] recommends that multi-line docstrings consist of "a summary line
/// just like a one-line docstring, followed by a blank line, followed by a
/// more elaborate description."
///
/// The summary line should be located on the first physical line of the
/// docstring, immediately after the opening quotes.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, this rule is enabled when using the `google`
/// convention, and disabled when using the `numpy` and `pep257` conventions.
///
/// For an alternative, see [D213].
///
/// ## Example
/// ```python
/// def sort_list(l: list[int]) -> list[int]:
///     """
///     Return a sorted copy of the list.
///
///     Sort the list in ascending order and return a copy of the result using the
///     bubble sort algorithm.
///     """
/// ```
///
/// Use instead:
/// ```python
/// def sort_list(l: list[int]) -> list[int]:
///     """Return a sorted copy of the list.
///
///     Sort the list in ascending order and return a copy of the result using the bubble
///     sort algorithm.
///     """
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
///
/// [D213]: https://docs.astral.sh/ruff/rules/multi-line-summary-second-line
/// [PEP 257]: https://peps.python.org/pep-0257
#[derive(ViolationMetadata)]
pub struct MultiLineSummaryFirstLine;

impl AlwaysFixableViolation for MultiLineSummaryFirstLine {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multi-line docstring summary should start at the first line".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove whitespace after opening quotes".to_string()
    }
}

/// ## What it does
/// Checks for docstring summary lines that are not positioned on the second
/// physical line of the docstring.
///
/// ## Why is this bad?
/// [PEP 257] recommends that multi-line docstrings consist of "a summary line
/// just like a one-line docstring, followed by a blank line, followed by a
/// more elaborate description."
///
/// The summary line should be located on the second physical line of the
/// docstring, immediately after the opening quotes and the blank line.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, this rule is disabled when using the `google`,
/// `numpy`, and `pep257` conventions.
///
/// For an alternative, see [D212].
///
/// ## Example
/// ```python
/// def sort_list(l: list[int]) -> list[int]:
///     """Return a sorted copy of the list.
///
///     Sort the list in ascending order and return a copy of the result using the
///     bubble sort algorithm.
///     """
/// ```
///
/// Use instead:
/// ```python
/// def sort_list(l: list[int]) -> list[int]:
///     """
///     Return a sorted copy of the list.
///
///     Sort the list in ascending order and return a copy of the result using the bubble
///     sort algorithm.
///     """
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
///
/// [D212]: https://docs.astral.sh/ruff/rules/multi-line-summary-first-line
/// [PEP 257]: https://peps.python.org/pep-0257
#[derive(ViolationMetadata)]
pub struct MultiLineSummarySecondLine;

impl AlwaysFixableViolation for MultiLineSummarySecondLine {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multi-line docstring summary should start at the second line".to_string()
    }

    fn fix_title(&self) -> String {
        "Insert line break and indentation after opening quotes".to_string()
    }
}
