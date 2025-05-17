use std::fmt;

use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `enumerate` that discard either the index or the value
/// when iterating over a sequence.
///
/// ## Why is this bad?
/// The built-in `enumerate` function is useful when you need both the index and
/// value of a sequence.
///
/// If you only need the index or values of a sequence, you should iterate over
/// `range(len(...))` or the sequence itself, respectively, instead. This is
/// more efficient and communicates the intent of the code more clearly.
///
/// ## Known problems
/// This rule is prone to false negatives due to type inference limitations;
/// namely, it will only suggest a fix using the `len` builtin function if the
/// sequence passed to `enumerate` is an instantiated as a list, set, dict, or
/// tuple literal, or annotated as such with a type annotation.
///
/// The `len` builtin function is not defined for all object types (such as
/// generators), and so refactoring to use `len` over `enumerate` is not always
/// safe.
///
/// ## Example
/// ```python
/// for index, _ in enumerate(sequence):
///     print(index)
///
/// for _, value in enumerate(sequence):
///     print(value)
/// ```
///
/// Use instead:
/// ```python
/// for index in range(len(sequence)):
///     print(index)
///
/// for value in sequence:
///     print(value)
/// ```
///
/// ## References
/// - [Python documentation: `enumerate`](https://docs.python.org/3/library/functions.html#enumerate)
/// - [Python documentation: `range`](https://docs.python.org/3/library/stdtypes.html#range)
/// - [Python documentation: `len`](https://docs.python.org/3/library/functions.html#len)
#[derive(ViolationMetadata)]
pub struct UnnecessaryEnumerate {
    subset: EnumerateSubset,
}

impl Violation for UnnecessaryEnumerate {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        match self.subset {
            EnumerateSubset::Indices => {
                "`enumerate` value is unused, use `for x in range(len(y))` instead".to_string()
            }
            EnumerateSubset::Values => {
                "`enumerate` index is unused, use `for x in y` instead".to_string()
            }
        }
    }

    fn fix_title(&self) -> Option<String> {
        let title = match self.subset {
            EnumerateSubset::Indices => "Replace with `range(len(...))`",
            EnumerateSubset::Values => "Remove `enumerate`",
        };
        Some(title.to_string())
    }
}

// FIX: duplicated with ruff_rule_refurb::unnecessary_enumerate
#[derive(Debug, PartialEq, Eq)]
enum EnumerateSubset {
    /// E.g., `for _, value in enumerate(sequence):`.
    Indices,
    /// E.g., `for index, _ in enumerate(sequence):`.
    Values,
}

impl fmt::Display for EnumerateSubset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnumerateSubset::Indices => write!(f, "indices"),
            EnumerateSubset::Values => write!(f, "values"),
        }
    }
}