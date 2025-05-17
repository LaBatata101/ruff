use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for docstrings that are written via f-strings.
///
/// ## Why is this bad?
/// Python will interpret the f-string as a joined string, rather than as a
/// docstring. As such, the "docstring" will not be accessible via the
/// `__doc__` attribute, nor will it be picked up by any automated
/// documentation tooling.
///
/// ## Example
/// ```python
/// def foo():
///     f"""Not a docstring."""
/// ```
///
/// Use instead:
/// ```python
/// def foo():
///     """A docstring."""
/// ```
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
/// - [Python documentation: Formatted string literals](https://docs.python.org/3/reference/lexical_analysis.html#f-strings)
#[derive(ViolationMetadata)]
pub struct FStringDocstring;

impl Violation for FStringDocstring {
    #[derive_message_formats]
    fn message(&self) -> String {
        "f-string used as docstring. Python will interpret this as a joined string, rather than a docstring.".to_string()
    }
}