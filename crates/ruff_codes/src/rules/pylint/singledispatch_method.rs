use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for methods decorated with `@singledispatch`.
///
/// ## Why is this bad?
/// The `@singledispatch` decorator is intended for use with functions, not methods.
///
/// Instead, use the `@singledispatchmethod` decorator, or migrate the method to a
/// standalone function.
///
/// ## Example
///
/// ```python
/// from functools import singledispatch
///
///
/// class Class:
///     @singledispatch
///     def method(self, arg): ...
/// ```
///
/// Use instead:
///
/// ```python
/// from functools import singledispatchmethod
///
///
/// class Class:
///     @singledispatchmethod
///     def method(self, arg): ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as migrating from `@singledispatch` to
/// `@singledispatchmethod` may change the behavior of the code.
#[derive(ViolationMetadata)]
pub struct SingledispatchMethod;

impl Violation for SingledispatchMethod {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`@singledispatch` decorator should not be used on methods".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `@singledispatchmethod`".to_string())
    }
}