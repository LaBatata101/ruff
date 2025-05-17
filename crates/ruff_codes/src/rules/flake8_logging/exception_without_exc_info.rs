use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `logging.exception()` with `exc_info` set to `False`.
///
/// ## Why is this bad?
/// The `logging.exception()` method captures the exception automatically, but
/// accepts an optional `exc_info` argument to override this behavior. Setting
/// `exc_info` to `False` disables the automatic capture of the exception and
/// stack trace.
///
/// Instead of setting `exc_info` to `False`, prefer `logging.error()`, which
/// has equivalent behavior to `logging.exception()` with `exc_info` set to
/// `False`, but is clearer in intent.
///
/// ## Example
/// ```python
/// logging.exception("...", exc_info=False)
/// ```
///
/// Use instead:
/// ```python
/// logging.error("...")
/// ```
#[derive(ViolationMetadata)]
pub struct ExceptionWithoutExcInfo;

impl Violation for ExceptionWithoutExcInfo {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use of `logging.exception` with falsy `exc_info`".to_string()
    }
}