use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary nested `Literal`.
///
/// ## Why is this bad?
/// Prefer using a single `Literal`, which is equivalent and more concise.
///
/// Parameterization of literals by other literals is supported as an ergonomic
/// feature as proposed in [PEP 586], to enable patterns such as:
/// ```python
/// ReadOnlyMode         = Literal["r", "r+"]
/// WriteAndTruncateMode = Literal["w", "w+", "wt", "w+t"]
/// WriteNoTruncateMode  = Literal["r+", "r+t"]
/// AppendMode           = Literal["a", "a+", "at", "a+t"]
///
/// AllModes = Literal[ReadOnlyMode, WriteAndTruncateMode,
///                   WriteNoTruncateMode, AppendMode]
/// ```
///
/// As a consequence, type checkers also support nesting of literals
/// which is less readable than a flat `Literal`:
/// ```python
/// AllModes = Literal[Literal["r", "r+"], Literal["w", "w+", "wt", "w+t"],
///                   Literal["r+", "r+t"], Literal["a", "a+", "at", "a+t"]]
/// ```
///
/// ## Example
/// ```python
/// AllModes = Literal[
///     Literal["r", "r+"],
///     Literal["w", "w+", "wt", "w+t"],
///     Literal["r+", "r+t"],
///     Literal["a", "a+", "at", "a+t"],
/// ]
/// ```
///
/// Use instead:
/// ```python
/// AllModes = Literal[
///     "r", "r+", "w", "w+", "wt", "w+t", "r+", "r+t", "a", "a+", "at", "a+t"
/// ]
/// ```
///
/// or assign the literal to a variable as in the first example.
///
/// ## Fix safety
/// The fix for this rule is marked as unsafe when the `Literal` slice is split
/// across multiple lines and some of the lines have trailing comments.
///
/// ## References
/// - [Typing documentation: Legal parameters for `Literal` at type check time](https://typing.python.org/en/latest/spec/literal.html#legal-parameters-for-literal-at-type-check-time)
///
/// [PEP 586](https://peps.python.org/pep-0586/)
#[derive(ViolationMetadata)]
pub struct UnnecessaryNestedLiteral;

impl Violation for UnnecessaryNestedLiteral {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary nested `Literal`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with flattened `Literal`".to_string())
    }
}