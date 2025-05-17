use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::helpers::AsyncModule;

/// ## What it does
/// Checks for uses of `trio.sleep(0)` or `anyio.sleep(0)`.
///
/// ## Why is this bad?
/// `trio.sleep(0)` is equivalent to calling `trio.lowlevel.checkpoint()`.
/// However, the latter better conveys the intent of the code.
///
/// ## Example
/// ```python
/// import trio
///
///
/// async def func():
///     await trio.sleep(0)
/// ```
///
/// Use instead:
/// ```python
/// import trio
///
///
/// async def func():
///     await trio.lowlevel.checkpoint()
/// ```
#[derive(ViolationMetadata)]
pub struct AsyncZeroSleep {
    module: AsyncModule,
}

impl AlwaysFixableViolation for AsyncZeroSleep {
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { module } = self;
        format!("Use `{module}.lowlevel.checkpoint()` instead of `{module}.sleep(0)`")
    }

    fn fix_title(&self) -> String {
        let Self { module } = self;
        format!("Replace with `{module}.lowlevel.checkpoint()`")
    }
}