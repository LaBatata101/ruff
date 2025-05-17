use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `itertools.starmap` calls where the second argument is a `zip` call.
///
/// ## Why is this bad?
/// `zip`-ping iterables only to unpack them later from within `starmap` is unnecessary.
/// For such cases, `map()` should be used instead.
///
/// ## Example
///
/// ```python
/// from itertools import starmap
///
///
/// starmap(func, zip(a, b))
/// starmap(func, zip(a, b, strict=True))
/// ```
///
/// Use instead:
///
/// ```python
/// map(func, a, b)
/// map(func, a, b, strict=True)  # 3.14+
/// ```
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe if the `starmap` or `zip` expressions contain comments that
/// would be deleted by applying the fix. Otherwise, the fix can be applied safely.
///
/// ## Fix availability
///
/// This rule will emit a diagnostic but not suggest a fix if `map` has been shadowed from its
/// builtin binding.
#[derive(ViolationMetadata)]
pub struct StarmapZip;

impl Violation for StarmapZip {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`itertools.starmap` called on `zip` iterable".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Use `map` instead".to_string())
    }
}