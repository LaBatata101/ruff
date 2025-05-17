use std::fmt;

use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `dict.items()` that discard either the key or the value
/// when iterating over the dictionary.
///
/// ## Why is this bad?
/// If you only need the keys or values of a dictionary, you should use
/// `dict.keys()` or `dict.values()` respectively, instead of `dict.items()`.
/// These specialized methods are more efficient than `dict.items()`, as they
/// avoid allocating tuples for every item in the dictionary. They also
/// communicate the intent of the code more clearly.
///
/// Note that, as with all `perflint` rules, this is only intended as a
/// micro-optimization, and will have a negligible impact on performance in
/// most cases.
///
/// ## Example
/// ```python
/// obj = {"a": 1, "b": 2}
/// for key, value in obj.items():
///     print(value)
/// ```
///
/// Use instead:
/// ```python
/// obj = {"a": 1, "b": 2}
/// for value in obj.values():
///     print(value)
/// ```
///
/// ## Fix safety
/// The fix does not perform any type analysis and, as such, may suggest an
/// incorrect fix if the object in question does not duck-type as a mapping
/// (e.g., if it is missing a `.keys()` or `.values()` method, or if those
/// methods behave differently than they do on standard mapping types).
#[derive(ViolationMetadata)]
pub struct IncorrectDictIterator {
    subset: DictSubset,
}

impl AlwaysFixableViolation for IncorrectDictIterator {
    #[derive_message_formats]
    fn message(&self) -> String {
        let IncorrectDictIterator { subset } = self;
        format!("When using only the {subset} of a dict use the `{subset}()` method")
    }

    fn fix_title(&self) -> String {
        let IncorrectDictIterator { subset } = self;
        format!("Replace `.items()` with `.{subset}()`")
    }
}

// FIX: duplicated with ruff_rule_perflint::incorrect_dict_iterator
#[derive(Debug, PartialEq, Eq)]
enum DictSubset {
    Keys,
    Values,
}

impl fmt::Display for DictSubset {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DictSubset::Keys => fmt.write_str("keys"),
            DictSubset::Values => fmt.write_str("values"),
        }
    }
}