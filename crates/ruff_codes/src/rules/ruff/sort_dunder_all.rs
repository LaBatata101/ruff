use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `__all__` definitions that are not ordered
/// according to an "isort-style" sort.
///
/// An isort-style sort orders items first according to their casing:
/// SCREAMING_SNAKE_CASE names (conventionally used for global constants)
/// come first, followed by CamelCase names (conventionally used for
/// classes), followed by anything else. Within each category,
/// a [natural sort](https://en.wikipedia.org/wiki/Natural_sort_order)
/// is used to order the elements.
///
/// ## Why is this bad?
/// Consistency is good. Use a common convention for `__all__` to make your
/// code more readable and idiomatic.
///
/// ## Example
/// ```python
/// import sys
///
/// __all__ = [
///     "b",
///     "c",
///     "a",
/// ]
///
/// if sys.platform == "win32":
///     __all__ += ["z", "y"]
/// ```
///
/// Use instead:
/// ```python
/// import sys
///
/// __all__ = [
///     "a",
///     "b",
///     "c",
/// ]
///
/// if sys.platform == "win32":
///     __all__ += ["y", "z"]
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe if there are any comments that take up
/// a whole line by themselves inside the `__all__` definition, for example:
/// ```py
/// __all__ = [
///     # eggy things
///     "duck_eggs",
///     "chicken_eggs",
///     # hammy things
///     "country_ham",
///     "parma_ham",
/// ]
/// ```
///
/// This is a common pattern used to delimit categories within a module's API,
/// but it would be out of the scope of this rule to attempt to maintain these
/// categories when alphabetically sorting the items of `__all__`.
///
/// The fix is also marked as unsafe if there are more than two `__all__` items
/// on a single line and that line also has a trailing comment, since here it
/// is impossible to accurately gauge which item the comment should be moved
/// with when sorting `__all__`:
/// ```py
/// __all__ = [
///     "a", "c", "e",  # a comment
///     "b", "d", "f",  # a second  comment
/// ]
/// ```
///
/// Other than this, the rule's fix is marked as always being safe, in that
/// it should very rarely alter the semantics of any Python code.
/// However, note that (although it's rare) the value of `__all__`
/// could be read by code elsewhere that depends on the exact
/// iteration order of the items in `__all__`, in which case this
/// rule's fix could theoretically cause breakage.
#[derive(ViolationMetadata)]
pub struct UnsortedDunderAll;

impl Violation for UnsortedDunderAll {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`__all__` is not sorted".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Apply an isort-style sorting to `__all__`".to_string())
    }
}