use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for insecure `logging.config.listen` calls.
///
/// ## Why is this bad?
/// `logging.config.listen` starts a server that listens for logging
/// configuration requests. This is insecure, as parts of the configuration are
/// passed to the built-in `eval` function, which can be used to execute
/// arbitrary code.
///
/// ## Example
/// ```python
/// import logging
///
/// logging.config.listen(9999)
/// ```
///
/// ## References
/// - [Python documentation: `logging.config.listen()`](https://docs.python.org/3/library/logging.config.html#logging.config.listen)
#[derive(ViolationMetadata)]
pub struct LoggingConfigInsecureListen;

impl Violation for LoggingConfigInsecureListen {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of insecure `logging.config.listen` detected".to_string()
    }
}