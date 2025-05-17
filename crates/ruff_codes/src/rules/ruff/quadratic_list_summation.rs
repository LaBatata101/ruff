use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of `sum()` to flatten lists of lists, which has
/// quadratic complexity.
///
/// ## Why is this bad?
/// The use of `sum()` to flatten lists of lists is quadratic in the number of
/// lists, as `sum()` creates a new list for each element in the summation.
///
/// Instead, consider using another method of flattening lists to avoid
/// quadratic complexity. The following methods are all linear in the number of
/// lists:
///
/// - `functools.reduce(operator.iadd, lists, [])`
/// - `list(itertools.chain.from_iterable(lists))`
/// - `[item for sublist in lists for item in sublist]`
///
/// When fixing relevant violations, Ruff defaults to the `functools.reduce`
/// form, which outperforms the other methods in [microbenchmarks].
///
/// ## Example
/// ```python
/// lists = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
/// joined = sum(lists, [])
/// ```
///
/// Use instead:
/// ```python
/// import functools
/// import operator
///
///
/// lists = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
/// functools.reduce(operator.iadd, lists, [])
/// ```
///
/// ## Fix safety
///
/// This fix is always marked as unsafe because `sum` uses the `__add__` magic method while
/// `operator.iadd` uses the `__iadd__` magic method, and these behave differently on lists.
/// The former requires the right summand to be a list, whereas the latter allows for any iterable.
/// Therefore, the fix could inadvertently cause code that previously raised an error to silently
/// succeed. Moreover, the fix could remove comments from the original code.
///
/// ## References
/// - [_How Not to Flatten a List of Lists in Python_](https://mathieularose.com/how-not-to-flatten-a-list-of-lists-in-python)
/// - [_How do I make a flat list out of a list of lists?_](https://stackoverflow.com/questions/952914/how-do-i-make-a-flat-list-out-of-a-list-of-lists/953097#953097)
///
/// [microbenchmarks]: https://github.com/astral-sh/ruff/issues/5073#issuecomment-1591836349
#[derive(ViolationMetadata)]
pub struct QuadraticListSummation;

impl AlwaysFixableViolation for QuadraticListSummation {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Avoid quadratic list summation".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `functools.reduce`".to_string()
    }
}