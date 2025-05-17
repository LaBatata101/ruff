use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::helpers::AsyncModule;

/// ## What it does
/// Checks for uses of `trio.sleep()` or `anyio.sleep()` with a delay greater than 24 hours.
///
/// ## Why is this bad?
/// Calling `sleep()` with a delay greater than 24 hours is usually intended
/// to sleep indefinitely. Instead of using a large delay,
/// `trio.sleep_forever()` or `anyio.sleep_forever()` better conveys the intent.
///
///
/// ## Example
/// ```python
/// import trio
///
///
/// async def func():
///     await trio.sleep(86401)
/// ```
///
/// Use instead:
/// ```python
/// import trio
///
///
/// async def func():
///     await trio.sleep_forever()
/// ```
///
/// ## Fix safety
///
/// This fix is marked as unsafe as it changes program behavior.
#[derive(ViolationMetadata)]
pub struct LongSleepNotForever {
    module: AsyncModule,
}

impl Violation for LongSleepNotForever {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;
    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { module } = self;
        format!(
            "`{module}.sleep()` with >24 hour interval should usually be `{module}.sleep_forever()`"
        )
    }

    fn fix_title(&self) -> Option<String> {
        let Self { module } = self;
        Some(format!("Replace with `{module}.sleep_forever()`"))
    }
}