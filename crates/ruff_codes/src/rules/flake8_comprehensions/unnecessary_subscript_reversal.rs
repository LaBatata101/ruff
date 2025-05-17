use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary subscript reversal of iterable.
///
/// ## Why is this bad?
/// It's unnecessary to reverse the order of an iterable when passing it
/// into `reversed()`, `set()` or `sorted()` functions as they will change
/// the order of the elements again.
///
/// ## Example
/// ```python
/// sorted(iterable[::-1])
/// set(iterable[::-1])
/// reversed(iterable[::-1])
/// ```
///
/// Use instead:
/// ```python
/// sorted(iterable)
/// set(iterable)
/// iterable
/// ```
#[derive(ViolationMetadata)]
pub struct UnnecessarySubscriptReversal {
    func: String,
}

impl Violation for UnnecessarySubscriptReversal {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessarySubscriptReversal { func } = self;
        format!("Unnecessary subscript reversal of iterable within `{func}()`")
    }
}