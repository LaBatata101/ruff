use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for long exception messages that are not defined in the exception
/// class itself.
///
/// ## Why is this bad?
/// By formatting an exception message at the `raise` site, the exception class
/// becomes less reusable, and may now raise inconsistent messages depending on
/// where it is raised.
///
/// If the exception message is instead defined within the exception class, it
/// will be consistent across all `raise` invocations.
///
/// This rule is not enforced for some built-in exceptions that are commonly
/// raised with a message and would be unusual to subclass, such as
/// `NotImplementedError`.
///
/// ## Example
/// ```python
/// class CantBeNegative(Exception):
///     pass
///
///
/// def foo(x):
///     if x < 0:
///         raise CantBeNegative(f"{x} is negative")
/// ```
///
/// Use instead:
/// ```python
/// class CantBeNegative(Exception):
///     def __init__(self, number):
///         super().__init__(f"{number} is negative")
///
///
/// def foo(x):
///     if x < 0:
///         raise CantBeNegative(x)
/// ```
#[derive(ViolationMetadata)]
pub struct RaiseVanillaArgs;

impl Violation for RaiseVanillaArgs {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Avoid specifying long messages outside the exception class".to_string()
    }
}