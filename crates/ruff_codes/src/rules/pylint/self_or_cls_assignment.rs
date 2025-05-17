use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for assignment of `self` and `cls` in instance and class methods respectively.
///
/// This check also applies to `__new__` even though this is technically
/// a static method.
///
/// ## Why is this bad?
/// The identifiers `self` and `cls` are conventional in Python for the first parameter of instance
/// methods and class methods, respectively. Assigning new values to these variables can be
/// confusing for others reading your code; using a different variable name can lead to clearer
/// code.
///
/// ## Example
///
/// ```python
/// class Version:
///     def add(self, other):
///         self = self + other
///         return self
///
///     @classmethod
///     def superclass(cls):
///         cls = cls.__mro__[-1]
///         return cls
/// ```
///
/// Use instead:
/// ```python
/// class Version:
///     def add(self, other):
///         new_version = self + other
///         return new_version
///
///     @classmethod
///     def superclass(cls):
///         supercls = cls.__mro__[-1]
///         return supercls
/// ```
#[derive(ViolationMetadata)]
pub struct SelfOrClsAssignment {
    method_type: MethodType,
}

impl Violation for SelfOrClsAssignment {
    #[derive_message_formats]
    fn message(&self) -> String {
        let SelfOrClsAssignment { method_type } = self;

        format!(
            "Reassigned `{}` variable in {method_type} method",
            method_type.arg_name(),
        )
    }

    fn fix_title(&self) -> Option<String> {
        Some("Consider using a different variable name".to_string())
    }
}

// FIX: duplicated with rufe_rule_pylint::self_or_cls_assignment
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MethodType {
    Instance,
    Class,
    New,
}

impl MethodType {
    const fn arg_name(self) -> &'static str {
        match self {
            MethodType::Instance => "self",
            MethodType::Class => "cls",
            MethodType::New => "cls",
        }
    }
}

impl std::fmt::Display for MethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodType::Instance => f.write_str("instance"),
            MethodType::Class => f.write_str("class"),
            MethodType::New => f.write_str("`__new__`"),
        }
    }
}