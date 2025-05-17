use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for usages of `collections.deque` that have an empty iterable as the first argument.
///
/// ## Why is this bad?
/// It's unnecessary to use an empty literal as a deque's iterable, since this is already the default behavior.
///
/// ## Example
///
/// ```python
/// from collections import deque
///
/// queue = deque(set())
/// queue = deque([], 10)
/// ```
///
/// Use instead:
///
/// ```python
/// from collections import deque
///
/// queue = deque()
/// queue = deque(maxlen=10)
/// ```
///
/// ## References
/// - [Python documentation: `collections.deque`](https://docs.python.org/3/library/collections.html#collections.deque)
#[derive(ViolationMetadata)]
pub struct UnnecessaryEmptyIterableWithinDequeCall {
    has_maxlen: bool,
}

impl AlwaysFixableViolation for UnnecessaryEmptyIterableWithinDequeCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary empty iterable within a deque call".to_string()
    }

    fn fix_title(&self) -> String {
        let title = if self.has_maxlen {
            "Replace with `deque(maxlen=...)`"
        } else {
            "Replace with `deque()`"
        };
        title.to_string()
    }
}