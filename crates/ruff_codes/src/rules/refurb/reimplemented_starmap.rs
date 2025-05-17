use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for generator expressions, list and set comprehensions that can
/// be replaced with `itertools.starmap`.
///
/// ## Why is this bad?
/// When unpacking values from iterators to pass them directly to
/// a function, prefer `itertools.starmap`.
///
/// Using `itertools.starmap` is more concise and readable. Furthermore, it is
/// more efficient than generator expressions, and in some versions of Python,
/// it is more efficient than comprehensions.
///
/// ## Known problems
/// Since Python 3.12, `itertools.starmap` is less efficient than
/// comprehensions ([#7771]). This is due to [PEP 709], which made
/// comprehensions faster.
///
/// ## Example
/// ```python
/// all(predicate(a, b) for a, b in some_iterable)
/// ```
///
/// Use instead:
/// ```python
/// from itertools import starmap
///
///
/// all(starmap(predicate, some_iterable))
/// ```
///
/// ## References
/// - [Python documentation: `itertools.starmap`](https://docs.python.org/3/library/itertools.html#itertools.starmap)
///
/// [PEP 709]: https://peps.python.org/pep-0709/
/// [#7771]: https://github.com/astral-sh/ruff/issues/7771
#[derive(ViolationMetadata)]
pub struct ReimplementedStarmap;

impl Violation for ReimplementedStarmap {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `itertools.starmap` instead of the generator".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `itertools.starmap`".to_string())
    }
}