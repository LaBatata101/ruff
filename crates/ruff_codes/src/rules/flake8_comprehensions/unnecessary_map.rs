use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::common::ObjectType;

/// ## What it does
/// Checks for unnecessary `map()` calls with lambda functions.
///
/// ## Why is this bad?
/// Using `map(func, iterable)` when `func` is a lambda is slower than
/// using a generator expression or a comprehension, as the latter approach
/// avoids the function call overhead, in addition to being more readable.
///
/// This rule also applies to `map()` calls within `list()`, `set()`, and
/// `dict()` calls. For example:
///
/// - Instead of `list(map(lambda num: num * 2, nums))`, use
///   `[num * 2 for num in nums]`.
/// - Instead of `set(map(lambda num: num % 2 == 0, nums))`, use
///   `{num % 2 == 0 for num in nums}`.
/// - Instead of `dict(map(lambda v: (v, v ** 2), values))`, use
///   `{v: v ** 2 for v in values}`.
///
/// ## Example
/// ```python
/// map(lambda x: x + 1, iterable)
/// ```
///
/// Use instead:
/// ```python
/// (x + 1 for x in iterable)
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as it may occasionally drop comments
/// when rewriting the call. In most cases, though, comments will be preserved.
#[derive(ViolationMetadata)]
pub struct UnnecessaryMap {
    object_type: ObjectType,
}

impl Violation for UnnecessaryMap {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryMap { object_type } = self;
        format!("Unnecessary `map()` usage (rewrite using a {object_type})")
    }

    fn fix_title(&self) -> Option<String> {
        let UnnecessaryMap { object_type } = self;
        Some(format!("Replace `map()` with a {object_type}"))
    }
}