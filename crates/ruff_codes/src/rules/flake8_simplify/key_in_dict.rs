use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for key-existence checks against `dict.keys()` calls.
///
/// ## Why is this bad?
/// When checking for the existence of a key in a given dictionary, using
/// `key in dict` is more readable and efficient than `key in dict.keys()`,
/// while having the same semantics.
///
/// ## Example
/// ```python
/// key in foo.keys()
/// ```
///
/// Use instead:
/// ```python
/// key in foo
/// ```
///
/// ## Fix safety
/// Given `key in obj.keys()`, `obj` _could_ be a dictionary, or it could be
/// another type that defines a `.keys()` method. In the latter case, removing
/// the `.keys()` attribute could lead to a runtime error. The fix is marked
/// as safe when the type of `obj` is known to be a dictionary; otherwise, it
/// is marked as unsafe.
///
/// ## References
/// - [Python documentation: Mapping Types](https://docs.python.org/3/library/stdtypes.html#mapping-types-dict)
#[derive(ViolationMetadata)]
pub struct InDictKeys {
    operator: String,
}

impl AlwaysFixableViolation for InDictKeys {
    #[derive_message_formats]
    fn message(&self) -> String {
        let InDictKeys { operator } = self;
        format!("Use `key {operator} dict` instead of `key {operator} dict.keys()`")
    }

    fn fix_title(&self) -> String {
        "Remove `.keys()`".to_string()
    }
}