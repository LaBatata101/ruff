use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `for` loops that can be replaced by a dictionary comprehension.
///
/// ## Why is this bad?
/// When creating or extending a dictionary in a for-loop, prefer a dictionary
/// comprehension. Comprehensions are more readable and more performant.
///
/// For example, when comparing `{x: x for x in list(range(1000))}` to the `for`
/// loop version, the comprehension is ~10% faster on Python 3.11.
///
/// Note that, as with all `perflint` rules, this is only intended as a
/// micro-optimization, and will have a negligible impact on performance in
/// most cases.
///
/// ## Example
/// ```python
/// pairs = (("a", 1), ("b", 2))
/// result = {}
/// for x, y in pairs:
///     if y % 2:
///         result[x] = y
/// ```
///
/// Use instead:
/// ```python
/// pairs = (("a", 1), ("b", 2))
/// result = {x: y for x, y in pairs if y % 2}
/// ```
///
/// If you're appending to an existing dictionary, use the `update` method instead:
/// ```python
/// pairs = (("a", 1), ("b", 2))
/// result.update({x: y for x, y in pairs if y % 2})
/// ```
#[derive(ViolationMetadata)]
pub struct ManualDictComprehension {
    fix_type: DictComprehensionType,
    is_async: bool,
}

impl Violation for ManualDictComprehension {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let modifier = if self.is_async { "an async" } else { "a" };

        match self.fix_type {
            DictComprehensionType::Comprehension => {
                format!("Use a dictionary comprehension instead of {modifier} for-loop")
            }
            DictComprehensionType::Update => {
                format!("Use `dict.update` instead of {modifier} for-loop")
            }
        }
    }
    fn fix_title(&self) -> Option<String> {
        let modifier = if self.is_async { "async " } else { "" };
        match self.fix_type {
            DictComprehensionType::Comprehension => Some(format!(
                "Replace {modifier}for loop with dict comprehension"
            )),
            DictComprehensionType::Update => {
                Some(format!("Replace {modifier}for loop with `dict.update`"))
            }
        }
    }
}

// FIX: duplicated with ruff_rule_perflint::manual_dict_comprehension
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DictComprehensionType {
    Update,
    Comprehension,
}