use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for loop control variables that override the loop iterable.
///
/// ## Why is this bad?
/// Loop control variables should not override the loop iterable, as this can
/// lead to confusing behavior.
///
/// Instead, use a distinct variable name for any loop control variables.
///
/// ## Example
/// ```python
/// items = [1, 2, 3]
///
/// for items in items:
///     print(items)
/// ```
///
/// Use instead:
/// ```python
/// items = [1, 2, 3]
///
/// for item in items:
///     print(item)
/// ```
///
/// ## References
/// - [Python documentation: The `for` statement](https://docs.python.org/3/reference/compound_stmts.html#the-for-statement)
#[derive(ViolationMetadata)]
pub struct LoopVariableOverridesIterator {
    name: String,
}

impl Violation for LoopVariableOverridesIterator {
    #[derive_message_formats]
    fn message(&self) -> String {
        let LoopVariableOverridesIterator { name } = self;
        format!("Loop control variable `{name}` overrides iterable it iterates")
    }
}