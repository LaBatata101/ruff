use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for surrounding whitespace in docstrings.
///
/// ## Why is this bad?
/// Remove surrounding whitespace from the docstring, for consistency.
///
/// ## Example
/// ```python
/// def factorial(n: int) -> int:
///     """ Return the factorial of n. """
/// ```
///
/// Use instead:
/// ```python
/// def factorial(n: int) -> int:
///     """Return the factorial of n."""
/// ```
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct SurroundingWhitespace;

impl Violation for SurroundingWhitespace {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "No whitespaces allowed surrounding docstring text".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Trim surrounding whitespace".to_string())
    }
}