use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `for` loops with explicit loop-index variables that can be replaced
/// with `enumerate()`.
///
/// ## Why is this bad?
/// When iterating over a sequence, it's often desirable to keep track of the
/// index of each element alongside the element itself. Prefer the `enumerate`
/// builtin over manually incrementing a counter variable within the loop, as
/// `enumerate` is more concise and idiomatic.
///
/// ## Example
/// ```python
/// fruits = ["apple", "banana", "cherry"]
/// for fruit in fruits:
///     print(f"{i + 1}. {fruit}")
///     i += 1
/// ```
///
/// Use instead:
/// ```python
/// fruits = ["apple", "banana", "cherry"]
/// for i, fruit in enumerate(fruits):
///     print(f"{i + 1}. {fruit}")
/// ```
///
/// ## References
/// - [Python documentation: `enumerate`](https://docs.python.org/3/library/functions.html#enumerate)
#[derive(ViolationMetadata)]
pub struct EnumerateForLoop {
    index: String,
}

impl Violation for EnumerateForLoop {
    #[derive_message_formats]
    fn message(&self) -> String {
        let EnumerateForLoop { index } = self;
        format!("Use `enumerate()` for index variable `{index}` in `for` loop")
    }
}