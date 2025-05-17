use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks that async functions do not call `time.sleep`.
///
/// ## Why is this bad?
/// Blocking an async function via a `time.sleep` call will block the entire
/// event loop, preventing it from executing other tasks while waiting for the
/// `time.sleep`, negating the benefits of asynchronous programming.
///
/// Instead of `time.sleep`, use `asyncio.sleep`.
///
/// ## Example
/// ```python
/// async def fetch():
///     time.sleep(1)
/// ```
///
/// Use instead:
/// ```python
/// async def fetch():
///     await asyncio.sleep(1)
/// ```
#[derive(ViolationMetadata)]
pub struct BlockingSleepInAsyncFunction;

impl Violation for BlockingSleepInAsyncFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Async functions should not call `time.sleep`".to_string()
    }
}