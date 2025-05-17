use ruff_diagnostics::AlwaysFixableViolation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for uses of `assert expression, print(message)`.
///
/// ## Why is this bad?
/// If an `assert x, y` assertion fails, the Python interpreter raises an
/// `AssertionError`, and the evaluated value of `y` is used as the contents of
/// that assertion error. The `print` function always returns `None`, however,
/// so the evaluated value of a call to `print` will always be `None`.
///
/// Using a `print` call in this context will therefore output the message to
/// `stdout`, before raising an empty `AssertionError(None)`. Instead, remove
/// the `print` and pass the message directly as the second expression,
/// allowing `stderr` to capture the message in a well-formatted context.
///
/// ## Example
/// ```python
/// assert False, print("This is a message")
/// ```
///
/// Use instead:
/// ```python
/// assert False, "This is a message"
/// ```
///
/// ## Fix safety
/// This rule's fix is marked as unsafe, as changing the second expression
/// will result in a different `AssertionError` message being raised, as well as
/// a change in `stdout` output.
///
/// ## References
/// - [Python documentation: `assert`](https://docs.python.org/3/reference/simple_stmts.html#the-assert-statement)
#[derive(ViolationMetadata)]
pub struct AssertWithPrintMessage;

impl AlwaysFixableViolation for AssertWithPrintMessage {
    #[derive_message_formats]
    fn message(&self) -> String {
        "`print()` call in `assert` statement is likely unintentional".to_string()
    }

    fn fix_title(&self) -> String {
        "Remove `print`".to_owned()
    }
}