use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for docstrings that do not start with a capital letter.
///
/// ## Why is this bad?
/// The first non-whitespace character in a docstring should be
/// capitalized for grammatical correctness and consistency.
///
/// ## Example
/// ```python
/// def average(values: list[float]) -> float:
///     """return the mean of the given values."""
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
pub struct FirstWordUncapitalized {
    first_word: String,
    capitalized_word: String,
}

impl AlwaysFixableViolation for FirstWordUncapitalized {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!(
            "First word of the docstring should be capitalized: `{}` -> `{}`",
            self.first_word, self.capitalized_word
        )
    }

    fn fix_title(&self) -> String {
        format!(
            "Capitalize `{}` to `{}`",
            self.first_word, self.capitalized_word
        )
    }
}