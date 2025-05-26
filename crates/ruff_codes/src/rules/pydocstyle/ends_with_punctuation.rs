use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for docstrings in which the first line does not end in a punctuation
/// mark, such as a period, question mark, or exclamation point.
///
/// ## Why is this bad?
/// The first line of a docstring should end with a period, question mark, or
/// exclamation point, for grammatical correctness and consistency.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, this rule is enabled when using the `google`
/// convention, and disabled when using the `numpy` and `pep257` conventions.
///
/// ## Example
/// ```python
/// def average(values: list[float]) -> float:
///     """Return the mean of the given values"""
/// ```
///
/// Use instead:
/// ```python
/// def average(values: list[float]) -> float:
///     """Return the mean of the given values."""
/// ```
///
/// ## Options
/// - `lint.pydocstyle.convention`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct MissingTerminalPunctuation;

impl Violation for MissingTerminalPunctuation {
    /// `None` in the case a fix is never available or otherwise Some
    /// [`FixAvailability`] describing the available fix.
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "First line should end with a period, question mark, or exclamation point".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Add closing punctuation".to_string())
    }
}