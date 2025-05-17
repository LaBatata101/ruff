use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for usages of the following `logging` top-level functions:
/// `debug`, `info`, `warn`, `warning`, `error`, `critical`, `log`, `exception`.
///
/// ## Why is this bad?
/// Using the root logger causes the messages to have no source information,
/// making them less useful for debugging.
///
/// ## Example
/// ```python
/// import logging
///
/// logging.info("Foobar")
/// ```
///
/// Use instead:
/// ```python
/// import logging
///
/// logger = logging.getLogger(__name__)
/// logger.info("Foobar")
/// ```
#[derive(ViolationMetadata)]
pub struct RootLoggerCall {
    attr: String,
}

impl Violation for RootLoggerCall {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("`{}()` call on root logger", self.attr)
    }

    fn fix_title(&self) -> Option<String> {
        Some("Use own logger instead".to_string())
    }
}