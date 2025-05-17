use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for unnecessary default type arguments for `Generator` and
/// `AsyncGenerator` on Python 3.13+.
///
/// ## Why is this bad?
/// Python 3.13 introduced the ability for type parameters to specify default
/// values. Following this change, several standard-library classes were
/// updated to add default values for some of their type parameters. For
/// example, `Generator[int]` is now equivalent to
/// `Generator[int, None, None]`, as the second and third type parameters of
/// `Generator` now default to `None`.
///
/// Omitting type arguments that match the default values can make the code
/// more concise and easier to read.
///
/// ## Example
///
/// ```python
/// from collections.abc import Generator, AsyncGenerator
///
///
/// def sync_gen() -> Generator[int, None, None]:
///     yield 42
///
///
/// async def async_gen() -> AsyncGenerator[int, None]:
///     yield 42
/// ```
///
/// Use instead:
///
/// ```python
/// from collections.abc import Generator, AsyncGenerator
///
///
/// def sync_gen() -> Generator[int]:
///     yield 42
///
///
/// async def async_gen() -> AsyncGenerator[int]:
///     yield 42
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as safe, unless the type annotation contains comments.
///
/// ## Options
/// - `target-version`
///
/// ## References
/// - [PEP 696 – Type Defaults for Type Parameters](https://peps.python.org/pep-0696/)
/// - [Annotating generators and coroutines](https://docs.python.org/3/library/typing.html#annotating-generators-and-coroutines)
/// - [Python documentation: `typing.Generator`](https://docs.python.org/3/library/typing.html#typing.Generator)
/// - [Python documentation: `typing.AsyncGenerator`](https://docs.python.org/3/library/typing.html#typing.AsyncGenerator)
#[derive(ViolationMetadata)]
pub struct UnnecessaryDefaultTypeArgs;

impl AlwaysFixableViolation for UnnecessaryDefaultTypeArgs {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Unnecessary default type arguments".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove default type arguments".to_string()
    }
}