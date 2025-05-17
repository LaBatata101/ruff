use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for functions declared `async` that do not await or otherwise use features requiring the
/// function to be declared `async`.
///
/// ## Why is this bad?
/// Declaring a function `async` when it's not is usually a mistake, and will artificially limit the
/// contexts where that function may be called. In some cases, labeling a function `async` is
/// semantically meaningful (e.g. with the trio library).
///
/// ## Example
/// ```python
/// async def foo():
///     bar()
/// ```
///
/// Use instead:
/// ```python
/// def foo():
///     bar()
/// ```
#[derive(ViolationMetadata)]
pub struct UnusedAsync {
    name: String,
}

impl Violation for UnusedAsync {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnusedAsync { name } = self;
        format!(
            "Function `{name}` is declared `async`, but doesn't `await` or use `async` features."
        )
    }
}