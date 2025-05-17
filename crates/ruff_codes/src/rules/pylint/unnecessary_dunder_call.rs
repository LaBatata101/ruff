use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for explicit use of dunder methods, like `__str__` and `__add__`.
///
/// ## Why is this bad?
/// Dunder names are not meant to be called explicitly and, in most cases, can
/// be replaced with builtins or operators.
///
/// ## Fix safety
/// This fix is always unsafe. When replacing dunder method calls with operators
/// or builtins, the behavior can change in the following ways:
///
/// 1. Types may implement only a subset of related dunder methods. Calling a
///    missing dunder method directly returns `NotImplemented`, but using the
///    equivalent operator raises a `TypeError`.
///    ```python
///    class C: pass
///    c = C()
///    c.__gt__(1)  # before fix: NotImplemented
///    c > 1        # after fix: raises TypeError
///    ```
/// 2. Instance-assigned dunder methods are ignored by operators and builtins.
///    ```python
///    class C: pass
///    c = C()
///    c.__bool__ = lambda: False
///    c.__bool__() # before fix: False
///    bool(c)      # after fix: True
///    ```
///
/// 3. Even with built-in types, behavior can differ.
///    ```python
///    (1).__gt__(1.0)  # before fix: NotImplemented
///    1 > 1.0          # after fix: False
///    ```
///
/// ## Example
/// ```python
/// three = (3.0).__str__()
/// twelve = "1".__add__("2")
///
///
/// def is_greater_than_two(x: int) -> bool:
///     return x.__gt__(2)
/// ```
///
/// Use instead:
/// ```python
/// three = str(3.0)
/// twelve = "1" + "2"
///
///
/// def is_greater_than_two(x: int) -> bool:
///     return x > 2
/// ```
///
#[derive(ViolationMetadata)]
pub struct UnnecessaryDunderCall {
    method: String,
    replacement: Option<String>,
}

impl Violation for UnnecessaryDunderCall {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let UnnecessaryDunderCall {
            method,
            replacement,
        } = self;

        if let Some(replacement) = replacement {
            format!("Unnecessary dunder call to `{method}`. {replacement}.")
        } else {
            format!("Unnecessary dunder call to `{method}`")
        }
    }

    fn fix_title(&self) -> Option<String> {
        let UnnecessaryDunderCall { replacement, .. } = self;
        replacement.clone()
    }
}