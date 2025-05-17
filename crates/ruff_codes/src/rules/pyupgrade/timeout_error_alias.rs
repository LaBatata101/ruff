use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of exceptions that alias `TimeoutError`.
///
/// ## Why is this bad?
/// `TimeoutError` is the builtin error type used for exceptions when a system
/// function timed out at the system level.
///
/// In Python 3.10, `socket.timeout` was aliased to `TimeoutError`. In Python
/// 3.11, `asyncio.TimeoutError` was aliased to `TimeoutError`.
///
/// These aliases remain in place for compatibility with older versions of
/// Python, but may be removed in future versions.
///
/// Prefer using `TimeoutError` directly, as it is more idiomatic and future-proof.
///
/// ## Example
/// ```python
/// raise asyncio.TimeoutError
/// ```
///
/// Use instead:
/// ```python
/// raise TimeoutError
/// ```
///
/// ## References
/// - [Python documentation: `TimeoutError`](https://docs.python.org/3/library/exceptions.html#TimeoutError)
#[derive(ViolationMetadata)]
pub struct TimeoutErrorAlias {
    name: Option<String>,
}

impl AlwaysFixableViolation for TimeoutErrorAlias {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Replace aliased errors with `TimeoutError`".to_string()
    }

    fn fix_title(&self) -> String {
        let TimeoutErrorAlias { name } = self;
        match name {
            None => "Replace with builtin `TimeoutError`".to_string(),
            Some(name) => format!("Replace `{name}` with builtin `TimeoutError`"),
        }
    }
}