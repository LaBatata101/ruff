use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

use super::helpers::MethodName;

/// ## What it does
/// Checks for calls to trio functions that are not immediately awaited.
///
/// ## Why is this bad?
/// Many of the functions exposed by trio are asynchronous, and must be awaited
/// to take effect. Calling a trio function without an `await` can lead to
/// `RuntimeWarning` diagnostics and unexpected behaviour.
///
/// ## Example
/// ```python
/// async def double_sleep(x):
///     trio.sleep(2 * x)
/// ```
///
/// Use instead:
/// ```python
/// async def double_sleep(x):
///     await trio.sleep(2 * x)
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as adding an `await` to a function
/// call changes its semantics and runtime behavior.
#[derive(ViolationMetadata)]
pub struct TrioSyncCall {
    method_name: MethodName,
}

impl Violation for TrioSyncCall {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let Self { method_name } = self;
        format!("Call to `{method_name}` is not immediately awaited")
    }

    fn fix_title(&self) -> Option<String> {
        Some("Add `await`".to_string())
    }
}