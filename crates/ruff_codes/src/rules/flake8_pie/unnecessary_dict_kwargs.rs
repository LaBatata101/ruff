use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary `dict` kwargs.
///
/// ## Why is this bad?
/// If the `dict` keys are valid identifiers, they can be passed as keyword
/// arguments directly.
///
/// ## Example
///
/// ```python
/// def foo(bar):
///     return bar + 1
///
///
/// print(foo(**{"bar": 2}))  # prints 3
/// ```
///
/// Use instead:
///
/// ```python
/// def foo(bar):
///     return bar + 1
///
///
/// print(foo(bar=2))  # prints 3
/// ```
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe for dictionaries with comments interleaved between
/// the items, as comments may be removed.
///
/// For example, the fix would be marked as unsafe in the following case:
///
/// ```python
/// foo(
///     **{
///         # comment
///         "x": 1.0,
///         # comment
///         "y": 2.0,
///     }
/// )
/// ```
///
/// as this is converted to `foo(x=1.0, y=2.0)` without any of the comments.
///
/// ## References
/// - [Python documentation: Dictionary displays](https://docs.python.org/3/reference/expressions.html#dictionary-displays)
/// - [Python documentation: Calls](https://docs.python.org/3/reference/expressions.html#calls)
#[derive(ViolationMetadata)]
pub struct UnnecessaryDictKwargs;

impl Violation for UnnecessaryDictKwargs {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary `dict` kwargs".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Remove unnecessary kwargs".to_string())
    }
}