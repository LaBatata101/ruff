use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for property definitions that accept function parameters.
///
/// ## Why is this bad?
/// Properties cannot be called with parameters.
///
/// If you need to pass parameters to a property, create a method with the
/// desired parameters and call that method instead.
///
/// ## Example
///
/// ```python
/// class Cat:
///     @property
///     def purr(self, volume): ...
/// ```
///
/// Use instead:
///
/// ```python
/// class Cat:
///     @property
///     def purr(self): ...
///
///     def purr_volume(self, volume): ...
/// ```
///
/// ## References
/// - [Python documentation: `property`](https://docs.python.org/3/library/functions.html#property)
#[derive(ViolationMetadata)]
pub struct PropertyWithParameters;

impl Violation for PropertyWithParameters {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Cannot have defined parameters for properties".to_string()
    }
}