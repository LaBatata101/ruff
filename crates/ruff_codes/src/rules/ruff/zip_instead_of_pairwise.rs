use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for use of `zip()` to iterate over successive pairs of elements.
///
/// ## Why is this bad?
/// When iterating over successive pairs of elements, prefer
/// `itertools.pairwise()` over `zip()`.
///
/// `itertools.pairwise()` is more readable and conveys the intent of the code
/// more clearly.
///
/// ## Example
/// ```python
/// letters = "ABCD"
/// zip(letters, letters[1:])  # ("A", "B"), ("B", "C"), ("C", "D")
/// ```
///
/// Use instead:
/// ```python
/// from itertools import pairwise
///
/// letters = "ABCD"
/// pairwise(letters)  # ("A", "B"), ("B", "C"), ("C", "D")
/// ```
///
/// ## Fix safety
///
/// The fix is always marked unsafe because it assumes that slicing an object
/// (e.g., `obj[1:]`) produces a value with the same type and iteration behavior
/// as the original object, which is not guaranteed for user-defined types that
/// override `__getitem__` without properly handling slices. Moreover, the fix
/// could delete comments.
///
/// ## References
/// - [Python documentation: `itertools.pairwise`](https://docs.python.org/3/library/itertools.html#itertools.pairwise)
#[derive(ViolationMetadata)]
pub struct ZipInsteadOfPairwise;

impl Violation for ZipInsteadOfPairwise {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Prefer `itertools.pairwise()` over `zip()` when iterating over successive pairs"
            .to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace `zip()` with `itertools.pairwise()`".to_string())
    }
}