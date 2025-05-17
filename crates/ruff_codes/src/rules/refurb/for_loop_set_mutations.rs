use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for code that updates a set with the contents of an iterable by
/// using a `for` loop to call `.add()` or `.discard()` on each element
/// separately.
///
/// ## Why is this bad?
/// When adding or removing a batch of elements to or from a set, it's more
/// idiomatic to use a single method call rather than adding or removing
/// elements one by one.
///
/// ## Example
/// ```python
/// s = set()
///
/// for x in (1, 2, 3):
///     s.add(x)
///
/// for x in (1, 2, 3):
///     s.discard(x)
/// ```
///
/// Use instead:
/// ```python
/// s = set()
///
/// s.update((1, 2, 3))
/// s.difference_update((1, 2, 3))
/// ```
///
/// ## Fix safety
/// The fix will be marked as unsafe if applying the fix would delete any comments.
/// Otherwise, it is marked as safe.
///
/// ## References
/// - [Python documentation: `set`](https://docs.python.org/3/library/stdtypes.html#set)
#[derive(ViolationMetadata)]
pub struct ForLoopSetMutations {
    method_name: &'static str,
    batch_method_name: &'static str,
}

impl AlwaysFixableViolation for ForLoopSetMutations {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Use of `set.{}()` in a for loop", self.method_name)
    }

    fn fix_title(&self) -> String {
        format!("Replace with `.{}()`", self.batch_method_name)
    }
}