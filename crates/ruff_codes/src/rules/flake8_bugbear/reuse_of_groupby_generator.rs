use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for multiple usage of the generator returned from
/// `itertools.groupby()`.
///
/// ## Why is this bad?
/// Using the generator more than once will do nothing on the second usage.
/// If that data is needed later, it should be stored as a list.
///
/// ## Example:
/// ```python
/// import itertools
///
/// for name, group in itertools.groupby(data):
///     for _ in range(5):
///         do_something_with_the_group(group)
/// ```
///
/// Use instead:
/// ```python
/// import itertools
///
/// for name, group in itertools.groupby(data):
///     values = list(group)
///     for _ in range(5):
///         do_something_with_the_group(values)
/// ```
#[derive(ViolationMetadata)]
pub struct ReuseOfGroupbyGenerator;

impl Violation for ReuseOfGroupbyGenerator {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Using the generator returned from `itertools.groupby()` more than once will do nothing on the second usage".to_string()
    }
}