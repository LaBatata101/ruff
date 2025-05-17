use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for `except` clauses that catch all exceptions.  This includes
/// bare `except`, `except BaseException` and `except Exception`.
///
///
/// ## Why is this bad?
/// Overly broad `except` clauses can lead to unexpected behavior, such as
/// catching `KeyboardInterrupt` or `SystemExit` exceptions that prevent the
/// user from exiting the program.
///
/// Instead of catching all exceptions, catch only those that are expected to
/// be raised in the `try` block.
///
/// ## Example
/// ```python
/// try:
///     foo()
/// except BaseException:
///     ...
/// ```
///
/// Use instead:
/// ```python
/// try:
///     foo()
/// except FileNotFoundError:
///     ...
/// ```
///
/// Exceptions that are re-raised will _not_ be flagged, as they're expected to
/// be caught elsewhere:
/// ```python
/// try:
///     foo()
/// except BaseException:
///     raise
/// ```
///
/// Exceptions that are logged via `logging.exception()` or `logging.error()`
/// with `exc_info` enabled will _not_ be flagged, as this is a common pattern
/// for propagating exception traces:
/// ```python
/// try:
///     foo()
/// except BaseException:
///     logging.exception("Something went wrong")
/// ```
///
/// ## References
/// - [Python documentation: The `try` statement](https://docs.python.org/3/reference/compound_stmts.html#the-try-statement)
/// - [Python documentation: Exception hierarchy](https://docs.python.org/3/library/exceptions.html#exception-hierarchy)
/// - [PEP 8: Programming Recommendations on bare `except`](https://peps.python.org/pep-0008/#programming-recommendations)
#[derive(ViolationMetadata)]
pub struct BlindExcept {
    name: String,
}

impl Violation for BlindExcept {
    #[derive_message_formats]
    fn message(&self) -> String {
        let BlindExcept { name } = self;
        format!("Do not catch blind exception: `{name}`")
    }
}