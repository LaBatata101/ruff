use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for static methods that use `self` or `cls` as their first argument.
/// This rule also applies to `__new__` methods, which are implicitly static.
///
/// ## Why is this bad?
/// [PEP 8] recommends the use of `self` and `cls` as the first arguments for
/// instance methods and class methods, respectively. Naming the first argument
/// of a static method as `self` or `cls` can be misleading, as static methods
/// do not receive an instance or class reference as their first argument.
///
/// ## Example
/// ```python
/// class Wolf:
///     @staticmethod
///     def eat(self):
///         pass
/// ```
///
/// Use instead:
/// ```python
/// class Wolf:
///     @staticmethod
///     def eat(sheep):
///         pass
/// ```
///
/// [PEP 8]: https://peps.python.org/pep-0008/#function-and-method-arguments
#[derive(ViolationMetadata)]
pub struct BadStaticmethodArgument {
    argument_name: String,
}

impl Violation for BadStaticmethodArgument {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { argument_name } = self;
        format!("First argument of a static method should not be named `{argument_name}`")
    }
}