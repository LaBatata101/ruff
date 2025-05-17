use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of `__metaclass__ = type` in class definitions.
///
/// ## Why is this bad?
/// Since Python 3, `__metaclass__ = type` is implied and can thus be omitted.
///
/// ## Example
///
/// ```python
/// class Foo:
///     __metaclass__ = type
/// ```
///
/// Use instead:
///
/// ```python
/// class Foo: ...
/// ```
///
/// ## References
/// - [PEP 3115 – Metaclasses in Python 3000](https://peps.python.org/pep-3115/)
#[derive(ViolationMetadata)]
pub struct UselessMetaclassType;

impl AlwaysFixableViolation for UselessMetaclassType {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`__metaclass__ = type` is implied".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `__metaclass__ = type`".to_string()
    }
}