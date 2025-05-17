use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for docstrings on functions that are separated by one or more blank
/// lines from the function definition.
///
/// ## Why is this bad?
/// Remove any blank lines between the function definition and its docstring,
/// for consistency.
///
/// ## Example
/// ```python
/// def average(values: list[float]) -> float:
///
///     """Return the mean of the given values."""
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
pub struct BlankLineBeforeFunction {
    num_lines: usize,
}

impl AlwaysFixableViolation for BlankLineBeforeFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BlankLineBeforeFunction { num_lines } = self;
        format!("No blank lines allowed before function docstring (found {num_lines})")
    }

    fn fix_title(&self) -> String {
        "Remove blank line(s) before function docstring".to_string()
    }
}

/// ## What it does
/// Checks for docstrings on functions that are separated by one or more blank
/// lines from the function body.
///
/// ## Why is this bad?
/// Remove any blank lines between the function body and the function
/// docstring, for consistency.
///
/// ## Example
/// ```python
/// def average(values: list[float]) -> float:
///     """Return the mean of the given values."""
///
///     return sum(values) / len(values)
/// ```
///
/// Use instead:
/// ```python
/// def average(values: list[float]) -> float:
///     """Return the mean of the given values."""
///     return sum(values) / len(values)
/// ```
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [NumPy Style Guide](https://numpydoc.readthedocs.io/en/latest/format.html)
/// - [Google Python Style Guide - Docstrings](https://google.github.io/styleguide/pyguide.html#38-comments-and-docstrings)
#[derive(ViolationMetadata)]
pub struct BlankLineAfterFunction {
    num_lines: usize,
}

impl AlwaysFixableViolation for BlankLineAfterFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BlankLineAfterFunction { num_lines } = self;
        format!("No blank lines allowed after function docstring (found {num_lines})")
    }

    fn fix_title(&self) -> String {
        "Remove blank line(s) after function docstring".to_string()
    }
}
