use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for docstrings in which the first line does not end in a period.
///
/// ## Why is this bad?
/// [PEP 257] recommends that the first line of a docstring is written in the
/// form of a command, ending in a period.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, this rule is enabled when using the `numpy` and
/// `pep257` conventions, and disabled when using the `google` convention.
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
///
/// [PEP 257]: https://peps.python.org/pep-0257/
#[derive(ViolationMetadata)]
pub struct MissingTrailingPeriod;

impl Violation for MissingTrailingPeriod {
    /// `None` in the case a fix is never available or otherwise Some
    /// [`FixAvailability`] describing the available fix.
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "First line should end with a period".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Add period".to_string())
    }
}