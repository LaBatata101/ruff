use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for classes that inherit from `object`.
///
/// ## Why is this bad?
/// Since Python 3, all classes inherit from `object` by default, so `object` can
/// be omitted from the list of base classes.
///
/// ## Example
///
/// ```python
/// class Foo(object): ...
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
pub struct UselessObjectInheritance {
    name: String,
}

impl AlwaysFixableViolation for UselessObjectInheritance {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UselessObjectInheritance { name } = self;
        format!("Class `{name}` inherits from `object`")
    }

    fn fix_title(&self) -> String {
        "Remove `object` inheritance".to_string()
    }
}