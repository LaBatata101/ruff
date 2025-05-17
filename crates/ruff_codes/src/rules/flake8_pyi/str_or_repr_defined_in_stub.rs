use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for redundant definitions of `__str__` or `__repr__` in stubs.
///
/// ## Why is this bad?
/// Defining `__str__` or `__repr__` in a stub is almost always redundant,
/// as the signatures are almost always identical to those of the default
/// equivalent, `object.__str__` and `object.__repr__`, respectively.
///
/// ## Example
///
/// ```pyi
/// class Foo:
///     def __repr__(self) -> str: ...
/// ```
#[derive(ViolationMetadata)]
pub struct StrOrReprDefinedInStub {
    name: String,
}

impl AlwaysFixableViolation for StrOrReprDefinedInStub {
    #[derive_message_formats]
    fn message(&self) -> String {
        let StrOrReprDefinedInStub { name } = self;
        format!("Defining `{name}` in a stub is almost always redundant")
    }

    fn fix_title(&self) -> String {
        let StrOrReprDefinedInStub { name } = self;
        format!("Remove definition of `{name}`")
    }
}