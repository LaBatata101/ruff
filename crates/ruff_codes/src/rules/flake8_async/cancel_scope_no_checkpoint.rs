use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::helpers::MethodName;

/// ## What it does
/// Checks for timeout context managers which do not contain a checkpoint.
///
/// For the purposes of this check, `yield` is considered a checkpoint,
/// since checkpoints may occur in the caller to which we yield.
///
/// ## Why is this bad?
/// Some asynchronous context managers, such as `asyncio.timeout` and
/// `trio.move_on_after`, have no effect unless they contain a checkpoint.
/// The use of such context managers without an `await`, `async with` or
/// `async for` statement is likely a mistake.
///
/// ## Example
/// ```python
/// async def func():
///     async with asyncio.timeout(2):
///         do_something()
/// ```
///
/// Use instead:
/// ```python
/// async def func():
///     async with asyncio.timeout(2):
///         do_something()
///         await awaitable()
/// ```
///
/// ## References
/// - [`asyncio` timeouts](https://docs.python.org/3/library/asyncio-task.html#timeouts)
/// - [`anyio` timeouts](https://anyio.readthedocs.io/en/stable/cancellation.html)
/// - [`trio` timeouts](https://trio.readthedocs.io/en/stable/reference-core.html#cancellation-and-timeouts)
#[derive(ViolationMetadata)]
pub struct CancelScopeNoCheckpoint {
    method_name: MethodName,
}

impl Violation for CancelScopeNoCheckpoint {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { method_name } = self;
        format!("A `with {method_name}(...):` context does not contain any `await` statements. This makes it pointless, as the timeout can only be triggered by a checkpoint.")
    }
}