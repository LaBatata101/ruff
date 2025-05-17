use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for direct instantiation of `logging.Logger`, as opposed to using
/// `logging.getLogger()`.
///
/// ## Why is this bad?
/// The [Logger Objects] documentation states that:
///
/// > Note that Loggers should NEVER be instantiated directly, but always
/// > through the module-level function `logging.getLogger(name)`.
///
/// If a logger is directly instantiated, it won't be added to the logger
/// tree, and will bypass all configuration. Messages logged to it will
/// only be sent to the "handler of last resort", skipping any filtering
/// or formatting.
///
/// ## Example
/// ```python
/// import logging
///
/// logger = logging.Logger(__name__)
/// ```
///
/// Use instead:
/// ```python
/// import logging
///
/// logger = logging.getLogger(__name__)
/// ```
///
/// [Logger Objects]: https://docs.python.org/3/library/logging.html#logger-objects
#[derive(ViolationMetadata)]
pub struct DirectLoggerInstantiation;

impl Violation for DirectLoggerInstantiation {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        "Use `logging.getLogger()` to instantiate loggers".to_string()
    }

    fn fix_title(&self) -> Option<String> {
        Some("Replace with `logging.getLogger()`".to_string())
    }
}