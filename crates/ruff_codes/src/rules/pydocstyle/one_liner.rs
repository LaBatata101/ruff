use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for single-line docstrings that are broken across multiple lines.
///
/// ## Why is this bad?
/// [PEP 257] recommends that docstrings that _can_ fit on one line should be
/// formatted on a single line, for consistency and readability.
///
/// ## Example
/// ```python
/// def average(values: list[float]) -> float:
///     """
///     Return the mean of the given values.
///     """
/// ```
///
/// Use instead:
/// ```python
/// def average(values: list[float]) -> float:
///     """Return the mean of the given values."""
/// ```
///
/// ## Fix safety
/// The fix is marked as unsafe because it could affect tools that parse docstrings,
/// documentation generators, or custom introspection utilities that rely on
/// specific docstring formatting.
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
///
/// [PEP 257]: https://peps.python.org/pep-0257/
#[derive(ViolationMetadata)]
pub struct UnnecessaryMultilineDocstring;

impl Violation for UnnecessaryMultilineDocstring {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "One-line docstring should fit on one line".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Reformat to one line".to_string())
    }
}