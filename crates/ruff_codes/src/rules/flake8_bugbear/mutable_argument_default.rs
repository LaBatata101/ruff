use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of mutable objects as function argument defaults.
///
/// ## Why is this bad?
/// Function defaults are evaluated once, when the function is defined.
///
/// The same mutable object is then shared across all calls to the function.
/// If the object is modified, those modifications will persist across calls,
/// which can lead to unexpected behavior.
///
/// Instead, prefer to use immutable data structures, or take `None` as a
/// default, and initialize a new mutable object inside the function body
/// for each call.
///
/// Arguments with immutable type annotations will be ignored by this rule.
/// Types outside of the standard library can be marked as immutable with the
/// [`lint.flake8-bugbear.extend-immutable-calls`] configuration option.
///
/// ## Known problems
/// Mutable argument defaults can be used intentionally to cache computation
/// results. Replacing the default with `None` or an immutable data structure
/// does not work for such usages. Instead, prefer the `@functools.lru_cache`
/// decorator from the standard library.
///
/// ## Example
/// ```python
/// def add_to_list(item, some_list=[]):
///     some_list.append(item)
///     return some_list
///
///
/// l1 = add_to_list(0)  # [0]
/// l2 = add_to_list(1)  # [0, 1]
/// ```
///
/// Use instead:
/// ```python
/// def add_to_list(item, some_list=None):
///     if some_list is None:
///         some_list = []
///     some_list.append(item)
///     return some_list
///
///
/// l1 = add_to_list(0)  # [0]
/// l2 = add_to_list(1)  # [1]
/// ```
///
/// ## Options
/// - `lint.flake8-bugbear.extend-immutable-calls`
///
/// ## References
/// - [Python documentation: Default Argument Values](https://docs.python.org/3/tutorial/controlflow.html#default-argument-values)
#[derive(ViolationMetadata)]
pub struct MutableArgumentDefault;

impl Violation for MutableArgumentDefault {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Do not use mutable data structures for argument defaults".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `None`; initialize within function".to_string())
    }
}