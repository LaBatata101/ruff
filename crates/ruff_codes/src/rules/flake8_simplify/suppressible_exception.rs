use ruff_diagnostics::{FixAvailability, Violation};
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `try`-`except`-`pass` blocks that can be replaced with the
/// `contextlib.suppress` context manager.
///
/// ## Why is this bad?
/// Using `contextlib.suppress` is more concise and directly communicates the
/// intent of the code: to suppress a given exception.
///
/// Note that `contextlib.suppress` is slower than using `try`-`except`-`pass`
/// directly. For performance-critical code, consider retaining the
/// `try`-`except`-`pass` pattern.
///
/// ## Example
/// ```python
/// try:
///     1 / 0
/// except ZeroDivisionError:
///     pass
/// ```
///
/// Use instead:
/// ```python
/// import contextlib
///
/// with contextlib.suppress(ZeroDivisionError):
///     1 / 0
/// ```
///
/// ## References
/// - [Python documentation: `contextlib.suppress`](https://docs.python.org/3/library/contextlib.html#contextlib.suppress)
/// - [Python documentation: `try` statement](https://docs.python.org/3/reference/compound_stmts.html#the-try-statement)
/// - [a simpler `try`/`except` (and why maybe shouldn't)](https://www.youtube.com/watch?v=MZAJ8qnC7mk)
#[derive(ViolationMetadata)]
pub struct SuppressibleException {
    exception: String,
}

impl Violation for SuppressibleException {
    const FIX_AVAILABILITY: FixAvailability = FixAvailability::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let SuppressibleException { exception } = self;
        format!("Use `contextlib.suppress({exception})` instead of `try`-`except`-`pass`")
    }

    fn fix_title(&self) -> Option<String> {
        let SuppressibleException { exception } = self;
        Some(format!("Replace with `contextlib.suppress({exception})`"))
    }
}