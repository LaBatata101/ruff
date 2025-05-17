use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `contextlib.suppress` without arguments.
///
/// ## Why is this bad?
/// `contextlib.suppress` is a context manager that suppresses exceptions. It takes,
/// as arguments, the exceptions to suppress within the enclosed block. If no
/// exceptions are specified, then the context manager won't suppress any
/// exceptions, and is thus redundant.
///
/// Consider adding exceptions to the `contextlib.suppress` call, or removing the
/// context manager entirely.
///
/// ## Example
/// ```python
/// import contextlib
///
/// with contextlib.suppress():
///     foo()
/// ```
///
/// Use instead:
/// ```python
/// import contextlib
///
/// with contextlib.suppress(Exception):
///     foo()
/// ```
///
/// ## References
/// - [Python documentation: `contextlib.suppress`](https://docs.python.org/3/library/contextlib.html#contextlib.suppress)
#[derive(ViolationMetadata)]
pub struct UselessContextlibSuppress;

impl Violation for UselessContextlibSuppress {
    #[derive_message_formats]
    fn message(&self) -> String {
        "No arguments passed to `contextlib.suppress`. No exceptions will be suppressed and \
            therefore this context manager is redundant"
            .to_string()
    }
}