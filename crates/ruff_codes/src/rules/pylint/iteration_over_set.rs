use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for iteration over a `set` literal where each element in the set is
/// itself a literal value.
///
/// ## Why is this bad?
/// Iterating over a `set` is less efficient than iterating over a sequence
/// type, like `list` or `tuple`.
///
/// ## Example
/// ```python
/// for number in {1, 2, 3}:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// for number in (1, 2, 3):
///     ...
/// ```
///
/// ## References
/// - [Python documentation: `set`](https://docs.python.org/3/library/stdtypes.html#set)
#[derive(ViolationMetadata)]
pub struct IterationOverSet;

impl AlwaysFixableViolation for IterationOverSet {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use a sequence type instead of a `set` when iterating over values".to_string()
    }

    fn fix_title(&self) -> String {
        "Convert to `tuple`".to_string()
    }
}