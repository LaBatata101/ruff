use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for empty docstrings.
///
/// ## Why is this bad?
/// An empty docstring is indicative of incomplete documentation. It should either
/// be removed or replaced with a meaningful docstring.
///
/// ## Example
/// ```python
/// def average(values: list[float]) -> float:
///     """"""
/// ```
///
/// Use instead:
/// ```python
/// def average(values: list[float]) -> float:
///     """Return the mean of the given values."""
/// ```
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct EmptyDocstring;

impl Violation for EmptyDocstring {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Docstring is empty".to_string()
    }
}