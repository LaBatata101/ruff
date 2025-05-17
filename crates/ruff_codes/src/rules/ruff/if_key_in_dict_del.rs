use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `if key in dictionary: del dictionary[key]`.
///
/// ## Why is this bad?
/// To remove a key-value pair from a dictionary, it's more concise to use `.pop(..., None)`.
///
/// ## Example
///
/// ```python
/// if key in dictionary:
///     del dictionary[key]
/// ```
///
/// Use instead:
///
/// ```python
/// dictionary.pop(key, None)
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as safe, unless the if statement contains comments.
#[derive(ViolationMetadata)]
pub struct IfKeyInDictDel;

impl AlwaysFixableViolation for IfKeyInDictDel {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `pop` instead of `key in dict` followed by `del dict[key]`".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace `if` statement with `.pop(..., None)`".to_string()
    }
}