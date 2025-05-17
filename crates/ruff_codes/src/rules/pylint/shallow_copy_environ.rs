use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Check for shallow `os.environ` copies.
///
/// ## Why is this bad?
/// `os.environ` is not a `dict` object, but rather, a proxy object. As such, mutating a shallow
/// copy of `os.environ` will also mutate the original object.
///
/// See [BPO 15373] for more information.
///
/// ## Example
/// ```python
/// import copy
/// import os
///
/// env = copy.copy(os.environ)
/// ```
///
/// Use instead:
/// ```python
/// import os
///
/// env = os.environ.copy()
/// ```
///
/// ## Fix safety
///
/// This rule's fix is marked as unsafe because replacing a shallow copy with a deep copy can lead
/// to unintended side effects. If the program modifies the shallow copy at some point, changing it
/// to a deep copy may prevent those modifications from affecting the original data, potentially
/// altering the program's behavior.
///
/// ## References
/// - [Python documentation: `copy` — Shallow and deep copy operations](https://docs.python.org/3/library/copy.html)
/// - [Python documentation: `os.environ`](https://docs.python.org/3/library/os.html#os.environ)
///
/// [BPO 15373]: https://bugs.python.org/issue15373
#[derive(ViolationMetadata)]
pub struct ShallowCopyEnviron;

impl AlwaysFixableViolation for ShallowCopyEnviron {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Shallow copy of `os.environ` via `copy.copy(os.environ)`".to_string()
    }

    fn fix_title(&self) -> String {
        "Replace with `os.environ.copy()`".to_string()
    }
}