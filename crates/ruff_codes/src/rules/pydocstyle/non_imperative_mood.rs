use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for docstring first lines that are not in an imperative mood.
///
/// ## Why is this bad?
/// [PEP 257] recommends that the first line of a docstring be written in the
/// imperative mood, for consistency.
///
/// Hint: to rewrite the docstring in the imperative, phrase the first line as
/// if it were a command.
///
/// This rule may not apply to all projects; its applicability is a matter of
/// convention. By default, this rule is enabled when using the `numpy` and
/// `pep257` conventions, and disabled when using the `google` conventions.
///
/// ## Example
/// ```python
/// def average(values: list[float]) -> float:
///     """Returns the mean of the given values."""
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
/// - `lint.pydocstyle.property-decorators`
/// - `lint.pydocstyle.ignore-decorators`
///
/// ## References
/// - [PEP 257 – Docstring Conventions](https://peps.python.org/pep-0257/)
///
/// [PEP 257]: https://peps.python.org/pep-0257/
#[derive(ViolationMetadata)]
pub struct NonImperativeMood {
    first_line: String,
}

impl Violation for NonImperativeMood {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NonImperativeMood { first_line } = self;
        format!("First line of docstring should be in imperative mood: \"{first_line}\"")
    }
}