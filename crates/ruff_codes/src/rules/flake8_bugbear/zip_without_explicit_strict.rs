use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `zip` calls without an explicit `strict` parameter.
///
/// ## Why is this bad?
/// By default, if the iterables passed to `zip` are of different lengths, the
/// resulting iterator will be silently truncated to the length of the shortest
/// iterable. This can lead to subtle bugs.
///
/// Pass `strict=True` to raise a `ValueError` if the iterables are of
/// non-uniform length. Alternatively, if the iterables are deliberately of
/// different lengths, pass `strict=False` to make the intention explicit.
///
/// ## Example
/// ```python
/// zip(a, b)
/// ```
///
/// Use instead:
/// ```python
/// zip(a, b, strict=True)
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe for `zip` calls that contain
/// `**kwargs`, as adding a `strict` keyword argument to such a call may lead
/// to a duplicate keyword argument error.
///
/// ## References
/// - [Python documentation: `zip`](https://docs.python.org/3/library/functions.html#zip)
#[derive(ViolationMetadata)]
pub struct ZipWithoutExplicitStrict;

impl AlwaysFixableViolation for ZipWithoutExplicitStrict {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`zip()` without an explicit `strict=` parameter".to_string()
    }

    fn fix_title(&self) -> String {
        "Add explicit value for parameter `strict=`".to_string()
    }
}