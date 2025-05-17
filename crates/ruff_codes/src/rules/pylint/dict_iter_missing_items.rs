use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for dictionary unpacking in a for loop without calling `.items()`.
///
/// ## Why is this bad?
/// When iterating over a dictionary in a for loop, if a dictionary is unpacked
/// without calling `.items()`, it could lead to a runtime error if the keys are not
/// a tuple of two elements.
///
/// It is likely that you're looking for an iteration over (key, value) pairs which
/// can only be achieved when calling `.items()`.
///
/// ## Example
/// ```python
/// data = {"Paris": 2_165_423, "New York City": 8_804_190, "Tokyo": 13_988_129}
///
/// for city, population in data:
///     print(f"{city} has population {population}.")
/// ```
///
/// Use instead:
/// ```python
/// data = {"Paris": 2_165_423, "New York City": 8_804_190, "Tokyo": 13_988_129}
///
/// for city, population in data.items():
///     print(f"{city} has population {population}.")
/// ```
///
/// ## Known problems
/// If the dictionary key is a tuple, e.g.:
///
/// ```python
/// d = {(1, 2): 3, (3, 4): 5}
/// for x, y in d:
///     print(x, y)
/// ```
///
/// The tuple key is unpacked into `x` and `y` instead of the key and values. This means that
/// the suggested fix of using `d.items()` would result in different runtime behavior. Ruff
/// cannot consistently infer the type of a dictionary's keys.
///
/// ## Fix safety
/// Due to the known problem with tuple keys, this fix is unsafe.
#[derive(ViolationMetadata)]
pub struct DictIterMissingItems;

impl AlwaysFixableViolation for DictIterMissingItems {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unpacking a dictionary in iteration without calling `.items()`".to_string()
    }

    fn fix_title(&self) -> String {
        "Add a call to `.items()`".to_string()
    }
}