use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for comparisons to empty strings.
///
/// ## Why is this bad?
/// An empty string is falsy, so it is unnecessary to compare it to `""`. If
/// the value can be something else Python considers falsy, such as `None`,
/// `0`, or another empty container, then the code is not equivalent.
///
/// ## Known problems
/// High false positive rate, as the check is context-insensitive and does not
/// consider the type of the variable being compared ([#4282]).
///
/// ## Example
/// ```python
/// x: str = ...
///
/// if x == "":
///     print("x is empty")
/// ```
///
/// Use instead:
/// ```python
/// x: str = ...
///
/// if not x:
///     print("x is empty")
/// ```
///
/// ## References
/// - [Python documentation: Truth Value Testing](https://docs.python.org/3/library/stdtypes.html#truth-value-testing)
///
/// [#4282]: https://github.com/astral-sh/ruff/issues/4282
#[derive(ViolationMetadata)]
pub struct CompareToEmptyString {
    existing: String,
    replacement: String,
}

impl Violation for CompareToEmptyString {
    #[derive_message_formats]
    fn message(&self) -> String {
        let CompareToEmptyString {
            existing,
            replacement,
        } = self;
        format!("`{existing}` can be simplified to `{replacement}` as an empty string is falsey",)
    }
}