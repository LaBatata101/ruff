use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `raise` statements that raise `NotImplemented`.
///
/// ## Why is this bad?
/// `NotImplemented` is an exception used by binary special methods to indicate
/// that an operation is not implemented with respect to a particular type.
///
/// `NotImplemented` should not be raised directly. Instead, raise
/// `NotImplementedError`, which is used to indicate that the method is
/// abstract or not implemented in the derived class.
///
/// ## Example
/// ```python
/// class Foo:
///     def bar(self):
///         raise NotImplemented
/// ```
///
/// Use instead:
/// ```python
/// class Foo:
///     def bar(self):
///         raise NotImplementedError
/// ```
///
/// ## References
/// - [Python documentation: `NotImplemented`](https://docs.python.org/3/library/constants.html#NotImplemented)
/// - [Python documentation: `NotImplementedError`](https://docs.python.org/3/library/exceptions.html#NotImplementedError)
#[derive(ViolationMetadata)]
pub struct RaiseNotImplemented;

impl Violation for RaiseNotImplemented {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "`raise NotImplemented` should be `raise NotImplementedError`".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Use `raise NotImplementedError`".to_string())
    }
}