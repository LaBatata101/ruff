use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for non-method functions decorated with `@singledispatchmethod`.
///
/// ## Why is this bad?
/// The `@singledispatchmethod` decorator is intended for use with methods, not
/// functions.
///
/// Instead, use the `@singledispatch` decorator.
///
/// ## Example
///
/// ```python
/// from functools import singledispatchmethod
///
///
/// @singledispatchmethod
/// def func(arg): ...
/// ```
///
/// Use instead:
///
/// ```python
/// from functools import singledispatch
///
///
/// @singledispatch
/// def func(arg): ...
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as migrating from `@singledispatchmethod` to
/// `@singledispatch` may change the behavior of the code.
#[derive(ViolationMetadata)]
pub struct SingledispatchmethodFunction;

impl Violation for SingledispatchmethodFunction {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`@singledispatchmethod` decorator should not be used on non-method functions".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `@singledispatch`".to_string())
    }
}