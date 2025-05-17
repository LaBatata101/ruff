use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for docstring summary lines that are not separated from the docstring
/// description by one blank line.
///
/// ## Why is this bad?
/// [PEP 257] recommends that multi-line docstrings consist of "a summary line
/// just like a one-line docstring, followed by a blank line, followed by a
/// more elaborate description."
///
/// ## Example
/// ```python
/// def sort_list(l: list[int]) -> list[int]:
///     """Return a sorted copy of the list.
///     Sort the list in ascending order and return a copy of the
///     result using the bubble sort algorithm.
///     """
/// ```
///
/// Use instead:
/// ```python
/// def sort_list(l: list[int]) -> list[int]:
///     """Return a sorted copy of the list.
///
///     Sort the list in ascending order and return a copy of the
///     result using the bubble sort algorithm.
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
pub struct MissingBlankLineAfterSummary {
    num_lines: usize,
}

impl Violation for MissingBlankLineAfterSummary {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let MissingBlankLineAfterSummary { num_lines } = self;
        if *num_lines == 0 {
            "1 blank line required between summary line and description".to_string()
        } else {
            format!(
                "1 blank line required between summary line and description (found {num_lines})"
            )
        }
    }

    fn fix_title(&self) -> Option<String> {
        Some("Insert single blank line".to_string())
    }
}