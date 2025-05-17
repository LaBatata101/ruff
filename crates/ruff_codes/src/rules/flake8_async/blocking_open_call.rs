use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks that async functions do not open files with blocking methods like `open`.
///
/// ## Why is this bad?
/// Blocking an async function via a blocking call will block the entire
/// event loop, preventing it from executing other tasks while waiting for the
/// call to complete, negating the benefits of asynchronous programming.
///
/// Instead of making a blocking call, use an equivalent asynchronous library
/// or function.
///
/// ## Example
/// ```python
/// async def foo():
///     with open("bar.txt") as f:
///         contents = f.read()
/// ```
///
/// Use instead:
/// ```python
/// import anyio
///
///
/// async def foo():
///     async with await anyio.open_file("bar.txt") as f:
///         contents = await f.read()
/// ```
#[derive(ViolationMetadata)]
pub struct BlockingOpenCallInAsyncFunction;

impl Violation for BlockingOpenCallInAsyncFunction {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Async functions should not open files with blocking methods like `open`".to_string()
    }
}