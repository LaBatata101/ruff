use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for the presence of unused `self` parameter in methods definitions.
///
/// ## Why is this bad?
/// Unused `self` parameters are usually a sign of a method that could be
/// replaced by a function, class method, or static method.
///
/// ## Example
/// ```python
/// class Person:
///     def greeting(self):
///         print("Greetings friend!")
/// ```
///
/// Use instead:
/// ```python
/// def greeting():
///     print("Greetings friend!")
/// ```
///
/// or
///
/// ```python
/// class Person:
///     @staticmethod
///     def greeting():
///         print("Greetings friend!")
/// ```
#[derive(ViolationMetadata)]
pub struct NoSelfUse {
    method_name: String,
}

impl Violation for NoSelfUse {
    #[derive_message_formats]
    fn message(&self) -> String {
        let NoSelfUse { method_name } = self;
        format!("Method `{method_name}` could be a function, class method, or static method")
    }
}