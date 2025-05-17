use ruff_diagnostics::Violation;
use ruff_macros::{derive_message_formats, ViolationMetadata};

/// ## What it does
/// Checks for cases where files are opened (e.g., using the builtin `open()` function)
/// without using a context manager.
///
/// ## Why is this bad?
/// If a file is opened without a context manager, it is not guaranteed that
/// the file will be closed (e.g., if an exception is raised), which can cause
/// resource leaks. The rule detects a wide array of IO calls where context managers
/// could be used, such as `open`, `pathlib.Path(...).open()`, `tempfile.TemporaryFile()`
/// or`tarfile.TarFile(...).gzopen()`.
///
/// ## Example
/// ```python
/// file = open("foo.txt")
/// ...
/// file.close()
/// ```
///
/// Use instead:
/// ```python
/// with open("foo.txt") as file:
///     ...
/// ```
///
/// ## References
/// - [Python documentation: `open`](https://docs.python.org/3/library/functions.html#open)
#[derive(ViolationMetadata)]
pub struct OpenFileWithContextHandler;

impl Violation for OpenFileWithContextHandler {
    #[derive_message_formats]
    fn message(&self) -> String {
        "Use a context manager for opening files".to_string()
    }
}