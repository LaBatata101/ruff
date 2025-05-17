use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for list reversals that can be performed in-place in lieu of
/// creating a new list.
///
/// ## Why is this bad?
/// When reversing a list, it's more efficient to use the in-place method
/// `.reverse()` instead of creating a new list, if the original list is
/// no longer needed.
///
/// ## Example
/// ```python
/// l = [1, 2, 3]
/// l = reversed(l)
///
/// l = [1, 2, 3]
/// l = list(reversed(l))
///
/// l = [1, 2, 3]
/// l = l[::-1]
/// ```
///
/// Use instead:
/// ```python
/// l = [1, 2, 3]
/// l.reverse()
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as calling `.reverse()` on a list
/// will mutate the list in-place, unlike `reversed`, which creates a new list
/// and leaves the original list unchanged.
///
/// If the list is referenced elsewhere, this could lead to unexpected
/// behavior.
///
/// ## References
/// - [Python documentation: More on Lists](https://docs.python.org/3/tutorial/datastructures.html#more-on-lists)
#[derive(ViolationMetadata)]
pub struct ListReverseCopy {
    name: String,
}

impl AlwaysFixableViolation for ListReverseCopy {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ListReverseCopy { name } = self;
        format!("Use of assignment of `reversed` on list `{name}`")
    }

    fn fix_title(&self) -> String {
        let ListReverseCopy { name } = self;
        format!("Replace with `{name}.reverse()`")
    }
}