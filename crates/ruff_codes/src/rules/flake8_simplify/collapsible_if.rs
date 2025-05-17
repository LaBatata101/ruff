use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for nested `if` statements that can be collapsed into a single `if`
/// statement.
///
/// ## Why is this bad?
/// Nesting `if` statements leads to deeper indentation and makes code harder to
/// read. Instead, combine the conditions into a single `if` statement with an
/// `and` operator.
///
/// ## Example
/// ```python
/// if foo:
///     if bar:
///         ...
/// ```
///
/// Use instead:
/// ```python
/// if foo and bar:
///     ...
/// ```
///
/// ## References
/// - [Python documentation: The `if` statement](https://docs.python.org/3/reference/compound_stmts.html#the-if-statement)
/// - [Python documentation: Boolean operations](https://docs.python.org/3/reference/expressions.html#boolean-operations)
#[derive(ViolationMetadata)]
pub struct CollapsibleIf;

impl Violation for CollapsibleIf {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use a single `if` statement instead of nested `if` statements".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Combine `if` statements using `and`".to_string())
    }
}