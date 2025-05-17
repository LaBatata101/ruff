use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the use of a classmethod being made without the decorator.
///
/// ## Why is this bad?
/// When it comes to consistency and readability, it's preferred to use the decorator.
///
/// ## Example
///
/// ```python
/// class Foo:
///     def bar(cls): ...
///
///     bar = classmethod(bar)
/// ```
///
/// Use instead:
///
/// ```python
/// class Foo:
///     @classmethod
///     def bar(cls): ...
/// ```
#[derive(ViolationMetadata)]
pub struct NoClassmethodDecorator;

impl AlwaysFixableViolation for NoClassmethodDecorator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Class method defined without decorator".to_string()
    }

    fn fix_title(&self) -> String {
        "Add @classmethod decorator".to_string()
    }
}

/// ## What it does
/// Checks for the use of a staticmethod being made without the decorator.
///
/// ## Why is this bad?
/// When it comes to consistency and readability, it's preferred to use the decorator.
///
/// ## Example
///
/// ```python
/// class Foo:
///     def bar(arg1, arg2): ...
///
///     bar = staticmethod(bar)
/// ```
///
/// Use instead:
///
/// ```python
/// class Foo:
///     @staticmethod
///     def bar(arg1, arg2): ...
/// ```
#[derive(ViolationMetadata)]
pub struct NoStaticmethodDecorator;

impl AlwaysFixableViolation for NoStaticmethodDecorator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Static method defined without decorator".to_string()
    }

    fn fix_title(&self) -> String {
        "Add @staticmethod decorator".to_string()
    }
}
