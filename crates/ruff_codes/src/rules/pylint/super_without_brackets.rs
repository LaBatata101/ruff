use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Detects attempts to use `super` without parentheses.
///
/// ## Why is this bad?
/// The [`super()` callable](https://docs.python.org/3/library/functions.html#super)
/// can be used inside method definitions to create a proxy object that
/// delegates attribute access to a superclass of the current class. Attempting
/// to access attributes on `super` itself, however, instead of the object
/// returned by a call to `super()`, will raise `AttributeError`.
///
/// ## Example
/// ```python
/// class Animal:
///     @staticmethod
///     def speak():
///         return "This animal says something."
///
///
/// class Dog(Animal):
///     @staticmethod
///     def speak():
///         original_speak = super.speak()  # ERROR: `super.speak()`
///         return f"{original_speak} But as a dog, it barks!"
/// ```
///
/// Use instead:
/// ```python
/// class Animal:
///     @staticmethod
///     def speak():
///         return "This animal says something."
///
///
/// class Dog(Animal):
///     @staticmethod
///     def speak():
///         original_speak = super().speak()  # Correct: `super().speak()`
///         return f"{original_speak} But as a dog, it barks!"
/// ```
#[derive(ViolationMetadata)]
pub struct SuperWithoutBrackets;

impl AlwaysFixableViolation for SuperWithoutBrackets {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`super` call is missing parentheses".to_string()
    }

    fn fix_title(&self) -> String {
        "Add parentheses to `super` call".to_string()
    }
}