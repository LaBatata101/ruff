use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for multi-line docstrings whose closing quotes are not on their
/// own line.
///
/// ## Why is this bad?
/// [PEP 257] recommends that the closing quotes of a multi-line docstring be
/// on their own line, for consistency and compatibility with documentation
/// tools that may need to parse the docstring.
///
/// ## Example
/// ```python
/// def sort_list(l: List[int]) -> List[int]:
///     """Return a sorted copy of the list.
///
///     Sort the list in ascending order and return a copy of the result using the
///     bubble sort algorithm."""
/// ```
///
/// Use instead:
/// ```python
/// def sort_list(l: List[int]) -> List[int]:
///     """Return a sorted copy of the list.
///
///     Sort the list in ascending order and return a copy of the result using the bubble
///     sort algorithm.
///     """
/// ```
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
///
/// [PEP 257]: https://peps.python.org/pep-0257/
#[derive(ViolationMetadata)]
pub struct NewLineAfterLastParagraph;

impl AlwaysFixableViolation for NewLineAfterLastParagraph {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Multi-line docstring closing quotes should be on a separate line".to_string()
    }

    fn fix_title(&self) -> String {
        "Move closing quotes to new line".to_string()
    }
}