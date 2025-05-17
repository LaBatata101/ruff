use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::helpers::AsyncModule;

/// ## What it does
/// Checks for the use of an async sleep function in a `while` loop.
///
/// ## Why is this bad?
/// Instead of sleeping in a `while` loop, and waiting for a condition
/// to become true, it's preferable to `await` on an `Event` object such
/// as: `asyncio.Event`, `trio.Event`, or `anyio.Event`.
///
/// ## Example
/// ```python
/// DONE = False
///
///
/// async def func():
///     while not DONE:
///         await asyncio.sleep(1)
/// ```
///
/// Use instead:
/// ```python
/// DONE = asyncio.Event()
///
///
/// async def func():
///     await DONE.wait()
/// ```
///
/// ## References
/// - [`asyncio` events](https://docs.python.org/3/library/asyncio-sync.html#asyncio.Event)
/// - [`anyio` events](https://anyio.readthedocs.io/en/latest/api.html#anyio.Event)
/// - [`trio` events](https://trio.readthedocs.io/en/latest/reference-core.html#trio.Event)
#[derive(ViolationMetadata)]
pub struct AsyncBusyWait {
    module: AsyncModule,
}

impl Violation for AsyncBusyWait {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { module } = self;
        format!("Use `{module}.Event` instead of awaiting `{module}.sleep` in a `while` loop")
    }
}