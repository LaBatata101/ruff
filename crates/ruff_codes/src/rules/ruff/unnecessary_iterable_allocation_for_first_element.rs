use ruff_diagnostics::AlwaysFixableViolation;
use ruff_linter_commons::snippet::SourceCodeSnippet;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks the following constructs, all of which can be replaced by
/// `next(iter(...))`:
///
/// - `list(...)[0]`
/// - `tuple(...)[0]`
/// - `list(i for i in ...)[0]`
/// - `[i for i in ...][0]`
/// - `list(...).pop(0)`
///
/// ## Why is this bad?
/// Calling e.g. `list(...)` will create a new list of the entire collection,
/// which can be very expensive for large collections. If you only need the
/// first element of the collection, you can use `next(...)` or
/// `next(iter(...)` to lazily fetch the first element. The same is true for
/// the other constructs.
///
/// ## Example
/// ```python
/// head = list(x)[0]
/// head = [x * x for x in range(10)][0]
/// ```
///
/// Use instead:
/// ```python
/// head = next(iter(x))
/// head = next(x * x for x in range(10))
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as migrating from (e.g.) `list(...)[0]`
/// to `next(iter(...))` can change the behavior of your program in two ways:
///
/// 1. First, all above-mentioned constructs will eagerly evaluate the entire
///    collection, while `next(iter(...))` will only evaluate the first
///    element. As such, any side effects that occur during iteration will be
///    delayed.
/// 2. Second, accessing members of a collection via square bracket notation
///    `[0]` of the `pop()` function will raise `IndexError` if the collection
///    is empty, while `next(iter(...))` will raise `StopIteration`.
///
/// ## References
/// - [Iterators and Iterables in Python: Run Efficient Iterations](https://realpython.com/python-iterators-iterables/#when-to-use-an-iterator-in-python)
#[derive(ViolationMetadata)]
pub struct UnnecessaryIterableAllocationForFirstElement {
    iterable: SourceCodeSnippet,
}

impl AlwaysFixableViolation for UnnecessaryIterableAllocationForFirstElement {
    #[derive_message_formats]
    fn message(&self) -> String {
        let iterable = &self.iterable.truncated_display();
        format!("Prefer `next({iterable})` over single element slice")
    }

    fn fix_title(&self) -> String {
        let iterable = &self.iterable.truncated_display();
        format!("Replace with `next({iterable})`")
    }
}