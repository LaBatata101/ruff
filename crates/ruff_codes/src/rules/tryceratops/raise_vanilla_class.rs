use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for code that raises `Exception` or `BaseException` directly.
///
/// ## Why is this bad?
/// Handling such exceptions requires the use of `except Exception` or
/// `except BaseException`. These will capture almost _any_ raised exception,
/// including failed assertions, division by zero, and more.
///
/// Prefer to raise your own exception, or a more specific built-in
/// exception, so that you can avoid over-capturing exceptions that you
/// don't intend to handle.
///
/// ## Example
/// ```python
/// def main_function():
///     if not cond:
///         raise Exception()
///
///
/// def consumer_func():
///     try:
///         do_step()
///         prepare()
///         main_function()
///     except Exception:
///         logger.error("Oops")
/// ```
///
/// Use instead:
/// ```python
/// def main_function():
///     if not cond:
///         raise CustomException()
///
///
/// def consumer_func():
///     try:
///         do_step()
///         prepare()
///         main_function()
///     except CustomException:
///         logger.error("Main function failed")
///     except Exception:
///         logger.error("Oops")
/// ```
#[derive(ViolationMetadata)]
pub struct RaiseVanillaClass;

impl Violation for RaiseVanillaClass {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Create your own exception".to_string()
    }
}